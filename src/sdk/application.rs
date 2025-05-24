use std::ptr::null;
use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use windows::Win32::System::Variant;
use crate::ffi::app;
use crate::sdk::types::*;
use crate::sdk::document::IvgDocument;
use crate::sdk::struct_exportoption;
use crate::sdk::struct_paletteoptions;

use super::document::IvgDocuments;

pub struct IvgApplication {
    disp:ComObject
}

impl IvgApplication {
    pub fn new(ver:&str)-> Option<Self> {
        let id_name = "coreldraw.application.".to_string() + ver; // Can't use + with two &str
        let res_data=  ComObject::new_from_name(&id_name,APP_INTER_IID);
        match res_data {
           Some(data)=>{
                return Some(Self{
                    disp:data
                });
           }
           None=>{
                return None;
           }
       }
    }
    fn get_Visible(&self){

    }
    fn put_Visible(&self){

    }

    pub fn get_version(&self)->String{
        let app_vers_res = self.disp.get_property("version");
        if app_vers_res.is_err() {
            return "".to_string();
        }
        let app_vers = app_vers_res.unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }
    pub fn get_version_major(&self)->String{
        let app_vers = self.disp.get_property("versionmajor").log_error("got error").unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }
    pub fn get_version_minor(&self)->String{
        let app_vers = self.disp.get_property("versionminor").log_error("got error").unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }

    // 获取所有列表
    pub fn get_documents(&self)->IvgDocuments{
        let doc = self.disp.get_property("Documents").log_error("got error").unwrap();
        let disp = doc.to_idispatch().expect("change error").clone();
        IvgDocuments::new(disp)
    }

    // 获取当前文档
    pub fn get_active_document(&self)->Option<IvgDocument>{
        let doc_res = self.disp.get_property("ActiveDocument");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_doc = IvgDocument::new(disp.clone());
                        return Some(ivg_doc);
                    }
                    Err(e)=>{
                        return None
                    }
                }
            }
            Err(err)=>{
                return None
            }
        }
    }

    pub fn open_document(&self,src:String)->bool{
        let mut opts = Vec::new();
        let vart_file = <VARIANT as VariantExt>::from_str(src);
        opts.push(vart_file);
        let page_code = <VARIANT as VariantExt>::from_i64(0);
        opts.push(page_code);
       let res = self.disp.invoke_method("opendocument", opts);
       match res {
           Ok(_)=>{
                return true;
           }
           Err(e)=>{
                return  false;
           }
       }
    }

    fn create_document(){

    }

    fn get_ActivePage(){

    }
    // 获取当前激活窗口
    fn get_ActiveWindow() {

    }
    // 获取窗口
    fn get_Windows(){

    }
    // // 获取coreldraw的状态
    // fn get_status(&self){
    //     let app_vers = self.disp.get_property("versionmajor").expect("got err");
    //     app_vers.to_string().expect("got error")
    // }

    // 获取
    pub fn create_struct_export_options(&self)->struct_exportoption::ExportOptions{
        let opt = self.disp.invoke_method("CreateStructExportOptions", vec![]).expect("got err");
        let opt_disp = opt.to_idispatch().expect("change err");
        struct_exportoption::ExportOptions::from_disp(opt_disp.clone())
    }

    pub fn create_struct_palette_options(&self)->struct_paletteoptions::PaletteoptionOption{
        let opt = self.disp.invoke_method("CreateStructPaletteOptions", vec![]).expect("got err");
        let pale_disp = opt.to_idispatch().expect("change err");
        struct_paletteoptions::PaletteoptionOption::from_disp(pale_disp.clone())
    }
    // 获取安装目录
    pub fn get_SetupPath(&self)->String{
        let app_vers = self.disp.get_property("SetupPath").log_error("got error").unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }
    // 获取安装目录
    pub fn get_ProgramPath(&self)->String{
        let app_vers = self.disp.get_property("ProgramPath").log_error("got error").unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }
    // 执行退出
    fn do_quit(&self) {
        let app_vers = self.disp.invoke_method("quit", Vec::new()).expect("got err");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // do_quit();
        // let version = get_version();
        // println!("{:?}",version);
        // let app =  IvgApplication::new();
        // app.
    }
}
