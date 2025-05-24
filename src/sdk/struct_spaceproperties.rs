use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;


pub struct SpaceProperties {
    obj:ComObject
}

impl SpaceProperties{
    fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructSpaceProperties", SPACE_PROPERTIES_IID).expect("got app err");
       return  Self{
        obj:data
       };
    }
    fn put_CharacterSpacing(){

    }
    fn get_CharacterSpacing(){
        
    }
    fn put_WordSpacing(){
        
    }
    fn get_WordSpacing(){
        
    }
    fn put_LineSpacing(){
        
    }
    fn get_LineSpacing(){
        
    }
    fn put_LineSpacingType(){
        
    }
    fn get_LineSpacingType(){
        
    }
    fn put_BeforeParagraphSpacing(){
        
    }
    fn get_BeforeParagraphSpacing(){
        
    }
    fn put_AfterParagraphSpacing(){
        
    }
    fn get_AfterParagraphSpacing(){
        
    }
    
}