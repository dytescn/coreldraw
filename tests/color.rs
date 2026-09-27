//! 颜色对象测试

mod common;

use common::ComGuard;

#[test]
fn color_rgb_roundtrip() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let c = app.create_rgb_color(255, 128, 0).expect("create_rgb_color");
    println!("RGB = ({:?}, {:?}, {:?})",
        c.rgb_red(), c.rgb_green(), c.rgb_blue());

    assert_eq!(c.rgb_red(),   Some(255));
    assert_eq!(c.rgb_green(), Some(128));
    assert_eq!(c.rgb_blue(),  Some(0));

    println!("hex = {:?}", c.hex_value());

    // 修改分量
    assert!(c.set_rgb_red(10));
    assert_eq!(c.rgb_red(), Some(10));
}

#[test]
fn color_cmyk_roundtrip() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let c = app.create_cmyk_color(10, 20, 30, 40).expect("create_cmyk_color");
    println!("CMYK = ({:?}, {:?}, {:?}, {:?})",
        c.cmyk_cyan(), c.cmyk_magenta(), c.cmyk_yellow(), c.cmyk_black());

    assert_eq!(c.cmyk_cyan(),    Some(10));
    assert_eq!(c.cmyk_magenta(), Some(20));
    assert_eq!(c.cmyk_yellow(),  Some(30));
    assert_eq!(c.cmyk_black(),   Some(40));
}

#[test]
fn color_gray() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let c = app.create_gray_color(128).expect("create_gray_color");
    assert_eq!(c.gray(), Some(128));
    assert_eq!(c.is_gray(), Some(true));
}

#[test]
fn color_assign() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let c = app.create_rgb_color(0, 0, 0).expect("create_rgb_color");
    assert_eq!(c.rgb_red(), Some(0));

    // 用 RGBAssign 一次性赋值
    assert!(c.assign_rgb(200, 100, 50));
    println!("after assign_rgb = ({:?}, {:?}, {:?})",
        c.rgb_red(), c.rgb_green(), c.rgb_blue());
    assert_eq!(c.rgb_red(),   Some(200));
    assert_eq!(c.rgb_green(), Some(100));
    assert_eq!(c.rgb_blue(),  Some(50));
}

#[test]
fn color_copy_and_compare() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let c1 = app.create_rgb_color(10, 20, 30).expect("c1");
    let c2 = app.create_rgb_color(10, 20, 30).expect("c2");

    assert_eq!(c1.is_same(&c2), Some(true), "相同颜色应相等");

    let c3 = app.create_rgb_color(99, 99, 99).expect("c3");
    assert_eq!(c1.is_same(&c3), Some(false), "不同颜色应不等");

    // 复制
    let c4 = c1.get_copy().expect("get_copy");
    assert_eq!(c4.rgb_red(), Some(10));

    // CopyAssign
    assert!(c3.copy_assign(&c1));
    println!("c3 after CopyAssign = ({:?}, {:?}, {:?})",
        c3.rgb_red(), c3.rgb_green(), c3.rgb_blue());
    assert_eq!(c3.rgb_red(), Some(10));
}

#[test]
fn color_convert_to_rgb() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let c = app.create_cmyk_color(0, 100, 100, 0).expect("cmyk");
    println!("CMYK = ({:?}, {:?}, {:?}, {:?})",
        c.cmyk_cyan(), c.cmyk_magenta(), c.cmyk_yellow(), c.cmyk_black());

    assert!(c.convert_to_rgb(), "ConvertToRGB 应成功");
    println!("after ConvertToRGB: RGB = ({:?}, {:?}, {:?})",
        c.rgb_red(), c.rgb_green(), c.rgb_blue());
    println!("color_type = {:?}", c.color_type());
}