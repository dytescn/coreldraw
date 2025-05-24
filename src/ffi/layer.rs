use std::fs;
use std::path::Path;
use dyteslogs::logs::LogError;
use windows::Win32::System::Com;
use crate::ffi::layer;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::page;
use crate::sdk::struct_importoption::ImportOption;
pub fn create_qrcode_layer(ver:&str,src:&str)->bool{
    unsafe{
        Com::CoInitialize(None).log_error("init com error").unwrap();
        let app = IvgApplication::new(&ver).log_error("init application error").unwrap();
        if let Some(doc) = app.get_active_document().log_error("init doc err"){
            let pages = doc.get_pages();
            let cur_page = pages.get_item(1);
            let layer_res = cur_page.createlayer();
            match layer_res{
                Ok(layer)=>{
                    let src_path = fs::canonicalize(src).unwrap();
                    let src_str = src_path.to_str().unwrap();
                    let name = layer.do_Import(ver,src_str).log_error("got err").unwrap();
                }
                Err(e)=>{
                    println!("{:?}",e);
                }
            }
        }
        Com::CoUninitialize();
    }
    return true;
}
