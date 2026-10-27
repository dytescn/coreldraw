//! `IVGHatchFill` —— 剖面线填充
//! `IVGHatchPattern(s)` —— 剖面线图案
//! `IVGHatchLibrary / Libraries` —— 剖面线库
//! `IVGHatchFills` —— 库里的填充集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::outline::IvgOutlineStyle;

// =============================================================
// IvgHatchFill
// =============================================================

/// CorelDRAW `IVGHatchFill` 对象的后期绑定包装（剖面线填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgHatchFill {
    disp: ComObject,
}

impl IvgHatchFill {
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

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

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

    // ---- 背景色----

    /// 读取后颜色（`BackColor`）。
    pub fn back_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("BackColor").map(IvgColor::new)
    }
    /// 设置后颜色（`BackColor`）。
    pub fn set_back_color(&self, c: &IvgColor) -> bool {
        self.put_color("BackColor", c)
    }

    /// 是否有背景（`HasBackground`）。
    pub fn has_background(&self) -> Option<bool> { self.prop_bool("HasBackground") }

    /// 设置无后颜色（`SetNoBackColor`）。
    pub fn set_no_back_color(&self) -> bool {
        self.disp.invoke_method("SetNoBackColor", vec![]).is_ok()
    }

    // ---- 变换 ----

    /// 读取变换图形（`TransformWithShape`）。
    pub fn transform_with_shape(&self) -> Option<bool> {
        self.prop_bool("TransformWithShape")
    }
    /// 设置变换图形（`TransformWithShape`）。
    pub fn set_transform_with_shape(&self, v: bool) -> bool {
        self.put_bool("TransformWithShape", v)
    }

    /// 读取缩放线图形（`ScaleLinesWithShape`）。
    pub fn scale_lines_with_shape(&self) -> Option<bool> {
        self.prop_bool("ScaleLinesWithShape")
    }
    /// 设置缩放线图形（`ScaleLinesWithShape`）。
    pub fn set_scale_lines_with_shape(&self, v: bool) -> bool {
        self.put_bool("ScaleLinesWithShape", v)
    }

    /// 读取 COM 属性 `UseWorldCoordinates`。
    pub fn use_world_coordinates(&self) -> Option<bool> {
        self.prop_bool("UseWorldCoordinates")
    }
    /// 设置 COM 属性 `UseWorldCoordinates`。
    pub fn set_use_world_coordinates(&self, v: bool) -> bool {
        self.put_bool("UseWorldCoordinates", v)
    }

    // ---- 应/ 名称 ----

    /// 读取库名称（`LibraryName`）。
    pub fn library_name(&self) -> Option<String> {
        self.prop_string("LibraryName")
    }

    /// 读取剖面线名称（`HatchName`）。
    pub fn hatch_name(&self) -> Option<String> {
        self.prop_string("HatchName")
    }

    /// 是否库（`IsFromLibrary`）。
    pub fn is_from_library(&self) -> Option<bool> { self.prop_bool("IsFromLibrary") }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取库（`Library`）。
    pub fn library(&self) -> Option<IvgHatchLibrary> {
        self.prop_dispatch("Library").map(IvgHatchLibrary::new)
    }

    // ---- 缩放 ----

    /// 读取填充缩放 X 坐标（`FillScaleX`）。
    pub fn fill_scale_x(&self) -> Option<f64> { self.prop_f64("FillScaleX") }
    /// 设置填充缩放 X 坐标（`FillScaleX`）。
    pub fn set_fill_scale_x(&self, v: f64) -> bool { self.put_f64("FillScaleX", v) }

    /// 读取填充缩放 Y 坐标（`FillScaleY`）。
    pub fn fill_scale_y(&self) -> Option<f64> { self.prop_f64("FillScaleY") }
    /// 设置填充缩放 Y 坐标（`FillScaleY`）。
    pub fn set_fill_scale_y(&self, v: f64) -> bool { self.put_f64("FillScaleY", v) }

    /// 读取旋转角度（`RotationAngle`）。
    pub fn rotation_angle(&self) -> Option<f64> {
        self.prop_f64("RotationAngle")
    }
    /// 设置旋转角度（`RotationAngle`）。
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    /// 读取倾斜角度（`SkewAngle`）。
    pub fn skew_angle(&self) -> Option<f64> { self.prop_f64("SkewAngle") }
    /// 设置倾斜角度（`SkewAngle`）。
    pub fn set_skew_angle(&self, v: f64) -> bool { self.put_f64("SkewAngle", v) }

    /// 设置填充缩放（`SetFillScale`）。
    pub fn set_fill_scale(&self, v: f64) -> bool {
        let args = vec![Variant::from_f64(v)];
        self.disp.invoke_method("SetFillScale", args).is_ok()
    }

    /// 读取填充缩放（`GetFillScale`）。
    pub fn get_fill_scale(&self) -> Option<f64> {
        self.disp.invoke_method("GetFillScale", vec![]).ok()?.to_f64().ok()
    }

    /// 设置线缩放（`SetLineScale`）。
    pub fn set_line_scale(&self, v: f64) -> bool {
        let args = vec![Variant::from_f64(v)];
        self.disp.invoke_method("SetLineScale", args).is_ok()
    }

    /// 读取线缩放（`GetLineScale`）。
    pub fn get_line_scale(&self) -> Option<f64> {
        self.disp.invoke_method("GetLineScale", vec![]).ok()?.to_f64().ok()
    }

    // ---- 图案 ----

    /// 读取 COM 属性 `Patterns`。
    pub fn patterns(&self) -> Option<IvgHatchPatterns> {
        self.prop_dispatch("Patterns").map(IvgHatchPatterns::new)
    }

    /// 添加图案。
    #[allow(clippy::too_many_arguments)]
    pub fn add_pattern(
        &self,
        angle: f64,
        spacing: f64,
        shift: f64,
        origin_x: f64,
        origin_y: f64,
        width: f64,
        color: &IvgColor,
        style: &IvgOutlineStyle,
        dash_dot_length: f64,
        pen_width: f64,
    ) -> Option<IvgHatchPattern> {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(spacing),
            Variant::from_f64(shift),
            Variant::from_f64(origin_x),
            Variant::from_f64(origin_y),
            Variant::from_f64(width),
            color.as_variant(),
            style.as_variant(),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
        ];
        self.disp
            .invoke_method("AddPattern", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchPattern::new)
    }

    /// 保存到剖面线库。
    pub fn add_to_library(
        &self,
        library_name: impl Into<String>,
        hatch_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(library_name.into()),
            Variant::from_str(hatch_name.into()),
        ];
        self.disp.invoke_method("AddToLibrary", args).is_ok()
    }

    /// 从剖面线库选择。
    #[allow(clippy::too_many_arguments)]
    pub fn select(
        &self,
        library_name: impl Into<String>,
        hatch_name_or_index: Variant,
        back_color: &IvgColor,
        transform_with_shape: bool,
        scale_lines_with_shape: bool,
        use_world_coordinates: bool,
        fill_scale: f64,
        line_scale: f64,
        fill_angle: f64,
        fill_skew: f64,
    ) -> bool {
        let args = vec![
            Variant::from_str(library_name.into()),
            hatch_name_or_index,
            back_color.as_variant(),
            Variant::from_bool(transform_with_shape),
            Variant::from_bool(scale_lines_with_shape),
            Variant::from_bool(use_world_coordinates),
            Variant::from_f64(fill_scale),
            Variant::from_f64(line_scale),
            Variant::from_f64(fill_angle),
            Variant::from_f64(fill_skew),
        ];
        self.disp.invoke_method("Select", args).is_ok()
    }
}

// =============================================================
// IvgHatchPatterns
// =============================================================

/// CorelDRAW `IVGHatchFill` 对象的后期绑定包装（剖面线图案）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgHatchPatterns {
    disp: ComObject,
}

impl IvgHatchPatterns {
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
    pub fn item(&self, index: i32) -> Option<IvgHatchPattern> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchPattern::new)
    }

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }
}

// =============================================================
// IvgHatchPattern
// =============================================================

/// CorelDRAW `IVGHatchFill` 对象的后期绑定包装（剖面线图案）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgHatchPattern {
    disp: ComObject,
}

impl IvgHatchPattern {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
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

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 基本属性----

    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    /// 设置角度（`Angle`）。
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    /// 读取原点 X 坐标（`OriginX`）。
    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    /// 设置原点 X 坐标（`OriginX`）。
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    /// 读取原点 Y 坐标（`OriginY`）。
    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    /// 设置原点 Y 坐标（`OriginY`）。
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    /// 读取间距（`Spacing`）。
    pub fn spacing(&self) -> Option<f64> { self.prop_f64("Spacing") }
    /// 设置间距（`Spacing`）。
    pub fn set_spacing(&self, v: f64) -> bool { self.put_f64("Spacing", v) }

    /// 读取偏移（`Shift`）。
    pub fn shift(&self) -> Option<f64> { self.prop_f64("Shift") }
    /// 设置偏移（`Shift`）。
    pub fn set_shift(&self, v: f64) -> bool { self.put_f64("Shift", v) }

    /// 读取偏移百分比（`ShiftPercent`）。
    pub fn shift_percent(&self) -> Option<f64> { self.prop_f64("ShiftPercent") }
    /// 设置偏移百分比（`ShiftPercent`）。
    pub fn set_shift_percent(&self, v: f64) -> bool {
        self.put_f64("ShiftPercent", v)
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 图案的轮廓（线条属性）。
    pub fn outline(&self) -> Option<crate::outline::IvgOutline> {
        self.prop_dispatch("Outline").map(crate::outline::IvgOutline::new)
    }

    // ---- 操作 ----

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 设置原点（`SetOrigin`）。
    pub fn set_origin(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetOrigin", args).is_ok()
    }

    /// 读取原点。
    pub fn get_origin(&self) -> Option<(f64, f64)> {
        // 发 out 参数，用属性代替
        Some((self.origin_x()?, self.origin_y()?))
    }

    /// 一次性设置。
    #[allow(clippy::too_many_arguments)]
    pub fn set_properties(
        &self,
        angle: Variant,
        spacing: f64,
        shift: Variant,
        origin_x: Variant,
        origin_y: Variant,
        width: f64,
        color: &IvgColor,
        style: &IvgOutlineStyle,
        dash_dot_length: f64,
        pen_width: f64,
    ) -> bool {
        let args = vec![
            angle,
            Variant::from_f64(spacing),
            shift,
            origin_x,
            origin_y,
            Variant::from_f64(width),
            color.as_variant(),
            style.as_variant(),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }
}

// =============================================================
// IvgHatchLibrary
// =============================================================

/// CorelDRAW `IVGHatchFill` 对象的后期绑定包装（剖面线库）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgHatchLibrary {
    disp: ComObject,
}

impl IvgHatchLibrary {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
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

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取显示名称（`DisplayName`）。
    pub fn display_name(&self) -> Option<String> {
        self.prop_string("DisplayName")
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取活动（`Active`）。
    pub fn active(&self) -> Option<bool> { self.prop_bool("Active") }

    /// 读取填充（`Fills`）。
    pub fn fills(&self) -> Option<IvgHatchFills> {
        self.prop_dispatch("Fills").map(IvgHatchFills::new)
    }

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }
}

// =============================================================
// IvgHatchLibraries
// =============================================================

/// CorelDRAW `IVGHatchFill` 对象的后期绑定包装（剖面线库集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgHatchLibraries {
    disp: ComObject,
}

impl IvgHatchLibraries {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgHatchLibrary> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchLibrary::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, name: impl Into<String>) -> Option<IvgHatchLibrary> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgHatchLibrary::new)
    }

    /// 读取活动库（`ActiveLibrary`）。
    pub fn active_library(&self) -> Option<IvgHatchLibrary> {
        self.prop_dispatch("ActiveLibrary").map(IvgHatchLibrary::new)
    }

    /// 读取默认值库（`DefaultLibrary`）。
    pub fn default_library(&self) -> Option<IvgHatchLibrary> {
        self.prop_dispatch("DefaultLibrary").map(IvgHatchLibrary::new)
    }
}

// =============================================================
// IvgHatchFills
// =============================================================

/// CorelDRAW `IVGHatchFill` 对象的后期绑定包装（剖面线填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgHatchFills {
    disp: ComObject,
}

impl IvgHatchFills {
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
    pub fn item(&self, index_or_name: Variant) -> Option<crate::fill::IvgFill> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgFill::new)
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, name: impl Into<String>) -> Option<crate::fill::IvgFill> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgFill::new)
    }
}