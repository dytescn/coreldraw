use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;

// 保存封面图
pub fn export_preview(preview_src:String,ver:&str)->Option<usize>{
    unsafe {
        let _ = Com::CoInitialize(None).expect("init com error");
        let app = IvgApplication::new(ver).unwrap();
        // 保存封面
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
            let src = format!("{}{}_preview.png",preview_src,temp_i);
            let cur_page = pages.get_item(i.clone().into());
            cur_page.activate();
            doc.export(&src,802,1,exp_opt_info.clone(),pale.clone());    
        } 
        active_page.activate();
        return Some(sum as usize);
    
        let _ = Com::CoUninitialize();
    }
}


// 保存封面图
pub fn export_preview_sum(ver:&str)->Option<usize>{
    unsafe {
        let _ = Com::CoInitialize(None).unwrap();
        let app = IvgApplication::new(ver).unwrap();
        let doc = app.get_active_document().expect("err info:");
        let active_page = doc.get_activepage().log_error("got active page err").unwrap();
        let pages = doc.get_pages();
        // let cur_page = pages.get
        let sum = pages.get_count();
        return Some(sum as usize);
        let _ = Com::CoUninitialize();
    }
}