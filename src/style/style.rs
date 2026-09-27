//! `IVGStyle` 鈥斺€?鍗曚釜鏍峰紡

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::style::{
    IvgStyleCharacter, IvgStyleFill, IvgStyleFrame, IvgStyleOutline,
    IvgStyleParagraph, IvgStyleTransparency, IvgStyles,
};

pub struct IvgStyle {
    disp: ComObject,
}

impl IvgStyle {
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
    // 鏍囪瘑
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn display_name(&self) -> Option<String> { self.prop_string("DisplayName") }

    pub fn category_name(&self) -> Option<String> {
        self.prop_string("CategoryName")
    }
    pub fn display_category_name(&self) -> Option<String> {
        self.prop_string("DisplayCategoryName")
    }

    // ---------------------------------------------------------
    // 绫诲瀷鍒ゅ畾
    // ---------------------------------------------------------

    pub fn is_style_set(&self) -> Option<bool> { self.prop_bool("IsStyleSet") }
    pub fn is_object_defaults(&self) -> Option<bool> { self.prop_bool("IsObjectDefaults") }

    // ---------------------------------------------------------
    // 缁ф壙鍏崇郴
    // ---------------------------------------------------------

    pub fn based_on(&self) -> Option<IvgStyle> {
        self.prop_dispatch("BasedOn").map(IvgStyle::new)
    }

    pub fn derived_styles(&self) -> Option<IvgStyles> {
        self.prop_dispatch("DerivedStyles").map(IvgStyles::new)
    }

    pub fn set_based_on(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("SetBasedOn", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 瀛愬睘鎬у璞?
    // ---------------------------------------------------------

    pub fn outline(&self) -> Option<IvgStyleOutline> {
        self.prop_dispatch("Outline").map(IvgStyleOutline::new)
    }

    pub fn fill(&self) -> Option<IvgStyleFill> {
        self.prop_dispatch("Fill").map(IvgStyleFill::new)
    }

    pub fn character(&self) -> Option<IvgStyleCharacter> {
        self.prop_dispatch("Character").map(IvgStyleCharacter::new)
    }

    pub fn paragraph(&self) -> Option<IvgStyleParagraph> {
        self.prop_dispatch("Paragraph").map(IvgStyleParagraph::new)
    }

    pub fn frame(&self) -> Option<IvgStyleFrame> {
        self.prop_dispatch("Frame").map(IvgStyleFrame::new)
    }

    pub fn transparency(&self) -> Option<IvgStyleTransparency> {
        self.prop_dispatch("Transparency").map(IvgStyleTransparency::new)
    }

    // ---------------------------------------------------------
    // 閫氱敤灞炴€ц闂?
    // ---------------------------------------------------------

    /// 鏋氫妇鎵€鏈夊睘鎬у悕锛堣繑鍥?`SAFEARRAY`锛夈€?
    pub fn all_property_names(&self) -> Option<Variant> {
        self.disp.invoke_method("GetAllPropertyNames", vec![]).ok()
    }

    pub fn override_property_names(&self) -> Option<Variant> {
        self.disp.invoke_method("GetOverridePropertyNames", vec![]).ok()
    }

    pub fn is_property_inherited(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("IsPropertyInherited", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn get_property(&self, name: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("GetProperty", args).ok()
    }

    pub fn set_property(&self, name: impl Into<String>, value: Variant) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            value,
        ];
        self.disp.invoke_method("SetProperty", args).is_ok()
    }

    pub fn clear_property(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("ClearProperty", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn get_property_as_string(&self, name: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("GetPropertyAsString", args)
            .ok()?
            .to_string()
            .ok()
    }

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
    // 澶嶅埗 / 瀛楃涓?
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgStyle> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    pub fn assign(&self, other: &IvgStyle) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("Assign", args).is_ok()
    }

    pub fn to_style_string(&self) -> Option<String> {
        self.disp.invoke_method("ToString", vec![]).ok()?.to_string().ok()
    }

    pub fn string_assign(&self, s: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(s.into())];
        self.disp
            .invoke_method("StringAssign", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    pub fn rename(&self, new_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(new_name.into())];
        self.disp.invoke_method("Rename", args).ok()?.to_bool().ok()
    }

    pub fn delete(&self) -> Option<bool> {
        self.disp.invoke_method("Delete", vec![]).ok()?.to_bool().ok()
    }

    pub fn set_name_public(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    /// 搴旂敤缁欏浘褰紙鎶婂綋鍓嶆牱寮忚缁?`shape.style`锛夈€?
    pub fn apply_to(&self, shape: &crate::shape::IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("ApplyTo", args).is_ok()
    }

    pub fn set_transparency_public(&self, t: &IvgStyleTransparency) -> bool {
        self.put_dispatch("Transparency", t.as_variant())
    }
}