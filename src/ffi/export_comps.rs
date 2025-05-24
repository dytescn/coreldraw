use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::pages;
use crate::ffi::filetype;
pub fn export_select_comps(file_src:&str,file_type_name:&str,ver:&str){
    unsafe{
        // 初始化系统
        let res = Com::CoInitialize(None);
        if res.is_err(){
            return;
        }
        let app_res = IvgApplication::new(ver);
        // 保存封面
        match app_res {
            Some(app)=>{
                let exp_opt = app.create_struct_export_options();
                let exp_opt_info = exp_opt.to_variant();
                let pale_obj = app.create_struct_palette_options();
                let pale = pale_obj.to_variant();
                let doc = app.get_active_document().log_error("got doc err").unwrap();
                let file_type = filetype::cdr_file_type_to_u32(file_type_name);
                let _ = doc.export(&file_src,file_type,2,exp_opt_info.clone(),pale.clone());    
            }
            None=>{
                return;
            }
        }
    }
}