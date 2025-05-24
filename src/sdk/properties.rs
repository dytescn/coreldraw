use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

use super::layer::IvgLayer;

pub struct IvgProperties{
    disp:ComObject
}

impl IvgProperties{
    
    fn get_Item(){}
    fn put_Item(){}
    fn get__NewEnum(){}
    fn get_Index(&self)->i32{
        let vart = self.disp.get_property("index").expect("got active page error:");
        vart.to_i32().unwrap()
    }
    fn get_ItemByIndex(){}
    fn Delete(){}
    fn DeleteByIndex(){}
    pub fn get_Count(&self)->i32{
        let vart = self.disp.get_property("count").expect("got active page error:");
        vart.to_i32().unwrap()
    }
    fn Description(){

    }
    fn PutFile(){

    }
    fn GetFile(){

    }
    fn Exists(){

    }
    fn DeleteAll(){

    }
    fn GetPoint(){

    }
    fn SetPoint(){

    }
    fn GetVector(){

    }
    fn SetVector(){

    }
    fn GetCurve(){

    }
    fn SetCurve(){

    }
}
