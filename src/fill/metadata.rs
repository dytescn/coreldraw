//! `IVGFillMetadata` —— 填充元数据
//! `IVGLocalizableString` —— 可本地化字符串

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgFillMetadata
// =============================================================

/// CorelDRAW `IVGFillMetadata` 对象的后期绑定包装（填充元数据）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgFillMetadata {
    disp: ComObject,
}

impl IvgFillMetadata {
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

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 读取标题（`Title`）。
    pub fn title(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Title").map(IvgLocalizableString::new)
    }

    /// 读取描述（`Description`）。
    pub fn description(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Description").map(IvgLocalizableString::new)
    }

    /// 读取关键词（`Keywords`）。
    pub fn keywords(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Keywords").map(IvgLocalizableString::new)
    }

    /// 读取主题（`Subject`）。
    pub fn subject(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Subject").map(IvgLocalizableString::new)
    }

    /// 读取版权（`Copyright`）。
    pub fn copyright(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Copyright").map(IvgLocalizableString::new)
    }

    /// 读取类别（`Category`）。
    pub fn category(&self) -> Option<IvgLocalizableString> {
        self.prop_dispatch("Category").map(IvgLocalizableString::new)
    }

    // ---- 普通字符串属性----

    /// 读取 COM 属性 `CreatorTool`。
    pub fn creator_tool(&self) -> Option<String> {
        self.prop_string("CreatorTool")
    }

    /// 读取文档标识（`DocumentID`）。
    pub fn document_id(&self) -> Option<String> {
        self.prop_string("DocumentID")
    }

    /// 读取实例标识（`InstanceID`）。
    pub fn instance_id(&self) -> Option<String> {
        self.prop_string("InstanceID")
    }

    /// 派生来源（字符串数组）。
    pub fn derived_from(&self) -> Option<Variant> {
        self.disp.get_property("DerivedFrom").ok()
    }

    // ---- 日期属性----

    /// 读取 COM 属性 `CreationDate`。
    pub fn creation_date(&self) -> Option<f64> {
        self.prop_f64("CreationDate")
    }

    /// 读取 COM 属性 `ModificationDate`。
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

/// CorelDRAW `IVGFillMetadata` 对象的后期绑定包装（可本地化字符串）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgLocalizableString {
    disp: ComObject,
}

impl IvgLocalizableString {
    /// 由 COM `IDispatch` 构造包装对象。
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

    // ---- 状态----

    /// 是否空（`IsEmpty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    /// 是否有默认值语言字符串（`HasDefaultLangString`）。
    pub fn has_default_lang_string(&self) -> Option<bool> {
        self.prop_bool("HasDefaultLangString")
    }

    /// 是否有默认值语言字符串仅（`HasDefaultLangStringOnly`）。
    pub fn has_default_lang_string_only(&self) -> Option<bool> {
        self.prop_bool("HasDefaultLangStringOnly")
    }

    /// 读取 COM 属性 `HasNonDefaultLangStrings`。
    pub fn has_non_default_lang_strings(&self) -> Option<bool> {
        self.prop_bool("HasNonDefaultLangStrings")
    }

    // ---- 默认语言字符串----

    /// 读取默认值语言字符串（`DefaultLangString`）。
    pub fn default_lang_string(&self) -> Option<String> {
        self.prop_string("DefaultLangString")
    }
    /// 设置默认值语言字符串（`DefaultLangString`）。
    pub fn set_default_lang_string(&self, v: impl Into<String>) -> bool {
        self.put_string("DefaultLangString", v)
    }

    // ---- 查询 / 设置特定语言 ----

    /// 读取语言字符串（`GetLangString`）。
    pub fn get_lang_string(&self, language: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(language.into())];
        self.disp.invoke_method("GetLangString", args).ok()?.to_string().ok()
    }

    /// 设置语言字符串（`SetLangString`）。
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

    /// 是否有语言字符串（COM 方法 `HasLangString`）。
    pub fn has_lang_string(&self, language: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(language.into())];
        self.disp
            .invoke_method("HasLangString", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 返回所有语言（字符串数组）。
    pub fn languages(&self) -> Option<Variant> {
        self.disp.invoke_method("GetLanguages", vec![]).ok()
    }

    // ---- 操作 ----

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }
}