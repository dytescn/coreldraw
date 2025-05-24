use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use dyteslogs::logs::LogError;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;
pub struct SaveOptions{
    obj:ComObject
}

impl SaveOptions {
    pub fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructSaveAsOptions", SAVE_OPTIONS_IID).log_error("got app err").unwrap();
       return  Self{
          obj:data
       };
    }
    fn put_Filter(&self){

    }
    fn get_Filter(&self){

    }
    // 修改版本
    pub fn put_Version(&self,value:u16){
            let mut vart_vec = Vec::new();
            let vart_name = <VARIANT as VariantExt>::from_u16(16);
            vart_vec.push(vart_name);
            self.obj.set_property("version", vart_vec);
    }

    // 获取当前版本
    pub fn get_Version(&self)->String{
        let cur_vers = self.obj.get_property("version").log_error("got err").unwrap();
        cur_vers.to_string().log_error("got error").unwrap()
    }

    fn put_ThumbnailSize(&self){

    }
    fn get_ThumbnailSize(&self){

    }
    fn put_Range(&self){

    }
    fn get_Range(&self){

    }
    fn put_Overwrite(&self){

    }
    fn get_Overwrite(&self){

    }
    fn put_EmbedICCProfile(&self){

    }

    fn get_EmbedICCProfile(&self){

    }

    fn put_EmbedVBAProject(&self){

    }
    fn get_EmbedVBAProject(&self){

    }
    fn put_IncludeCMXData(&self){
        
    }
    fn get_IncludeCMXData(&self){

    }
    fn put_KeepAppearance(&self){

    }
    fn get_KeepAppearance(&self){

    }

    pub fn get_opt(&self)->VARIANT{
        self.obj.get_variant().expect("got variant err:")
    }
}