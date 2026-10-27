//! `IVGGrid` —— 文档网格

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGGrid` 对象的后期绑定包装（网格）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgGrid {
    disp: ComObject,
}

impl IvgGrid {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取可见性（`Visible`）。
    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    /// 设置可见性（`Visible`）。
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    /// `cdrGridType`
    pub fn grid_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置网格类型（`Type`）。
    pub fn set_grid_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取捕捉（`Snap`）。
    pub fn snap(&self) -> Option<bool> { self.prop_bool("Snap") }
    /// 设置捕捉（`Snap`）。
    pub fn set_snap(&self, v: bool) -> bool { self.put_bool("Snap", v) }

    /// 设置频率（`SetFrequency`）。
    pub fn set_frequency(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetFrequency", args).is_ok()
    }

    /// 读取间距 X 坐标（`SpacingX`）。
    pub fn spacing_x(&self) -> Option<f64> { self.prop_f64("SpacingX") }
    /// 设置间距 X 坐标（`SpacingX`）。
    pub fn set_spacing_x(&self, v: f64) -> bool { self.put_f64("SpacingX", v) }

    /// 读取间距 Y 坐标（`SpacingY`）。
    pub fn spacing_y(&self) -> Option<f64> { self.prop_f64("SpacingY") }
    /// 设置间距 Y 坐标（`SpacingY`）。
    pub fn set_spacing_y(&self, v: f64) -> bool { self.put_f64("SpacingY", v) }
}