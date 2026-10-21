//! `ui`（ICUI*）模块接口测试
//!
//! CUI 框架只在 CorelDRAW 插件宿主中可用；先探测 `Application.Framework`，
//! 不可用则打印 `skip:`。

mod common;

use cdrsdk::prelude::*;
use cdrsdk::ui::{ICuiApplication, ICuiFrameWork};
use common::ComGuard;

#[test]
fn ui_framework_probe() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    // 不同版本暴露的属性名可能不同，逐个探测。
    let disp = ["Framework", "UIApplication", "UI"]
        .iter()
        .find_map(|name| {
            app.raw()
                .get_property(name)
                .ok()
                .and_then(|v| v.to_idispatch().ok())
        });

    let Some(disp) = disp else {
        eprintln!("skip: UI framework not exposed by this CorelDRAW version");
        return;
    };

    // 该对象可能是 ICUIApplication，也可能直接是 ICUIFrameWork。
    let ui = ICuiApplication::new(disp.clone());
    let framework = ui.framework().unwrap_or_else(|| ICuiFrameWork::new(disp.clone()));

    // ICUIFrameWork
    if let Some(bars) = framework.command_bars() {
        println!("command bars = {:?}", bars.count());
        assert!(bars.count().is_some());
    }
    if let Some(menu) = framework.main_menu() {
        println!("main menu = {:?}", menu.name());
        assert!(menu.name().is_some() || menu.name_local().is_some());
        assert!(menu.bar_type().is_some());
    }
    if let Some(status) = framework.status_bar() {
        assert!(status.name().is_some() || status.bar_type().is_some());
    }
    if let Some(windows) = framework.frame_windows() {
        println!("frame windows = {:?}", windows.count());
        if let Some(first) = windows.first() {
            assert!(first.caption().is_some() || first.id().is_some());
            assert!(first.is_main_frame().is_some());
        }
    }
    if let Some(main) = framework.main_frame_window() {
        assert!(main.caption().is_some() || main.id().is_some());
        assert!(main.handle().is_some());
        let _ = main.position();
        let _ = main.view_hosts();
    }
    assert!(framework.task_manager().is_some());

    // ICUIApplication
    if let Some(data) = ui.data_context() {
        assert!(data.categories().is_some() || data.has_category("cdrsdk").is_some());
    }
    let _ = framework.application();
}
