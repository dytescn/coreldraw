use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::struct_importoption::ImportOption;
pub fn import_produce_tpl(ver:&str)->bool{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).expect("init com error");
        let imp_res = ImportOption::new(ver);
        match imp_res{
            Some(app)=>{
                println!("..............");
                return  false;
            }
            None=>{
                return false;
            }
        }
        Com::CoUninitialize();
    }
}