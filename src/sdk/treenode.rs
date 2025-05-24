use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct IVGTreeNode {
    disp:ComObject
}

impl IVGTreeNode{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    fn get_Type() {
        
    }
    fn get_ShapeType(){
        
    }
    fn get_Shape(){
        
    }
    fn get_VirtualShape(){
        
    }
    fn get_Page(){
        
    }
    fn get_Layer(){
        
    }
    fn get_Document(){
        
    }
    
    fn get_Next(){
        
    }
    fn get_Previous(){
        
    }
    fn get_Parent(){
        
    }
    fn get_FirstChild(){
        
    }
    fn get_LastChild(){
        
    }
    fn get_Children(){
        
    }
    fn get_IsGroupChild(){
        
    }
    fn get_Selected(){
        
    }
    fn get_NextSelected(){
        
    }
    fn UnLink(){
        
    }
    fn LinkBefore(){
        
    }
    fn LinkAfter(){
        
    }
    
    fn LinkAsChildOf(){
        
    }
    fn MoveToFirst(){
        
    }
    fn MoveToLast(){
        
    }
    fn MoveBefore(){
        
    }
    
    fn MoveAfter(){
        
    }
    fn IsDescendentOf(){
        
    }
    fn Delete(){
        
    }
    fn GetCopy(){
        
    }
    fn SwapData(){
        
    }
    fn SwapGroupData(){
        
    }
    fn get_Handle(){
        
    }
    fn get_Name(){
        
    }
    fn put_Name(){

    }
}
