use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct PaletteoptionOption {
    obj:ComObject
}

impl PaletteoptionOption {
    pub fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructPaletteOptions", PALETTE_OPTIONS_IID).expect("got app err");
       return  Self{
            obj:data
        };
    }
    pub fn from_disp(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, PALETTE_OPTIONS_IID).expect("init core error");
        Self{
            obj:obj
        }
    }
    fn put_NumColors(){
        
    }
    fn get_NumColors(){
        
    }
    fn put_DitherIntensity(){
        
    }
    fn get_DitherIntensity(){
        
    }
    fn put_Smoothing(){
        
    }
    fn get_Smoothing(){
        
    }
    fn put_DitherType(){
        
    }
    fn get_DitherType(){
        
    }
    fn put_PaletteType(){
        
    }
    fn get_PaletteType(){
        
    }
    fn put_Importance(){
        
    }
    fn get_Importance(){
        
    }
    fn put_Lightness(){
        
    }
    fn get_Lightness(){
        
    }
    fn put_ToleranceA(){
        
    }
    fn get_ToleranceA(){
        
    }
    fn put_ToleranceB(){
        
    }
    fn get_ToleranceB(){
        
    }
    fn put_ColorSensitive(){
        
    }
    fn get_ColorSensitive(){
        
    }
    fn put_TargetColor(){
        
    }
    fn get_TargetColor(){
        
    }
    fn put_Palette(){
        
    }
    fn get_Palette(){

    }
    fn get(){
        
    }
    pub fn to_variant(&self)->VARIANT{
        self.obj.get_variant().expect("got variant err:")
    }
}