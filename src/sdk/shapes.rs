use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

use super::shape::IvgShape;

pub struct IvgShapes {
    disp:ComObject
}

impl IvgShapes{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }

    // 获取数量
    pub fn get_Count(&self)->i32{
        let app_vers = self.disp.get_property("count").log_error("got error").unwrap();
        app_vers.to_i32().log_error("got error").unwrap()
    }
    
    // 获取第一个
    pub fn get_First(&self)->Option<IvgShape>{
        // 获取当前文档
        let doc_res = self.disp.get_property("top");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_layer = IvgShape::new(disp.clone());
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
    
    //获取指定图形
    pub fn get_Item(&self,cur:i32)->IvgShape{
        // 获取当前文档
        let i32_cur =  cur;
        let param = VARIANT::from_i32(cur);
        let res_vart = self.disp.get_property_by_vart("item",param).expect("got page error");
        let page_disp = res_vart.to_idispatch().unwrap();
        IvgShape::new(page_disp.clone())
    }	
    
    // 获取最后一个图形
    pub fn get_Last(&self)->Option<IvgShape>{
        // 获取当前文档
        let doc_res = self.disp.get_property("last");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_layer = IvgShape::new(disp.clone());
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
    }	// Returns the last shape in collection
}