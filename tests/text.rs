//! `IvgText` 接口测试

mod common;

use cdrsdk::prelude::*;
use common::ComGuard;

#[test]
fn text_create_edit_and_query() {
    let _serial = common::serial();
    let _com = ComGuard::init().expect("CoInitialize failed");
    let Some(app) = common::bind_app() else {
        eprintln!("skip: CorelDRAW not running");
        return;
    };

    let doc = app.create_document().expect("create_document");
    let layer = doc.active_layer().expect("active_layer");

    let shape = layer
        .create_artistic_text(
            20.0,
            40.0,
            "Hello cdrsdk",
            0,
            0,
            "Arial",
            24.0,
            0,
            0,
            0,
            cdrAlignment::LeftAlignment as i32,
        )
        .expect("create_artistic_text");

    let text = shape.text().expect("shape.text()");
    assert_eq!(text.text_type(), Some(cdrTextType::Artistic as i64));

    let story = text.story().expect("text.story()");
    let contents = story.text().unwrap_or_default();
    assert!(contents.contains("Hello cdrsdk"), "contents = {contents:?}");

    assert!(story.set_text("Hello cdrsdk updated"));
    let contents = story.text().unwrap_or_default();
    assert!(contents.contains("updated"), "contents = {contents:?}");

    assert!(story.length().unwrap_or(0) > 0);
    assert!(text.range(0, 5).is_some());

    if let Some(props) = text.font_properties(0) {
        println!("font = {:?}, size = {:?}", props.name(), props.size());
    } else {
        eprintln!("skip: Text.FontProperties returned no struct via late binding");
    }
    let chars = story.characters().expect("story.characters()");
    assert!(chars.count().unwrap_or(0) > 0);

    let _ = doc.close_without_saving();
}
