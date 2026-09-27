//! `IVGPage` 鈥斺€?椤甸潰瀵硅薄
//!
//! 鐢?`document.active_page()` / `document.pages().item(n)` 鍙栧緱銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::geometry::IvgRect;
use crate::page::IvgPages;
use crate::layer::{IvgLayer, IvgLayers};
use crate::shape::{IvgShape, IvgShapeRange, IvgShapes};
use crate::tree::IvgTreeNode;

pub struct IvgPage {
    disp: ComObject,
}

impl IvgPage {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    /// 1-based 椤靛彿銆?
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---------------------------------------------------------
    // 鍏崇郴
    // ---------------------------------------------------------

    pub fn parent(&self) -> Option<IvgPages> {
        self.prop_dispatch("Parent").map(IvgPages::new)
    }

    pub fn previous(&self) -> Option<IvgPage> {
        self.prop_dispatch("Previous").map(IvgPage::new)
    }

    pub fn next(&self) -> Option<IvgPage> {
        self.prop_dispatch("Next").map(IvgPage::new)
    }

    pub fn spread(&self) -> Option<crate::page::IvgSpread> {
        self.prop_dispatch("Spread").map(crate::page::IvgSpread::new)
    }

    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 椤甸潰璁剧疆
    // ---------------------------------------------------------

    /// 椤甸潰棰勮鍚嶏紙濡?"A4"锛夈€?
    pub fn paper(&self) -> Option<String> { self.prop_string("Paper") }

    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    pub fn set_size_width(&self, v: f64) -> bool { self.put_f64("SizeWidth", v) }

    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }
    pub fn set_size_height(&self, v: f64) -> bool { self.put_f64("SizeHeight", v) }

    pub fn resolution(&self) -> Option<i64> { self.prop_i64("Resolution") }
    pub fn set_resolution(&self, v: i32) -> bool { self.put_i64("Resolution", v as i64) }

    /// 鍑鸿銆?
    pub fn bleed(&self) -> Option<f64> { self.prop_f64("Bleed") }
    pub fn set_bleed(&self, v: f64) -> bool { self.put_f64("Bleed", v) }

    /// `cdrPageOrientation`
    pub fn orientation(&self) -> Option<i64> { self.prop_i64("Orientation") }
    pub fn set_orientation(&self, v: i32) -> bool { self.put_i64("Orientation", v as i64) }

    /// `cdrPageBackground`
    pub fn background(&self) -> Option<i64> { self.prop_i64("Background") }
    pub fn set_background(&self, v: i32) -> bool { self.put_i64("Background", v as i64) }

    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    pub fn print_export_background(&self) -> Option<bool> {
        self.prop_bool("PrintExportBackground")
    }
    pub fn set_print_export_background(&self, v: bool) -> bool {
        self.put_bool("PrintExportBackground", v)
    }

    // ---------------------------------------------------------
    // 鍑犱綍锛氬洓杈?+ 涓績 + 鍖呭洿鐩?
    // ---------------------------------------------------------

    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    pub fn get_size(&self) -> Option<(f64, f64)> {
        Some((self.size_width()?, self.size_height()?))
    }

    pub fn set_size(&self, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetSize", args).is_ok()
    }

    pub fn get_center_position(&self) -> Option<(f64, f64)> {
        Some((self.center_x()?, self.center_y()?))
    }

    /// 閫夋嫨鍛藉悕椤甸潰灏哄銆?
    pub fn select_size(&self, page_size_name: impl Into<String>, landscape: bool) -> bool {
        let args = vec![
            Variant::from_str(page_size_name.into()),
            Variant::from_bool(landscape),
        ];
        self.disp.invoke_method("SelectSize", args).is_ok()
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

    /// 鎸夊悕绉?/ 绫诲瀷 / 闈欐€?ID 鏌ユ壘銆?
    pub fn find_shape(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        static_id: i32,
        recursive: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_i64(static_id as i64),
            Variant::from_bool(recursive),
        ];
        self.disp
            .invoke_method("FindShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn find_shapes(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        recursive: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_bool(recursive),
        ];
        self.disp
            .invoke_method("FindShapes", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 鎵惧埌鏌愮偣鏈€涓婂眰鐨勫浘褰€?
    pub fn find_shape_at_point(
        &self,
        x: f64,
        y: f64,
        treat_as_filled: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(treat_as_filled),
        ];
        self.disp
            .invoke_method("FindShapeAtPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 閫夋嫨
    // ---------------------------------------------------------

    pub fn select_shapes_at_point(
        &self,
        x: f64,
        y: f64,
        select_unfilled: bool,
        hot_area: f64,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(select_unfilled),
            Variant::from_f64(hot_area),
        ];
        self.disp
            .invoke_method("SelectShapesAtPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn select_shapes_from_rectangle(
        &self,
        x1: f64, y1: f64, x2: f64, y2: f64,
        touch: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
            Variant::from_bool(touch),
        ];
        self.disp
            .invoke_method("SelectShapesFromRectangle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn unlock_all_shapes(&self) -> bool {
        self.disp.invoke_method("UnlockAllShapes", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 杈呭姪绾?
    // ---------------------------------------------------------

    /// `cdrGuideType`
    pub fn guides(&self, guide_type: i32) -> Option<IvgShapeRange> {
        let args = vec![Variant::from_i64(guide_type as i64)];
        self.disp
            .invoke_method("Guides", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 鍏跺畠
    // ---------------------------------------------------------

    pub fn properties(&self) -> Option<crate::misc::IvgProperties> {
        self.prop_dispatch("Properties").map(crate::misc::IvgProperties::new)
    }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn move_to(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("MoveTo", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鏂囨湰鏌ユ壘
    // ---------------------------------------------------------

    pub fn text_find(
        &self,
        text: impl Into<String>,
        case_sensitive: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(text.into()),
            Variant::from_bool(case_sensitive),
        ];
        self.disp
            .invoke_method("TextFind", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn text_replace(
        &self,
        old_text: impl Into<String>,
        new_text: impl Into<String>,
        case_sensitive: bool,
        replace_selected_only: bool,
    ) -> bool {
        let args = vec![
            Variant::from_str(old_text.into()),
            Variant::from_str(new_text.into()),
            Variant::from_bool(case_sensitive),
            Variant::from_bool(replace_selected_only),
        ];
        self.disp.invoke_method("TextReplace", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鑷畾涔夊懡浠?
    // ---------------------------------------------------------

    pub fn custom_command(
        &self,
        component_id: impl Into<String>,
        command_id: impl Into<String>,
        parameters: Variant,
    ) -> Option<Variant> {
        let args = vec![
            Variant::from_str(component_id.into()),
            Variant::from_str(command_id.into()),
            parameters,
        ];
        self.disp.invoke_method("CustomCommand", args).ok()
    }
}