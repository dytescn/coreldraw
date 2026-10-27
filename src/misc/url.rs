//! `IVGURL` —— 图形上挂的URL / 书签

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGURL` 对象的后期绑定包装（URL）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgUrl {
    disp: ComObject,
}

impl IvgUrl {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取 COM 属性 `Address`。
    pub fn address(&self) -> Option<String> { self.prop_string("Address") }
    /// 设置 COM 属性 `Address`。
    pub fn set_address(&self, v: impl Into<String>) -> bool {
        self.put_string("Address", v)
    }

    /// 读取目标文本框（`TargetFrame`）。
    pub fn target_frame(&self) -> Option<String> { self.prop_string("TargetFrame") }
    /// 设置目标文本框（`TargetFrame`）。
    pub fn set_target_frame(&self, v: impl Into<String>) -> bool {
        self.put_string("TargetFrame", v)
    }

    /// 读取 COM 属性 `AltComment`。
    pub fn alt_comment(&self) -> Option<String> { self.prop_string("AltComment") }
    /// 设置 COM 属性 `AltComment`。
    pub fn set_alt_comment(&self, v: impl Into<String>) -> bool {
        self.put_string("AltComment", v)
    }

    /// 读取 COM 属性 `BookMark`。
    pub fn bookmark(&self) -> Option<String> { self.prop_string("BookMark") }
    /// 设置 COM 属性 `BookMark`。
    pub fn set_bookmark(&self, v: impl Into<String>) -> bool {
        self.put_string("BookMark", v)
    }

    /// `cdrURLRegion`
    pub fn region(&self) -> Option<i64> { self.prop_i64("Region") }
    /// 设置 COM 属性 `Region`。
    pub fn set_region(&self, v: i32) -> bool { self.put_i64("Region", v as i64) }
}