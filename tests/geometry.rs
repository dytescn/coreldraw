//! 几何对象测试

mod common;

use common::ComGuard;

#[test]
fn geom_rect() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let r = app.create_rect(10.0, 20.0, 100.0, 50.0).expect("create_rect");
    println!("x={:?} y={:?} w={:?} h={:?}",
        r.x(), r.y(), r.width(), r.height());

    assert_eq!(r.x(),      Some(10.0));
    assert_eq!(r.y(),      Some(20.0));
    assert_eq!(r.width(),  Some(100.0));
    assert_eq!(r.height(), Some(50.0));

    println!("left={:?} right={:?} top={:?} bottom={:?}",
        r.left(), r.right(), r.top(), r.bottom());
    println!("center = ({:?}, {:?})", r.center_x(), r.center_y());

    // 修改
    assert!(r.set_x(5.0));
    assert_eq!(r.x(), Some(5.0));

    assert!(r.set_width(200.0));
    assert_eq!(r.width(), Some(200.0));
}

#[test]
fn geom_snap_point() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    let sp = app.create_snap_point(100.0, 200.0).expect("create_snap_point");
    println!("position = {:?}", sp.get_position());
    assert_eq!(sp.get_position(), Some((100.0, 200.0)));

    assert!(sp.set_position(50.0, 75.0));
    assert_eq!(sp.get_position(), Some((50.0, 75.0)));

    println!("point_type = {:?}", sp.point_type());
}

#[test]
fn geom_units() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    // 全表覆盖
    for (val, from, to, expected, label) in [
        (1.0,   0, 1, 25.4,   "1 inch → mm"),
        (1.0,   0, 3, 72.0,   "1 inch → pt"),
        (1.0,   0, 4, 2.54,   "1 inch → cm"),
        (25.4,  1, 0, 1.0,    "25.4 mm → inch"),
        (2.54,  4, 0, 1.0,    "2.54 cm → inch"),
        (100.0, 4, 1, 1000.0, "100 cm → mm"),
        (72.0,  3, 0, 1.0,    "72 pt → inch"),
        (72.0,  3, 1, 25.4,   "72 pt → mm"),
    ] {
        match app.convert_units(val, from, to) {
            Some(v) => {
                let ok = (v - expected).abs() < 1e-9;
                println!("{:20} = {:.10}  (expect {:.10}) {}", label, v, expected,
                    if ok { "✔" } else { "✘" });
                assert!(ok, "{}: got {}, expected {}", label, v, expected);
            }
            None => panic!("{} returned None", label),
        }
    }
}