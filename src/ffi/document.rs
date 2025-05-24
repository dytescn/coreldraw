use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;

// 获取coreldraw信息
pub fn get_documents_count(cur_version:String)->usize{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app_res = IvgApplication::new(&cur_version).unwrap();
        let docs = app_res.get_documents();
        let sum = docs.get_Count();
        return  sum as usize;
        Com::CoUninitialize();
    }
}


// 获取coreldraw信息
pub fn get_document_name(cur_ver:String)->String{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app_res = IvgApplication::new(&cur_ver).unwrap();
        let doc = app_res.get_active_document().unwrap();
        return doc.get_filename();
        Com::CoUninitialize();
    }
}

// 获取coreldraw信息
pub fn get_document_path(cur_ver:String)->String{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app_res = IvgApplication::new(&cur_ver).unwrap();
        let doc = app_res.get_active_document().unwrap();
        let file_path = doc.get_filepath();
        return file_path;
        Com::CoUninitialize();
    }
}

// 获取coreldraw信息
pub fn get_document_fullfilename(cur_ver:String)->String{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app_res = IvgApplication::new(&cur_ver).unwrap();
        let doc = app_res.get_active_document().unwrap();
        let file_path = doc.get_fullfilename();
        return file_path;
        Com::CoUninitialize();
    }
}
