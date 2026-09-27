//! `IVGCustomShape` 鈥斺€?鑷畾涔夊浘褰?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

pub struct IvgCustomShape {
    disp: ComObject,
}

impl IvgCustomShape {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    /// 鍥惧舰绫诲瀷 ID銆?
    pub fn type_id(&self) -> Option<String> {
        self.disp.get_property("TypeID").ok()?.to_string().ok()
    }
}