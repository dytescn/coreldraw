use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use cdrsdk::sdk::application::IvgApplication;
use cdrsdk::sdk::document::IvgDocument;

const PNG_FILE_TYPE: u32 = 802;

// 轻量 RAII 守卫，确保 COM 在任何退出路径都被释放
struct ComGuard;
impl ComGuard {
    fn init() -> Self {
        unsafe { let _ = Com::CoInitialize(None); }
        Self
    }
}
impl Drop for ComGuard {
    fn drop(&mut self) {
        unsafe { Com::CoUninitialize(); }
    }
}

// 提取公共的初始化逻辑
unsafe fn get_app_and_doc(ver: &str) -> Option<(IvgApplication, IvgDocument)> {
    let app = IvgApplication::new(ver)?;
    let doc = app.get_active_document()?;
    Some((app, doc))
}

pub fn export_preview(preview_src: String, ver: &str) -> Option<usize> {
    let _guard = ComGuard::init();               // 自动管理 COM
    let (app, doc) = unsafe { get_app_and_doc(ver) }?;

    let exp_opt = app.create_struct_export_options();
    exp_opt.put_SizeX(2000);
    exp_opt.put_SizeY(2000);
    exp_opt.put_maintainaspect(true);
    exp_opt.put_transparent(true);
    let exp_opt_info = exp_opt.to_variant();
    let pale = app.create_struct_palette_options().to_variant();

    let active_page = doc.get_activepage().log_error("got active page err")?;
    let pages = doc.get_pages();
    let page_count = pages.get_count();

    for i in 1..=page_count {
        let src = format!("{}{}_preview.png", preview_src, i);
        let cur_page = pages.get_item(i.into());
        cur_page.activate();
        // 按原逻辑忽略导出返回值（如需错误处理可在此添加）
        doc.export(&src, PNG_FILE_TYPE, 1, exp_opt_info.clone(), pale.clone());
    }
    active_page.activate();

    Some(page_count as usize)
}

pub fn export_preview_sum(ver: &str) -> Option<usize> {
    let _guard = ComGuard::init();
     let (app, doc) = unsafe { get_app_and_doc(ver) }.or_else(|| {
        eprintln!("Failed to get app or doc for version: {}", ver);
        None
    })?;
    let pages = doc.get_pages();
    Some(pages.get_count() as usize)
}