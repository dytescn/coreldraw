use windows::Win32::System::Com;
use cdrsdk::sdk::application::IvgApplication;
use cdrsdk::sdk::document::IvgDocument;

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

unsafe fn get_app_and_doc(ver: &str) -> Option<(IvgApplication, IvgDocument)> {
    let app = IvgApplication::new(ver)?;
    let doc = app.get_active_document()?;
    Some((app, doc))
}

pub fn export_file(file_src: String, ver: &str) -> Option<()> {
    let _guard = ComGuard::init();
    let (_, doc) = unsafe { get_app_and_doc(ver) }?;
    
    let doc_file_path = doc.get_fullfilename();
    if doc_file_path.is_empty() {
        doc.put_fullfilename(&file_src);
        doc.save();
        return Some(());
    }
    
    if let Err(e) = std::fs::copy(doc_file_path, file_src.clone()) {
        eprintln!("copy failed: {:?}", e);
        return None;
    }
    let _ = doc.put_fullfilename(&file_src);
    Some(())
}

pub fn export_cdr_file(file_src: String, ver: &str) -> Option<()> {
    let _guard = ComGuard::init();
    let (_app, doc) = unsafe { get_app_and_doc(ver) }?;
    
    let doc_file_path = doc.get_fullfilename();
    if let Err(e) = std::fs::copy(doc_file_path, file_src.clone()) {
        eprintln!("copy failed: {:?}", e);
        return None;
    }
    Some(())
}

pub fn get_current_file_path(ver: &str) -> Option<String> {
    let _guard = ComGuard::init();
    let (_app, doc) = unsafe { get_app_and_doc(ver) }?;
    Some(doc.get_fullfilename())
}