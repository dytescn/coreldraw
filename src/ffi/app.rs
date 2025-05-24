use windows::Win32::System::Com;
use crate::sdk::application::IvgApplication;
use crate::sdk::document::IvgDocument;

// 获取coreldraw信息
pub fn do_cdr_app_open(name:String,cdr_src:String,cur_version:String)->bool{
    unsafe{
        // 初始化系统
        let con_res = Com::CoInitialize(None);
        if con_res.is_err() {
            return false;
        }
        let app_res = IvgApplication::new(&cur_version);
        match app_res {
            Some(app)=>{
                // 打开文件
                let res = app.open_document(cdr_src);
                if res {
                    // 修改文件名称
                    let doc = app.get_active_document().unwrap();
                    let res = doc.put_name(&name);
                    return true;   
                }
                return false;
            }
            None=>{
                return false;
            }
        }
        Com::CoUninitialize();
    }
}

// 获取coreldraw信息
pub fn get_cdr_app_info(cur_version:String)->bool{
    unsafe{
        // 初始化系统
        let con_res = Com::CoInitialize(None);
        if con_res.is_err() {
            return false;
        }
    }
    let app = IvgApplication::new(&cur_version).unwrap();
    let fullname = app.get_SetupPath();
    println!("{:?}",fullname);
    let program = app.get_ProgramPath();
    println!("{:?}",program);
    return true;
    unsafe{
        Com::CoUninitialize();
    }
    return true;
}

// 获取执行路径
pub fn get_app_execute_path(cur_version:String)->String{
    unsafe{
        // 初始化系统
        let con_res = Com::CoInitialize(None);
        if con_res.is_err() {
            return "".to_string();
        }
    }
    let app_res = IvgApplication::new(&cur_version);
    if app_res.is_none() {
        return "".to_string();
    }
    let app = app_res.unwrap();
    let program_path = app.get_ProgramPath();
    let execute_path = format!("{}CorelDRW.exe",program_path);
    unsafe{
        Com::CoUninitialize();
    }
    return execute_path;
    
}