//! 结构体工厂测试

mod common;

use common::ComGuard;

#[test]
fn structs_export_options() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let opts = app.create_export_options().expect("create_export_options");
    println!("anti_aliasing_type = {:?}", opts.anti_aliasing_type());
    println!("image_type         = {:?}", opts.image_type());
    println!("compression        = {:?}", opts.compression());
    println!("overwrite          = {:?}", opts.overwrite());
    println!("transparent        = {:?}", opts.transparent());

    // 修改
    assert!(opts.set_overwrite(true));
    assert_eq!(opts.overwrite(), Some(true));

    assert!(opts.set_size_x(1024));
    assert!(opts.set_size_y(768));
    println!("size = ({:?}, {:?})", opts.size_x(), opts.size_y());

    assert!(opts.set_resolution_x(300));
    assert!(opts.set_resolution_y(300));
    println!("resolution = ({:?}, {:?})",
        opts.resolution_x(), opts.resolution_y());
}

#[test]
fn structs_save_as_options() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let opts = app.create_save_as_options().expect("create_save_as_options");
    println!("filter         = {:?}", opts.filter());
    println!("version        = {:?}", opts.version());
    println!("thumbnail_size = {:?}", opts.thumbnail_size());
    println!("range          = {:?}", opts.range());
    println!("overwrite      = {:?}", opts.overwrite());
    println!("embed_icc      = {:?}", opts.embed_icc_profile());
}

#[test]
fn structs_import_options() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let opts = app.create_import_options().expect("create_import_options");
    println!("use_color_profile   = {:?}", opts.use_color_profile());
    println!("maintain_layers     = {:?}", opts.maintain_layers());
    println!("detect_watermark    = {:?}", opts.detect_watermark());
    println!("combine_multipage   = {:?}", opts.combine_multipage());
}

#[test]
fn structs_open_options() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let opts = app.create_open_options().expect("create_open_options");
    println!("code_page = {:?}", opts.code_page());

    assert!(opts.set_code_page(65001));
    assert_eq!(opts.code_page(), Some(65001));
}

#[test]
fn structs_font_properties_set_then_read() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let font = app.create_font_properties().expect("create_font_properties");

    // 先设后读
    assert!(font.set_name("Arial"));
    assert!(font.set_size(24.0));
    assert!(font.set_style(1));         // Bold

    assert_eq!(font.name(), Some("Arial".to_string()));
    assert_eq!(font.size(), Some(24.0));
    println!("name = {:?}, size = {:?}, style = {:?}",
        font.name(), font.size(), font.style());
}

#[test]
fn structs_align_properties_set_then_read() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let align = app.create_align_properties().expect("create_align_properties");

    assert!(align.set_alignment(3));           // Center
    assert!(align.set_first_line_indent(10.0));
    assert!(align.set_left_indent(5.0));
    assert!(align.set_right_indent(5.0));

    assert_eq!(align.alignment(), Some(3));
    assert_eq!(align.first_line_indent(), Some(10.0));
    assert_eq!(align.left_indent(), Some(5.0));
    assert_eq!(align.right_indent(), Some(5.0));

    println!("alignment = {:?}, first_line = {:?}, left = {:?}, right = {:?}",
        align.alignment(),
        align.first_line_indent(),
        align.left_indent(),
        align.right_indent());
}

#[test]
fn structs_space_properties_set_then_read() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let sp = app.create_space_properties().expect("create_space_properties");

    assert!(sp.set_character_spacing(100.0));
    assert!(sp.set_word_spacing(120.0));
    assert!(sp.set_line_spacing(150.0));

    assert_eq!(sp.character_spacing(), Some(100.0));
    assert_eq!(sp.word_spacing(),      Some(120.0));
    assert_eq!(sp.line_spacing(),      Some(150.0));

    println!("char={:?} word={:?} line={:?}",
        sp.character_spacing(), sp.word_spacing(), sp.line_spacing());
}

#[test]
fn structs_hyphenation_set_then_read() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let h = app.create_hyphenation_settings().expect("create_hyphenation_settings");

    assert!(h.set_use_automatic_hyphenation(true));
    assert!(h.set_hot_zone(0.25));
    assert!(h.set_min_word_length(5));

    assert_eq!(h.use_automatic_hyphenation(), Some(true));
    assert_eq!(h.hot_zone(),         Some(0.25));
    assert_eq!(h.min_word_length(), Some(5));

    println!("auto={:?} hot_zone={:?} min_len={:?}",
        h.use_automatic_hyphenation(), h.hot_zone(), h.min_word_length());
}