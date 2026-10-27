//! `IVGColorContext` —— 颜色上下文（RGB / CMYK / 灰度 profile + 渲染意图）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::manager::{IvgColorProfile, IvgColorProfiles};

/// CorelDRAW `IVGColorContext` 对象的后期绑定包装（颜色上下文）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgColorContext {
    disp: ComObject,
}

impl IvgColorContext {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
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

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, disp: &IDispatch) -> bool {
        let arg = Variant::from_dispatch(disp);

        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 各模式的 Profile
    // ---------------------------------------------------------

    /// 读取 RGB 颜色配置文件（`RGBColorProfile`）。
    pub fn rgb_color_profile(&self) -> Option<IvgColorProfile> {
        self.prop_dispatch("RGBColorProfile").map(IvgColorProfile::new)
    }

    /// 设置 RGB 颜色配置文件（`RGBColorProfile`）。
    pub fn set_rgb_color_profile(&self, p: &IvgColorProfile) -> bool {
        self.put_dispatch("RGBColorProfile", p.dispatch())
    }

    /// 读取 CMYK 颜色配置文件（`CMYKColorProfile`）。
    pub fn cmyk_color_profile(&self) -> Option<IvgColorProfile> {
        self.prop_dispatch("CMYKColorProfile").map(IvgColorProfile::new)
    }

    /// 设置 CMYK 颜色配置文件（`CMYKColorProfile`）。
    pub fn set_cmyk_color_profile(&self, p: &IvgColorProfile) -> bool {
        self.put_dispatch("CMYKColorProfile", p.dispatch())
    }

    /// 读取灰度颜色配置文件（`GrayscaleColorProfile`）。
    pub fn grayscale_color_profile(&self) -> Option<IvgColorProfile> {
        self.prop_dispatch("GrayscaleColorProfile").map(IvgColorProfile::new)
    }

    /// 设置灰度颜色配置文件（`GrayscaleColorProfile`）。
    pub fn set_grayscale_color_profile(&self, p: &IvgColorProfile) -> bool {
        self.put_dispatch("GrayscaleColorProfile", p.dispatch())
    }

    /// `clrColorModel` 枚举值
    pub fn color_profile(&self, model: i32) -> Option<IvgColorProfile> {
        let args = vec![Variant::from_i64(model as i64)];
        self.disp
            .invoke_method("ColorProfile", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorProfile::new)
    }

    // ---------------------------------------------------------
    // 渲染意图 / 混合模式
    // ---------------------------------------------------------

    /// `clrRenderingIntent`
    pub fn rendering_intent(&self) -> Option<i64> {
        self.prop_i64("RenderingIntent")
    }
    /// 设置渲染意图（`RenderingIntent`）。
    pub fn set_rendering_intent(&self, v: i32) -> bool {
        self.put_i64("RenderingIntent", v as i64)
    }

    /// `clrColorModel`
    pub fn blending_color_model(&self) -> Option<i64> {
        self.prop_i64("BlendingColorModel")
    }
    /// 设置 COM 属性 `BlendingColorModel`。
    pub fn set_blending_color_model(&self, v: i32) -> bool {
        self.put_i64("BlendingColorModel", v as i64)
    }

    // ---------------------------------------------------------
    // 复制 / 比较
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgColorContext> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColorContext::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgColorContext) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 是否相同（COM 方法 `IsSame`）。
    pub fn is_same(&self, other: &IvgColorContext) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsSame", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 合并（COM 方法 `Merge`）。
    pub fn merge(&self, other: &IvgColorContext) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Merge", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 其它
    // ---------------------------------------------------------

    /// 读取颜色配置文件集（`ColorProfiles`）。
    pub fn color_profiles(&self) -> Option<IvgColorProfiles> {
        self.prop_dispatch("ColorProfiles").map(IvgColorProfiles::new)
    }

    /// 读取读取仅（`ReadOnly`）。
    pub fn read_only(&self) -> Option<bool> {
        self.prop_bool("ReadOnly")
    }

    /// 读取颜色配置文件名称列表（`ColorProfileNameList`）。
    pub fn color_profile_name_list(&self) -> Option<String> {
        self.prop_string("ColorProfileNameList")
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 分发。
    pub fn dispatch(&self) -> &IDispatch {
        self.disp.as_dispatch()
    }
}