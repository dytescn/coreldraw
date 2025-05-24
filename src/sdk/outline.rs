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
pub struct IvgOutLine {
    disp:ComObject
}

impl IvgOutLine{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    pub fn get_Width(&self)->String{
        let app_vers = self.disp.get_property("width").expect("got err");
        app_vers.to_string().log_error("got error").unwrap()
    }
    fn put_Width(){}
    
    pub fn get_Color(&self)->Option<IvgColor>{
        let doc_res = self.disp.get_property("color");
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
    fn put_Color(){}
    fn ConvertToObject(){}
    fn get_Type(){}
    fn put_Type(){}
    fn get_Style(){}
    fn put_Style(){}
    fn get_StartArrow(){}
    fn put_StartArrow(){}
    fn get_EndArrow(){}
    fn put_EndArrow(){}
    fn get_NibStretch(){}
    fn put_NibStretch(){}
    fn get_NibAngle(){}
    fn put_NibAngle(){}
    fn get_BehindFill(){}
    fn put_BehindFill(){}
    fn get_LineCaps(){}
    fn put_LineCaps(){}
    fn get_LineJoin(){}
    fn put_LineJoin(){}
    fn get_ScaleWithShape(){}
    fn put_ScaleWithShape(){}
    fn SetProperties(){}
    fn get_Size(){}
    fn put_Size(){}
    fn GetCopy(){}
    fn CopyAssign(){}
    fn UserAssign(){}
    fn get_PSScreen(){}
    fn CompareWith(){}
    fn get_DashDotLength(){}
    fn put_DashDotLength(){}
    fn ToString(){}
    fn StringAssign(){}
    pub fn get_PenWidth(&self)->String{
        let app_vers = self.disp.get_property("penwidth").expect("got err");
        app_vers.to_string().log_error("got error").unwrap()
    }
    fn put_PenWidth(){}
    fn get_MiterLimit(){}
    fn put_MiterLimit(){}
    fn SetNoOutline(){}
    fn get_StartArrowOptions(){}
    fn put_StartArrowOptions(){}
    fn get_EndArrowOptions(){}
    fn put_EndArrowOptions(){}
    fn get_Justification(){}
    fn put_Justification(){}
    fn SetPropertiesEx(){}
    fn get_AdjustDashes(){}
    fn put_AdjustDashes(){}
}