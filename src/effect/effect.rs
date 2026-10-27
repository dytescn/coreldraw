//! `IVGEffect` —— 效果对象（通用入口）
//!
//! 一个 `IVGShape` 可以挂多个效果（混合、轮廓、阴影……）。
//! 通过 `shape.effect()` 或 `shape.effects().item(n)` 得到。
//!
//! `Type` 属性区分具体类型（`cdrEffectType`，见 `enums/effect`），
//! 然后用对应 getter 拿到强类型子对象：
//! - `.blend()`       → `IvgEffectBlend`
//! - `.contour()`     → `IvgEffectContour`
//! - `.extrude()`     → `IvgEffectExtrude`
//! - `.envelope()`    → `IvgEffectEnvelope`
//! - `.drop_shadow()` → `IvgEffectDropShadow`
//! - `.lens()`        → `IvgEffectLens`
//! - `.perspective()` → `IvgEffectPerspective`
//! - `.distortion()`  → `IvgEffectDistortion`
//! - `.text_on_path()`→ `IvgEffectTextOnPath`
//! - `.custom()`      → `IvgCustomEffect`

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::effect::{
    IvgEffectBlend, IvgEffectContour, IvgEffectControlPath, IvgEffectDistortion,
    IvgEffectDropShadow, IvgEffectEnvelope, IvgEffectExtrude, IvgEffectLens,
    IvgEffectPerspective, IvgEffectTextOnPath, IvgEffects,
};
use crate::shape::IvgShapeRange;

// =============================================================
// IvgEffect
// =============================================================

/// CorelDRAW `IVGEffect` 对象的后期绑定包装（效果）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffect {
    disp: ComObject,
}

impl IvgEffect {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 工具
    // ---------------------------------------------------------

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// `cdrEffectType` ——见 `enums/effect`
    pub fn effect_type(&self) -> Option<i64> {
        self.prop_i64("Type")
    }

    // ---------------------------------------------------------
    // 子效果（按类型取强类型）
    // ---------------------------------------------------------

    /// 读取调和（`Blend`）。
    pub fn blend(&self) -> Option<IvgEffectBlend> {
        self.prop_dispatch("Blend").map(IvgEffectBlend::new)
    }

    /// 读取控件路径（`ControlPath`）。
    pub fn control_path(&self) -> Option<IvgEffectControlPath> {
        self.prop_dispatch("ControlPath").map(IvgEffectControlPath::new)
    }

    /// 读取立体化（`Extrude`）。
    pub fn extrude(&self) -> Option<IvgEffectExtrude> {
        self.prop_dispatch("Extrude").map(IvgEffectExtrude::new)
    }

    /// 读取封套（`Envelope`）。
    pub fn envelope(&self) -> Option<IvgEffectEnvelope> {
        self.prop_dispatch("Envelope").map(IvgEffectEnvelope::new)
    }

    /// 读取路径上的文本（`TextOnPath`）。
    pub fn text_on_path(&self) -> Option<IvgEffectTextOnPath> {
        self.prop_dispatch("TextOnPath").map(IvgEffectTextOnPath::new)
    }

    /// 读取投影阴影（`DropShadow`）。
    pub fn drop_shadow(&self) -> Option<IvgEffectDropShadow> {
        self.prop_dispatch("DropShadow").map(IvgEffectDropShadow::new)
    }

    /// 读取轮廓（`Contour`）。
    pub fn contour(&self) -> Option<IvgEffectContour> {
        self.prop_dispatch("Contour").map(IvgEffectContour::new)
    }

    /// 读取变形（`Distortion`）。
    pub fn distortion(&self) -> Option<IvgEffectDistortion> {
        self.prop_dispatch("Distortion").map(IvgEffectDistortion::new)
    }

    /// 读取透镜（`Lens`）。
    pub fn lens(&self) -> Option<IvgEffectLens> {
        self.prop_dispatch("Lens").map(IvgEffectLens::new)
    }

    /// 读取透视（`Perspective`）。
    pub fn perspective(&self) -> Option<IvgEffectPerspective> {
        self.prop_dispatch("Perspective").map(IvgEffectPerspective::new)
    }

    /// 读取自定义（`Custom`）。
    pub fn custom(&self) -> Option<IvgCustomEffect> {
        self.prop_dispatch("Custom").map(IvgCustomEffect::new)
    }

    // ---------------------------------------------------------
    // 克隆关系
    // ---------------------------------------------------------

    /// 通过克隆连接到本效果的其它效果集合。
    pub fn clones(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Clones").map(IvgEffects::new)
    }

    /// 本效果克隆自哪个效果。
    pub fn clone_parent(&self) -> Option<IvgEffect> {
        self.prop_dispatch("CloneParent").map(IvgEffect::new)
    }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// 移除效果。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 分离效果（把效果拆成独立图形）。
    pub fn separate(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("Separate", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}

// =============================================================
// IvgCustomEffect
// =============================================================

/// CorelDRAW `IVGEffect` 对象的后期绑定包装（自定义效果）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgCustomEffect {
    disp: ComObject,
}

impl IvgCustomEffect {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 效果 ID（GUID 字符串）。
    pub fn effect_id(&self) -> Option<String> {
        self.prop_string("EffectID")
    }

    /// 效果产生的图形分组。
    pub fn effect_group(&self) -> Option<crate::shape::IvgShape> {
        self.prop_dispatch("EffectGroup").map(crate::shape::IvgShape::new)
    }
}