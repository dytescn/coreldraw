//! `IVGCustomShape` —— 自定义图形

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGCustomShape` 对象的后期绑定包装（自定义图形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgCustomShape {
    disp: ComObject,
}

impl IvgCustomShape {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    /// 图形类型 ID。
    pub fn type_id(&self) -> Option<String> {
        self.disp.get_property("TypeID").ok()?.to_string().ok()
    }
}