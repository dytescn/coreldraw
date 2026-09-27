//! `IVGWindows` 鈥斺€?鏂囨。绐楀彛闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::view::IvgWindow;

pub struct IvgWindows {
    disp: ComObject,
}

impl IvgWindows {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 1-based 绱㈠紩銆?
    pub fn item(&self, index: i32) -> Option<IvgWindow> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWindow::new)
    }

    pub fn all(&self) -> Vec<IvgWindow> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    // ---------------------------------------------------------
    // 鎵归噺鎿嶄綔
    // ---------------------------------------------------------

    pub fn close_all(&self) -> bool {
        self.disp.invoke_method("CloseAll", vec![]).is_ok()
    }

    /// `cdrWindowArrangeStyle` 鈥斺€?瑙?`enums::view`
    pub fn arrange(&self, style: i32) -> bool {
        let args = vec![Variant::from_i64(style as i64)];
        self.disp.invoke_method("Arrange", args).is_ok()
    }

    pub fn refresh(&self) -> bool {
        self.disp.invoke_method("Refresh", vec![]).is_ok()
    }

    /// 鎸夋爣棰樻煡鎵剧獥鍙ｃ€?
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