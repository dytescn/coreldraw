use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct IvgImage {
    disp:ComObject
}

impl IvgImage{
    fn get_Type(){

    }

    fn get_Width(){

    }

    fn get_Height(){

    }

    fn get_Pixel(){

    }

    fn GetCopy(){

    }

    fn get_ReadOnly(){

    }

    fn get_Tiles(){

    }

    fn Blit(){

    }

    fn FillArea(){

    }

    fn FlipArea(){

    }
}

pub struct IvgImageTitles {
    disp:ComObject
}

impl IvgImageTitles{
    fn get_Count(){

    }

    fn get_Item(){

    }

    fn get__NewEnum(){

    }

    fn get_First(){

    }

    fn get_Last(){

    }
}

pub struct IvgImageTitle {
    disp:ComObject
}

impl IvgImageTitle{
    fn get_Left(){

    }

    fn get_Top(){

    }

    fn get_Right(){

    }

    fn get_Bottom(){

    }

    fn get_Width(){

    }

    fn get_Height(){

    }

    fn get_BytesPerTile(){

    }

    fn get_BytesPerLine(){

    }
    
    fn get_BytesPerPixel(){

    }
    
    fn get_PixelData(){

    }
    
    fn put_PixelData(){

    }
    
    fn get_ReadOnly(){

    }
}