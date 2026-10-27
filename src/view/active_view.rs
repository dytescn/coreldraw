//! `IVGActiveView` / `IVGProofColorSettings` —— 活动视图 / 校样设置

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::{IvgColor, IvgColorContext};
use crate::shape::{IvgShape, IvgShapeRange};

// =============================================================
// IvgActiveView
// =============================================================

/// CorelDRAW `IVGActiveView` 对象的后期绑定包装（活动视图）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgActiveView {
    disp: ComObject,
}

impl IvgActiveView {
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

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 通用工具
    // ---------------------------------------------------------

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
    // 视图类型 / 缩放 / 原点
    // ---------------------------------------------------------

    /// `cdrViewType` ——见 `enums::view`
    pub fn view_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置视图类型（`Type`）。
    pub fn set_view_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取原点 X 坐标（`OriginX`）。
    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    /// 设置原点 X 坐标（`OriginX`）。
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    /// 读取原点 Y 坐标（`OriginY`）。
    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    /// 设置原点 Y 坐标（`OriginY`）。
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    /// 缩放（`Zoom` 为百分比，如 100.0）。
    pub fn zoom(&self) -> Option<f64> { self.prop_f64("Zoom") }
    /// 设置缩放（`Zoom`）。
    pub fn set_zoom(&self, v: f64) -> bool { self.put_f64("Zoom", v) }

    // ---------------------------------------------------------
    // 适配视图
    // ---------------------------------------------------------

    /// 转换为 fit 页面（COM 方法 `ToFitPage`）。
    pub fn to_fit_page(&self) -> bool {
        self.disp.invoke_method("ToFitPage", vec![]).is_ok()
    }

    /// 转换为 fit 页面宽度（COM 方法 `ToFitPageWidth`）。
    pub fn to_fit_page_width(&self) -> bool {
        self.disp.invoke_method("ToFitPageWidth", vec![]).is_ok()
    }

    /// 转换为 fit 页面高度（COM 方法 `ToFitPageHeight`）。
    pub fn to_fit_page_height(&self) -> bool {
        self.disp.invoke_method("ToFitPageHeight", vec![]).is_ok()
    }

    /// 转换为 fit 图形（COM 方法 `ToFitShape`）。
    pub fn to_fit_shape(&self, shape: &IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("ToFitShape", args).is_ok()
    }

    /// 转换为 fit 选择（COM 方法 `ToFitSelection`）。
    pub fn to_fit_selection(&self) -> bool {
        self.disp.invoke_method("ToFitSelection", vec![]).is_ok()
    }

    /// 转换为 fit 区域（COM 方法 `ToFitArea`）。
    pub fn to_fit_area(&self, left: f64, top: f64, right: f64, bottom: f64) -> bool {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
        ];
        self.disp.invoke_method("ToFitArea", args).is_ok()
    }

    /// 转换为 fit 全部对象集（COM 方法 `ToFitAllObjects`）。
    pub fn to_fit_all_objects(&self) -> bool {
        self.disp.invoke_method("ToFitAllObjects", vec![]).is_ok()
    }

    /// 转换为 fit 图形范围（COM 方法 `ToFitShapeRange`）。
    pub fn to_fit_shape_range(&self, range: &IvgShapeRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("ToFitShapeRange", args).is_ok()
    }

    // ---------------------------------------------------------
    // 视点 / 缩放操作
    // ---------------------------------------------------------

    /// 设置视图点（`SetViewPoint`）。
    pub fn set_view_point(&self, x: f64, y: f64, zoom: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(zoom),
        ];
        self.disp.invoke_method("SetViewPoint", args).is_ok()
    }

    /// 设置 COM 属性 `SetActualSize`。
    pub fn set_actual_size(&self) -> bool {
        self.disp.invoke_method("SetActualSize", vec![]).is_ok()
    }

    /// 缩放在（COM 方法 `ZoomIn`）。
    pub fn zoom_in(&self) -> bool {
        self.disp.invoke_method("ZoomIn", vec![]).is_ok()
    }

    /// 缩放点处的在（COM 方法 `ZoomInAtPoint`）。
    pub fn zoom_in_at_point(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("ZoomInAtPoint", args).is_ok()
    }

    /// 缩放输出（COM 方法 `ZoomOut`）。
    pub fn zoom_out(&self) -> bool {
        self.disp.invoke_method("ZoomOut", vec![]).is_ok()
    }

    /// 取视图区域（`x, y, w, h`）。
    pub fn get_view_area(&self) -> Option<(f64, f64, f64, f64)> {
        // 4 out 参数在 IDispatch 里拿不到，用属性代替（OriginX/Y + Zoom）
        // 如果确实需要，可改走 vtable。
        None
    }

    /// 设置视图区域（`SetViewArea`）。
    pub fn set_view_area(&self, x: f64, y: f64, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetViewArea", args).is_ok()
    }

    // ---------------------------------------------------------
    // 校样颜色
    // ---------------------------------------------------------

    /// 读取 COM 属性 `SimulateOverprints`。
    pub fn simulate_overprints(&self) -> Option<bool> {
        self.prop_bool("SimulateOverprints")
    }
    /// 设置 COM 属性 `SimulateOverprints`。
    pub fn set_simulate_overprints(&self, v: bool) -> bool {
        self.put_bool("SimulateOverprints", v)
    }

    /// 读取显示校样颜色集（`ShowProofColors`）。
    pub fn show_proof_colors(&self) -> Option<bool> { self.prop_bool("ShowProofColors") }
    /// 设置显示校样颜色集（`ShowProofColors`）。
    pub fn set_show_proof_colors(&self, v: bool) -> bool {
        self.put_bool("ShowProofColors", v)
    }

    /// 读取校样颜色设置集（`ProofColorSettings`）。
    pub fn proof_color_settings(&self) -> Option<IvgProofColorSettings> {
        self.prop_dispatch("ProofColorSettings").map(IvgProofColorSettings::new)
    }

    /// 设置校样颜色设置集（`ProofColorSettings`）。
    pub fn set_proof_color_settings(&self, s: &IvgProofColorSettings) -> bool {
        self.put_dispatch("ProofColorSettings", s.as_variant())
    }
}

// =============================================================
// IvgProofColorSettings
// =============================================================

/// CorelDRAW `IVGActiveView` 对象的后期绑定包装（校样颜色设置集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgProofColorSettings {
    disp: ComObject,
}

impl IvgProofColorSettings {
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

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
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

    /// 校样用的颜色上下文（用于把文档颜色转换到打印机/ 显示器色域）。
    pub fn color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("ColorContext").map(IvgColorContext::new)
    }
    /// 设置颜色上下文（`ColorContext`）。
    pub fn set_color_context(&self, c: &IvgColorContext) -> bool {
        self.put_dispatch("ColorContext", c.as_variant())
    }

    /// 是否显示"超出色域"警告。
    pub fn show_out_of_gamut_warning(&self) -> Option<bool> {
        self.prop_bool("ShowOutOfGamutWarning")
    }
    /// 设置显示输出色域警告（`ShowOutOfGamutWarning`）。
    pub fn set_show_out_of_gamut_warning(&self, v: bool) -> bool {
        self.put_bool("ShowOutOfGamutWarning", v)
    }

    /// 读取输出色域颜色（`OutOfGamutColor`）。
    pub fn out_of_gamut_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutOfGamutColor").map(IvgColor::new)
    }
    /// 设置输出色域颜色（`OutOfGamutColor`）。
    pub fn set_out_of_gamut_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("OutOfGamutColor", c.as_variant())
    }

    /// 读取输出色域透明度（`OutOfGamutTransparency`）。
    pub fn out_of_gamut_transparency(&self) -> Option<f64> {
        self.prop_f64("OutOfGamutTransparency")
    }
    /// 设置输出色域透明度（`OutOfGamutTransparency`）。
    pub fn set_out_of_gamut_transparency(&self, v: f64) -> bool {
        self.put_f64("OutOfGamutTransparency", v)
    }

    /// 是否保留原颜色值（不做颜色转换）。
    pub fn preserve_color_values(&self) -> Option<bool> {
        self.prop_bool("PreserveColorValues")
    }
    /// 设置保留颜色值集（`PreserveColorValues`）。
    pub fn set_preserve_color_values(&self, v: bool) -> bool {
        self.put_bool("PreserveColorValues", v)
    }

    // ---------------------------------------------------------
    // 复制
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgProofColorSettings> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgProofColorSettings::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, src: &IvgProofColorSettings) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }
}