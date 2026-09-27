//! `IVGColors` 鈥斺€?棰滆壊闆嗗悎
//!
//! 鈿狅笍 RIDL 閲?`IVGColors` 鏄?*绌烘帴鍙?*锛堟棤鏂规硶/灞炴€э級锛?
//! 鍙綔涓鸿繑鍥炵被鍨嬪嚭鐜般€備繚鐣欓鏋朵互澶囧皢鏉ユ墿灞曘€?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

pub struct IvgColors {
    disp: ComObject,
}

impl IvgColors {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }
}