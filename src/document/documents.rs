//! `Documents` —— 文档集合
//!
//! `IVGDocuments`（RIDL）：
//! ```text
//! get_Application  → Application
//! get_Parent       → Application
//! get_Item(Index)  → Document
//! get__NewEnum     → IEnumVARIANT
//! get_Count        → i32
//! ```

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::document::IvgDocument;

/// CorelDRAW `IVGDocuments` 对象的后期绑定包装（文档集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDocuments {
    disp: ComObject,
}

impl IvgDocuments {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 读取应用（`Application`）。
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

    /// 通过索引＂-based）或名称获取文档。
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

    /// 读取条目索引（`Item`）。
    pub fn item_by_index(&self, i: i32) -> Option<IvgDocument> {
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_i64(i as i64)])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_i64(i as i64)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgDocument::new)
    }

    /// 读取条目名称（`Item`）。
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
    // 迭代辅助：遍历所有文档
    // ---------------------------------------------------------

    /// 返回所有文档的 Vec。
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

    /// 新建空文档。
    pub fn add(&self) -> Option<IvgDocument> {
        self.disp
            .invoke_method("Add", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDocument::new)
    }

    /// 从模板新建文档。
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

    /// 打开文件。
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

    /// 关闭所有文档。
    pub fn close_all(&self) -> bool {
        self.disp.invoke_method("CloseAll", vec![]).is_ok()
    }
}