use gpui::{px, size, App, Bounds, DevicePixels, Point, WindowBounds, WindowOptions};

use views::root_view::RootView;
mod components;
mod views;

fn init_window_options() -> WindowOptions {
    WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(Bounds {
            origin: Point::new(DevicePixels::from(0), DevicePixels::from(0)),
            size: size(px(1632.), px(940.)).into(),
        })),

        ..Default::default()
    }
}

fn main() {
    println!("===starting app===");

    // let app = App::new();
    let app = App::new();
    app.run(move |cx| {
        let options = init_window_options();

        cx.open_window(options, |cx| RootView::new(cx));
    });
}
