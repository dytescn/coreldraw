//! `IVGShape` 鈥斺€?鍥惧舰瀵硅薄锛堟牳蹇冿級

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::curve::IvgCurve;
use crate::effect::{IvgEffect, IvgEffects};
use crate::fill::IvgFill;
use crate::geometry::{IvgRect, IvgSnapPoint, IvgSnapPoints, IvgTransformMatrix};
use crate::layer::IvgLayer;
use crate::misc::{IvgCloneLink, IvgProperties, IvgTransparency, IvgUrl};
use crate::outline::IvgOutline;
use crate::page::IvgPage;
use crate::shape::{
    IvgBitmap, IvgConnector, IvgCustomShape, IvgEllipse, IvgEps, IvgGuide,
    IvgPolygon, IvgPowerClip, IvgRectangle, IvgShapeRange, IvgShapes,
};
use crate::symbol::IvgSymbol;
use crate::text::IvgText;
use crate::tree::IvgTreeNode;

pub struct IvgShape {
    disp: ComObject,
}

impl IvgShape {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn dispatch(&self) -> &IDispatch {
        unimplemented!("鎸?wincom 瀹為檯 API 杩斿洖鍐呴儴 IDispatch")
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

    /// 鍥惧舰绫诲瀷锛坄cdrShapeType`锛夈€?
    pub fn shape_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 鍥惧舰鐨勯潤鎬?ID锛堝湪鏂囨。鍐呭敮涓€锛夈€?
    pub fn static_id(&self) -> Option<i64> { self.prop_i64("StaticID") }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn locked(&self) -> Option<bool> { self.prop_bool("Locked") }
    pub fn set_locked(&self, v: bool) -> bool { self.put_bool("Locked", v) }

    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    pub fn selectable(&self) -> Option<bool> { self.prop_bool("Selectable") }

    pub fn virtual_shape(&self) -> Option<bool> { self.prop_bool("Virtual") }
    pub fn can_have_fill(&self) -> Option<bool> { self.prop_bool("CanHaveFill") }
    pub fn can_have_outline(&self) -> Option<bool> { self.prop_bool("CanHaveOutline") }
    pub fn is_simple_shape(&self) -> Option<bool> { self.prop_bool("IsSimpleShape") }

    // ---------------------------------------------------------
    // 浣嶇疆 / 灏哄
    // ---------------------------------------------------------

    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    pub fn set_size_width(&self, v: f64) -> bool { self.put_f64("SizeWidth", v) }

    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }
    pub fn set_size_height(&self, v: f64) -> bool { self.put_f64("SizeHeight", v) }

    pub fn original_width(&self) -> Option<f64> { self.prop_f64("OriginalWidth") }
    pub fn original_height(&self) -> Option<f64> { self.prop_f64("OriginalHeight") }

    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    pub fn set_left_x(&self, v: f64) -> bool { self.put_f64("LeftX", v) }

    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    pub fn set_right_x(&self, v: f64) -> bool { self.put_f64("RightX", v) }

    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    pub fn set_top_y(&self, v: f64) -> bool { self.put_f64("TopY", v) }

    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }
    pub fn set_bottom_y(&self, v: f64) -> bool { self.put_f64("BottomY", v) }

    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn set_center_x(&self, v: f64) -> bool { self.put_f64("CenterX", v) }

    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
    pub fn set_center_y(&self, v: f64) -> bool { self.put_f64("CenterY", v) }

    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.position_x()?, self.position_y()?))
    }

    pub fn set_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
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

    /// 鐢ㄥ弬鑰冪偣瀹氫綅銆俙cdrReferencePoint` 瑙?`enums::shape`銆?
    pub fn set_position_ex(&self, ref_point: i32, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_i64(ref_point as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPositionEx", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鍙樻崲
    // ---------------------------------------------------------

    pub fn rotation_angle(&self) -> Option<f64> { self.prop_f64("RotationAngle") }
    pub fn set_rotation_angle(&self, v: f64) -> bool { self.put_f64("RotationAngle", v) }

    pub fn rotation_center_x(&self) -> Option<f64> { self.prop_f64("RotationCenterX") }
    pub fn set_rotation_center_x(&self, v: f64) -> bool {
        self.put_f64("RotationCenterX", v)
    }

    pub fn rotation_center_y(&self) -> Option<f64> { self.prop_f64("RotationCenterY") }
    pub fn set_rotation_center_y(&self, v: f64) -> bool {
        self.put_f64("RotationCenterY", v)
    }

    pub fn set_rotation_center(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetRotationCenter", args).is_ok()
    }

    pub fn rotate(&self, angle: f64) -> bool {
        let args = vec![Variant::from_f64(angle)];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    pub fn rotate_ex(&self, angle: f64, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("RotateEx", args).is_ok()
    }

    pub fn skew(&self, ax: f64, ay: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
        ];
        self.disp.invoke_method("Skew", args).is_ok()
    }

    pub fn skew_ex(&self, ax: f64, ay: f64, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("SkewEx", args).is_ok()
    }

    pub fn stretch(&self, sx: f64, sy: f64, stretch_chars: bool) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_bool(stretch_chars),
        ];
        self.disp.invoke_method("Stretch", args).is_ok()
    }

    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// `cdrFlipAxes`
    pub fn flip(&self, axes: i32) -> bool {
        let args = vec![Variant::from_i64(axes as i64)];
        self.disp.invoke_method("Flip", args).is_ok()
    }

    pub fn get_matrix(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        None
    }

    pub fn set_matrix(
        &self,
        d11: f64, d12: f64, d21: f64, d22: f64, tx: f64, ty: f64,
    ) -> bool {
        let args = vec![
            Variant::from_f64(d11),
            Variant::from_f64(d12),
            Variant::from_f64(d21),
            Variant::from_f64(d22),
            Variant::from_f64(tx),
            Variant::from_f64(ty),
        ];
        self.disp.invoke_method("SetMatrix", args).is_ok()
    }

    pub fn transformation_matrix(&self) -> Option<IvgTransformMatrix> {
        self.prop_dispatch("TransformationMatrix").map(IvgTransformMatrix::new)
    }

    pub fn set_transformation_matrix(&self, m: &IvgTransformMatrix) -> bool {
        self.put_dispatch("TransformationMatrix", m.as_variant())
    }

    pub fn apply_transform_matrix(&self, m: &IvgTransformMatrix) -> bool {
        let args = vec![m.as_variant()];
        self.disp.invoke_method("ApplyTransformMatrix", args).is_ok()
    }

    pub fn clear_transformations(&self) -> bool {
        self.disp.invoke_method("ClearTransformations", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 灞?/ 椤?/ 鏍?
    // ---------------------------------------------------------

    pub fn layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("Layer").map(IvgLayer::new)
    }
    pub fn set_layer(&self, l: &IvgLayer) -> bool {
        self.put_dispatch("Layer", l.as_variant())
    }

    pub fn move_to_layer(&self, l: &IvgLayer) -> bool {
        let args = vec![l.as_variant()];
        self.disp.invoke_method("MoveToLayer", args).is_ok()
    }

    pub fn copy_to_layer(&self, l: &IvgLayer) -> Option<IvgShape> {
        let args = vec![l.as_variant()];
        self.disp
            .invoke_method("CopyToLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn copy_to_layer_as_range(&self, l: &IvgLayer) -> Option<IvgShapeRange> {
        let args = vec![l.as_variant()];
        self.disp
            .invoke_method("CopyToLayerAsRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn page(&self) -> Option<IvgPage> { self.prop_dispatch("Page").map(IvgPage::new) }
    pub fn spread(&self) -> Option<crate::page::IvgSpread> {
        self.prop_dispatch("Spread").map(crate::page::IvgSpread::new)
    }
    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 濉厖 / 杞粨 / 閫忔槑
    // ---------------------------------------------------------

    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }
    pub fn set_fill(&self, f: &IvgFill) -> bool {
        self.put_dispatch("Fill", f.as_variant())
    }

    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }

    pub fn transparency(&self) -> Option<IvgTransparency> {
        self.prop_dispatch("Transparency").map(IvgTransparency::new)
    }

    /// `cdrFillMode`
    pub fn fill_mode(&self) -> Option<i64> { self.prop_i64("FillMode") }
    pub fn set_fill_mode(&self, v: i32) -> bool { self.put_i64("FillMode", v as i64) }

    pub fn overprint_fill(&self) -> Option<bool> { self.prop_bool("OverprintFill") }
    pub fn set_overprint_fill(&self, v: bool) -> bool { self.put_bool("OverprintFill", v) }

    pub fn overprint_outline(&self) -> Option<bool> { self.prop_bool("OverprintOutline") }
    pub fn set_overprint_outline(&self, v: bool) -> bool {
        self.put_bool("OverprintOutline", v)
    }

    pub fn overprint_bitmap(&self) -> Option<bool> { self.prop_bool("OverprintBitmap") }
    pub fn set_overprint_bitmap(&self, v: bool) -> bool {
        self.put_bool("OverprintBitmap", v)
    }

    pub fn drape_fill(&self) -> Option<bool> { self.prop_bool("DrapeFill") }
    
    pub fn set_drape_fill(&self, v: bool) -> bool { self.put_bool("DrapeFill", v) }

    pub fn apply_no_fill(&self) -> bool {
        self.fill().map(|f| f.apply_no_fill()).unwrap_or(false)
    }

    pub fn apply_uniform_fill(&self, color: &IvgColor) -> bool {
        self.fill()
            .map(|f| f.apply_uniform_fill(color))
            .unwrap_or(false)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn apply_fountain_fill(
        &self,
        start_color: &IvgColor,
        end_color: &IvgColor,
        fill_type: i32,
        angle: f64,
        steps: i32,
        edge_pad: i32,
        mid_point: i32,
        blend_type: i32,
        center_offset_x: f64,
        center_offset_y: f64,
    ) -> bool {
        let args = vec![
            start_color.as_variant(),
            end_color.as_variant(),
            Variant::from_i64(fill_type as i64),
            Variant::from_f64(angle),
            Variant::from_i64(steps as i64),
            Variant::from_i64(edge_pad as i64),
            Variant::from_i64(mid_point as i64),
            Variant::from_i64(blend_type as i64),
            Variant::from_f64(center_offset_x),
            Variant::from_f64(center_offset_y),
        ];
        self.disp.invoke_method("ApplyFountainFill", args).is_ok()
    }

    /// 鎶婅疆寤撹浆涓虹嫭绔嬪浘褰€?
    pub fn convert_outline_to_object(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("ConvertOutlineToObject", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 鏇茬嚎
    // ---------------------------------------------------------

    pub fn curve(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Curve").map(IvgCurve::new)
    }

    pub fn convert_to_curves(&self) -> bool {
        self.disp.invoke_method("ConvertToCurves", vec![]).is_ok()
    }

    pub fn display_curve(&self) -> Option<IvgCurve> {
        self.prop_dispatch("DisplayCurve").map(IvgCurve::new)
    }

    // ---------------------------------------------------------
    // 瀛愮被鍨嬭鍥?
    // ---------------------------------------------------------

    pub fn rectangle(&self) -> Option<IvgRectangle> {
        self.prop_dispatch("Rectangle").map(IvgRectangle::new)
    }
    pub fn ellipse(&self) -> Option<IvgEllipse> {
        self.prop_dispatch("Ellipse").map(IvgEllipse::new)
    }
    pub fn polygon(&self) -> Option<IvgPolygon> {
        self.prop_dispatch("Polygon").map(IvgPolygon::new)
    }
    pub fn bitmap(&self) -> Option<IvgBitmap> {
        self.prop_dispatch("Bitmap").map(IvgBitmap::new)
    }
    pub fn text(&self) -> Option<IvgText> {
        self.prop_dispatch("Text").map(IvgText::new)
    }
    pub fn eps(&self) -> Option<IvgEps> {
        self.prop_dispatch("EPS").map(IvgEps::new)
    }
    pub fn connector(&self) -> Option<IvgConnector> {
        self.prop_dispatch("Connector").map(IvgConnector::new)
    }
    pub fn guide(&self) -> Option<IvgGuide> {
        self.prop_dispatch("Guide").map(IvgGuide::new)
    }
    pub fn power_clip(&self) -> Option<IvgPowerClip> {
        self.prop_dispatch("PowerClip").map(IvgPowerClip::new)
    }
    pub fn custom(&self) -> Option<IvgCustomShape> {
        self.prop_dispatch("Custom").map(IvgCustomShape::new)
    }
    pub fn symbol(&self) -> Option<IvgSymbol> {
        self.prop_dispatch("Symbol").map(IvgSymbol::new)
    }

    // ---------------------------------------------------------
    // 鏁堟灉
    // ---------------------------------------------------------

    pub fn effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Effects").map(IvgEffects::new)
    }
    pub fn effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("Effect").map(IvgEffect::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_drop_shadow(
        &self,
        shadow_type: i32,
        opacity: i32,
        feather: i32,
        offset_x: f64,
        offset_y: f64,
        color: &IvgColor,
        feather_type: i32,
        feather_edge: i32,
        perspective_angle: f64,
        perspective_stretch: f64,
        fade: i32,
        merge_mode: i32,
    ) -> Option<IvgEffect> {
        let args = vec![
            Variant::from_i64(shadow_type as i64),
            Variant::from_i64(opacity as i64),
            Variant::from_i64(feather as i64),
            Variant::from_f64(offset_x),
            Variant::from_f64(offset_y),
            color.as_variant(),
            Variant::from_i64(feather_type as i64),
            Variant::from_i64(feather_edge as i64),
            Variant::from_f64(perspective_angle),
            Variant::from_f64(perspective_stretch),
            Variant::from_i64(fade as i64),
            Variant::from_i64(merge_mode as i64),
        ];
        self.disp
            .invoke_method("CreateDropShadow", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_blend(
        &self,
        target: &IvgShape,
        steps: i64,
        color_blend_type: i32,
        mode: i32,
        spacing: f64,
        angle: f64,
        loop_: bool,
        path: Option<&IvgShape>,
        rotate_shapes: bool,
        spacing_accel: i32,
        color_accel: i32,
        accel_size: bool,
    ) -> Option<IvgEffect> {
        let path_var = match path {
            Some(p) => p.as_variant(),
            None => Variant::default(),
        };
        let args = vec![
            target.as_variant(),
            Variant::from_i64(steps),
            Variant::from_i64(color_blend_type as i64),
            Variant::from_i64(mode as i64),
            Variant::from_f64(spacing),
            Variant::from_f64(angle),
            Variant::from_bool(loop_),
            path_var,
            Variant::from_bool(rotate_shapes),
            Variant::from_i64(spacing_accel as i64),
            Variant::from_i64(color_accel as i64),
            Variant::from_bool(accel_size),
        ];
        self.disp
            .invoke_method("CreateBlend", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_contour(
        &self,
        direction: i32,
        offset: f64,
        steps: i32,
        blend_type: i32,
        outline_color: &IvgColor,
        fill_color: &IvgColor,
        fill_color2: &IvgColor,
        spacing_accel: i32,
        color_accel: i32,
        end_cap_type: i32,
        corner_type: i32,
        miter_limit: f64,
    ) -> Option<IvgEffect> {
        let args = vec![
            Variant::from_i64(direction as i64),
            Variant::from_f64(offset),
            Variant::from_i64(steps as i64),
            Variant::from_i64(blend_type as i64),
            outline_color.as_variant(),
            fill_color.as_variant(),
            fill_color2.as_variant(),
            Variant::from_i64(spacing_accel as i64),
            Variant::from_i64(color_accel as i64),
            Variant::from_i64(end_cap_type as i64),
            Variant::from_i64(corner_type as i64),
            Variant::from_f64(miter_limit),
        ];
        self.disp
            .invoke_method("CreateContour", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    pub fn create_envelope(
        &self,
        preset_index: i32,
        mode: i32,
        keep_lines: bool,
    ) -> Option<IvgEffect> {
        let args = vec![
            Variant::from_i64(preset_index as i64),
            Variant::from_i64(mode as i64),
            Variant::from_bool(keep_lines),
        ];
        self.disp
            .invoke_method("CreateEnvelope", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    pub fn create_perspective(
        &self,
        hx: Variant,
        hy: Variant,
        vx: Variant,
        vy: Variant,
    ) -> Option<IvgEffect> {
        let args = vec![hx, hy, vx, vy];
        self.disp
            .invoke_method("CreatePerspective", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    pub fn clear_effect(&self, effect_type: i32) -> bool {
        let args = vec![Variant::from_i64(effect_type as i64)];
        self.disp.invoke_method("ClearEffect", args).is_ok()
    }

    pub fn flatten_effects(&self) -> bool {
        self.disp.invoke_method("FlattenEffects", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 缇ょ粍 / 缁勫悎
    // ---------------------------------------------------------

    pub fn group(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Group", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn ungroup(&self) -> bool {
        self.disp.invoke_method("Ungroup", vec![]).is_ok()
    }

    pub fn ungroup_all(&self) -> bool {
        self.disp.invoke_method("UngroupAll", vec![]).is_ok()
    }

    pub fn parent_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ParentGroup").map(IvgShape::new)
    }

    pub fn combine(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Combine", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn break_apart(&self) -> bool {
        self.disp.invoke_method("BreakApart", vec![]).is_ok()
    }

    pub fn separate(&self) -> bool {
        self.disp.invoke_method("Separate", vec![]).is_ok()
    }

    pub fn children(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    // ---------------------------------------------------------
    // 甯冨皵杩愮畻
    // ---------------------------------------------------------

    pub fn weld(&self, target: &IvgShape, leave_src: bool, leave_tgt: bool) -> Option<IvgShape> {
        let args = vec![
            target.as_variant(),
            Variant::from_bool(leave_src),
            Variant::from_bool(leave_tgt),
        ];
        self.disp
            .invoke_method("Weld", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn trim(&self, target: &IvgShape, leave_src: bool, leave_tgt: bool) -> Option<IvgShape> {
        let args = vec![
            target.as_variant(),
            Variant::from_bool(leave_src),
            Variant::from_bool(leave_tgt),
        ];
        self.disp
            .invoke_method("Trim", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn intersect(&self, target: &IvgShape, leave_src: bool, leave_tgt: bool) -> Option<IvgShape> {
        let args = vec![
            target.as_variant(),
            Variant::from_bool(leave_src),
            Variant::from_bool(leave_tgt),
        ];
        self.disp
            .invoke_method("Intersect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 澶嶅埗 / 鍏嬮殕
    // ---------------------------------------------------------

    pub fn duplicate(&self, dx: f64, dy: f64) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Duplicate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn duplicate_as_range(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("DuplicateAsRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn clone(&self, dx: f64, dy: f64) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Clone", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn clone_as_range(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("CloneAsRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn clone_link(&self) -> Option<IvgCloneLink> {
        self.prop_dispatch("CloneLink").map(IvgCloneLink::new)
    }

    pub fn clones(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("Clones").map(IvgShapeRange::new)
    }

    pub fn copy(&self) -> bool {
        self.disp.invoke_method("Copy", vec![]).is_ok()
    }
    pub fn cut(&self) -> bool {
        self.disp.invoke_method("Cut", vec![]).is_ok()
    }
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // Z 椤哄簭
    // ---------------------------------------------------------

    pub fn order_to_front(&self) -> bool {
        self.disp.invoke_method("OrderToFront", vec![]).is_ok()
    }
    pub fn order_to_back(&self) -> bool {
        self.disp.invoke_method("OrderToBack", vec![]).is_ok()
    }
    pub fn order_forward_one(&self) -> bool {
        self.disp.invoke_method("OrderForwardOne", vec![]).is_ok()
    }
    pub fn order_back_one(&self) -> bool {
        self.disp.invoke_method("OrderBackOne", vec![]).is_ok()
    }

    pub fn order_front_of(&self, other: &IvgShape) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("OrderFrontOf", args).is_ok()
    }

    pub fn order_back_of(&self, other: &IvgShape) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("OrderBackOf", args).is_ok()
    }

    pub fn order_reverse(&self) -> bool {
        self.disp.invoke_method("OrderReverse", vec![]).is_ok()
    }

    pub fn z_order(&self) -> Option<i64> { self.prop_i64("ZOrder") }

    // ---------------------------------------------------------
    // 灞炴€у寘 / URL / 鍏冩暟鎹?
    // ---------------------------------------------------------

    pub fn properties(&self) -> Option<IvgProperties> {
        self.prop_dispatch("Properties").map(IvgProperties::new)
    }

    pub fn url(&self) -> Option<IvgUrl> {
        self.prop_dispatch("URL").map(IvgUrl::new)
    }

    // ---------------------------------------------------------
    // 鎶撳彇鐐?
    // ---------------------------------------------------------

    pub fn snap_points(&self) -> Option<IvgSnapPoints> {
        self.prop_dispatch("SnapPoints").map(IvgSnapPoints::new)
    }

    pub fn snap_points_of_type(&self, type_set: i32) -> Option<IvgSnapPoints> {
        let args = vec![Variant::from_i64(type_set as i64)];
        self.disp
            .invoke_method("SnapPointsOfType", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoints::new)
    }

    pub fn find_snap_point(&self, reference_data: impl Into<String>) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_str(reference_data.into())];
        self.disp
            .invoke_method("FindSnapPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    // ---------------------------------------------------------
    // 甯冨皵鏌ヨ
    // ---------------------------------------------------------

    pub fn is_on_shape(&self, x: f64, y: f64, hot_area: f64) -> Option<i64> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp.invoke_method("IsOnShape", args).ok()?.to_i64().ok()
    }

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

    pub fn is_type_any_of(&self, type_list: Variant) -> Option<bool> {
        let args = vec![type_list];
        self.disp
            .invoke_method("IsTypeAnyOf", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 閫夋嫨
    // ---------------------------------------------------------

    pub fn add_to_selection(&self) -> bool {
        self.disp.invoke_method("AddToSelection", vec![]).is_ok()
    }
    pub fn remove_from_selection(&self) -> bool {
        self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok()
    }
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 浣嶅浘鍖?
    // ---------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn convert_to_bitmap(
        &self,
        bit_depth: i32,
        grayscale: bool,
        dithered: bool,
        transparent_bg: bool,
        resolution: i32,
        anti_aliasing: i32,
        use_color_profile: bool,
        multi_channel: bool,
        always_overprint_black: bool,
        overprint_black_limit: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_i64(bit_depth as i64),
            Variant::from_bool(grayscale),
            Variant::from_bool(dithered),
            Variant::from_bool(transparent_bg),
            Variant::from_i64(resolution as i64),
            Variant::from_i64(anti_aliasing as i64),
            Variant::from_bool(use_color_profile),
            Variant::from_bool(multi_channel),
            Variant::from_bool(always_overprint_black),
            Variant::from_i64(overprint_black_limit as i64),
        ];
        self.disp
            .invoke_method("ConvertToBitmap", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鍥炬绮剧‘瑁佸壀 / 杈圭晫 / 鐩搁櫎
    // ---------------------------------------------------------

    pub fn add_to_power_clip(&self, shape: &IvgShape, center_in_container: i32) -> bool {
        let args = vec![
            shape.as_variant(),
            Variant::from_i64(center_in_container as i64),
        ];
        self.disp.invoke_method("AddToPowerClip", args).is_ok()
    }

    pub fn remove_from_container(&self, level: i32) -> bool {
        let args = vec![Variant::from_i64(level as i64)];
        self.disp.invoke_method("RemoveFromContainer", args).is_ok()
    }

    pub fn power_clip_parent(&self) -> Option<IvgShape> {
        self.prop_dispatch("PowerClipParent").map(IvgShape::new)
    }

    pub fn create_boundary(
        &self,
        x: f64,
        y: f64,
        place_on_top: bool,
        delete_source: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(place_on_top),
            Variant::from_bool(delete_source),
        ];
        self.disp
            .invoke_method("CreateBoundary", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn equal_divide(
        &self,
        divisions: i32,
        gap: f64,
        group: bool,
        combine: bool,
        delete_source: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_i64(divisions as i64),
            Variant::from_f64(gap),
            Variant::from_bool(group),
            Variant::from_bool(combine),
            Variant::from_bool(delete_source),
        ];
        self.disp
            .invoke_method("EqualDivide", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn step_and_repeat(
        &self,
        num_copies: i32,
        distance_x: f64,
        distance_y: f64,
        mode_x: i32,
        direction_x: i32,
        mode_y: i32,
        direction_y: i32,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_i64(num_copies as i64),
            Variant::from_f64(distance_x),
            Variant::from_f64(distance_y),
            Variant::from_i64(mode_x as i64),
            Variant::from_i64(direction_x as i64),
            Variant::from_i64(mode_y as i64),
            Variant::from_i64(direction_y as i64),
        ];
        self.disp
            .invoke_method("StepAndRepeat", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 瀵归綈 / 鍒嗗竷
    // ---------------------------------------------------------

    /// `cdrAlignType` / `cdrTextAlignOrigin`
    pub fn align_to_shape(&self, align_type: i32, shape: &IvgShape, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            shape.as_variant(),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToShape", args).is_ok()
    }

    pub fn align_to_shape_range(
        &self,
        align_type: i32,
        range: &IvgShapeRange,
        text_origin: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            range.as_variant(),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToShapeRange", args).is_ok()
    }

    pub fn align_to_page(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPage", args).is_ok()
    }

    pub fn align_to_page_center(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPageCenter", args).is_ok()
    }

    pub fn align_to_grid(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToGrid", args).is_ok()
    }

    pub fn align_to_point(
        &self,
        align_type: i32,
        x: f64,
        y: f64,
        text_origin: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPoint", args).is_ok()
    }

    /// `cdrDistributeType`
    pub fn distribute(&self, dist_type: i32, page_extent: bool) -> bool {
        let args = vec![
            Variant::from_i64(dist_type as i64),
            Variant::from_bool(page_extent),
        ];
        self.disp.invoke_method("Distribute", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鍦嗚 / 鍊掕 / 鎵囪礉
    // ---------------------------------------------------------

    pub fn fillet(&self, radius: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Fillet", args).is_ok()
    }

    pub fn chamfer(&self, a: f64, b: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(a),
            Variant::from_f64(b),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Chamfer", args).is_ok()
    }

    pub fn scallop(&self, radius: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Scallop", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鏍峰紡
    // ---------------------------------------------------------

    pub fn apply_style(&self, style_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(style_name.into())];
        self.disp.invoke_method("ApplyStyle", args).is_ok()
    }

    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }

    // ---------------------------------------------------------
    // 鏂囨湰鐜粫
    // ---------------------------------------------------------

    /// `cdrWrapStyle`
    pub fn wrap_text(&self) -> Option<i64> { self.prop_i64("WrapText") }
    pub fn set_wrap_text(&self, v: i32) -> bool { self.put_i64("WrapText", v as i64) }

    pub fn text_wrap_offset(&self) -> Option<f64> { self.prop_f64("TextWrapOffset") }
    pub fn set_text_wrap_offset(&self, v: f64) -> bool {
        self.put_f64("TextWrapOffset", v)
    }

    pub fn place_text_inside(&self, text_shape: &IvgShape) -> Option<IvgShape> {
        let args = vec![text_shape.as_variant()];
        self.disp
            .invoke_method("PlaceTextInside", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
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

    // ---------------------------------------------------------
    // 澶嶅埗灞炴€?/ 姣旇緝
    // ---------------------------------------------------------

    /// `cdrCopyProperties` 浣嶆帺鐮併€?
    pub fn copy_properties_from(&self, source: &IvgShape, properties: i32) -> Option<bool> {
        let args = vec![
            source.as_variant(),
            Variant::from_i64(properties as i64),
        ];
        self.disp
            .invoke_method("CopyPropertiesFrom", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// `cdrCompareType` / `cdrCompareCondition`
    pub fn compare_to(
        &self,
        other: &IvgShape,
        compare_type: i32,
        condition: i32,
    ) -> Option<bool> {
        let args = vec![
            other.as_variant(),
            Variant::from_i64(compare_type as i64),
            Variant::from_i64(condition as i64),
        ];
        self.disp
            .invoke_method("CompareTo", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 
    // ---------------------------------------------------------

    pub fn get_colors(&self, max_bitmap_colors: i32) -> Option<Variant> {
        let args = vec![Variant::from_i64(max_bitmap_colors as i64)];
        self.disp.invoke_method("GetColors", args).ok()
    }

    pub fn get_color_types(&self) -> Option<Variant> {
        self.disp.invoke_method("GetColorTypes", vec![]).ok()
    }

    pub fn evaluate(&self, expr: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(expr.into())];
        self.disp.invoke_method("Evaluate", args).ok()
    }
}