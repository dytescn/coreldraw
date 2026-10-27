//! `IVGAppWindow` —— 应用主窗口
//!
//! 与 [`crate::view::IvgWindow`]（文档窗口）区分。

use wincom::{ComObject, Variant};

/// CorelDRAW `IVGAppWindow` 对象的后期绑定包装（应用窗口）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgAppWindow {
    disp: ComObject,
}

impl IvgAppWindow {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: ComObject) -> Self {
        Self { disp }
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 属性读取工具
    // ---------------------------------------------------------

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    // ---------------------------------------------------------
    // 属性写工具
    // ---------------------------------------------------------

    fn put(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        self.put(name, Variant::from_i64(v))
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        self.put(name, Variant::from_str(&v.into()))
    }

    // ---------------------------------------------------------
    // 属性
    // ---------------------------------------------------------

    /// 窗口标题。
    pub fn caption(&self) -> Option<String> {
        self.prop_string("Caption")
    }

    /// 设置标题（`Caption`）。
    pub fn set_caption(&self, s: impl Into<String>) -> bool {
        self.put_string("Caption", s)
    }

    /// 是否为活动窗口。
    pub fn active(&self) -> Option<bool> {
        self.prop_bool("Active")
    }

    // ---- 位置 ----

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<i64> {
        self.prop_i64("Left")
    }

    /// 设置左（`Left`）。
    pub fn set_left(&self, v: i32) -> bool {
        self.put_i64("Left", v as i64)
    }

    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<i64> {
        self.prop_i64("Top")
    }

    /// 设置顶（`Top`）。
    pub fn set_top(&self, v: i32) -> bool {
        self.put_i64("Top", v as i64)
    }

    // ---- 尺寸 ----

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> {
        self.prop_i64("Width")
    }

    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool {
        self.put_i64("Width", v as i64)
    }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> {
        self.prop_i64("Height")
    }

    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool {
        self.put_i64("Height", v as i64)
    }

    // ---- 客户区 ----

    /// 读取 COM 属性 `ClientWidth`。
    pub fn client_width(&self) -> Option<i64> {
        self.prop_i64("ClientWidth")
    }

    /// 读取 COM 属性 `ClientHeight`。
    pub fn client_height(&self) -> Option<i64> {
        self.prop_i64("ClientHeight")
    }

    // ---- 句柄 / 状态 ----

    /// 内部 `HWND`。
    pub fn handle(&self) -> Option<i64> {
        self.prop_i64("Handle")
    }

    /// `cdrWindowState` —— 见 `enums::view`。
    pub fn window_state(&self) -> Option<i64> {
        self.prop_i64("WindowState")
    }

    /// 设置窗口状态（`WindowState`）。
    pub fn set_window_state(&self, v: i32) -> bool {
        self.put_i64("WindowState", v as i64)
    }

    // ---------------------------------------------------------
    // 方法
    // ---------------------------------------------------------

    /// 激活主窗口。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }
}