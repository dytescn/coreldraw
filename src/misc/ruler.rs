//! `IVGRulers` —— 文档标尺

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGRulers` 对象的后期绑定包装（标尺）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgRulers {
    disp: ComObject,
}

impl IvgRulers {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 垂直标尺单位（`cdrUnit`）。
    pub fn v_units(&self) -> Option<i64> { self.prop_i64("VUnits") }
    /// 设置单位（`VUnits`）。
    pub fn set_v_units(&self, v: i32) -> bool { self.put_i64("VUnits", v as i64) }

    /// 水平标尺单位。
    pub fn h_units(&self) -> Option<i64> { self.prop_i64("HUnits") }
    /// 设置高单位（`HUnits`）。
    pub fn set_h_units(&self, v: i32) -> bool { self.put_i64("HUnits", v as i64) }
}