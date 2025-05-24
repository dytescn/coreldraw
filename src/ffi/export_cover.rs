use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;

// 保存文件
pub fn export_selection_cover(cover_src:&str,ver:&str){
    // 初始化系统
    unsafe { 
        let _ = Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res {
            Some(app)=>{
            // 保存封面
                let exp_opt = app.create_struct_export_options();
                exp_opt.put_SizeX(600);
                exp_opt.put_SizeY(600);
                exp_opt.put_maintainaspect(true);
                exp_opt.put_transparent(true);

                let exp_opt_info = exp_opt.to_variant();
                let pale_obj = app.create_struct_palette_options();
                let pale = pale_obj.to_variant();
                let res_doc = app.get_active_document();
                match res_doc {
                        Some(doc)=>{
                            println!("...........aaaaaa");
                            let res = doc.export(&cover_src,802,2,exp_opt_info,pale);
                        }
                        None=>{
                            println!("...........22222");
                            return;
                        }
                    }
            }
            None=>{
            }
        }
    }
}

// 保存封面图
pub fn export_cover(cover_src:String,ver:&str)->Option<()>{
    unsafe {
        let _ = Com::CoInitialize(None).expect("init com error");
        let app_res = IvgApplication::new(ver);
        match app_res {
            Some(app)=>{
            // 保存封面
            let exp_opt = app.create_struct_export_options();
            exp_opt.put_SizeX(600);
            exp_opt.put_SizeY(600);
            exp_opt.put_maintainaspect(true);
            exp_opt.put_transparent(true);
            let exp_opt_info = exp_opt.to_variant();
            let pale_obj = app.create_struct_palette_options();
            let pale = pale_obj.to_variant();
            let res_doc = app.get_active_document();
            match res_doc {
                    Some(doc)=>{
                        doc.export(&cover_src,802,1,exp_opt_info,pale);
                        return Some(());
                    }
                    None=>{
                        return None;
                    }
                }
            }
            None=>{
                return None;
            }
        }
        let _ = Com::CoUninitialize();
    }
}