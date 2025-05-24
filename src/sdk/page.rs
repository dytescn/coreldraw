use std::io::Error;

use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

use super::layer::IvgLayer;
use super::layers::IvgLayers;

pub struct IvgPage {
    disp:ComObject
}

impl IvgPage{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, PAGE_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    pub fn get_name(&self)->String{
        let name = self.disp.get_property("name").expect("got error");
        name.to_string().unwrap()
    }
    fn put_name(){
        
    }
    fn get_Application(){

    }
    fn get_Parent(){

    }
    fn get_Layers(){

    }
    fn get_Shapes(){

    }
    pub fn get_activeLayer(&self)->Option<IvgLayer>{
              // 获取当前文档
              let doc_res = self.disp.get_property("ActiveLayer");
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
    fn get_paper(){

    }
    fn get_sizeWidth(){

    }
    fn put_sizeWidth(){

    }
    fn get_sizeHeight(){

    }
    fn put_sizeHeight(){

    }
    fn get_resolution(){

    }
    fn put_resolution(){

    }
    fn get_bleed(){

    }
    fn put_bleed(){

    }
    fn get_orientation(){

    }
    fn put_orientation(){

    }
    pub fn get_index(&self)->u8{
        let index = self.disp.get_property("index").expect("got error");
        index.to_u8().unwrap()
    }
    pub fn activate(&self){
        self.disp.invoke_method("activate", vec![]).unwrap();
    }
    fn delete(){

    }
    pub fn createlayer(&self)->Result<IvgLayer,Error>{
        let mut params = Vec::new();
        let name = VARIANT::from_str("fx");
        params.push(name);
        let res = self.disp.invoke_method("createlayer", params);
        match res {
            Ok(res_disp)=>{
                // return true;
                match res_disp.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_layer = IvgLayer::new(disp.clone());
                        return Ok(ivg_layer);
                    }
                    Err(e)=>{
                        return Err(e.into());
                    }
                }
            }
            Err(e)=>{
                println!("{:?}",e);
                return Err(e.into());
            }
        }
    }
    fn textfind(){

    }
    fn textreplace(){

    }
    fn selectshapesatpoint(){

    }
    fn selectshapesfromrectangle(){

    }
    fn get_background(){

    }
    fn put_background(){

    }
    fn get_color(){

    }
    fn put_color(){

    }
    fn get_printexportbackground(){

    }
    fn put_printexportbackground(){

    }
    fn get_guides(){

    }
    fn findshape(){

    }
    fn findshapes(){

    }
    fn moveto(){

    }
    fn unlockallshapes(){

    }
    fn get_properties(){

    }
    fn getsize(){

    }
    fn setsize(){

    }
    fn get_centerX(){

    }
    fn get_centerY(){

    }
    fn customcommand(){

    }
    fn get_previous(){

    }
    fn get_next(){

    }
    fn get_selectableshapes(){

    }
    fn get_treenode(){

    }
    fn getcenterposition(){

    }
    fn selectsize(){

    }
    fn get_guideslayer(){

    }
    fn get_desktoplayer(){

    }
    fn get_gridlayer(){

    }
    fn getboundingbox(){

    }
    fn get_left_x(){

    }
    fn get_right_x(){

    }
    fn get_bottom_y(){

    }
    fn get_top_y(){

    }
    pub fn get_all_layers(&self)->Option<IvgLayers>{
              // 获取当前文档
              let doc_res = self.disp.get_property("alllayers");
              match doc_res {
                  Ok(doc)=>{
                      match doc.to_idispatch(){
                          Ok(disp)=>{
                              let ivg_layer = IvgLayers::new(disp.clone());
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
    fn get_bounding_box(){

    }
    fn get_spread(){

    }
    fn findclosest_snap_point(){

    }
    fn get_objects_bounding_box(){

    }
    fn find_shape_at_point(){

    }

}