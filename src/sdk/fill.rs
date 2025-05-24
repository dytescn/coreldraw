use std::io::Error;
use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;
use crate::sdk::enum_filter;
use crate::sdk::struct_importoption::ImportOption;
use crate::sdk::color::IvgColor;

use super::shape::IvgShape;
use super::shapes::IvgShapes;
pub struct IvgFill {
    disp:ComObject
}

impl IvgFill{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    fn get_Type(){}
    pub fn get_UniformColor(&self)->Option<IvgColor>{
        let doc_res = self.disp.get_property("UniformColor");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivgcolor = IvgColor::from(disp.clone());
                        return Some(ivgcolor);
                    }
                    Err(e)=>{
                        return None
                    }
                }
            }
            Err(err)=>{
                return None
            }
        }
    }
    fn put_UniformColor(&self,color:IvgColor){}
    fn get_Fountain(){}
    fn put_Fountain(){}
    fn get_Pattern(){}
    fn put_Pattern(){}
    fn get_Texture(){}
    fn put_Texture(){}
    fn get_PostScript(){}
    fn put_PostScript(){}
    fn ApplyNoFill(){}
    fn ApplyUniformFill(){}
    fn ApplyFountainFill(){}
    fn ApplyPatternFill(){}
    fn ApplyTextureFill(){}
    fn ApplyPostscriptFill(){}
    fn GetCopy(){}
    fn CopyAssign(){}
    fn UserAssign(){}
    fn get_PSScreen(){}
    fn get_Hatch(){}
    fn put_Hatch(){}
    fn CompareWith(){}
    fn ApplyCustomHatchFill(){}
    fn ApplyHatchFill(){}
    fn ToString(){}
    fn StringAssign(){}
}