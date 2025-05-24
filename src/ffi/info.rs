use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use std::collections::HashMap;
pub struct CdrInfo{
    pub space_uid:String,
    pub version_uid:String,
    pub name:String,
    pub cur_version:u16
}

// 获取coreldraw信息
pub fn get_info(ver:&str)->CdrInfo{
    unsafe{
        let mut name =String::from("");
        let mut s_uid =String::from("");
        let mut v_uid =String::from("");
        let mut v_ver = 0;
        // 初始化系统
        Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res {
            Some(app)=>{
                let ivgdoc = app.get_active_document().unwrap();
                name = ivgdoc.get_name();
                let src = &ivgdoc.get_filename() as &str;
                let cdr_file_name = src.trim_end_matches(".cdr");
                let cdr_file_names:Vec<&str> = cdr_file_name.split("_").collect();
                if cdr_file_names.len()==3&&cdr_file_names[0].to_string()!="Backup"{
                    s_uid =  cdr_file_names[0].to_string();  
                    v_uid =  cdr_file_names[1].to_string();
                    v_ver =  cdr_file_names[2].parse::<u16>().unwrap();
                }
                return CdrInfo{
                    space_uid:s_uid,
                    version_uid:v_uid,
                    name:name,
                    cur_version:v_ver
                };
            }
            None=>{
                return CdrInfo{
                    space_uid:"".to_string(),
                    version_uid:"".to_string(),
                    name:"".to_string(),
                    cur_version:v_ver
                };
            }
        }
     
        Com::CoUninitialize();
    }
}
