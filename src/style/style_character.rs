//! `IVGStyleCharacter` 鈥斺€?鏍峰紡閲岀殑瀛楃閮ㄥ垎
//!
//! RIDL 閲屽彧鏈?`Style` 灞炴€э紱瀹為檯瀛楃灞炴€ч€氳繃 `IVGStyle` 鐨?
//! `GetProperty` / `SetProperty` 璁块棶銆傛澶勪粎淇濈暀楠ㄦ灦銆?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

pub struct IvgStyleCharacter {
    disp: ComObject,
}

impl IvgStyleCharacter {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// 鍏宠仈鐨勭埗鏍峰紡銆?
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.disp
            .get_property("Style")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::style::IvgStyle::new)
    }
}