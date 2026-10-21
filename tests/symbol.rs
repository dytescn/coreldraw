//! `IvgSymbol*` 接口测试
//!
//! CorelDRAW 没有在所有版本都暴露 `SymbolLibraries` 属性，因此这里先探测；
//! 不可用时打印 skip，而不是 fail。

mod common;

use cdrsdk::symbol::IvgSymbolLibraries;
use common::ComGuard;

#[test]
fn symbol_libraries_access() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");

    // 先试 Application.SymbolLibraries，再试 Document.SymbolLibraries。
    let libs_disp = app
        .raw()
        .get_property("SymbolLibraries")
        .ok()
        .and_then(|v| v.to_idispatch().ok())
        .or_else(|| {
            doc.raw()
                .get_property("SymbolLibraries")
                .ok()
                .and_then(|v| v.to_idispatch().ok())
        });

    let Some(disp) = libs_disp else {
        eprintln!("skip: SymbolLibraries not exposed by this CorelDRAW version");
        let _ = doc.close_without_saving();
        return;
    };

    let libs = IvgSymbolLibraries::new(disp);
    let count = libs.count().unwrap_or(0);
    println!("symbol libraries = {count}");
    for lib in libs.all().iter().take(3) {
        println!(
            "  library name={:?} symbols={:?}",
            lib.name(),
            lib.symbols().and_then(|s| s.count())
        );
        assert!(lib.name().is_some());
    }

    let _ = doc.close_without_saving();
}
