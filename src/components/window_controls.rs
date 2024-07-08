use gpui::{div, prelude::*, px, ElementId, Pixels, Rgba, WindowContext};

#[derive(IntoElement)]
pub struct WindowControls {
    button_height: Pixels,
}

impl WindowControls {
    pub fn new(button_height: Pixels) -> Self {
        Self { button_height }
    }
}

impl RenderOnce for WindowControls {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id("window-controls")
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(TitlebarButton::new(
                "minimize",
                TitlebarButtonType::Minimize,
            ))
            .child(TitlebarButton::new(
                "maximize-or-restore",
                if cx.is_maximized() {
                    TitlebarButtonType::Restore
                } else {
                    TitlebarButtonType::Maximize
                },
            ))
            .child(TitlebarButton::new("close", TitlebarButtonType::Close))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum TitlebarButtonType {
    Minimize,
    Restore,
    Maximize,
    Close,
}

#[derive(IntoElement)]
struct TitlebarButton {
    id: ElementId,
    icon: TitlebarButtonType,
    // background_color: Rgba,
    // hover_background_color: Rgba,
    // close_window_action: Box<dyn Action>,
}

impl TitlebarButton {
    const CLOSE_BUTTON_COLOR: Rgba = Rgba {
        r: 204.0 / 255.0,
        g: 0.0 / 255.0,
        b: 0.0 / 255.0,
        a: 1.0,
    };
    const MIN_BUTTON_COLOR: Rgba = Rgba {
        r: 0.0 / 255.0,
        g: 204.0 / 255.0,
        b: 0.0 / 255.0,
        a: 1.0,
    };
    const MAX_BUTTON_COLOR: Rgba = Rgba {
        r: 204.0 / 255.0,
        g: 204.0 / 255.0,
        b: 0.0 / 255.0,
        a: 1.0,
    };

    pub fn new(
        id: impl Into<ElementId>,
        icon: TitlebarButtonType,
        // close_window_action: Box<dyn Action>,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
        }
    }

    fn btn_color(&self) -> Rgba {
        match self.icon {
            TitlebarButtonType::Minimize => Self::MIN_BUTTON_COLOR,
            TitlebarButtonType::Restore => Self::MAX_BUTTON_COLOR,
            TitlebarButtonType::Maximize => Self::MAX_BUTTON_COLOR,
            TitlebarButtonType::Close => Self::CLOSE_BUTTON_COLOR,
        }
    }
}
impl RenderOnce for TitlebarButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let width = px(36.);
        div()
            .id(self.id.clone())
            .justify_center()
            .content_center()
            .w(width)
            .h_full()
            .bg(self.btn_color())
            .hover(|style| {
                let mut color = self.btn_color();
                color.r *= 0.8;
                color.g *= 0.8;
                color.b *= 0.8;
                color.a *= 0.8;
                style.bg(color)
            })
            .active(|style| {
                let mut active_color = self.btn_color();
                active_color.a *= 0.2;

                style.bg(active_color)
            })
            .on_mouse_move(|_, cx| cx.stop_propagation())
            .on_click(move |_, cx| {
                cx.stop_propagation();

                match self.icon {
                    TitlebarButtonType::Minimize => cx.minimize_window(),
                    TitlebarButtonType::Restore => cx.zoom_window(),
                    TitlebarButtonType::Maximize => cx.zoom_window(),
                    TitlebarButtonType::Close => cx.quit(),
                }
            })
    }
}
