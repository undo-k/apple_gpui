use crate::components::button::Button;
use crate::components::title_bar::TitleBar;
use data_url::DataUrl;
use deno_ast::{MediaType, ParseParams};
use deno_core::{
    error::AnyError, futures::FutureExt, ModuleLoadResponse, ModuleLoader, ModuleSource,
    ModuleSourceCode, ModuleSpecifier, ModuleType, PollEventLoopOptions, RequestedModuleType,
};
use gpui::{div, prelude::*, rgb, Length, Pixels, View, ViewContext, WindowContext};
use std::{env::current_dir, path::Path, rc::Rc};

pub struct RootView {
    title_bar: TitleBar,
}

impl RootView {
    pub fn new(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_| Self {
            title_bar: TitleBar::new(),
        })
    }
}

impl Render for RootView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div().child(self.title_bar.clone()).child(
            div()
                .flex()
                .flex_col()
                .w_full()
                .size_full()
                .text_xl()
                .text_color(rgb(0x00ff00))
                .size(Length::Definite(Pixels(300.0).into()))
                .child(Button::new(
                    "runjs",
                    "runjs",
                    cx.listener(move |_, _, _| {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap();

                        if let Err(error) = rt
                            .block_on(run_js("./src/example.js", current_dir().unwrap().as_path()))
                        {
                            panic!("error: {error}");
                        }
                    }),
                )),
        )
    }
}

async fn run_js(file_path: &str, current_dir: &Path) -> Result<(), AnyError> {
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        // initialize runtime with SimpleModuleLoader, capable of loading remote modules
        module_loader: Some(Rc::new(SimpleModuleLoader)),
        ..Default::default()
    });

    let main_mod_specifier = deno_core::resolve_path(file_path, current_dir)?;
    let main_mod_id = js_runtime.load_main_es_module(&main_mod_specifier).await?;
    let main_result = js_runtime.mod_evaluate(main_mod_id);

    js_runtime
        .run_event_loop(PollEventLoopOptions {
            ..Default::default()
        })
        .await?;

    main_result.await
}

/// incorporates remote loading from [here](https://github.com/andreubotella/deno-simple-module-loader/blob/main/src/lib.rs)
/// and typescript [here](https://deno.com/blog/roll-your-own-javascript-runtime-pt2)
pub struct SimpleModuleLoader;

impl ModuleLoader for SimpleModuleLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _kind: deno_core::ResolutionKind,
    ) -> Result<ModuleSpecifier, AnyError> {
        Ok(deno_core::resolve_import(specifier, referrer)?)
    }

    fn load(
        &self,
        module_specifier: &ModuleSpecifier,
        _maybe_referrer: Option<&ModuleSpecifier>,
        _is_dyn_import: bool,
        requested_module_type: RequestedModuleType,
    ) -> deno_core::ModuleLoadResponse {
        let module_specifier = module_specifier.clone();

        let should_transpile = {
            // this is an afront to god
            // todo: stop the blaspheme
            let str = module_specifier.clone().to_string();
            let ext = &str[module_specifier
                .clone()
                .to_string()
                .char_indices()
                .nth_back(2)
                .unwrap()
                .0..];
            if ext == ".ts" {
                true
            } else {
                false
            }
        };

        // get file from local or remote
        ModuleLoadResponse::Async(
            async move {
                let mut redirect_module_url = None;
                let bytes = match module_specifier.scheme() {
                    "http" | "https" => {
                        let res = reqwest::get(module_specifier.clone()).await?;

                        let res = res.error_for_status()?;

                        if res.url() != &module_specifier {
                            redirect_module_url = Some(res.url().clone());
                        }
                        res.bytes().await?.to_vec()
                    }
                    "file" => {
                        let path = match module_specifier.to_file_path() {
                            Ok(path) => path,
                            Err(_) => panic!("Invalid file URL."),
                        };
                        tokio::fs::read(path).await?
                    }
                    "data" => {
                        let url = match DataUrl::process(module_specifier.as_str()) {
                            Ok(url) => url,
                            Err(_) => panic!("Not a valid data URL."),
                        };
                        match url.decode_to_vec() {
                            Ok((bytes, _)) => bytes,
                            Err(_) => panic!("Not a valid data URL."),
                        }
                    }
                    schema => panic!("Invalid schema {}", schema),
                };

                let module_type = match requested_module_type {
                    RequestedModuleType::None => ModuleType::JavaScript,
                    RequestedModuleType::Json => ModuleType::Json,
                    RequestedModuleType::Other(_) => {
                        unreachable!("Import types other than JSON are not supported")
                    }
                };

                let bytes = if should_transpile {
                    let parsed = deno_ast::parse_module(ParseParams {
                        specifier: module_specifier.clone(),
                        text: std::str::from_utf8(&bytes)?.into(),
                        media_type: MediaType::TypeScript,
                        capture_tokens: false,
                        scope_analysis: false,
                        maybe_syntax: None,
                    })?;
                    parsed
                        .transpile(&Default::default(), &Default::default())?
                        .into_source()
                        .source
                } else {
                    bytes
                };

                if let Some(redirect_module_url) = redirect_module_url {
                    Ok(ModuleSource::new_with_redirect(
                        module_type,
                        ModuleSourceCode::Bytes(bytes.into_boxed_slice().into()),
                        &module_specifier,
                        &redirect_module_url,
                        None,
                    ))
                } else {
                    Ok(ModuleSource::new(
                        module_type,
                        ModuleSourceCode::Bytes(bytes.into_boxed_slice().into()),
                        &module_specifier,
                        None,
                    ))
                }
            }
            .boxed_local(),
        )
    }
}
