use windows::Win32::System::Com;
use cdrsdk::sdk::application::IvgApplication;
use cdrsdk::sdk::document::IvgDocument;

const PNG_FILE_TYPE: u32 = 802;

// 轻量 RAII 守卫，自动管理 COM 生命周期
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

// 内部导出逻辑，export_type 用于区分导出模式
fn export_cover_internal(file_path: &str, export_type: u32, ver: &str) -> Option<()> {
    let _guard = ComGuard::init();
    let (app, doc) = unsafe { get_app_and_doc(ver) }?;

    let exp_opt = app.create_struct_export_options();
    exp_opt.put_SizeX(600);
    exp_opt.put_SizeY(600);
    exp_opt.put_maintainaspect(true);
    exp_opt.put_transparent(true);
    let exp_opt_info = exp_opt.to_variant();
    let pale = app.create_struct_palette_options().to_variant();

    doc.export(file_path, PNG_FILE_TYPE, export_type, exp_opt_info, pale);
    Some(())
}

pub fn export_selection_cover(cover_src: &str, ver: &str) {
    export_cover_internal(cover_src, 2, ver);
}

pub fn export_cover(cover_src: String, ver: &str) -> Option<()> {
    export_cover_internal(&cover_src, 1, ver)
}