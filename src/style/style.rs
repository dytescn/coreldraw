//! `IVGStyle` —— 单个样式

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::style::{
    IvgStyleCharacter, IvgStyleFill, IvgStyleFrame, IvgStyleOutline,
    IvgStyleParagraph, IvgStyleTransparency, IvgStyles,
};

/// CorelDRAW `IVGStyle` 对象的后期绑定包装（样式）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyle {
    disp: ComObject,
}

impl IvgStyle {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 标识
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取显示名称（`DisplayName`）。
    pub fn display_name(&self) -> Option<String> { self.prop_string("DisplayName") }

    /// 读取类别名称（`CategoryName`）。
    pub fn category_name(&self) -> Option<String> {
        self.prop_string("CategoryName")
    }
    /// 读取显示类别名称（`DisplayCategoryName`）。
    pub fn display_category_name(&self) -> Option<String> {
        self.prop_string("DisplayCategoryName")
    }

    // ---------------------------------------------------------
    // 类型判定
    // ---------------------------------------------------------

    /// 是否样式设置（`IsStyleSet`）。
    pub fn is_style_set(&self) -> Option<bool> { self.prop_bool("IsStyleSet") }
    /// 读取 COM 属性 `IsObjectDefaults`。
    pub fn is_object_defaults(&self) -> Option<bool> { self.prop_bool("IsObjectDefaults") }

    // ---------------------------------------------------------
    // 继承关系
    // ---------------------------------------------------------

    /// 读取 COM 属性 `BasedOn`。
    pub fn based_on(&self) -> Option<IvgStyle> {
        self.prop_dispatch("BasedOn").map(IvgStyle::new)
    }

    /// 读取 COM 属性 `DerivedStyles`。
    pub fn derived_styles(&self) -> Option<IvgStyles> {
        self.prop_dispatch("DerivedStyles").map(IvgStyles::new)
    }

    /// 设置 COM 属性 `SetBasedOn`。
    pub fn set_based_on(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("SetBasedOn", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 子属性对象
    // ---------------------------------------------------------

    /// 读取轮廓（`Outline`）。
    pub fn outline(&self) -> Option<IvgStyleOutline> {
        self.prop_dispatch("Outline").map(IvgStyleOutline::new)
    }

    /// 读取填充（`Fill`）。
    pub fn fill(&self) -> Option<IvgStyleFill> {
        self.prop_dispatch("Fill").map(IvgStyleFill::new)
    }

    /// 读取字符（`Character`）。
    pub fn character(&self) -> Option<IvgStyleCharacter> {
        self.prop_dispatch("Character").map(IvgStyleCharacter::new)
    }

    /// 读取段落（`Paragraph`）。
    pub fn paragraph(&self) -> Option<IvgStyleParagraph> {
        self.prop_dispatch("Paragraph").map(IvgStyleParagraph::new)
    }

    /// 读取文本框（`Frame`）。
    pub fn frame(&self) -> Option<IvgStyleFrame> {
        self.prop_dispatch("Frame").map(IvgStyleFrame::new)
    }

    /// 读取透明度（`Transparency`）。
    pub fn transparency(&self) -> Option<IvgStyleTransparency> {
        self.prop_dispatch("Transparency").map(IvgStyleTransparency::new)
    }

    // ---------------------------------------------------------
    // 通用属性访问
    // ---------------------------------------------------------

    /// 枚举所有属性名（返回 `SAFEARRAY`）。
    pub fn all_property_names(&self) -> Option<Variant> {
        self.disp.invoke_method("GetAllPropertyNames", vec![]).ok()
    }

    /// 调用 COM 方法 `GetOverridePropertyNames`。
    pub fn override_property_names(&self) -> Option<Variant> {
        self.disp.invoke_method("GetOverridePropertyNames", vec![]).ok()
    }

    /// 调用 COM 方法 `IsPropertyInherited`。
    pub fn is_property_inherited(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("IsPropertyInherited", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 读取属性（`GetProperty`）。
    pub fn get_property(&self, name: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("GetProperty", args).ok()
    }

    /// 设置属性（`SetProperty`）。
    pub fn set_property(&self, name: impl Into<String>, value: Variant) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            value,
        ];
        self.disp.invoke_method("SetProperty", args).is_ok()
    }

    /// 清除属性（COM 方法 `ClearProperty`）。
    pub fn clear_property(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("ClearProperty", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 读取属性 as 字符串（`GetPropertyAsString`）。
    pub fn get_property_as_string(&self, name: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("GetPropertyAsString", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 设置属性 as 字符串（`SetPropertyAsString`）。
    pub fn set_property_as_string(
        &self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_str(value.into()),
        ];
        self.disp
            .invoke_method("SetPropertyAsString", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 复制 / 字符串
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgStyle> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    /// assign（COM 方法 `Assign`）。
    pub fn assign(&self, other: &IvgStyle) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Assign", args).is_ok()
    }

    /// 转换为样式字符串（COM 方法 `ToString`）。
    pub fn to_style_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    /// 字符串 assign（COM 方法 `StringAssign`）。
    pub fn string_assign(&self, s: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// 调用 COM 方法 `Rename`。
    pub fn rename(&self, new_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(new_name.into())];
        self.disp.invoke_method("Rename", args).ok()?.to_bool().ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> Option<bool> {
        self.disp.invoke_method("Delete", vec![]).ok()?.to_bool().ok()
    }

    /// 设置 COM 属性 `Name`。
    pub fn set_name_public(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    /// 应用给图形（把当前样式设经 `shape.style`）。
    pub fn apply_to(&self, shape: &crate::shape::IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("ApplyTo", args).is_ok()
    }

    /// 设置 COM 属性 `Transparency`。
    pub fn set_transparency_public(&self, t: &IvgStyleTransparency) -> bool {
        self.put_dispatch("Transparency", t.as_variant())
    }
}