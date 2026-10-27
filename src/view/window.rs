//! `IVGWindow` —— 文档窗口
//!
//! 由 `application.windows()` / `document.windows()` 取得。
//! 与 [`IvgAppWindow`](crate::app::IvgAppWindow) 的区别：后者是 CorelDRAW
//! 主窗口，前者是某个文档的窗口。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::document::IvgDocument;
use crate::page::IvgPage;
use crate::ui::ICuiViewWindow;
use crate::view::{IvgActiveView, IvgWindows};

/// CorelDRAW `IVGWindow` 对象的后期绑定包装（窗口）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgWindow {
    disp: ComObject,
}

impl IvgWindow {
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

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 通用工具
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 读取标题（`Caption`）。
    pub fn caption(&self) -> Option<String> { self.prop_string("Caption") }
    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取手柄（`Handle`）。
    pub fn handle(&self) -> Option<i64> { self.prop_i64("Handle") }

    /// 读取活动（`Active`）。
    pub fn active(&self) -> Option<bool> { self.prop_bool("Active") }
    /// 读取完整屏幕（`FullScreen`）。
    pub fn full_screen(&self) -> Option<bool> { self.prop_bool("FullScreen") }
    /// 设置完整屏幕（`FullScreen`）。
    pub fn set_full_screen(&self, v: bool) -> bool { self.put_bool("FullScreen", v) }

    /// `cdrWindowState` ——见 `enums::view`
    pub fn window_state(&self) -> Option<i64> { self.prop_i64("WindowState") }
    /// 设置窗口状态（`WindowState`）。
    pub fn set_window_state(&self, v: i32) -> bool {
        self.put_i64("WindowState", v as i64)
    }

    // ---------------------------------------------------------
    // 位置 / 尺寸
    // ---------------------------------------------------------

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    /// 设置左（`Left`）。
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    /// 设置顶（`Top`）。
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    // ---------------------------------------------------------
    // 关联对象
    // ---------------------------------------------------------

    /// 读取父对象（`Parent`）。
    pub fn parent(&self) -> Option<IvgWindows> {
        self.prop_dispatch("Parent").map(IvgWindows::new)
    }

    /// 读取文档（`Document`）。
    pub fn document(&self) -> Option<IvgDocument> {
        self.prop_dispatch("Document").map(IvgDocument::new)
    }

    /// 读取页面（`Page`）。
    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }

    /// 读取活动视图（`ActiveView`）。
    pub fn active_view(&self) -> Option<IvgActiveView> {
        self.prop_dispatch("ActiveView").map(IvgActiveView::new)
    }

    /// UI 框架层视图窗口（可用于进一步查询宿主/ 位置等）。
    pub fn view_window(&self) -> Option<ICuiViewWindow> {
        self.prop_dispatch("ViewWindow").map(ICuiViewWindow::new)
    }

    // ---------------------------------------------------------
    // 前后窗口
    // ---------------------------------------------------------

    /// 读取上一个（`Previous`）。
    pub fn previous(&self) -> Option<IvgWindow> {
        self.prop_dispatch("Previous").map(IvgWindow::new)
    }

    /// 读取下一个（`Next`）。
    pub fn next(&self) -> Option<IvgWindow> {
        self.prop_dispatch("Next").map(IvgWindow::new)
    }

    // ---------------------------------------------------------
    // 坐标转换
    // ---------------------------------------------------------

    /// 屏幕坐标 → 文档坐标。
    pub fn screen_to_document(
        &self,
        xs: i32,
        ys: i32,
    ) -> Option<(f64, f64)> {
        // 发 out 参数，IDispatch 后期绑定拿不到。
        // 由 `ScreenDistanceToDocumentDistance` 只是距离，不是点。
        let _ = (xs, ys);
        None
    }

    /// 文档坐标 → 屏幕坐标。
    pub fn document_to_screen(
        &self,
        xd: f64,
        yd: f64,
    ) -> Option<(i32, i32)> {
        let _ = (xd, yd);
        None
    }

    /// 屏幕距离 → 文档距离。
    pub fn screen_distance_to_document_distance(&self, dist: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(dist)];
        self.disp
            .invoke_method("ScreenDistanceToDocumentDistance", args)
            .ok()?
            .to_f64()
            .ok()
    }

    /// 文档距离 → 屏幕距离。
    pub fn document_distance_to_screen_distance(&self, dist: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(dist)];
        self.disp
            .invoke_method("DocumentDistanceToScreenDistance", args)
            .ok()?
            .to_f64()
            .ok()
    }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    /// 关闭（COM 方法 `Close`）。
    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    /// 刷新（COM 方法 `Refresh`）。
    pub fn refresh(&self) -> bool {
        self.disp.invoke_method("Refresh", vec![]).is_ok()
    }

    /// 新建一个同文档的窗口。
    pub fn new_window(&self) -> Option<IvgWindow> {
        self.disp
            .invoke_method("NewWindow", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWindow::new)
    }
}