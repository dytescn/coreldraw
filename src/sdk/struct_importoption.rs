use std::io::Error;

use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use windows::Win32::System::Variant::VARIANT;
use crate::sdk::types::*;

pub struct ImportOption {
    disp:ComObject
}

impl ImportOption {
    pub fn new(ver:&str)-> Option<Self> {
        let id_name = "CorelDRAW.StructImportOptions.".to_string() + ver; // Can't use + with two &str
       let res_data=  ComObject::new_from_name(&id_name, IMPORT_OPTIONS_IID);
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
    fn get_LinkBitmapExternally(){

    }
    
    fn put_LinkBitmapExternally(){

    }

    pub fn get_CombineMultilayerBitmaps(&self)->bool{
        let vart = self.disp.get_property("CombineMultilayerBitmaps").log_error("got mode err:").unwrap();
        let res = vart.to_bool().log_error("got u16 err").unwrap();
        return res;
    }

    fn put_CombineMultilayerBitmaps(){

    }

    fn get_ExtractICCProfile(){

    }

    fn put_ExtractICCProfile(){

    }

    fn get_ICCFileName(){

    }

    fn put_ICCFileName(){

    }
    pub fn get_MaintainLayers(&self)->bool{
        let vart = self.disp.get_property("MaintainLayers").log_error("got mode err:").unwrap();
        let res = vart.to_bool().log_error("err").unwrap();
        return res;
    }
    pub fn put_MaintainLayers(&self,value:bool)->bool{
        // let para = var
        let mut para = Vec::new();
        let vart_y = VARIANT::from_bool(value);
        para.push(vart_y);
        let vart = self.disp.set_property("MaintainLayers",para).log_error("got mode err:").unwrap();
        return true;
    }
    fn get_UseOPILinks(){

    }
    fn put_UseOPILinks(){

    }
    fn get_DetectWatermark(){

    }
    fn put_DetectWatermark(){

    }
    pub fn get_Mode(&self)->u16{
        let vart = self.disp.get_property("mode").log_error("got mode err:").unwrap();
        let num = vart.to_u16().log_error("got u16 err").unwrap();
        return num;
    }
    fn put_Mode(){

    }
    fn get_CustomData(){

    }
    fn put_CustomData(){

    }
    fn get_ImageIndex(){

    }
    fn put_ImageIndex(){

    }
    fn get_CropHandler(){

    }
    fn putref_CropHandler(){

    }
    fn get_ResampleHandler(){

    }
    fn putref_ResampleHandler(){

    }
    fn get_TextFormatting(){

    }
    fn put_TextFormatting(){

    }
    fn get_TableOutline(){

    }
    fn put_TableOutline(){

    }
    fn get_CodePage(){

    }
    fn put_CodePage(){

    }
    fn get_ResampleWidth(){

    }
    fn put_ResampleWidth(){

    }
    fn get_ResampleHeight(){

    }
    fn put_ResampleHeight(){

    }
    fn get_CropLeft(){

    }
    fn put_CropLeft(){

    }
    fn get_CropTop(){

    }
    fn put_CropTop(){

    }
    fn get_CropWidth(){

    }
    fn put_CropWidth(){

    }
    fn get_CropHeight(){

    }
    fn put_CropHeight(){

    }
    fn get_ResampleDpiX(){

    }
    fn put_ResampleDpiX(){

    }
    fn get_ResampleDpiY(){

    }
    fn put_ResampleDpiY(){

    }
    fn get_ForceCMYKBlackText(){

    }
    fn put_ForceCMYKBlackText(){

    }
    fn get_ConvertTablesToText(){

    }
    fn put_ConvertTablesToText(){

    }
    fn get_TableColumnDelimiter(){

    }
    fn put_TableColumnDelimiter(){

    }
    fn get_ColorConversionOptions(){

    }
    pub fn get_opt(&self)->Result<VARIANT,Error>{
        let res = self.disp.get_variant();
        match res {
            Ok(var)=>{
                Ok(var)
            }
            Err(e)=>{
                Err(e.into())
            }
        }
    }
}