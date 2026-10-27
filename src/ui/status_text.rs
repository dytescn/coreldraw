//! `ICUIStatusText` —— 状态栏文本

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::ICuiBitmapImage;

/// CorelDRAW 对象的后期绑定包装（状态文本）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiStatusText {
    disp: ComObject,
}

impl ICuiStatusText {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 设置标题文本（`SetCaptionText`）。
    pub fn set_caption_text(&self, text: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(text.into())];
        self.disp.invoke_method("SetCaptionText", args).is_ok()
    }

    /// 设置位图（`SetBitmap`）。
    pub fn set_bitmap(&self, bitmap: &ICuiBitmapImage) -> bool {
        let args = vec![bitmap.as_variant()];
        self.disp.invoke_method("SetBitmap", args).is_ok()
    }
}