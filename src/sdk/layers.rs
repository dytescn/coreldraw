use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

use super::layer::IvgLayer;

pub struct IvgLayers {
    disp:ComObject
}

impl IvgLayers{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    
    fn get_Application(){

    }
    fn get_Parent(){

    }
    pub fn get_Item(&self,cur:u32)->IvgLayer{
        let i32_cur = cur as i32;
        let param = VARIANT::from_i32(i32_cur);
        let res_vart = self.disp.get_property_by_vart("item",param).expect("got page error");
        let page_disp = res_vart.to_idispatch().unwrap();
        IvgLayer::new(page_disp.clone())
    }
    fn get__NewEnum(){

    }
    pub fn get_Count(&self)->u32{
        let app_vers = self.disp.get_property("count").log_error("got error").unwrap();
        app_vers.to_u32().log_error("got error").unwrap()
    }
    fn Find(){

    }
    pub fn get_Top(&self)->Option<IvgLayer>{
        // 获取当前文档
        let doc_res = self.disp.get_property("top");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_layer = IvgLayer::new(disp.clone());
                        return Some(ivg_layer);
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
    pub fn get_Bottom(&self)->Option<IvgLayer>{
        // 获取当前文档
        let doc_res = self.disp.get_property("bottom");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_layer = IvgLayer::new(disp.clone());
                        return Some(ivg_layer);
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
}