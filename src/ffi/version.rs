use std::num::ParseIntError;
use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use windows::core;

// 获取coreldraw
pub fn get_version_info(ver:&str)->Option<u32>{
    unsafe{
        // 初始化系统
        let com_res = Com::CoInitialize(None);
        if com_res.is_err() {
            println!("com init error");
            return None;
        }
        let id_name = "coreldraw.application.".to_string() + ver; // Can't use + with two &str
        let lpsz = core::HSTRING::from(id_name);
        let rclsid_res = Com::CLSIDFromProgID(&lpsz);
        if rclsid_res.is_err(){
            return None;
        }
        let ver_n = ver.parse::<u32>().unwrap();
        return Some(ver_n);
        Com::CoUninitialize();
    }
}

// 判断当前已安装的coreldraw版本
pub fn get_version_list()->Vec<u32>{
    let mut ver_data:Vec<u32> = Vec::new();
    for i in 15..26  {
        let ver = i.to_string();
        let res = get_version_info(&ver);
        match res {
            Some(ver_name)=>{
                ver_data.push(ver_name);
            }
            None=>{
                println!("...{}",i);
            }
        }
    }
    return ver_data;
}
