//! `Documents` 鈥斺€?鏂囨。闆嗗悎
//!
//! `IVGDocuments`（RIDL）：
//! ```text
//! get_Application  鈫?Application
//! get_Parent       鈫?Application
//! get_Item(Index)  鈫?Document
//! get__NewEnum     鈫?IEnumVARIANT
//! get_Count        鈫?i32
//! ```

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::document::IvgDocument;

pub struct IvgDocuments {
    disp: ComObject,
}

impl IvgDocuments {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn application(&self) -> Option<crate::app::IvgApplication> {
        self.disp
            .get_property("Application")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::app::IvgApplication::from_disp)
    }

    // ---------------------------------------------------------
    // 索引访问
    // ---------------------------------------------------------

    /// 閫氳繃绱㈠紩锛?-based锛夋垨鍚嶇О鑾峰彇鏂囨。銆?
    pub fn item(&self, index_or_name: Variant) -> Option<IvgDocument> {
        // `Item` 是带参数的属性，必须用 DISPATCH_PROPERTYGET。
        let args = vec![index_or_name];
        self.disp
            .get_property_with("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDocument::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgDocument> {
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_i64(i as i64)])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_i64(i as i64)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgDocument::new)
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgDocument> {
        let name = name.into();
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_str(name.clone())])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_str(name)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgDocument::new)
    }

    // ---------------------------------------------------------
    // 杩唬杈呭姪锛氶亶鍘嗘墍鏈夋枃妗?
    // ---------------------------------------------------------

    /// 杩斿洖鎵€鏈夋枃妗ｇ殑 Vec銆?
    pub fn all(&self) -> Vec<IvgDocument> {
        let n = self.count().unwrap_or(0) as i32;
        let mut out = Vec::with_capacity(n.max(0) as usize);
        for i in 1..=n {
            if let Some(doc) = self.item_by_index(i) {
                out.push(doc);
            }
        }
        out
    }

    // ---------------------------------------------------------
    // 集合操作
    // ---------------------------------------------------------

    /// 鏂板缓绌烘枃妗ｃ€?
    pub fn add(&self) -> Option<IvgDocument> {
        self.disp
            .invoke_method("Add", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDocument::new)
    }

    /// 浠庢ā鏉挎柊寤烘枃妗ｃ€?
    pub fn add_from_template(
        &self,
        template: impl Into<String>,
        include_graphics: bool,
    ) -> Option<IvgDocument> {
        let args = vec![
            Variant::from_str(template.into()),
            Variant::from_bool(include_graphics),
        ];
        self.disp
            .invoke_method("AddFromTemplate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDocument::new)
    }

    /// 鎵撳紑鏂囦欢銆?
    /// 打开文档。
    ///
    /// `Documents` 集合没有 `Open` 成员；正确入口是
    /// `Application.OpenDocument(FileName)`，随后取活动文档。
    pub fn open(&self, file_name: impl Into<String>) -> Option<IvgDocument> {
        let app = self.application()?;
        if !app.open_document(file_name) {
            return None;
        }
        app.active_document()
    }

    /// 鍏抽棴鎵€鏈夋枃妗ｃ€?
    pub fn close_all(&self) -> bool {
        self.disp.invoke_method("CloseAll", vec![]).is_ok()
    }
}