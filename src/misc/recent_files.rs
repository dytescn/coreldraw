//! `IVGRecentFile(s)` —— 最近打开的文件

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgRecentFiles
// =============================================================

/// CorelDRAW 对象的后期绑定包装（最近文件集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgRecentFiles {
    disp: ComObject,
}

impl IvgRecentFiles {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    /// 读取最大（`Maximum`）。
    pub fn maximum(&self) -> Option<i64> { self.prop_i64("Maximum") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgRecentFile> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRecentFile::new)
    }

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, name: impl Into<String>, path: impl Into<String>) -> Option<IvgRecentFile> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_str(path.into()),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRecentFile::new)
    }
}

// =============================================================
// IvgRecentFile
// =============================================================

/// CorelDRAW 对象的后期绑定包装（最近文件）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgRecentFile {
    disp: ComObject,
}

impl IvgRecentFile {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取路径（`Path`）。
    pub fn path(&self) -> Option<String> { self.prop_string("Path") }
    /// 设置路径（`Path`）。
    pub fn set_path(&self, v: impl Into<String>) -> bool { self.put_string("Path", v) }

    /// 读取完整名称（`FullName`）。
    pub fn full_name(&self) -> Option<String> { self.prop_string("FullName") }
    /// 设置完整名称（`FullName`）。
    pub fn set_full_name(&self, v: impl Into<String>) -> bool {
        self.put_string("FullName", v)
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 打开（COM 方法 `Open`）。
    pub fn open(&self) -> Option<crate::document::IvgDocument> {
        self.disp
            .invoke_method("Open", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::document::IvgDocument::new)
    }
}