//! `ICorelExportFilter` —— 导出过滤器
//!
//! 由 `IVGBitmap::SaveAs` 等 API 返回；调用方通过它控制导出过程。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW 对象的后期绑定包装（导出过滤器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICorelExportFilter {
    disp: ComObject,
}

impl ICorelExportFilter {
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
    // 属性
    // ---------------------------------------------------------

    /// 是否有对话框（`HasDialog`）。
    pub fn has_dialog(&self) -> Option<bool> {
        self.disp.get_property("HasDialog").ok()?.to_bool().ok()
    }

    // ---------------------------------------------------------
    // 方法
    // ---------------------------------------------------------

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// 完成（COM 方法 `Finish`）。
    pub fn finish(&self) -> bool {
        self.disp.invoke_method("Finish", vec![]).is_ok()
    }

    /// 显示对话框（COM 方法 `ShowDialog`）。
    pub fn show_dialog(&self, h_wnd: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(h_wnd as i64)];
        self.disp
            .invoke_method("ShowDialog", args)
            .ok()?
            .to_bool()
            .ok()
    }
}