//! `IVGTextFrame` / `IVGTextFrames` 鈥斺€?鏂囨湰妗?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::fill::IvgFill;
use crate::outline::IvgOutline;
use crate::shape::IvgShape;
use crate::text::IvgTextRange;

// =============================================================
// IvgTextFrames
// =============================================================

pub struct IvgTextFrames {
    disp: ComObject,
}

impl IvgTextFrames {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgTextFrame> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextFrame::new)
    }

    pub fn first(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("First").map(IvgTextFrame::new)
    }

    pub fn last(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Last").map(IvgTextFrame::new)
    }

    /// 鎸夌储寮曡寖鍥村彇鏂囨湰鑼冨洿銆?
    pub fn range(&self, index: i32, count: i32) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(count as i64),
        ];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 鎵€鏈夋鐨勬枃鏈寖鍥淬€?
    pub fn all(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("All").map(IvgTextRange::new)
    }
}

// =============================================================
// IvgTextFrame
// =============================================================

pub struct IvgTextFrame {
    disp: ComObject,
}

impl IvgTextFrame {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    // fn prop_f64(&self, name: &str) -> Option<f64> {
    //     self.disp.get_property(name).ok()?.to_f64().ok()
    // }

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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 鏂囨湰鑼冨洿 ----

    pub fn range(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Range").map(IvgTextRange::new)
    }

    // ---- 涓婁笅妗?----

    pub fn previous(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Previous").map(IvgTextFrame::new)
    }

    pub fn next(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Next").map(IvgTextFrame::new)
    }

    // ---- 鐘舵€?----

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("Empty") }
    pub fn is_first(&self) -> Option<bool> { self.prop_bool("IsFirst") }
    pub fn is_last(&self) -> Option<bool> { self.prop_bool("IsLast") }
    pub fn is_inside_container(&self) -> Option<bool> {
        self.prop_bool("IsInsideContainer")
    }
    pub fn is_fitted_to_path(&self) -> Option<bool> { self.prop_bool("IsFittedToPath") }

    // ---- 鍨傜洿瀵归綈 ----

    /// `cdrVerticalAlignment`
    pub fn vertical_alignment(&self) -> Option<i64> { self.prop_i64("VerticalAlignment") }
    pub fn set_vertical_alignment(&self, v: i32) -> bool {
        self.put_i64("VerticalAlignment", v as i64)
    }

    // ---- 鍒嗘爮 ----

    pub fn multicolumn(&self) -> Option<bool> { self.prop_bool("Multicolumn") }
    pub fn column_count(&self) -> Option<i64> { self.prop_i64("ColumnCount") }

    pub fn column_width(&self, index: i32) -> Option<f64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("ColumnWidth", args).ok()?.to_f64().ok()
    }

    pub fn column_gutter(&self, index: i32) -> Option<f64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("ColumnGutter", args).ok()?.to_f64().ok()
    }

    /// `WidthsAndGutters` 鏄?`SAFEARRAY`銆?
    pub fn set_columns(
        &self,
        num_columns: i32,
        equal_columns: bool,
        widths_and_gutters: Variant,
    ) -> bool {
        let args = vec![
            Variant::from_i64(num_columns as i64),
            Variant::from_bool(equal_columns),
            widths_and_gutters,
        ];
        self.disp.invoke_method("SetColumns", args).is_ok()
    }

    // ---- 閾炬帴 ----

    /// 鎶婃湰妗嗛摼鎺ュ埌鍙︿竴涓浘褰€?
    pub fn link_to(&self, shape: &IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("LinkTo", args).is_ok()
    }

    /// 鏂紑閾炬帴銆?
    pub fn unlink(&self) -> bool {
        self.disp.invoke_method("UnLink", vec![]).is_ok()
    }

    // ---- 瀹瑰櫒 / 璺緞 ----

    pub fn container(&self) -> Option<IvgShape> {
        self.prop_dispatch("Container").map(IvgShape::new)
    }

    pub fn path(&self) -> Option<IvgShape> {
        self.prop_dispatch("Path").map(IvgShape::new)
    }

    pub fn frame_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FrameShape").map(IvgShape::new)
    }

    // ---- 濉厖 / 杞粨 ----

    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }
    pub fn set_fill(&self, f: &IvgFill) -> bool {
        self.put_dispatch("Fill", f.as_variant())
    }

    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }
    pub fn set_outline(&self, o: &IvgOutline) -> bool {
        self.put_dispatch("Outline", o.as_variant())
    }
}