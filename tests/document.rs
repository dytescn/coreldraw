//! `IvgDocument` / `IvgDocuments` 测试
//!
//! 会创建一个临时文档，测完关闭（不保存）。

mod common;

use common::ComGuard;

// =============================================================
// 文档基本信息
// =============================================================

#[test]
fn doc_basic_info() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip: CorelDRAW not running"); return; }
    };

    let doc = match app.create_document() {
        Some(d) => d,
        None => { eprintln!("skip: cannot create document"); return; }
    };

    println!("doc.name             = {:?}", doc.name());
    println!("doc.file_name        = {:?}", doc.file_name());
    println!("doc.full_file_name   = {:?}", doc.full_file_name());
    println!("doc.file_path        = {:?}", doc.file_path());
    println!("doc.dirty            = {:?}", doc.dirty());
    println!("doc.read_only        = {:?}", doc.read_only());
    println!("doc.saved            = {:?}", doc.saved());
    println!("doc.version          = {:?}", doc.version());
    println!("doc.unit             = {:?}", doc.unit());
    println!("doc.reference_point  = {:?}", doc.reference_point());
    println!("doc.world_scale      = {:?}", doc.world_scale());

    // ---- set_unit 是可写属性 ----
    let unit_before = doc.unit();
    assert!(doc.set_unit(4), "set_unit(4)=cm 应成功");
    let unit_after = doc.unit();
    println!("after set_unit(4)    = {:?}", unit_after);
    assert_eq!(unit_after, Some(4), "unit 应变为 4");

    // 恢复
    if let Some(v) = unit_before {
        let _ = doc.set_unit(v as i32);
    }

    // ---- set_reference_point ----
    assert!(doc.set_reference_point(4), "set_reference_point(4)=Center 应成功");
    println!("after set_reference  = {:?}", doc.reference_point());

    // ---- reset_dirty ----
    let _ = doc.reset_dirty();
    println!("dirty after reset    = {:?}", doc.dirty());

    // 关闭（不保存）
    assert!(doc.close_without_saving(), "close 应成功");
    println!("doc closed");
}

// =============================================================
// 页面
// =============================================================

#[test]
fn doc_pages() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip"); return; }
    };
    let doc = match app.create_document() {
        Some(d) => d,
        None => { eprintln!("skip"); return; }
    };

    // 页面集合
    let pages = doc.pages().expect("doc.pages() 应非 None");
    println!("pages.count = {:?}", pages.count());
    println!("pages.first = {:?}", pages.first().and_then(|p| p.name()));
    println!("pages.last  = {:?}", pages.last().and_then(|p| p.name()));

    // 当前页面
    if let Some(page) = doc.active_page() {
        println!("active_page.name         = {:?}", page.name());
        println!("active_page.index        = {:?}", page.index());
        println!("active_page.size         = {:?}", page.get_size());
        println!("active_page.orientation  = {:?}", page.orientation());
        println!("active_page.bleed        = {:?}", page.bleed());
        println!("active_page.background   = {:?}", page.background());

        // 修改页面尺寸
        assert!(page.set_size(200.0, 300.0), "set_size 应成功");
        let new_size = page.get_size();
        println!("after set_size           = {:?}", new_size);
        assert_eq!(new_size, Some((200.0, 300.0)));

        // 修改方向
        assert!(page.set_orientation(1), "set_orientation(landscape) 应成功");
        assert_eq!(page.orientation(), Some(1));
        println!("after set_orientation    = {:?}", page.orientation());
    }

    // 新建页面
    let page_count_before = pages.count().unwrap_or(0);
    if let Some(new_page) = doc.create_page() {
        println!(
            "created new page: name={:?} index={:?}",
            new_page.name(),
            new_page.index()
        );
    }
    let page_count_after = doc.pages().and_then(|p| p.count()).unwrap_or(0);
    println!("pages.count: {} -> {}", page_count_before, page_count_after);

    let _ = doc.close_without_saving();
}

// =============================================================
// 图层
//
// ⚠️ CorelDRAW 的 `Layers` 挂在 `Page` 上，不在 `Document` 上。
//    所以这里用 `doc.active_page()?.layers()`。
// =============================================================

#[test]
fn doc_layers() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip"); return; }
    };
    let doc = match app.create_document() {
        Some(d) => d,
        None => { eprintln!("skip"); return; }
    };

    let page = doc.active_page().expect("active_page 应非 None");

    // ---- Page.Layers ----
    let layers = page.layers().expect("page.layers() 应非 None");
    println!("layers.count = {:?}", layers.count());

    for (i, l) in layers.all().iter().enumerate() {
        println!(
            "  [{}] name={:?} visible={:?} editable={:?} printable={:?}",
            i + 1,
            l.name(),
            l.visible(),
            l.editable(),
            l.printable()
        );
    }

    // ---- Document.ActiveLayer ----
    if let Some(active) = doc.active_layer() {
        println!("doc.active_layer.name = {:?}", active.name());
        println!("doc.active_layer.idx  = {:?}", active.index());
    }

    // ---- 便捷方法：doc.active_page_layers() ----
    if let Some(ls) = doc.active_page_layers() {
        println!("doc.active_page_layers().count = {:?}", ls.count());
    }

    // ---- 新建图层 ----
    if let Some(new_layer) = doc.create_layer("RustTest") {
        println!(
            "created layer: name={:?} index={:?}",
            new_layer.name(),
            new_layer.index()
        );
        println!("  visible   = {:?}", new_layer.visible());
        println!("  editable  = {:?}", new_layer.editable());
        println!("  printable = {:?}", new_layer.printable());

        assert!(new_layer.set_visible(false), "set_visible(false) 应成功");
        assert_eq!(new_layer.visible(), Some(false));
        println!("  after set_visible(false) = {:?}", new_layer.visible());

        assert!(new_layer.set_visible(true), "set_visible(true) 应成功");

        // 删除图层
        assert!(new_layer.delete(), "delete 应成功");
        println!("  layer deleted");
    } else {
        println!("create_layer returned None");
    }

    let _ = doc.close_without_saving();
}

// =============================================================
// 选择 / 图形计数
//
// ⚠️ `Document.SelectAll` 在 CorelDRAW 里不存在，
//    用 `page.selectable_shapes()` 或 `ShapeRange` 替代。
// =============================================================

#[test]
fn doc_shapes_count() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip"); return; }
    };
    let doc = match app.create_document() {
        Some(d) => d,
        None => { eprintln!("skip"); return; }
    };

    // Document.Shapes（等价于 ActivePage.Shapes）
    if let Some(shapes) = doc.shapes() {
        println!("doc.shapes.count = {:?}", shapes.count());
        for (i, s) in shapes.all().iter().enumerate() {
            println!(
                "  [{}] type={:?} name={:?} size={:?}",
                i + 1,
                s.shape_type(),
                s.name(),
                s.get_size()
            );
        }
    }

    // SelectionRange
    if let Some(sel) = doc.selection_range() {
        println!("doc.selection_range.count = {:?}", sel.count());
    }

    // ActiveShape
    println!("doc.active_shape        = {:?}",
        doc.active_shape().map(|s| s.shape_type()));

    // Page.Shapes
    if let Some(page) = doc.active_page() {
        if let Some(shapes) = page.shapes() {
            println!("page.shapes.count            = {:?}", shapes.count());
        }
        if let Some(sel_shapes) = page.selectable_shapes() {
            println!("page.selectable_shapes.count = {:?}", sel_shapes.count());
        }
    }

    // 清空选择（幂等）
    let _ = doc.clear_selection();

    let _ = doc.close_without_saving();
}

// =============================================================
// 保存 / 另存为
// =============================================================

#[test]
fn doc_save_as_temp() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip"); return; }
    };
    let doc = match app.create_document() {
        Some(d) => d,
        None => { eprintln!("skip"); return; }
    };

    // 写到系统临时目录
    let mut path = std::env::temp_dir();
    path.push("cdrsdk_test_save.cdr");
    let path_str = path.to_string_lossy().to_string();

    // 先删除可能残留的文件
    let _ = std::fs::remove_file(&path);

    println!("saving to: {}", path_str);

    // 带 SaveAsOptions
    if let Some(opts) = app.create_save_as_options() {
        let _ = opts.set_overwrite(true);

        if doc.save_as_ex(&path_str, &opts) {
            println!("saved via save_as_ex");
            println!("doc.file_name      = {:?}", doc.file_name());
            println!("doc.full_file_name = {:?}", doc.full_file_name());
            println!("doc.file_path      = {:?}", doc.file_path());
            println!("doc.dirty          = {:?}", doc.dirty());
            println!("doc.saved          = {:?}", doc.saved());

            // 文件应存在
            if path.exists() {
                let size = std::fs::metadata(&path)
                    .map(|m| m.len())
                    .unwrap_or(0);
                println!("file size = {} bytes", size);
                assert!(size > 0, ".cdr 文件不应为空");
            } else {
                println!("⚠ file not created (CorelDRAW may have prompted a dialog)");
            }

            // 清理
            let _ = std::fs::remove_file(&path);
        } else {
            println!("save_as_ex returned false (may have prompted dialog)");
        }
    } else {
        println!("could not create save-as options");
    }

    let _ = doc.close_without_saving();
}

// =============================================================
// 文档集合（Documents）
// =============================================================

#[test]
fn doc_collection() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip"); return; }
    };

    // 在创建前记录数量
    let count_before = app
        .documents()
        .and_then(|d| d.count())
        .unwrap_or(0);
    println!("documents before = {}", count_before);

    // 创建一个新文档
    let doc = app.create_document().expect("create_document");
    let name = doc.name();
    println!("created: {:?}", name);

    // 数量应 +1
    let count_after_create = app
        .documents()
        .and_then(|d| d.count())
        .unwrap_or(0);
    println!("documents after  = {}", count_after_create);
    assert!(count_after_create >= count_before + 1);

    // 遍历集合
    if let Some(docs) = app.documents() {
        for (i, d) in docs.all().iter().enumerate() {
            println!("  [{}] name = {:?}", i + 1, d.name());
        }

        // 按名字查找
        if let Some(name) = &name {
            if let Some(found) = docs.item_by_name(name) {
                println!("found by name: {:?}", found.name());
            } else {
                println!("item_by_name({:?}) returned None", name);
            }
        }

        // 按索引访问
        if let Some(first) = docs.item_by_index(1) {
            println!("item_by_index(1).name = {:?}", first.name());
        }
    }

    // 关闭刚创建的
    let _ = doc.close_without_saving();

    let count_after_close = app
        .documents()
        .and_then(|d| d.count())
        .unwrap_or(0);
    println!("documents after close = {}", count_after_close);
}

// =============================================================
// 元数据
// =============================================================

#[test]
fn doc_metadata() {
    let _com = ComGuard::init().expect("CoInitialize failed");
    let app = match common::bind_app() {
        Some(a) => a,
        None => { eprintln!("skip"); return; }
    };
    let doc = match app.create_document() {
        Some(d) => d,
        None => { eprintln!("skip"); return; }
    };

    if let Some(meta) = doc.metadata() {
        // 先写后读（验证可读写）
        assert!(meta.set_title("Rust SDK Document"), "set_title");
        assert!(meta.set_author("Test Suite"), "set_author");
        assert!(meta.set_subject("Automated Test"), "set_subject");
        assert!(meta.set_keywords("rust, com, cdrsdk"), "set_keywords");
        assert!(meta.set_notes("Created by cdrsdk test"), "set_notes");

        println!("title    = {:?}", meta.title());
        println!("author   = {:?}", meta.author());
        println!("subject  = {:?}", meta.subject());
        println!("keywords = {:?}", meta.keywords());
        println!("notes    = {:?}", meta.notes());
        println!("doc_id   = {:?}", meta.doc_id());
    } else {
        println!("no metadata (may not be supported)");
    }

    let _ = doc.close_without_saving();
}