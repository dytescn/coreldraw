use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct PasteOption {
    obj:ComObject
}

impl PasteOption {
    fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructPasteOptions", PASTE_OPTIONS_IID).expect("got app err");
       return  Self{
            obj:data
       };
    }
    fn get_ColorConversionOptions(){

    }
}