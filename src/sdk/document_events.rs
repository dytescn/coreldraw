use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use crate::dispatch::dispatch::ComObject;
use crate::coreldraw::types::*;
use crate::dispatch::utils::VariantExt;

pub struct IvgDocumentEvents {
    disp:ComObject
}

impl IvgDocumentEvents{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
}
