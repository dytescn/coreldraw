//! `IVGFillMetadata` 鈥斺€?濉厖鍏冩暟鎹?
//! `IVGLocalizableString` 鈥斺€?鍙湰鍦板寲瀛楃涓?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgFillMetadata
// =============================================================

pub struct IvgFillMetadata {
    disp: ComObject,
}

impl IvgFillMetadata {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn title(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Title").map(IvgLocalizableString::new)
    }

    pub fn description(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Description").map(IvgLocalizableString::new)
    }

    pub fn keywords(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Keywords").map(IvgLocalizableString::new)
    }

    pub fn subject(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Subject").map(IvgLocalizableString::new)
    }

    pub fn copyright(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Copyright").map(IvgLocalizableString::new)
    }

    pub fn category(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Category").map(IvgLocalizableString::new)
    }

    // ---- 鏅€氬瓧绗︿覆灞炴€?----

    pub fn creator_tool(&self) -> Option<String> {
        self.prop_string("CreatorTool")
    }

    pub fn document_id(&self) -> Option<String> {
        self.prop_string("DocumentID")
    }

    pub fn instance_id(&self) -> Option<String> {
        self.prop_string("InstanceID")
    }

    /// 娲剧敓鏉ユ簮锛堝瓧绗︿覆鏁扮粍锛夈€?
    pub fn derived_from(&self) -> Option<Variant> {
        self.disp.get_property("DerivedFrom").ok()
    }

    // ---- 鏃ユ湡灞炴€?----

    pub fn creation_date(&self) -> Option<f64> {
        self.prop_f64("CreationDate")
    }

    pub fn modification_date(&self) -> Option<f64> {
        self.prop_f64("ModificationDate")
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }
}

// =============================================================
// IvgLocalizableString
// =============================================================

pub struct IvgLocalizableString {
    disp: ComObject,
}

impl IvgLocalizableString {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 鐘舵€?----

    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    pub fn has_default_lang_string(&self) -> Option<bool> {
        self.prop_bool("HasDefaultLangString")
    }

    pub fn has_default_lang_string_only(&self) -> Option<bool> {
        self.prop_bool("HasDefaultLangStringOnly")
    }

    pub fn has_non_default_lang_strings(&self) -> Option<bool> {
        self.prop_bool("HasNonDefaultLangStrings")
    }

    // ---- 榛樿璇█瀛楃涓?----

    pub fn default_lang_string(&self) -> Option<String> {
        self.prop_string("DefaultLangString")
    }
    pub fn set_default_lang_string(&self, v: impl Into<String>) -> bool {
        self.put_string("DefaultLangString", v)
    }

    // ---- 鏌ヨ / 璁剧疆鐗瑰畾璇█ ----

    pub fn get_lang_string(&self, language: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(language.into())];
        self.disp.invoke_method("GetLangString", args).ok()?.to_string().ok()
    }

    pub fn set_lang_string(
        &self,
        language: impl Into<String>,
        value: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(language.into()),
            Variant::from_str(value.into()),
        ];
        self.disp.invoke_method("SetLangString", args).is_ok()
    }

    pub fn has_lang_string(&self, language: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(language.into())];
        self.disp
            .invoke_method("HasLangString", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 杩斿洖鎵€鏈夎瑷€锛堝瓧绗︿覆鏁扮粍锛夈€?
    pub fn languages(&self) -> Option<Variant> {
        self.disp.invoke_method("GetLanguages", vec![]).ok()
    }

    // ---- 鎿嶄綔 ----

    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }
}