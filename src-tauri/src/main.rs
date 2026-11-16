use gpui::{
    App, AppContext as _, Bounds, TitlebarOptions, WindowBackgroundAppearance, WindowBounds,
    WindowOptions, px, size,
};
use gpui_platform::application;

fn main() {
    application().run(|cx: &mut App| {
        set_dock_icon();
        let bounds = Bounds::centered(None, size(px(1080.), px(740.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Cristal".into()),
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_min_size: Some(size(px(920.), px(640.))),
                window_background: WindowBackgroundAppearance::Transparent,
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

/// El binario suelto no lee `CFBundleIconFile`. El `.app` sí usa el `.icns`.
#[cfg(target_os = "macos")]
fn set_dock_icon() {
    use objc2::{AnyThread, MainThreadMarker};
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::NSData;

    let Some(marker) = MainThreadMarker::new() else {
        return;
    };
    let data = NSData::with_bytes(include_bytes!("../icons/icon.png"));
    let Some(image) = NSImage::initWithData(NSImage::alloc(), &data) else {
        eprintln!("no se pudo leer el icono de Cristal");
        return;
    };
    // La imagen ya decodificó. `None` no se pasa: el setter no lo acepta.
    unsafe {
        NSApplication::sharedApplication(marker).setApplicationIconImage(Some(&image));
    }
}

#[cfg(not(target_os = "macos"))]
fn set_dock_icon() {}
