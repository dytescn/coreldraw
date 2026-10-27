//! `IVGSymbolLibrary` / `IVGSymbolLibraries` —— 符号库

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::symbol::{IvgSymbolDefinition, IvgSymbolDefinitions};

// =============================================================
// IvgSymbolLibraries
// =============================================================

/// CorelDRAW `IVGSymbolLibrary` 对象的后期绑定包装（符号库集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSymbolLibraries {
    disp: ComObject,
}

impl IvgSymbolLibraries {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgSymbolLibrary> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolLibrary::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgSymbolLibrary> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgSymbolLibrary> {
        self.item(Variant::from_str(name.into()))
    }

    /// 添加库文件。
    pub fn add(
        &self,
        file_name: impl Into<String>,
        copy_locally: bool,
    ) -> Option<IvgSymbolLibrary> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_bool(copy_locally),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolLibrary::new)
    }

    /// 从文件夹批量添加。
    pub fn add_from_folder(
        &self,
        folder: impl Into<String>,
        recursive: bool,
        copy_locally: bool,
    ) -> Option<i64> {
        let args = vec![
            Variant::from_str(folder.into()),
            Variant::from_bool(recursive),
            Variant::from_bool(copy_locally),
        ];
        self.disp
            .invoke_method("AddFromFolder", args)
            .ok()?
            .to_i64()
            .ok()
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgSymbolLibrary> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}

// =============================================================
// IvgSymbolLibrary
// =============================================================

/// CorelDRAW `IVGSymbolLibrary` 对象的后期绑定包装（符号库）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSymbolLibrary {
    disp: ComObject,
}

impl IvgSymbolLibrary {
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

    // ---- 基本信息 ----

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 读取文件路径（`FilePath`）。
    pub fn file_path(&self) -> Option<String> { self.prop_string("FilePath") }
    /// 读取读取仅（`ReadOnly`）。
    pub fn read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }

    /// 库内所有符号定义。
    pub fn symbols(&self) -> Option<IvgSymbolDefinitions> {
        self.prop_dispatch("Symbols").map(IvgSymbolDefinitions::new)
    }

    // ---- 操作 ----

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 清掉未被引用的符号。
    pub fn purge_unused_symbols(&self) -> bool {
        self.disp.invoke_method("PurgeUnusedSymbols", vec![]).is_ok()
    }

    /// 把符号粘贴到本库。
    pub fn paste(&self, name: impl Into<String>) -> Option<IvgSymbolDefinition> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("Paste", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSymbolDefinition::new)
    }
}