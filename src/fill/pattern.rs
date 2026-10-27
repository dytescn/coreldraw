//! `IVGPatternFill` —— 图案填充
//! `IVGPatternCanvas(es)` —— 图案画布

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgPatternFill
// =============================================================

/// CorelDRAW `IVGPatternFill` 对象的后期绑定包装（图案填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPatternFill {
    disp: ComObject,
}

impl IvgPatternFill {
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

    // ---- 类型 / 颜色 ----

    /// `cdrPatternFillType`
    pub fn pattern_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置图案类型（`Type`）。
    pub fn set_pattern_type(&self, v: i32) -> bool {
        self.put_i64("Type", v as i64)
    }

    /// 读取前颜色（`FrontColor`）。
    pub fn front_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FrontColor").map(IvgColor::new)
    }
    /// 设置前颜色（`FrontColor`）。
    pub fn set_front_color(&self, c: &IvgColor) -> bool {
        self.put_color("FrontColor", c)
    }

    /// 读取后颜色（`BackColor`）。
    pub fn back_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BackColor").map(IvgColor::new)
    }
    /// 设置后颜色（`BackColor`）。
    pub fn set_back_color(&self, c: &IvgColor) -> bool {
        self.put_color("BackColor", c)
    }

    // ---- 画布 ----

    /// 读取 COM 属性 `Canvas`。
    pub fn canvas(&self) -> Option<IvgPatternCanvas> {
        self.prop_dispatch("Canvas").map(IvgPatternCanvas::new)
    }

    /// 设置 COM 属性 `Canvas`。
    pub fn set_canvas(&self, c: &IvgPatternCanvas) -> bool {
        self.disp.set_property("Canvas", vec![c.as_variant()]).is_ok()
    }

    /// 读取文件路径（`FilePath`）。
    pub fn file_path(&self) -> Option<String> {
        self.prop_string("FilePath")
    }

    // ---- 原点 ----

    /// 读取原点 X 坐标（`OriginX`）。
    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    /// 设置原点 X 坐标（`OriginX`）。
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    /// 读取原点 Y 坐标（`OriginY`）。
    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    /// 设置原点 Y 坐标（`OriginY`）。
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    // ---- 平铺尺寸 ----

    /// 读取平铺宽度（`TileWidth`）。
    pub fn tile_width(&self) -> Option<f64> { self.prop_f64("TileWidth") }
    /// 设置平铺宽度（`TileWidth`）。
    pub fn set_tile_width(&self, v: f64) -> bool { self.put_f64("TileWidth", v) }

    /// 读取平铺高度（`TileHeight`）。
    pub fn tile_height(&self) -> Option<f64> { self.prop_f64("TileHeight") }
    /// 设置平铺高度（`TileHeight`）。
    pub fn set_tile_height(&self, v: f64) -> bool { self.put_f64("TileHeight", v) }

    /// `cdrTileOffsetType`
    pub fn tile_offset_type(&self) -> Option<i64> {
        self.prop_i64("TileOffsetType")
    }
    /// 设置平铺偏移类型（`TileOffsetType`）。
    pub fn set_tile_offset_type(&self, v: i32) -> bool {
        self.put_i64("TileOffsetType", v as i64)
    }

    /// 读取平铺偏移（`TileOffset`）。
    pub fn tile_offset(&self) -> Option<i64> { self.prop_i64("TileOffset") }
    /// 设置平铺偏移（`TileOffset`）。
    pub fn set_tile_offset(&self, v: i32) -> bool { self.put_i64("TileOffset", v as i64) }

    // ---- 旋转 / 倾斜 ----

    /// 读取倾斜角度（`SkewAngle`）。
    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    /// 设置倾斜角度（`SkewAngle`）。
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    /// 读取旋转角度（`RotationAngle`）。
    pub fn rotation_angle(&self) -> Option<f64> {
        self.prop_f64("RotationAngle")
    }
    /// 设置旋转角度（`RotationAngle`）。
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    // ---- 变换标志 ----

    /// 读取变换图形（`TransformWithShape`）。
    pub fn transform_with_shape(&self) -> Option<bool> {
        self.prop_bool("TransformWithShape")
    }
    /// 设置变换图形（`TransformWithShape`）。
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    // ---- 镜像 ----

    /// 读取镜像填充（`MirrorFill`）。
    pub fn mirror_fill(&self) -> Option<bool> { self.prop_bool("MirrorFill") }
    /// 设置镜像填充（`MirrorFill`）。
    pub fn set_mirror_fill(&self, v: bool) -> bool { self.put_bool("MirrorFill", v) }

    /// 读取镜像填充 X 坐标（`MirrorFillX`）。
    pub fn mirror_fill_x(&self) -> Option<bool> { self.prop_bool("MirrorFillX") }
    /// 设置镜像填充 X 坐标（`MirrorFillX`）。
    pub fn set_mirror_fill_x(&self, v: bool) -> bool { self.put_bool("MirrorFillX", v) }

    /// 读取镜像填充 Y 坐标（`MirrorFillY`）。
    pub fn mirror_fill_y(&self) -> Option<bool> { self.prop_bool("MirrorFillY") }
    /// 设置镜像填充 Y 坐标（`MirrorFillY`）。
    pub fn set_mirror_fill_y(&self, v: bool) -> bool { self.put_bool("MirrorFillY", v) }

    // ---- 加载 ----

    /// load（COM 方法 `Load`）。
    pub fn load(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IvgPatternCanvas
// =============================================================

/// CorelDRAW `IVGPatternFill` 对象的后期绑定包装（图案画布）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPatternCanvas {
    disp: ComObject,
}

impl IvgPatternCanvas {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
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

    // ---- 基本属性----

    /// `cdrPatternCanvasSize`
    pub fn size(&self) -> Option<i64> { self.prop_i64("Size") }
    /// 设置尺寸（`Size`）。
    pub fn set_size(&self, v: i32) -> bool { self.put_i64("Size", v as i64) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    /// 画布内容（位图字符串，格式由 CorelDRAW 定义）。
    pub fn data(&self) -> Option<String> { self.prop_string("Data") }
    /// 设置数据（`Data`）。
    pub fn set_data(&self, v: impl Into<String>) -> bool {
        self.put_string("Data", v)
    }

    // ---- 像素操作 ----

    /// 像素（COM 方法 `Pixel`）。
    pub fn pixel(&self, x: i32, y: i32) -> Option<bool> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("Pixel", args).ok()?.to_bool().ok()
    }

    /// 设置像素（`put_Pixel`）。
    pub fn set_pixel(&self, x: i32, y: i32, v: bool) -> bool {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
            Variant::from_bool(v),
        ];
        self.disp.invoke_method("put_Pixel", args).is_ok()
    }

    // ---- 区域操作 ----

    /// 填充区域（COM 方法 `FillArea`）。
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

    /// 复制区域（COM 方法 `CopyArea`）。
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

    /// 翻转区域（COM 方法 `FlipArea`）。
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

    /// 旋转区域（COM 方法 `RotateArea`）。
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

    // ---- 其它 ----

    /// 选择（COM 方法 `Select`）。
    pub fn select(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Select", args).is_ok()
    }

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 设置复制（`PutCopy`）。
    pub fn put_copy(&self, src: &IvgPatternCanvas) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("PutCopy", args).is_ok()
    }

    /// 调用 COM 方法 `PSet`。
    pub fn p_set(&self, step: i16, x: i32, y: i32, color: bool) -> bool {
        let args = vec![
            Variant::from_i64(step as i64),
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
            Variant::from_bool(color),
        ];
        self.disp.invoke_method("PSet", args).is_ok()
    }

    /// 线（COM 方法 `Line`）。
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

/// CorelDRAW `IVGPatternFill` 对象的后期绑定包装（图案画布集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPatternCanvases {
    disp: ComObject,
}

impl IvgPatternCanvases {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPatternCanvas> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPatternCanvas::new)
    }

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, canvas: &IvgPatternCanvas) -> Option<i64> {
        let args = vec![canvas.as_variant()];
        self.disp.invoke_method("Add", args).ok()?.to_i64().ok()
    }

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
}