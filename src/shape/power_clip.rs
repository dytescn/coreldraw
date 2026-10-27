//! `IVGPowerClip` —— 图框精确裁剪

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::{IvgShapeRange, IvgShapes};

/// CorelDRAW `IVGPowerClip` 对象的后期绑定包装（幂裁剪）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPowerClip {
    disp: ComObject,
}

impl IvgPowerClip {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取图形集（`Shapes`）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    /// 读取内容锁定状态（`ContentsLocked`）。
    pub fn contents_locked(&self) -> Option<bool> { self.prop_bool("ContentsLocked") }
    /// 设置内容锁定状态（`ContentsLocked`）。
    pub fn set_contents_locked(&self, v: bool) -> bool {
        self.put_bool("ContentsLocked", v)
    }

    /// 输入编辑模式（COM 方法 `EnterEditMode`）。
    pub fn enter_edit_mode(&self) -> bool {
        self.disp.invoke_method("EnterEditMode", vec![]).is_ok()
    }

    /// 调用 COM 方法 `LeaveEditMode`。
    pub fn leave_edit_mode(&self) -> bool {
        self.disp.invoke_method("LeaveEditMode", vec![]).is_ok()
    }

    /// 提取图形集（COM 方法 `ExtractShapes`）。
    pub fn extract_shapes(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("ExtractShapes", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}