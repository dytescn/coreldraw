//! `IVGAppWindow` —— 应用主窗口
//!
//! 与 [`crate::view::IvgWindow`]（文档窗口）区分。

use wincom::{ComObject, Variant};

pub struct IvgAppWindow {
    disp: ComObject,
}

impl IvgAppWindow {
    pub fn new(disp: ComObject) -> Self {
        Self { disp }
    }

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

    pub fn set_caption(&self, s: impl Into<String>) -> bool {
        self.put_string("Caption", s)
    }

    /// 是否为活动窗口。
    pub fn active(&self) -> Option<bool> {
        self.prop_bool("Active")
    }

    // ---- 位置 ----

    pub fn left(&self) -> Option<i64> {
        self.prop_i64("Left")
    }

    pub fn set_left(&self, v: i32) -> bool {
        self.put_i64("Left", v as i64)
    }

    pub fn top(&self) -> Option<i64> {
        self.prop_i64("Top")
    }

    pub fn set_top(&self, v: i32) -> bool {
        self.put_i64("Top", v as i64)
    }

    // ---- 尺寸 ----

    pub fn width(&self) -> Option<i64> {
        self.prop_i64("Width")
    }

    pub fn set_width(&self, v: i32) -> bool {
        self.put_i64("Width", v as i64)
    }

    pub fn height(&self) -> Option<i64> {
        self.prop_i64("Height")
    }

    pub fn set_height(&self, v: i32) -> bool {
        self.put_i64("Height", v as i64)
    }

    // ---- 客户区 ----

    pub fn client_width(&self) -> Option<i64> {
        self.prop_i64("ClientWidth")
    }

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