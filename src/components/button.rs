use gpui::{div, prelude::*, px, ClickEvent, ElementId, Rgba, SharedString, WindowContext};

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: SharedString,
    on_click: Box<dyn Fn(&ClickEvent, &mut WindowContext)>,
}

impl Button {
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        on_click: impl Fn(&ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .id(self.id)
            .w(px(192.))
            .h(px(64.))
            .rounded_lg()
            .p_2()
            .bg(Rgba {
                r: 225. / 255.0,
                g: 150. / 255.0,
                b: 0. / 255.0,
                a: 1.,
            })
            .text_color(Rgba {
                r: 255. / 255.0,
                g: 255. / 255.0,
                b: 255. / 255.0,
                a: 1.,
            })
            .hover(|style| {
                style.bg(Rgba {
                    r: 235. / 255.0,
                    g: 160. / 255.0,
                    b: 0. / 255.0,
                    a: 1.,
                })
            })
            .active(|style| {
                style.bg(Rgba {
                    r: 255. / 255.0,
                    g: 180. / 255.0,
                    b: 0. / 255.0,
                    a: 1.,
                })
            })
            .child(self.label.clone())
            .flex()
            .flex_row()
            .items_center()
            .on_click(self.on_click)
    }
}
