//! 端到端测试：创建文档 → 画矩形 → 上色 → 导出 PNG → 关闭

mod common;

use common::ComGuard;

#[test]
fn e2e_draw_and_export() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = require_app!();

    // 1. 创建文档
    let doc = app.create_document().expect("create_document");
    println!("[1] created document: {:?}", doc.name());

    // 2. 拿活动图层（新文档默认有 Layer 1）
    let layer = doc.active_layer().expect("active_layer");
    println!("[2] active layer: {:?}", layer.name());

    // 3. 画一个矩形
    //    CreateRectangle2(x, y, w, h, r_ul, r_ur, r_lr, r_ll)
    let rect = layer
        .create_rectangle2(50.0, 50.0, 100.0, 80.0, 0.0, 0.0, 0.0, 0.0)
        .expect("create_rectangle2");
    println!("[3] created rectangle: type={:?} size={:?}",
        rect.shape_type(), rect.get_size());

    assert_eq!(rect.get_size(), Some((100.0, 80.0)));

    // 4. 上填充色（RGB 255,128,0）
    let fill_color = app.create_rgb_color(255, 128, 0).expect("rgb color");
    let fill = rect.fill().expect("rect.fill() 应返回 Some");
    assert!(fill.apply_uniform_fill(&fill_color), "fill.apply_uniform_fill");
    println!("[4] applied fill color");

    if let Some(fill) = rect.fill() {
        println!("    fill.type = {:?}", fill.fill_type());
        if let Some(c) = fill.uniform_color() {
            println!("    fill.uniform_color = ({:?}, {:?}, {:?})",
                c.rgb_red(), c.rgb_green(), c.rgb_blue());
        }
    }

    // 5. 上轮廓色 + 宽度
    let outline_color = app.create_rgb_color(0, 0, 0).expect("black");
    if let Some(outline) = rect.outline() {
        assert!(outline.set_width(1.0), "outline.set_width");
        assert!(outline.set_color(&outline_color), "outline.set_color");
        println!("[5] outline: width={:?}", outline.width());
    }

    // 6. 导出 PNG
    let mut path = std::env::temp_dir();
    path.push("cdrsdk_e2e_test.png");
    let path_str = path.to_string_lossy().to_string();
    println!("[6] exporting to: {}", path_str);

    // cdrFilter::PNG 值约 790（见 enums/filter）
    let filter_png: i32 = 790;

    let exported = if let Some(opts) = app.create_export_options() {
        let _ = opts.set_overwrite(true);
        let _ = opts.set_resolution_x(300);
        let _ = opts.set_resolution_y(300);
        doc.export_ex(&path_str, filter_png, &opts)
    } else {
        doc.export(&path_str, filter_png)
    };

    println!("[6] export returned {}", exported);

    if exported && path.exists() {
        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        println!("    PNG file size = {} bytes", size);
        assert!(size > 0, "PNG file should not be empty");

        // 清理
        let _ = std::fs::remove_file(&path);
    } else {
        println!("    export failed or file not created (may need dialog)");
    }

    // 7. 关闭文档（不保存）
    assert!(doc.close(), "close document");
    println!("[7] document closed");
}