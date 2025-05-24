use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
pub fn check_corel_status(ver:&str)->bool{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res{
            Some(app)=>{
                let doc_res = app.get_active_document();
                match doc_res{
                    Some(doc)=>{
                        return true;
                    }
                    None=>{
                        return false;
                    }
                }
            }
            None=>{
                return false;
            }
        }
        Com::CoUninitialize();
    }
}

pub fn check_corel_is_open(ver:&str)->bool{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res{
            Some(app)=>{
                return true;
            }
            None=>{
                return false;
            }
        }
        Com::CoUninitialize();
    }
}