use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::pages;

// 获取coreldraw信息
pub fn do_cdr_file_save(name:String,cover_src:String,preview_src:String,file_src:String,cur_ver:u16,ver:&str)->usize{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app_res = IvgApplication::new(ver);
        match app_res {
            Some(app)=>{
                let doc = app.get_active_document();
                save_cover(cover_src,ver);
                let sum = save_preview(preview_src,ver);
                save_file(file_src,name,cur_ver,ver);
                return sum;
            }
            None=>{
                return 0;
            }
        }
        Com::CoUninitialize();
        return  0;
    }
}
// 保存封面图
fn save_cover(cover_src:String,ver:&str){
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
                    }
                    None=>{
                        return;
                    }
                }
            }
        None=>{

        }
    }
}
// 保存预览图
fn save_preview(preview_src:String,ver:&str)->usize{
    let app_res = IvgApplication::new(ver);
    // 保存封面
    match app_res {
        Some(app)=>{
            let exp_opt = app.create_struct_export_options();
            exp_opt.put_SizeX(2000);
            exp_opt.put_SizeY(2000);
            exp_opt.put_maintainaspect(true);
            exp_opt.put_transparent(true);    
            let exp_opt_info = exp_opt.to_variant();
            let pale_obj = app.create_struct_palette_options();
            let pale = pale_obj.to_variant();
            let doc = app.get_active_document().expect("err info:");
            let active_page = doc.get_activepage().log_error("got active page err").unwrap();
            let pages = doc.get_pages();
            // let cur_page = pages.get
            let sum = pages.get_count();
            for i in  1..=sum{
                let temp_i = i.clone().to_string();
                let src = preview_src.to_owned()+"_"+temp_i.as_str()+".png";
                let cur_page = pages.get_item(i.clone().into());
                cur_page.activate();
                doc.export(&src,802,1,exp_opt_info.clone(),pale.clone());    
            } 
            active_page.activate();
            return sum as usize;
        }
        None=>{
            return 0;
        }
    }
}
// 保存文件
fn save_file(file_src:String,name:String,cur_ver:u16,ver:&str){
    let app_res = IvgApplication::new(ver);
    match app_res {
        Some(app)=>{
           if let Some(doc) = app.get_active_document().log_error("init get active document err"){
                // doc.put_fullfilename(&file_src);  // 修改文件路径
                //  doc.save_as(&file_src,cur_ver);
                // 获取文件路径，
                let doc_file_path = doc.get_fullfilename();
                if doc_file_path=="" {
                    doc.put_fullfilename(&file_src);  // 修改文件路径
                    // doc.save();
                    doc.save_as(&file_src,cur_ver);
                    doc.put_name(&name);
                    return;
                }
                // 文件保存
                doc.save();
                let _ = std::fs::copy(doc_file_path, file_src).log_error("copy file error").unwrap();
                doc.put_name(&name);
           }
        }
        None=>{
            return
        }
    }
}
