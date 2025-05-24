use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct ExportOptions {
    obj:ComObject
}

impl ExportOptions{
    pub fn new()-> Self {
       let data=  ComObject::new_from_name("CorelDRAW.StructExportOptions", EXPORT_OPTIONS_IID).expect("got app err");
       return  Self{
            obj:data
       };
    }
    pub fn from_disp(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, EXPORT_OPTIONS_IID).expect("init core error");
        Self{
            obj:obj
        }
    }
    pub fn put_SizeX(&self,x:i32){
        let mut vart_vec = Vec::new();
        let vart_x = <VARIANT as VariantExt>::from_i32(x);
        vart_vec.push(vart_x);
        self.obj.set_property("sizex",vart_vec).expect("got error");        
    }
    pub fn get_SizeX(&self)->i32{
        let x = self.obj.get_property("sizex").expect("got error");
        return x.to_i32().unwrap();
    }
    pub fn put_SizeY(&self,y:i32){
        let mut vart_vec = Vec::new();
        let vart_y = <VARIANT as VariantExt>::from_i32(y);
        vart_vec.push(vart_y);
        self.obj.set_property("sizey",vart_vec).expect("got error");   
    }
    pub fn get_SizeY(&self)->i32{
        let y = self.obj.get_property("sizey").expect("got error");
        return y.to_i32().unwrap();
    }
    pub fn put_ResolutionX(&self,x:i32){
        let mut vart_vec = Vec::new();
        let vart_x = <VARIANT as VariantExt>::from_i32(x);
        vart_vec.push(vart_x);
        self.obj.set_property("resolutionx",vart_vec).expect("got error");        
    }
    pub fn get_ResolutionX(&self)->i32{
        let y = self.obj.get_property("resolutionx").expect("got error");
        return y.to_i32().unwrap();
    }
    pub fn put_ResolutionY(&self,y:i32){
        let mut vart_vec = Vec::new();
        let vart_y = <VARIANT as VariantExt>::from_i32(y);
        vart_vec.push(vart_y);
        self.obj.set_property("resolutiony",vart_vec).expect("got error");        
    }
    pub fn get_ResolutionY(&self)->i32{
        let y = self.obj.get_property("resolutiony").expect("got error");
        return y.to_i32().unwrap();
    }
    fn put_AntiAliasingType(&self){

    }
    fn get_AntiAliasingType(&self){

    }
    fn put_Overwrite(&self){

    }
    fn get_Overwrite(&self){

    }
    fn put_imagetype(&self){

    }
    fn get_imagetype(&self){


    }
    fn put_Dithered(&self){

    }
    fn get_dithered(&self){

    }
    pub fn put_transparent(&self,tran:bool){
        let mut vart_vec = Vec::new();
        let vart_tran = VARIANT::from_bool(tran);
        vart_vec.push(vart_tran);
        self.obj.set_property("Transparent",vart_vec).expect("got error");   
    }
    pub fn get_transparent(&self)->bool{
        let tran = self.obj.get_property("Transparent").expect("got error");
        tran.to_bool().unwrap()
    }
    fn put_usecolorprofile(&self){

    }
    fn get_usecolorprofile(&self){

    }
    fn put_compression(&self){

    }
    
    fn get_compression(&self){

    }

    fn put_maintainlayers(&self){

    }

    fn get_maintainlayers(&self){

    }

    pub fn put_maintainaspect(&self,aspect:bool){
        let mut vart_vec = Vec::new();
        let vart_aspect = <VARIANT as VariantExt>::from_bool(aspect);
        vart_vec.push(vart_aspect);
        self.obj.set_property("maintainaspect",vart_vec).expect("got error");     
    }

    pub fn get_maintainaspect(&self)->bool{
        let res = self.obj.get_property("maintainaspect").expect("got error");
        res.to_bool().unwrap()
    }
    fn get_exportarea(&self){

    }
    fn putref_exportarea(&self){

    }
    pub fn put_matted(&self,mat:bool){
        let mut vart_vec = Vec::new();
        let vart_mat = <VARIANT as VariantExt>::from_bool(mat);
        vart_vec.push(vart_mat);
        self.obj.set_property("matted",vart_vec).expect("got error"); 
    }
    pub fn get_matted(&self)->bool{
        let res = self.obj.get_property("matted").expect("got error");
        res.to_bool().unwrap()
    }
    fn get_MatteColor(&self){

    }
    fn put_MatteColor(&self){

    }
    fn put_MatteMaskedOnly(&self){

    }
    fn get_MatteMaskedOnly(&self){

    }
    fn put_AlwaysOverprintBlack(&self){

    }
    fn get_AlwaysOverprintBlack(&self){

    }
    fn get_ProofColorSettings(&self){

    }
    fn put_ProofColorSettings(&self){

    }

    pub fn to_variant(&self)->VARIANT{
        // self.obj.
        self.obj.get_variant().expect("got variant err:")
    }
}
