use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::page;

// 保存文件
pub fn save_as_file(file_src:&str,name:String,cur_ver:u16,ver:&str){
    // 初始化系统
    unsafe {
        let _ = Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res {
            Some(app)=>{
                let doc = app.get_active_document().unwrap();
                // 获取文件路径，
                let doc_file_path = doc.get_fullfilename();
                println!("{:?}",doc_file_path);
                if doc_file_path=="" {
                    doc.save_as(&file_src,cur_ver);
                    return;
                }
                // 文件保存
                // doc.save();
                let _ = std::fs::copy(doc_file_path, file_src).log_error("copy file error").unwrap();
            }
            None=>{
                return
            }
        }
        let _ = Com::CoUninitialize();
    }
}