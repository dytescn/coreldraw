use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;
use crate::sdk::page::IvgPage;

pub struct IvgPages {
    disp:ComObject
}

impl IvgPages{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, PAGES_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    pub fn get_item(&self,index:i32)->IvgPage{
        let param = <VARIANT as VariantExt>::from_i32(index);
        let res_vart = self.disp.get_property_by_vart("item",param).expect("got page error");
        let page_disp = res_vart.to_idispatch().unwrap();
        IvgPage::new(page_disp.clone())
    }
    fn get__NewEnum(){

    }
    pub fn get_count(&self)->u8{
        let vart = self.disp.get_property("count").expect("got active page error:");
        vart.to_u8().unwrap()
    }
    pub fn get_first(&self)->IvgPage{
        let vart = self.disp.get_property("first").expect("got active page error:");
        let disp = vart.to_idispatch().unwrap();
        let page = IvgPage::new(disp.clone());
        page 
    }
    pub fn get_last(&self)->IvgPage{
        let vart = self.disp.get_property("last").expect("got active page error:");
        let disp = vart.to_idispatch().unwrap();
        let page = IvgPage::new(disp.clone());
        page   
    }
}