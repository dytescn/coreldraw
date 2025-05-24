use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct AlignProperties {
    obj:ComObject
}

impl AlignProperties{
    fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructAlignProperties", ALIGN_PROPERTIES_IID).expect("got app err");
       return  Self{
            obj:data
       };
    }
}

fn do_it_work(){
    unsafe { 
        Com::CoInitialize(None).expect("init com error");
        // let align = <ComObject as AlignProperties >::new();
        // app.do_quit();
        Com::CoUninitialize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {           
        do_it_work();
        println!("hello");
    }
}
