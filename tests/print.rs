//! `print` 模块接口测试
//!
//! 只读取 / 回写打印与 PDF 设置，不触发真实打印。

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn print_settings_read_and_restore() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");

    let job = match doc.print_settings() {
        Some(job) => job,
        None => {
            eprintln!("skip: Document.PrintSettings unavailable");
            let _ = doc.close_without_saving();
            return;
        }
    };

    let settings = job.settings().expect("print job.settings()");
    assert!(settings.copies().is_some());
    assert!(settings.collate().is_some());
    assert!(settings.paper_orientation().is_some());
    assert!(settings.paper_size().is_some());
    assert!(settings.print_range().is_some());
    assert!(settings.print_to_file().is_some());

    if let Some(printer) = settings.printer() {
        println!("printer = {:?}", printer.name());
        assert!(printer.name().is_some());
        assert!(printer.is_default().is_some());
        assert!(printer.ready().is_some());
        assert!(printer.port().is_some());
    } else {
        eprintln!("skip: no printer installed");
    }

    // 回写原值，避免改变用户的打印设置。
    let copies = settings.copies().unwrap_or(1) as i32;
    assert!(settings.set_copies(copies));
    let collate = settings.collate().unwrap_or(false);
    assert!(settings.set_collate(collate));
    let to_file = settings.print_to_file().unwrap_or(false);
    assert!(settings.set_print_to_file(to_file));

    // 打印子对象：存在则读取一个属性。
    if let Some(prepress) = settings.prepress() {
        let _ = prepress.invert();
    }
    if let Some(postscript) = settings.postscript() {
        let _ = postscript.level();
    }
    if let Some(trapping) = settings.trapping() {
        let _ = trapping.width();
    }
    if let Some(options) = settings.options() {
        let _ = options.print_vectors();
    }
    if let Some(layout) = settings.layout() {
        let _ = layout.placement();
    }

    // 任务集合
    let _ = job.pages();
    let _ = job.documents();

    let _ = doc.close_without_saving();
}

#[test]
fn pdf_settings_read_and_restore() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let pdf = match doc.pdf_settings() {
        Some(pdf) => pdf,
        None => {
            eprintln!("skip: Document.PDFSettings unavailable");
            let _ = doc.close_without_saving();
            return;
        }
    };

    let _ = pdf.publish_range();
    let _ = pdf.page_range();
    let _ = pdf.author();
    let _ = pdf.subject();
    let _ = pdf.keywords();
    let _ = pdf.bitmap_compression();
    let _ = pdf.jpeg_quality_factor();
    let _ = pdf.text_as_curves();
    let _ = pdf.embed_fonts();
    let _ = pdf.encoding();

    // 回写无副作用的文本元数据。
    let author = pdf.author().unwrap_or_default();
    assert!(pdf.set_author(author));
    let subject = pdf.subject().unwrap_or_default();
    assert!(pdf.set_subject(subject));

    let _ = doc.close_without_saving();
}
