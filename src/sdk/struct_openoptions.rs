use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct OpenOptions {
    obj:ComObject
}

impl OpenOptions {
    fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructOpenOptions", OPEN_OPTIONS_IID).expect("got app err");
       return  Self{
          obj:data
       };
    }
    fn get_CodePage(){

    }
    fn put_CodePage(){

    }
    fn get_ColorConversionOptions(){
        
    }
}