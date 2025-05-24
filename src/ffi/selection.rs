use std::fs;
use std::path::Path;
use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::ffi::layer;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::page;
use crate::sdk::struct_importoption::ImportOption;
// 判断选择内容是否为空
pub fn check_selection(ver:&str)->i32{
    unsafe{
        // 初始化系统
        let res = Com::CoInitialize(None);
        if res.is_err(){
            return 0;
        }
        let app_res = IvgApplication::new(ver);
        // 保存封面
        match app_res {
            Some(app)=>{
                let act_doc = app.get_active_document().unwrap();
                let select_info = act_doc.get_SelectionInfo();
                let sum = select_info.get_Count();
                return sum;
            }
            None=>{
                return 0;
            }
        }
    }
}