use std::io::Error;

use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;
use crate::sdk::pages::IvgPages;
use crate::sdk::struct_exportoption;
use crate::sdk::struct_paletteoptions;
use crate::sdk::struct_saveoptions;
use crate::sdk::page;
use crate::sdk::layers;
use crate::sdk::layer;

use super::layer::IvgLayer;
use super::page::IvgPage;
use super::selection_info::IvgSelectInfo;
use super::shape::IvgShape;
pub struct IvgDocuments {
    disp:ComObject
}

impl IvgDocuments{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    pub fn get_Count(&self)->i32{
        let res =  self.disp.get_property("Count").expect("got name err:");
        res.to_i32().unwrap()
    }

}


pub struct IvgDocument {
    disp:ComObject
}

impl IvgDocument{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    fn get_Application(){

    }
    fn get_parent(){

    }
    pub fn get_name(&self)->String{
        let res =  self.disp.get_property("name").expect("got name err:");
        res.to_string().unwrap()
    }
    // 另存为
    pub fn save_as(&self,src:&str,ver:u16){
        let mut opts = Vec::new();
        let vart_file = <VARIANT as VariantExt>::from_str(src);
        opts.push(vart_file);
        let save_option = struct_saveoptions::SaveOptions::new();
        save_option.put_Version(ver);
        let ver = save_option.get_Version();
        let vart_opt:VARIANT = save_option.get_opt();
        opts.push(vart_opt);
        self.disp.invoke_method("saveas", opts);
    }

    // 保存
    pub fn save(&self){
        let vart_vec = Vec::new(); 
        self.disp.invoke_method("save", vart_vec);
    }
    pub fn get_pages(&self)->IvgPages{
        let vart = self.disp.get_property("pages").expect("got active page error:");
        let disp = vart.to_idispatch().unwrap();
        let pages = IvgPages::new(disp.clone());
        pages
    }
    fn get_ReferencePoint(){

    }
    fn put_ReferencePoint(){

    }
    fn get_ApplyToDuplicate(){

    }
    fn put_ApplyToDuplicate(){

    }
    pub fn get_activepage(&self)->Result<IvgPage,Error>{
        let vart = self.disp.get_property("activepage").expect("got active page error:");
        let disp_res = vart.to_idispatch();
        match disp_res {
            Ok(disp)=>{
                let page = page::IvgPage::new(disp.clone());
                Ok(page)
            }
            Err(e)=>{
                return Err(e.into());
            }
        }
    }
    pub fn do_LayerCreate(&self){
        
    }
    pub fn get_ActiveLayer(&self)->Option<layer::IvgLayer>{
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
    fn get_Windows(){

    }
    fn get_ActiveWindow(){

    }
    fn Close(){

    }
    fn Undo(){

    }
    fn Redo(){
        
    }
    fn Repeat(){

    }
    fn Activate(){

    }
    fn get_Unit(){

    }
    fn put_Unit(){

    }
    fn get_DrawingOriginX(){

    }
    
    fn put_DrawingOriginX(){

    }
    fn get_DrawingOriginY(){

    }
    fn put_DrawingOriginY(){

    }
    fn AddPages(){

    }
    fn InsertPages(){

    }
    fn Selection(){

    }
    fn ClearSelection(){

    }
    pub fn export(&self,cover_path:&str,file_type:u32,range_code:u32,exp_opt:VARIANT,pale_opt:VARIANT)->Option<()>{
        let mut params = Vec::new();
        let src_path = VARIANT::from_str(cover_path);
        params.push(src_path);
        let code = VARIANT::from_u32(file_type);
        params.push(code);
        let range = VARIANT::from_u32(range_code);
        params.push(range);
        params.push(exp_opt);
        params.push(pale_opt);
        let res_data = self.disp.invoke_method("export", params);
        match res_data {
            Ok(data)=>{
                return Some(());
            }
            Err(err)=>{
                return None;
            }
        }
    }

    pub fn export_as_dxf(&self,dist_src:&str,exp_opt:VARIANT,pale_opt:VARIANT){
        let mut params = Vec::new();
        let src_path = VARIANT::from_str(dist_src);
        params.push(src_path);
        let code = VARIANT::from_u32(1296);
        params.push(code);
        let range = VARIANT::from_u32(0);
        params.push(range);
        params.push(exp_opt);
        params.push(pale_opt);

        let res_data = self.disp.invoke_method("export", params);
        match res_data {
            Ok(data)=>{
                println!("{:?}",data.to_string())
            }
            Err(err)=>{
                println!("{:?}",err);
            }
        }
    }

    fn resolveallbitmapslinks(){
        
    }
    fn get_dirty(){

    }
    fn put_dirty(){

    }
    fn getuserclick(){

    }
    fn getuserarea(){

    }
    fn begincommandgroup(){

    }
    fn endcommandgroup(){

    }
    pub fn get_filepath(&self)->String{
        let res =  self.disp.get_property("filepath").expect("got name err:");
        res.to_string().unwrap()
    }
    pub fn get_filename(&self)->String{
        let res =  self.disp.get_property("filename").expect("got name err:");
        res.to_string().unwrap()
    }
    pub fn get_fullfilename(&self)->String{
        let res =  self.disp.get_property("fullfilename").expect("got name err:");
        res.to_string().unwrap()
    }
    fn get_resolution(){

    }
    fn put_resolution(){

    }
    fn get_shapeenumdirection(){

    }
    fn put_shapeenumdirection(){

    }
    fn get_selectionrange(){

    }
    fn get_Rulers(){

    }
    fn get_Grid(){

    }
    fn get_Views(){

    }
    fn CreateView(){

    }
    fn get_ActivePowerClip(){

    }
    fn AdviseEvents(){

    }
    fn UnadviseEvents(){

    }
    fn get_WorldScale(){

    }
    fn put_WorldScale(){

    }
    fn PrintOut(){

    }
    pub fn get_ActiveShape(&self)->Option<IvgShape>{
        // 当前选中图形
        let doc_res = self.disp.get_property("ActiveShape");
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
    fn get_CurvePrecision(){

    }
    fn AddPagesEx(){

    }
    fn InsertPagesEx(){

    }
    fn get_Title(){

    }
    fn SaveSettings(){

    }
    fn RestoreSettings(){

    }
    fn get_Active(){

    }
    fn get_Index(){

    }
    fn ExportEx(){

    }
    fn ExportBitmap(){

    }
    fn get_EditAcrossLayers(){

    }
    fn put_EditAcrossLayers(){

    }
    fn get_Properties(){

    }
    fn put_CurvePrecision(){

    }
    fn get_PrintSettings(){

    }
    fn get_Keywords(){

    }
    fn put_Keywords(){

    }
    fn get_Notes(){

    }
    fn put_Notes(){

    }
    fn get_PreserveSelection(){

    }
    fn put_PreserveSelection(){

    }
    fn ResetSettings(){

    }
    fn get_DataFields(){

    }
    fn get_PDFSettings(){

    }
    fn PublishToPDF(){

    }
    pub fn get_SelectionInfo(&self)->IvgSelectInfo{
        let vart = self.disp.get_property("selectioninfo").expect("got active page error:");
        let disp = vart.to_idispatch().unwrap();
        let select = IvgSelectInfo::new(disp.clone());
        select
    }
    fn get_PageSizes(&self){

    }
    fn get_Components(){

    }
    fn get_SymbolLibrary(){

    }
    fn CreateCurve(){

    }
    fn CreateCurveFromArray() {

    }
    fn LoadStyleSheet(){

    }
    fn SaveStyleSheet(){

    }
    fn SaveStyleSheetAsDefault(){

    }
    fn CreateSelection(){

    }
    fn AddToSelection(){

    }
    fn RemoveFromSelection(){

    }
    fn get_SelectableShapes(){

    }
    fn ToUnits(){

    }
    fn FromUnits(){

    }
    fn get_ResolutionX(){

    }
    fn put_ResolutionX(){

    }
    fn get_ResolutionY(){

    }
    fn put_ResolutionY(){

    }
    fn get_MasterPage(){

    }
    fn Revert(){

    }
    fn get_CodeName(){

    }
    fn get__CodeName(){

    }
    fn put__CodeName(){
        
    }
    pub fn put_name(&self,name:&str){
        let mut vart_vec = Vec::new();
        let vart_name = <VARIANT as VariantExt>::from_str(name);
        vart_vec.push(vart_name);
        self.disp.set_property("name", vart_vec);
    }

    pub fn put_fullfilename(&self,filename:&str){
        let mut vart_vec = Vec::new();
        let vart_name = <VARIANT as VariantExt>::from_str(filename);
        vart_vec.push(vart_name);
        self.disp.set_property("fullfilename", vart_vec);
    }
    fn get_TreeRoot(){

    }
    fn get_TreeManager(){

    }
    fn LogCreateShape(){

    }
    fn LogCreateShapeRange(){
        
    }
    fn CreateFill(){

    }
    fn CreateOutline(){

    }
    fn get_HatchLibraries(){

    }
    fn CreateShapeRangeFromArray(){}
    fn ClearUndoList(){}
    fn get_SourcePlatformVersion(){}
    fn put_SourcePlatformVersion(){}
    fn get_SourceFormat(){}
    fn get_IsCurrentVersion(){}
    fn get_SourceFileVersion(){}
    fn put_SourceFileVersion(){}
    fn CreateUniformFill(){}
    fn get_Metadata(){}
    fn get_Layout(){}
    fn put_Layout(){}
    fn get_FacingPages(){}
    fn put_FacingPages(){}
    fn get_FirstPageOnRightSide(){}
    fn put_FirstPageOnRightSide(){}
    fn SetLayout(){

    }
    fn get_Spreads(){}
    fn get_ActiveSpread(){

    }
    fn CreateArrowHead(){}
    fn CreateArrowHeadEx(){}
    fn DeletePages(){}
    fn CreateArrowHead2(){}
    fn CreateArrowHeadEx2(){}
    fn CreateArrowHeadOptions(){}
    fn SaveAsCopy(){

    }
    fn CreateFreeSnapPoint(){

    }
    fn CreateBSpline(){

    }
    fn Duplicate(){

    }
    fn Clone(){
        
    }
    fn get_IsTemporary(){

    }
    fn get_ColorContext(){

    }
    fn AssignColorContext(){

    }
    fn ConvertToColorContext(){

    }
    fn PrintColorProof(){

    }
    fn get_Palette(){

    }
    fn get_TextFormatter(){

    }
    fn put_TextFormatter(){

    }
    fn get_StyleSheet(){

    }
    fn InteractiveImport(){

    }
    fn AddColorsToDocPalette(){

    }
    fn CreateColorStyles(){

    }
    fn CreateCurveFitToPoints(){

    }
    fn CreateCurveFitToPointsAndCusps(){

    }
    fn SampleColorAtPoint(){

    }
    fn SampleColorInArea(){

    }
    fn showallhiddenobjects(){

    }
    fn InteractiveImportWithContentIdentifier(){

    }
    fn replace_content_by_identifier(){

    }
    fn get_contentidentifiers(){

    }
    fn customcommand(){

    }
    fn get_math(){

    }
    // 创建图像信息
    fn create_image(){

    }
    // 打开文件
    pub fn do_DocumentOpen(&self,src:String){
        // DocumentOpen
        let mut args:Vec<VARIANT> = Vec::new();
        let var_src = VARIANT::from_str(src);
        args.push(var_src);
        let app_vers = self.disp.invoke_method("documentopen", args).expect("got err");
    }
}

// impl<T: exdispatch::IdispatchExt> IvgApplication for T {
//     fn new()->Self {
//         return ;
//     }

//     fn do_quit(&self) {
        
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        // do_quit();
        // let version = get_version();
        // println!("{:?}",version);
    }
}
