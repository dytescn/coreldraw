//! `IVGPatternFill` 鈥斺€?鍥炬濉厖
//! `IVGPatternCanvas(es)` 鈥斺€?鍥炬鐢诲竷

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgPatternFill
// =============================================================

pub struct IvgPatternFill {
    disp: ComObject,
}

impl IvgPatternFill {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        self.disp.set_property(name, vec![c.as_variant()]).is_ok()
    }

    // ---- 绫诲瀷 / 棰滆壊 ----

    /// `cdrPatternFillType`
    pub fn pattern_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_pattern_type(&self, v: i32) -> bool {
        self.put_i64("Type", v as i64)
    }

    pub fn front_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FrontColor").map(IvgColor::new)
    }
    pub fn set_front_color(&self, c: &IvgColor) -> bool {
        self.put_color("FrontColor", c)
    }

    pub fn back_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BackColor").map(IvgColor::new)
    }
    pub fn set_back_color(&self, c: &IvgColor) -> bool {
        self.put_color("BackColor", c)
    }

    // ---- 鐢诲竷 ----

    pub fn canvas(&self) -> Option<IvgPatternCanvas> {
        self.prop_dispatch("Canvas").map(IvgPatternCanvas::new)
    }

    pub fn set_canvas(&self, c: &IvgPatternCanvas) -> bool {
        self.disp.set_property("Canvas", vec![c.as_variant()]).is_ok()
    }

    pub fn file_path(&self) -> Option<String> {
        self.prop_string("FilePath")
    }

    // ---- 鍘熺偣 ----

    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    // ---- 骞抽摵灏哄 ----

    pub fn tile_width(&self) -> Option<f64> { self.prop_f64("TileWidth") }
    pub fn set_tile_width(&self, v: f64) -> bool { self.put_f64("TileWidth", v) }

    pub fn tile_height(&self) -> Option<f64> { self.prop_f64("TileHeight") }
    pub fn set_tile_height(&self, v: f64) -> bool { self.put_f64("TileHeight", v) }

    /// `cdrTileOffsetType`
    pub fn tile_offset_type(&self) -> Option<i64> {
        self.prop_i64("TileOffsetType")
    }
    pub fn set_tile_offset_type(&self, v: i32) -> bool {
        self.put_i64("TileOffsetType", v as i64)
    }

    pub fn tile_offset(&self) -> Option<i64> { self.prop_i64("TileOffset") }
    pub fn set_tile_offset(&self, v: i32) -> bool { self.put_i64("TileOffset", v as i64) }

    // ---- 鏃嬭浆 / 鍊炬枩 ----

    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    pub fn rotation_angle(&self) -> Option<f64> {
        self.prop_f64("RotationAngle")
    }
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    // ---- 鍙樻崲鏍囧織 ----

    pub fn transform_with_shape(&self) -> Option<bool> {
        self.prop_bool("TransformWithShape")
    }
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    // ---- 闀滃儚 ----

    pub fn mirror_fill(&self) -> Option<bool> { self.prop_bool("MirrorFill") }
    pub fn set_mirror_fill(&self, v: bool) -> bool { self.put_bool("MirrorFill", v) }

    pub fn mirror_fill_x(&self) -> Option<bool> { self.prop_bool("MirrorFillX") }
    pub fn set_mirror_fill_x(&self, v: bool) -> bool { self.put_bool("MirrorFillX", v) }

    pub fn mirror_fill_y(&self) -> Option<bool> { self.prop_bool("MirrorFillY") }
    pub fn set_mirror_fill_y(&self, v: bool) -> bool { self.put_bool("MirrorFillY", v) }

    // ---- 鍔犺浇 ----

    pub fn load(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IvgPatternCanvas
// =============================================================

pub struct IvgPatternCanvas {
    disp: ComObject,
}

impl IvgPatternCanvas {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    // fn prop_bool(&self, name: &str) -> Option<bool> {
    //     self.disp.get_property(name).ok()?.to_bool().ok()
    // }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 鍩烘湰灞炴€?----

    /// `cdrPatternCanvasSize`
    pub fn size(&self) -> Option<i64> { self.prop_i64("Size") }
    pub fn set_size(&self, v: i32) -> bool { self.put_i64("Size", v as i64) }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    /// 鐢诲竷鍐呭锛堜綅鍥惧瓧绗︿覆锛屾牸寮忕敱 CorelDRAW 瀹氫箟锛夈€?
    pub fn data(&self) -> Option<String> { self.prop_string("Data") }
    pub fn set_data(&self, v: impl Into<String>) -> bool {
        self.put_string("Data", v)
    }

    // ---- 鍍忕礌鎿嶄綔 ----

    pub fn pixel(&self, x: i32, y: i32) -> Option<bool> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("Pixel", args).ok()?.to_bool().ok()
    }

    pub fn set_pixel(&self, x: i32, y: i32, v: bool) -> bool {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
            Variant::from_bool(v),
        ];
        self.disp.invoke_method("put_Pixel", args).is_ok()
    }

    // ---- 鍖哄煙鎿嶄綔 ----

    pub fn fill_area(&self, x1: i32, y1: i32, x2: i32, y2: i32, state: bool) -> bool {
        let args = vec![
            Variant::from_i64(x1 as i64),
            Variant::from_i64(y1 as i64),
            Variant::from_i64(x2 as i64),
            Variant::from_i64(y2 as i64),
            Variant::from_bool(state),
        ];
        self.disp.invoke_method("FillArea", args).is_ok()
    }

    pub fn copy_area(&self, x1: i32, y1: i32, x2: i32, y2: i32, x: i32, y: i32) -> bool {
        let args = vec![
            Variant::from_i64(x1 as i64),
            Variant::from_i64(y1 as i64),
            Variant::from_i64(x2 as i64),
            Variant::from_i64(y2 as i64),
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("CopyArea", args).is_ok()
    }

    pub fn flip_area(&self, x1: i32, y1: i32, x2: i32, y2: i32, axes: i32) -> bool {
        let args = vec![
            Variant::from_i64(x1 as i64),
            Variant::from_i64(y1 as i64),
            Variant::from_i64(x2 as i64),
            Variant::from_i64(y2 as i64),
            Variant::from_i64(axes as i64),
        ];
        self.disp.invoke_method("FlipArea", args).is_ok()
    }

    pub fn rotate_area(&self, x1: i32, y1: i32, x2: i32, y2: i32, angle: f64) -> bool {
        let args = vec![
            Variant::from_i64(x1 as i64),
            Variant::from_i64(y1 as i64),
            Variant::from_i64(x2 as i64),
            Variant::from_i64(y2 as i64),
            Variant::from_f64(angle),
        ];
        self.disp.invoke_method("RotateArea", args).is_ok()
    }

    // ---- 鍏跺畠 ----

    pub fn select(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Select", args).is_ok()
    }

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    pub fn put_copy(&self, src: &IvgPatternCanvas) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("PutCopy", args).is_ok()
    }

    pub fn p_set(&self, step: i16, x: i32, y: i32, color: bool) -> bool {
        let args = vec![
            Variant::from_i64(step as i64),
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
            Variant::from_bool(color),
        ];
        self.disp.invoke_method("PSet", args).is_ok()
    }

    pub fn line(
        &self,
        flags: i16,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: bool,
    ) -> bool {
        let args = vec![
            Variant::from_i64(flags as i64),
            Variant::from_i64(x1 as i64),
            Variant::from_i64(y1 as i64),
            Variant::from_i64(x2 as i64),
            Variant::from_i64(y2 as i64),
            Variant::from_bool(color),
        ];
        self.disp.invoke_method("Line", args).is_ok()
    }
}

// =============================================================
// IvgPatternCanvases
// =============================================================

pub struct IvgPatternCanvases {
    disp: ComObject,
}

impl IvgPatternCanvases {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgPatternCanvas> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPatternCanvas::new)
    }

    pub fn add(&self, canvas: &IvgPatternCanvas) -> Option<i64> {
        let args = vec![canvas.as_variant()];
        self.disp.invoke_method("Add", args).ok()?.to_i64().ok()
    }

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
}