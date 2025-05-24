use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;
pub fn convert_to_curve_status(ver:&str)->bool{
    unsafe{
        // 初始化系统
        let res = Com::CoInitialize(None);
        if res.is_err() {
            return false;
        }
        let app_res = IvgApplication::new(ver);
        match app_res{
            Some(app)=>{
                let res = app.get_active_document().unwrap();
                
                return true;
            }
            None=>{
                return false;
            }
        }
        Com::CoUninitialize();
    }
}
