use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use crate::dispatch::dispatch::ComObject;
use crate::coreldraw::types::*;
use crate::dispatch::utils::VariantExt;

pub struct IVGColorContext {
    disp:ComObject
}

impl IVGColorContext{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    
    fn get_RGBColorProfile(){

    }

    fn put_RGBColorProfile(){

    }

    fn get_CMYKColorProfile(){

    }

    fn put_CMYKColorProfile(){

    }

    fn get_GrayscaleColorProfile(){

    }

    fn put_GrayscaleColorProfile(){

    }

    fn get_RenderingIntent(){

    }

    fn put_RenderingIntent(){

    }

    fn get_BlendingColorModel(){

    }

    fn put_BlendingColorModel(){

    }    
    fn GetCopy(){

    }
    fn CopyAssign(){

    }
    fn get_ColorProfile(){

    }
    fn put_ColorProfile(){

    }
    fn Merge(){

    }

    fn IsSame(){

    }
    fn get_ColorProfiles(){

    }
    fn get_ReadOnly(){

    }
    fn get_ColorProfileNameList(){

    }

}