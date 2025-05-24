use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct IVGTransparency {
    disp:ComObject
}

impl IVGTransparency{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    fn get_Application(){}
    fn get_Parent(){}
    pub fn get_Type(&self)->u8{
        let app_vers = self.disp.get_property("type").expect("got err");
        app_vers.to_u8().log_error("got error").unwrap()        
    }
    pub fn get_Uniform(&self)->i32{
        let app_vers = self.disp.get_property("Uniform").expect("got err");
        app_vers.to_i32().log_error("got error").unwrap()
    }
    fn put_Uniform(){}
    fn get_Fountain(){}
    fn put_Fountain(){}
    fn get_Pattern(){}
    fn put_Pattern(){}
    fn get_Texture(){}
    fn put_Texture(){}
    fn get_Start(){}
    fn put_Start(){}
    fn get_End(){}
    fn put_End(){}
    fn get_Frozen(){}
    fn ApplyNoTransparency(){}
    fn ApplyUniformTransparency(){}
    fn ApplyFountainTransparency(){}
    fn ApplyPatternTransparency(){}
    fn ApplyTextureTransparency(){}
    fn Freeze(){}
    fn Unfreeze(){}
    fn get_AppliedTo(){}
    fn put_AppliedTo(){}
    fn get_MergeMode(){}
    fn put_MergeMode(){}
    fn UserAssign(){}
}