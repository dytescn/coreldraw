use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::core;
use crate::coreldraw::types::*;
use crate::coreldraw::document::IvgDocument;
use crate::dispatch::dispatch::ComObject;
use crate::dispatch::utils::VariantExt;

// 全部窗口
pub struct IvgWindows {
    disp:ComObject
}

impl IvgWindows{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
}

// 独立窗口
pub struct IvgWindow {
    disp:ComObject
}

impl IvgWindow{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
}
