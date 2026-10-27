//! `IVGView` —— 命名视图
//!
//! 保存在文档里，通过 `document.views()` 访问。
//! 与 [`IvgActiveView`](crate::view::IvgActiveView)（当前活动视图）不同，命名视图是持久化的。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::page::IvgPage;

/// CorelDRAW `IVGView` 对象的后期绑定包装（视图）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgView {
    disp: ComObject,
}

impl IvgView {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 名称 / 原点 / 缩放
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取原点 X 坐标（`OriginX`）。
    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    /// 设置原点 X 坐标（`OriginX`）。
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    /// 读取原点 Y 坐标（`OriginY`）。
    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    /// 设置原点 Y 坐标（`OriginY`）。
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    /// 是否使用页面视图（`UsePage = true` 时与页面绑定）。
    pub fn use_page(&self) -> Option<bool> { self.prop_bool("UsePage") }
    /// 设置使用页面（`UsePage`）。
    pub fn set_use_page(&self, v: bool) -> bool { self.put_bool("UsePage", v) }

    /// 读取页面（`Page`）。
    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }
    /// 设置页面（`Page`）。
    pub fn set_page(&self, p: &IvgPage) -> bool {
        self.put_dispatch("Page", p.as_variant())
    }

    /// 是否使用自定义缩放（`false` 时自动适配）。
    pub fn use_zoom(&self) -> Option<bool> { self.prop_bool("UseZoom") }
    /// 设置使用缩放（`UseZoom`）。
    pub fn set_use_zoom(&self, v: bool) -> bool { self.put_bool("UseZoom", v) }

    /// 读取缩放（`Zoom`）。
    pub fn zoom(&self) -> Option<f64> { self.prop_f64("Zoom") }
    /// 设置缩放（`Zoom`）。
    pub fn set_zoom(&self, v: f64) -> bool { self.put_f64("Zoom", v) }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// 激活该视图（切换活动窗口到本视图）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}