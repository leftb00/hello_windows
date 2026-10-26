#![forbid(unsafe_code)]
#![windows_subsystem = "windows"]

use winsafe::{co, gui, prelude::*, AnyResult};

fn main() -> AnyResult<()> {
    let window = gui::WindowMain::new(gui::WindowMainOpts {
        title: "Rust Windows 테스트",
        size: gui::dpi(800, 600),
        style: gui::WindowMainOpts::default().style
            | co::WS::SIZEBOX
            | co::WS::MINIMIZEBOX
            | co::WS::MAXIMIZEBOX,
        ..Default::default()
    });

    let paint_window = window.clone();
    window.on().wm_paint(move || {
        let hdc = paint_window.hwnd().BeginPaint()?;
        let mut bounds = paint_window.hwnd().GetClientRect()?;
        hdc.SetBkMode(co::BKMODE::TRANSPARENT)?;
        hdc.DrawText(
            "hello windows",
            &mut bounds,
            co::DT::CENTER | co::DT::VCENTER | co::DT::SINGLELINE,
        )?;
        Ok(())
    });

    let resize_window = window.clone();
    window.on().wm_size(move |event| {
        if event.request != co::SIZE_R::MINIMIZED {
            resize_window.hwnd().InvalidateRect(None, true)?;
        }
        Ok(())
    });

    window.run_main(None)?;
    Ok(())
}
