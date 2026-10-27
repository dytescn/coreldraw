//! `IVGWindows` —— 文档窗口集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::view::IvgWindow;

/// CorelDRAW `IVGWindows` 对象的后期绑定包装（窗口集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgWindows {
    disp: ComObject,
}

impl IvgWindows {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 1-based 索引。
    pub fn item(&self, index: i32) -> Option<IvgWindow> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWindow::new)
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgWindow> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    // ---------------------------------------------------------
    // 批量操作
    // ---------------------------------------------------------

    /// 关闭全部（COM 方法 `CloseAll`）。
    pub fn close_all(&self) -> bool {
        self.disp.invoke_method("CloseAll", vec![]).is_ok()
    }

    /// `cdrWindowArrangeStyle` ——见 `enums::view`
    pub fn arrange(&self, style: i32) -> bool {
        let args = vec![Variant::from_i64(style as i64)];
        self.disp.invoke_method("Arrange", args).is_ok()
    }

    /// 刷新（COM 方法 `Refresh`）。
    pub fn refresh(&self) -> bool {
        self.disp.invoke_method("Refresh", vec![]).is_ok()
    }

    /// 按标题查找窗口。
    pub fn find_window(&self, caption: impl Into<String>) -> Option<IvgWindow> {
        let args = vec![Variant::from_str(caption.into())];
        self.disp
            .invoke_method("FindWindow", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWindow::new)
    }
}