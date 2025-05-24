use std::ptr::null;
use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use windows::Win32::System::Variant;
use crate::sdk::types::*;

pub struct IvgColor {
    disp:ComObject
}

impl IvgColor{
    pub fn new(ver:&str)->Option<Self> {
        let id_name = "coreldraw.color.".to_string() + ver; // Can't use + with two &str
        let res_data=  ComObject::new_from_name(&id_name, APP_INTER_IID);
        //    println!("{:?}",);
            match res_data {
                Some(data)=>{
                    return Some(Self{
                        disp:data
                    });
                }
                None=>{
                    return None;
                }
            }
    }
    // from
    pub fn from(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }

    fn get_Application(){

    }

    fn get_Parent(){

    }

    fn get_Type(){

    }

    fn RGBAssign(){

    }

    fn get_RGBRed(){

    }

    fn put_RGBRed(){

    }

    fn get_RGBGreen(){

    }

    fn put_RGBGreen(){

    }

    fn get_RGBBlue(){

    }

    fn put_RGBBlue(){

    }

    fn ConvertToRGB(){

    }

    fn CMYKAssign(){

    }

    fn get_CMYKCyan(){

    }

    fn put_CMYKCyan(){

    }

    fn get_CMYKYellow(){

    }

    fn put_CMYKYellow(){

    }

    fn get_CMYKMagenta(){

    }

    fn put_CMYKMagenta(){

    }

    fn get_CMYKBlack(){

    }

    fn put_CMYKBlack(){

    }

    fn ConvertToCMYK(){

    }

    fn CMYAssign(){

    }

    fn get_CMYCyan(){

    }

    fn put_CMYCyan(){

    }

    fn get_CMYMagenta(){

    }

    fn put_CMYMagenta(){

    }

    fn get_CMYYellow(){

    }
    fn put_CMYYellow(){

    }

    fn ConvertToCMY(){

    }

    fn HSBAssign(){

    }

    fn get_HSBHue(){

    }

    fn put_HSBHue(){

    }

    fn get_HSBSaturation(){

    }

    fn put_HSBSaturation(){

    }

    fn get_HSBBrightness(){

    }

    fn put_HSBBrightness(){

    }

    fn ConvertToHSB(){

    }

    fn HLSAssign(){

    }

    fn get_HLSHue(){

    }

    fn put_HLSHue(){

    }
    fn get_HLSLightness(){

    }

    fn put_HLSLightness(){

    }

    fn get_HLSSaturation(){

    }

    fn put_HLSSaturation(){

    }

    fn ConvertToHLS(){

    }

    fn BWAssign(){

    }

    fn get_BW(){

    }

    fn put_BW(){

    }

    fn ConvertToBW(){

    }

    fn GrayAssign(){

    }

    fn get_Gray(){

    }

    fn put_Gray(){

    }

    fn ConvertToGray(){

    }

    fn CorelScriptAssign(){

    }

    fn CorelScriptGetComponent(){

    }

    fn UserAssign(){

    }

    fn CopyAssign(){

    }

    fn get_Name(){

    }

    fn YIQAssign(){

    }
    fn get_YIQLuminanceY(){

    }

    fn put_YIQLuminanceY(){

    }

    fn get_YIQChromaI(){

    }

    fn put_YIQChromaI(){

    }

    fn get_YIQChromaQ(){

    }

    fn put_YIQChromaQ(){

    }

    fn ConvertToYIQ(){

    }

    fn LabAssign(){

    }

    fn get_LabLuminance(){

    }

    fn put_LabLuminance(){

    }

    fn get_LabComponentA(){

    }

    fn put_LabComponentA(){

    }

    fn get_LabComponentB(){

    }

    fn put_LabComponentB(){

    }

    fn ConvertToLab(){

    }

    fn RegistrationAssign(){

    }

    fn FixedAssign(){

    }

    fn get_PaletteID(){

    }

    fn get_PaletteIndex(){

    }

    fn put_PaletteIndex(){

    }

    fn get_Tint(){

    }

    fn put_Tint(){

    }

    fn ConvertToFixed(){

    }

    fn UserAssignEx(){

    }

    fn SetName(){

    }

    fn BlendWith(){

    }

    fn IsSame(){

    }

    fn get_IsInGamut(){

    }
    fn get_InGamutColor(){

    }

    fn get_IsCMYK(){

    }

    fn get_IsGray(){

    }

    fn get_IsWhite(){

    }

    fn get_IsSpot(){

    }

    fn get_IsTintable(){

    }

    fn get_IsValidDuotoneColor(){

    }

    fn get_ValidDuotoneColor(){

    }

    fn GetColorDistanceFrom(){

    }

    fn IsSimilar(){

    }

    fn ToString(){

    }

    fn StringAssign(){

    }

    fn Invalidate(){

    }

    fn get_HexValue(){

    }

    fn put_HexValue(){

    }

    fn GetCopy(){

    }

    pub fn get_RGBValue(&self)->String{
        let app_vers = self.disp.get_property("rgbvalue").expect("got err");
        app_vers.to_string().log_error("got error").unwrap()
    }

    fn put_RGBValue(){

    }

    fn CopyAppearance(){

    }

    fn get_ColorContext(){

    }

    fn ConvertTo(){

    }

    fn get_Palette(){

    }

    fn SpotAssign(){
        
    }

    fn SpotAssignByName(){

    }

    fn ConvertToPalette(){

    }
    fn get_SpotColorID(){

    }
    
    fn put_SpotColorID(){

    }

    fn get_SpotColorName(){

    }

    fn PaletteAssign(){

    }

    fn get_PaletteIdentifier(){

    }

    fn get_IsColorStyle(){

    }

    fn get_ColorStyleName(){

    }

    fn ModifyColorStyleColor(){

    }
}
