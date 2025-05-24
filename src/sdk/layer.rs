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
pub struct IvgLayer {
    disp:ComObject
}

impl IvgLayer{
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

    pub fn get_Name(&self)->Result<String,Error>{
        let res_name = self.disp.get_property("name");
        match res_name {
            Ok(var_name)=>{
                let name = var_name.to_string().unwrap();
                Ok(name)
            }
            Err(e)=>{
                return Err(e.into());
            }
        }
    }

    pub fn put_Name(&self)->String{
        let app_vers = self.disp.get_property("name").log_error("got error").unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }

    pub fn get_Shapes(&self)->Option<IvgShapes>{
        // 获取当前文档
        let doc_res = self.disp.get_property("shapes");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivg_layer = IvgShapes::new(disp.clone());
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

    pub fn do_Activate(&self){
        let res = self.disp.invoke_method("activate", vec![]);
        match res {
            Ok(res_disp)=>{
               return;
            }
            Err(e)=>{
                println!("{:?}",e);
                return ;
            }
        }
    }

    fn get_Visible(){

    }
    
    fn put_Visible(){

    }
    
    fn get_Printable(){

    }
    
    fn put_Printable(){

    }
    
    fn get_Editable(){

    }
    
    fn put_Editable(){

    }
    
    fn get_Master(){

    }
    
    fn put_Master(){

    }
    
    fn get_OverrideColor(){

    }
    
    fn put_OverrideColor(){

    }
    // 获取颜色
    pub fn get_Color(&self)->Option<IvgColor>{
        // 获取当前文档
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
    
    fn put_Color(){

    }
    
    pub fn do_Delete(&self){
        let res = self.disp.invoke_method("delete", vec![]);
        match res {
            Ok(res_disp)=>{
               return;
            }
            Err(e)=>{
                println!("{:?}",e);
                return ;
            }
        }
    }
    
    fn MoveAbove(){

    }
    
    fn MoveBelow(){

    }
    
    pub fn do_Import(&self,cur_ver:&str,src:&str)->Result<(), Error>{
        let mut params = Vec::new();
        // 文件参数
        let src_path = VARIANT::from_str(src);
        params.push(src_path);
        let type_num: VARIANT = VARIANT::from_i32(0);
        params.push(type_num);
        let import_option = ImportOption::new(cur_ver).unwrap();
        // import_option
        let vart_opt:VARIANT = import_option.get_opt().log_error("got options").unwrap();
        params.push(vart_opt);
        
        let res = self.disp.invoke_method("import", params);
        match res {
            Ok(res_disp)=>{
               return Ok(());
            }
            Err(e)=>{
                return Err(e.into());
            }
        }
    }
    
    fn CreateRectangle(){

    }
    
    fn CreateEllipse(){

    }
    
    fn CreatePolygon(){

    }
    
    fn CreateGridBoxes(){

    }
    
    fn CreateSpiral(){

    }
    
    fn CreateArtisticText(){

    }

    fn CreateParagraphText(){

    }

    fn CreateCurveSegment(){

    }

    fn CreateLineSegment(){

    }

    fn CreateConnector(){

    }

    fn CreateCurve(){

    }

    fn Paste(){

    }

    fn CreateGuideAngle(){

    }

    fn CreateGuide(){

    }

    fn CreateEllipse2(){

    }

    fn FindShape(){

    }

    fn FindShapes(){

    }

    fn CreateRectangle2(){

    }

    fn CreateFreeConnector(){

    }

    fn get_Properties(){

    }

    fn get_MasterProperties(){

    }

    pub fn get_Index(&self)->i32{
        let index = self.disp.get_property("index").unwrap();
        index.to_i32().unwrap()
    }

    fn CreateCurveSegment2(){

    }

    fn ImportEx(){

    }

    fn CreateArtisticTextWide(){

    }

    fn CreateParagraphTextWide(){

    }

    fn CustomCommand(){

    }

    fn CreateCustomShape(){

    }

    fn CreateLinearDimension(){

    }

    fn CreateAngularDimension(){

    }
    
    fn CreateSymbol(){

    }

    fn CreatePolygon2(){

    }

    fn PasteSpecial(){

    }

    fn CreateOLEObject(){

    }

    fn CreateOLEObjectFromFile(){

    }

    fn get_SelectableShapes(){

    }

    fn get_TreeNode(){

    }

    fn get_IsGuidesLayer(){

    }

    fn get_IsDesktopLayer(){

    }

    fn get_IsGridLayer(){

    }

    fn get_IsSpecialLayer(){

    }

    fn get_MasterLayer(){

    }
    fn get_AbsoluteIndex(){

    }

    fn get_Page(){

    }
    fn get_Above(){

    }
    fn get_Below(){

    }
    fn CreateRectangleRect(){

    }
    fn CreateEllipseRect(){

    }
    fn CreateConnectorEx(){

    }
    fn CreateRightAngleConnector(){

    }
    fn CreateBSpline(){

    }
    fn PasteEx(){

    }
    fn CreateToolShape(){

    }
    fn CreateBitmap(){

    }
    fn CreateBitmap2(){

    }
    fn CreateBitmapRect(){

    }

}