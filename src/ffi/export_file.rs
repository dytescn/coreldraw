use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;

// 保存封面图
pub fn export_file(file_src:String,ver:&str)->Option<()>{
    unsafe {
        let _ = Com::CoInitialize(None).expect("init com error");
        let app = IvgApplication::new(ver).unwrap();
        // 保存封面
        let doc = app.get_active_document().unwrap();
        let doc_file_path = doc.get_fullfilename();
        // 处理文件为空
        if doc_file_path==""{
            doc.put_fullfilename(&file_src);  // 修改文件路径
            doc.save();
            return Some(());
        }
        // 转移文件
        let res = std::fs::copy(doc_file_path, file_src.clone());
        if res.is_err() {
            println!("{:?}",res.err());
            return None;
        }
        let res = doc.put_fullfilename(&file_src);
        let _ = Com::CoUninitialize();
        return  Some(());
    }
}


pub fn export_cdr_file(file_src:String,ver:&str)->Option<()>{
    unsafe {
        let res = Com::CoInitialize(None);
        if res.is_err() {
            return None;
        }
    }
    // 保存封面
    let app = IvgApplication::new(ver).unwrap();
    let doc = app.get_active_document().unwrap();
    let doc_file_path = doc.get_fullfilename();
    let res = std::fs::copy(doc_file_path, file_src.clone());
    if res.is_err() {
        println!("{:?}",res.err());
        return None;
    }
    unsafe {  
        let _ = Com::CoUninitialize();
    }
    return  Some(());
}