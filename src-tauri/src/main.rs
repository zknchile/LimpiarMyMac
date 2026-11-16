use gpui::{
    App, AppContext as _, Bounds, TitlebarOptions, WindowBounds, WindowOptions, px, size,
};
use gpui_platform::application;

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1080.), px(740.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Cristal".into()),
                    ..Default::default()
                }),
                window_min_size: Some(size(px(920.), px(640.))),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title("Cristal");
                cx.new(|_| cristal_lib::view::Cristal::new())
            },
        )
        .expect("no se pudo abrir Cristal");
        cx.activate(true);
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
    });
}
