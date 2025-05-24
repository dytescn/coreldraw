use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
use crate::sdk::struct_importoption::ImportOption;
pub fn import_qrcode(ver:&str)->bool{
    unsafe{
        // 初始化系统
        Com::CoInitialize(None).expect("init com error");
        let imp_res = ImportOption::new(ver);
        match imp_res{
            Some(app)=>{
                // let src ="./cache/123123.cdr";
                // let doc_res = app.open_document(src.to_string());
                // println!("{:?}",doc_res);
                // match doc_res{
                //     Some(doc)=>{
                //         return true;
                //     }
                //     None=>{
                //         return false;
                //     }
                // }
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