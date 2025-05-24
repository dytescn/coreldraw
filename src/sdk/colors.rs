use std::ptr::null;
use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use windows::Win32::System::Variant;
use crate::sdk::types::*;

pub struct IvgColors {
    disp:ComObject
}

impl IvgColors{
   pub fn new(ver:&str)->Option<Self> {
    let id_name = "coreldraw.color.".to_string() + ver; // Can't use + with two &str
    let res_data=  ComObject::new_from_name(&id_name, APP_INTER_IID);
    //    println!("{:?}",);
        match res_data {
            Some(data)=>{
                return Some(Self{
                    disp:data
                });
            }
            None=>{
                return None;
            }
        }
    }
    fn get_Application(){

    }
    fn get_Parent(){

    }
    fn get_Item(){

    }
    fn get__NewEnum(){

    }
    fn get_Count(){

    }
}