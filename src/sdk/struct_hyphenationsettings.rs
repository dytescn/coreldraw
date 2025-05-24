use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct HyphenationSettings {
    obj:ComObject
}

impl HyphenationSettings{
    fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructHyphenationSettings", HYPHENATION_SETTINGS_IID).expect("got app err");
       return  Self{
         obj:data
       };
    }
}