//! `Metadata` 鈥斺€?鏂囨。鍏冩暟鎹紙浣滆€呫€佹爣棰樸€佸叧閿瘝绛夛級

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgMetadata {
    disp: ComObject,
}

impl IvgMetadata {
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
    // 鍩烘湰瀛楁
    // ---------------------------------------------------------

    pub fn title(&self) -> Option<String> { self.prop_string("Title") }
    pub fn set_title(&self, v: impl Into<String>) -> bool { self.put_string("Title", v) }

    pub fn subject(&self) -> Option<String> { self.prop_string("Subject") }
    pub fn set_subject(&self, v: impl Into<String>) -> bool { self.put_string("Subject", v) }

    pub fn author(&self) -> Option<String> { self.prop_string("Author") }
    pub fn set_author(&self, v: impl Into<String>) -> bool { self.put_string("Author", v) }

    pub fn last_author(&self) -> Option<String> { self.prop_string("LastAuthor") }
    pub fn set_last_author(&self, v: impl Into<String>) -> bool { self.put_string("LastAuthor", v) }

    pub fn keywords(&self) -> Option<String> { self.prop_string("Keywords") }
    pub fn set_keywords(&self, v: impl Into<String>) -> bool { self.put_string("Keywords", v) }

    pub fn notes(&self) -> Option<String> { self.prop_string("Notes") }
    pub fn set_notes(&self, v: impl Into<String>) -> bool { self.put_string("Notes", v) }

    pub fn copyright(&self) -> Option<String> { self.prop_string("Copyright") }
    pub fn set_copyright(&self, v: impl Into<String>) -> bool { self.put_string("Copyright", v) }

    pub fn revision(&self) -> Option<i64> { self.prop_i64("Revision") }
    pub fn set_revision(&self, v: i32) -> bool { self.put_i64("Revision", v as i64) }

    pub fn doc_id(&self) -> Option<String> { self.prop_string("DocID") }
    pub fn set_doc_id(&self, v: impl Into<String>) -> bool { self.put_string("DocID", v) }

    pub fn doc_language(&self) -> Option<i64> { self.prop_i64("DocLanguage") }
    pub fn set_doc_language(&self, v: i32) -> bool { self.put_i64("DocLanguage", v as i64) }

    // ---------------------------------------------------------
    // 妯℃澘鐩稿叧
    // ---------------------------------------------------------

    pub fn template_sided(&self) -> Option<String> { self.prop_string("TemplateSided") }
    pub fn set_template_sided(&self, v: impl Into<String>) -> bool { self.put_string("TemplateSided", v) }

    pub fn template_folds(&self) -> Option<String> { self.prop_string("TemplateFolds") }
    pub fn set_template_folds(&self, v: impl Into<String>) -> bool { self.put_string("TemplateFolds", v) }

    pub fn template_type(&self) -> Option<String> { self.prop_string("TemplateType") }
    pub fn set_template_type(&self, v: impl Into<String>) -> bool { self.put_string("TemplateType", v) }

    pub fn template_industry(&self) -> Option<String> { self.prop_string("TemplateIndustry") }
    pub fn set_template_industry(&self, v: impl Into<String>) -> bool { self.put_string("TemplateIndustry", v) }

    pub fn template_designer_notes(&self) -> Option<String> {
        self.prop_string("TemplateDesignerNotes")
    }
    pub fn set_template_designer_notes(&self, v: impl Into<String>) -> bool {
        self.put_string("TemplateDesignerNotes", v)
    }

    // ---------------------------------------------------------
    // 鏈湴鍖栧瓧绗︿覆锛堝彲澶氳瑷€锛?
    // ---------------------------------------------------------

    pub fn localizable_keywords(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableKeywords")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    pub fn localizable_notes(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableNotes")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    pub fn localizable_title(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableTitle")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    pub fn localizable_subject(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableSubject")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

    pub fn localizable_copyright(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("LocalizableCopyright")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }

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

    pub fn category(&self) -> Option<crate::fill::IvgLocalizableString> {
        self.disp
            .get_property("Category")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::fill::IvgLocalizableString::new)
    }
}