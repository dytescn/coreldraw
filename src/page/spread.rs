//! `IVGSpread` / `IVGSpreads` 鈥斺€?璺ㄩ〉

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgRect;
use crate::layer::{IvgLayer, IvgLayers};
use crate::page::{IvgPage, IvgPages};
use crate::shape::IvgShapes;
use crate::tree::IvgTreeNode;

// =============================================================
// IvgSpreads
// =============================================================

pub struct IvgSpreads {
    disp: ComObject,
}

impl IvgSpreads {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn item(&self, index: i32) -> Option<IvgSpread> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSpread::new)
    }

    pub fn first(&self) -> Option<IvgSpread> {
        self.disp
            .get_property("First")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSpread::new)
    }

    pub fn last(&self) -> Option<IvgSpread> {
        self.disp
            .get_property("Last")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSpread::new)
    }

    pub fn all(&self) -> Vec<IvgSpread> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}

// =============================================================
// IvgSpread
// =============================================================

pub struct IvgSpread {
    disp: ComObject,
}

impl IvgSpread {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }

    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    pub fn get_bounding_box(&self) -> Option<(f64, f64, f64, f64)> {
        Some((
            self.left_x()?,
            self.top_y()?,
            self.size_width()?,
            self.size_height()?,
        ))
    }

    pub fn next(&self) -> Option<IvgSpread> {
        self.prop_dispatch("Next").map(IvgSpread::new)
    }

    pub fn previous(&self) -> Option<IvgSpread> {
        self.prop_dispatch("Previous").map(IvgSpread::new)
    }

    // ---------------------------------------------------------
    // 椤甸潰
    // ---------------------------------------------------------

    pub fn pages(&self) -> Option<IvgPages> {
        self.prop_dispatch("Pages").map(IvgPages::new)
    }

    // ---------------------------------------------------------
    // 鍥惧眰
    // ---------------------------------------------------------

    pub fn layers(&self) -> Option<IvgLayers> {
        self.prop_dispatch("Layers").map(IvgLayers::new)
    }

    pub fn all_layers(&self) -> Option<IvgLayers> {
        self.prop_dispatch("AllLayers").map(IvgLayers::new)
    }

    pub fn active_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveLayer").map(IvgLayer::new)
    }

    pub fn guides_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("GuidesLayer").map(IvgLayer::new)
    }

    pub fn desktop_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("DesktopLayer").map(IvgLayer::new)
    }

    pub fn grid_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("GridLayer").map(IvgLayer::new)
    }

    pub fn create_layer(&self, layer_name: impl Into<String>) -> Option<IvgLayer> {
        let args = vec![Variant::from_str(layer_name.into())];
        self.disp
            .invoke_method("CreateLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰
    // ---------------------------------------------------------

    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    pub fn selectable_shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("SelectableShapes").map(IvgShapes::new)
    }

    // ---------------------------------------------------------
    // 杈呭姪绾?/ 鏍?
    // ---------------------------------------------------------

    /// `cdrGuideType`
    pub fn guides(&self, guide_type: i32) -> Option<crate::shape::IvgShapeRange> {
        let args = vec![Variant::from_i64(guide_type as i64)];
        self.disp
            .invoke_method("Guides", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::shape::IvgShapeRange::new)
    }

    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // 渚涘閮ㄤ究鍒?
    pub fn first_page(&self) -> Option<IvgPage> {
        self.pages()?.first()
    }

    pub fn last_page(&self) -> Option<IvgPage> {
        self.pages()?.last()
    }
}