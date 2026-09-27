//! `IVGWindow` 鈥斺€?鏂囨。绐楀彛
//!
//! 鐢?`application.windows()` / `document.windows()` 鍙栧緱銆?
//! 涓?[`IvgAppWindow`](crate::app::IvgAppWindow) 鍖哄埆锛氬悗鑰呮槸 CorelDRAW
//! 涓荤獥鍙ｏ紝鍓嶈€呮槸鏌愪釜鏂囨。鐨勭獥鍙ｃ€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::document::IvgDocument;
use crate::page::IvgPage;
use crate::ui::ICuiViewWindow;
use crate::view::{IvgActiveView, IvgWindows};

pub struct IvgWindow {
    disp: ComObject,
}

impl IvgWindow {
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

    // ---------------------------------------------------------
    // 閫氱敤宸ュ叿
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
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    pub fn caption(&self) -> Option<String> { self.prop_string("Caption") }
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn handle(&self) -> Option<i64> { self.prop_i64("Handle") }

    pub fn active(&self) -> Option<bool> { self.prop_bool("Active") }
    pub fn full_screen(&self) -> Option<bool> { self.prop_bool("FullScreen") }
    pub fn set_full_screen(&self, v: bool) -> bool { self.put_bool("FullScreen", v) }

    /// `cdrWindowState` 鈥斺€?瑙?`enums::view`
    pub fn window_state(&self) -> Option<i64> { self.prop_i64("WindowState") }
    pub fn set_window_state(&self, v: i32) -> bool {
        self.put_i64("WindowState", v as i64)
    }

    // ---------------------------------------------------------
    // 浣嶇疆 / 灏哄
    // ---------------------------------------------------------

    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    // ---------------------------------------------------------
    // 鍏宠仈瀵硅薄
    // ---------------------------------------------------------

    pub fn parent(&self) -> Option<IvgWindows> {
        self.prop_dispatch("Parent").map(IvgWindows::new)
    }

    pub fn document(&self) -> Option<IvgDocument> {
        self.prop_dispatch("Document").map(IvgDocument::new)
    }

    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }

    pub fn active_view(&self) -> Option<IvgActiveView> {
        self.prop_dispatch("ActiveView").map(IvgActiveView::new)
    }

    /// UI 妗嗘灦灞傝鍥剧獥鍙ｏ紙鍙敤浜庤繘涓€姝ユ煡璇㈠涓?/ 浣嶇疆绛夛級銆?
    pub fn view_window(&self) -> Option<ICuiViewWindow> {
        self.prop_dispatch("ViewWindow").map(ICuiViewWindow::new)
    }

    // ---------------------------------------------------------
    // 鍓嶅悗绐楀彛
    // ---------------------------------------------------------

    pub fn previous(&self) -> Option<IvgWindow> {
        self.prop_dispatch("Previous").map(IvgWindow::new)
    }

    pub fn next(&self) -> Option<IvgWindow> {
        self.prop_dispatch("Next").map(IvgWindow::new)
    }

    // ---------------------------------------------------------
    // 鍧愭爣杞崲
    // ---------------------------------------------------------

    /// 灞忓箷鍧愭爣 鈫?鏂囨。鍧愭爣銆?
    pub fn screen_to_document(
        &self,
        xs: i32,
        ys: i32,
    ) -> Option<(f64, f64)> {
        // 鍙?out 鍙傛暟锛孖Dispatch 鍚庢湡缁戝畾鎷夸笉鍒般€?
        // 鐢?`ScreenDistanceToDocumentDistance` 鍙槸璺濈锛屼笉鏄偣銆?
        let _ = (xs, ys);
        None
    }

    /// 鏂囨。鍧愭爣 鈫?灞忓箷鍧愭爣銆?
    pub fn document_to_screen(
        &self,
        xd: f64,
        yd: f64,
    ) -> Option<(i32, i32)> {
        let _ = (xd, yd);
        None
    }

    /// 灞忓箷璺濈 鈫?鏂囨。璺濈銆?
    pub fn screen_distance_to_document_distance(&self, dist: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(dist)];
        self.disp
            .invoke_method("ScreenDistanceToDocumentDistance", args)
            .ok()?
            .to_f64()
            .ok()
    }

    /// 鏂囨。璺濈 鈫?灞忓箷璺濈銆?
    pub fn document_distance_to_screen_distance(&self, dist: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(dist)];
        self.disp
            .invoke_method("DocumentDistanceToScreenDistance", args)
            .ok()?
            .to_f64()
            .ok()
    }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    pub fn refresh(&self) -> bool {
        self.disp.invoke_method("Refresh", vec![]).is_ok()
    }

    /// 鏂板缓涓€涓悓鏂囨。鐨勭獥鍙ｃ€?
    pub fn new_window(&self) -> Option<IvgWindow> {
        self.disp
            .invoke_method("NewWindow", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgWindow::new)
    }
}