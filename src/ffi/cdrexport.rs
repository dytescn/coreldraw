use core::num;
use std::num::ParseIntError;

use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;

// 获取coreldraw
pub fn export_dxf_file(ver:&str,dist_src:&str)->Option<u32>{
    unsafe { 
        let _ = Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res {
            Some(app)=>{
                let doc = app.get_active_document().unwrap();
                // doc.save_as(&file_src,cur_ver);
                let exp_opt = app.create_struct_export_options();            
                let exp_opt_info = exp_opt.to_variant();
                let pale_obj = app.create_struct_palette_options();
                let pale = pale_obj.to_variant();
                let res_doc = app.get_active_document();
                // let src = "C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\123123.cdr".to_string();
                // let cover_src ="C:\\Users\\dowell\\codes\\dytes\\cdrsdk\\cache\\11111.dxf";
                match res_doc {
                    Some(doc)=>{
                        let res =  doc.export_as_dxf(&dist_src,exp_opt_info,pale);
                    }
                    None=>{
                        return None;
                    }                      
                }
                return None;
            }
            None=>{
                return None;
            }
        }
        let _ = Com::CoUninitialize();
    }
    return None;
}
