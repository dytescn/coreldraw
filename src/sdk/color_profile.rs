use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use crate::dispatch::dispatch::ComObject;
use crate::coreldraw::types::*;
use crate::dispatch::utils::VariantExt;

pub struct IVGColorProfile {
    disp:ComObject
}

impl IVGColorProfile{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }

    fn get_Name(){

    }

    fn get_FileName(){

    }

    fn get_Manufacturer(){

    }

    fn get_DeviceModel(){

    }

    fn get_DeviceType(){

    }

    fn get_Selected(){
        
    }
    
    fn Select(){

    }

    fn get_Generic(){

    }
    
    fn get_Installed(){

    }
    
    fn get_ID(){

    }
    
    fn get_ColorModel(){

    }
    
    fn CreateColorContext(){

    }

    fn IsSame(){

    }

}

pub struct IVGColorProfiles {
    disp:ComObject
}

impl IVGColorProfiles{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }

    fn get_Item(){

    }
    fn get_Count(){

    }
    fn get__NewEnum(){

    }

    fn get_DeviceType(){

    }
    fn SelectByName(){

    }
    fn Install(){

    }
    fn get_GenericProfile(){

    }

}