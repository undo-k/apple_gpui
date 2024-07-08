use super::window_controls::WindowControls;
use gpui::prelude::*;

use gpui::SharedString;
use gpui::{div, px, IntoElement, Pixels, Rgba, WindowContext};

#[derive(IntoElement, Clone)]
pub struct TitleBar {
    title: SharedString,
}

impl TitleBar {
    const TITLE: &'static str = "apple gpui baybeeee";
    const BG_COLOR: Rgba = Rgba {
        r: 32.0 / 255.0,
        g: 32.0 / 255.0,
        b: 32.0 / 255.0,
        a: 0.5,
    };

    fn height(cx: &mut WindowContext) -> Pixels {
        (1.75 * cx.rem_size()).max(px(34.))
    }
    fn top_padding(_cx: &WindowContext) -> Pixels {
        px(0.)
    }
    pub fn new() -> Self {
        Self {
            title: TitleBar::TITLE.into(),
        }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let height = Self::height(cx);
        let top_padding = Self::top_padding(cx);
        div()
            .id("titlebar")
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .pt(top_padding)
            .h(height + top_padding)
            .bg(Self::BG_COLOR)
            .text_color(gpui::white())
            .content_stretch()
            .child(
                div()
                    .child(self.title)
                    .flex()
                    .flex_row()
                    .justify_center()
                    .w_full(),
            )
            .child(WindowControls::new(height))
            .on_mouse_down(gpui::MouseButton::Right, move |ev, cx| {
                cx.show_window_menu(ev.position)
            })
            .on_mouse_move(move |ev, cx| {
                if ev.dragging() {
                    cx.start_system_move();
                }
            })
    }
}
