use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct CreateOptions {
    obj:ComObject
}

impl CreateOptions {
    fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructCreateOptions", CREATE_OPTIONS_IID).expect("got app err");
       return  Self{
            obj:data
       };
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
    }
}
