//! `Metadata` —— 文档元数据（作者、标题、关键词等）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW 对象的后期绑定包装（元数据）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgMetadata {
    disp: ComObject,
}

impl IvgMetadata {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 基本字段
    // ---------------------------------------------------------

    /// 读取标题（`Title`）。
    pub fn title(&self) -> Option<String> { self.prop_string("Title") }
    /// 设置标题（`Title`）。
    pub fn set_title(&self, v: impl Into<String>) -> bool { self.put_string("Title", v) }

    /// 读取主题（`Subject`）。
    pub fn subject(&self) -> Option<String> { self.prop_string("Subject") }
    /// 设置主题（`Subject`）。
    pub fn set_subject(&self, v: impl Into<String>) -> bool { self.put_string("Subject", v) }

    /// 读取作者（`Author`）。
    pub fn author(&self) -> Option<String> { self.prop_string("Author") }
    /// 设置作者（`Author`）。
    pub fn set_author(&self, v: impl Into<String>) -> bool { self.put_string("Author", v) }

    /// 读取最后一个作者（`LastAuthor`）。
    pub fn last_author(&self) -> Option<String> { self.prop_string("LastAuthor") }
    /// 设置最后一个作者（`LastAuthor`）。
    pub fn set_last_author(&self, v: impl Into<String>) -> bool { self.put_string("LastAuthor", v) }

    /// 读取关键词（`Keywords`）。
    pub fn keywords(&self) -> Option<String> { self.prop_string("Keywords") }
    /// 设置关键词（`Keywords`）。
    pub fn set_keywords(&self, v: impl Into<String>) -> bool { self.put_string("Keywords", v) }

    /// 读取备注（`Notes`）。
    pub fn notes(&self) -> Option<String> { self.prop_string("Notes") }
    /// 设置备注（`Notes`）。
    pub fn set_notes(&self, v: impl Into<String>) -> bool { self.put_string("Notes", v) }

    /// 读取版权（`Copyright`）。
    pub fn copyright(&self) -> Option<String> { self.prop_string("Copyright") }
    /// 设置版权（`Copyright`）。
    pub fn set_copyright(&self, v: impl Into<String>) -> bool { self.put_string("Copyright", v) }

    /// 读取修订（`Revision`）。
    pub fn revision(&self) -> Option<i64> { self.prop_i64("Revision") }
    /// 设置修订（`Revision`）。
    pub fn set_revision(&self, v: i32) -> bool { self.put_i64("Revision", v as i64) }

    /// 读取文档标识（`DocID`）。
    pub fn doc_id(&self) -> Option<String> { self.prop_string("DocID") }
    /// 设置文档标识（`DocID`）。
    pub fn set_doc_id(&self, v: impl Into<String>) -> bool { self.put_string("DocID", v) }

    /// 读取文档语言（`DocLanguage`）。
    pub fn doc_language(&self) -> Option<i64> { self.prop_i64("DocLanguage") }
    /// 设置文档语言（`DocLanguage`）。
    pub fn set_doc_language(&self, v: i32) -> bool { self.put_i64("DocLanguage", v as i64) }

    // ---------------------------------------------------------
    // 模板相关
    // ---------------------------------------------------------

    /// 读取 COM 属性 `TemplateSided`。
    pub fn template_sided(&self) -> Option<String> { self.prop_string("TemplateSided") }
    /// 设置 COM 属性 `TemplateSided`。
    pub fn set_template_sided(&self, v: impl Into<String>) -> bool { self.put_string("TemplateSided", v) }

    /// 读取 COM 属性 `TemplateFolds`。
    pub fn template_folds(&self) -> Option<String> { self.prop_string("TemplateFolds") }
    /// 设置 COM 属性 `TemplateFolds`。
    pub fn set_template_folds(&self, v: impl Into<String>) -> bool { self.put_string("TemplateFolds", v) }

    /// 读取模板类型（`TemplateType`）。
    pub fn template_type(&self) -> Option<String> { self.prop_string("TemplateType") }
    /// 设置模板类型（`TemplateType`）。
    pub fn set_template_type(&self, v: impl Into<String>) -> bool { self.put_string("TemplateType", v) }

    /// 读取 COM 属性 `TemplateIndustry`。
    pub fn template_industry(&self) -> Option<String> { self.prop_string("TemplateIndustry") }
    /// 设置 COM 属性 `TemplateIndustry`。
    pub fn set_template_industry(&self, v: impl Into<String>) -> bool { self.put_string("TemplateIndustry", v) }

    /// 读取模板设计器备注（`TemplateDesignerNotes`）。
    pub fn template_designer_notes(&self) -> Option<String> {
        self.prop_string("TemplateDesignerNotes")
    }
    /// 设置模板设计器备注（`TemplateDesignerNotes`）。
    pub fn set_template_designer_notes(&self, v: impl Into<String>) -> bool {
        self.put_string("TemplateDesignerNotes", v)
    }

    // ---------------------------------------------------------
    // 本地化字符串（可多语言）
    // ---------------------------------------------------------

    /// 读取可本地化关键词（`LocalizableKeywords`）。
    pub fn localizable_keywords(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableKeywords")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    /// 读取可本地化备注（`LocalizableNotes`）。
    pub fn localizable_notes(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableNotes")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    /// 读取可本地化标题（`LocalizableTitle`）。
    pub fn localizable_title(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableTitle")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    /// 读取可本地化主题（`LocalizableSubject`）。
    pub fn localizable_subject(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableSubject")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    /// 读取可本地化版权（`LocalizableCopyright`）。
    pub fn localizable_copyright(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableCopyright")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    /// 读取可本地化模板设计器备注（`LocalizableTemplateDesignerNotes`）。
    pub fn localizable_template_designer_notes(
        &self,
    ) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableTemplateDesignerNotes")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    /// 读取类别（`Category`）。
    pub fn category(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("Category")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }
}