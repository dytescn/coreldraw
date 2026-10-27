//! `IVGPalette(s)` / `IVGPaletteManager` —— 调色板

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgPalettes
// =============================================================

/// CorelDRAW `IVGPaletteManager` 对象的后期绑定包装（调色板集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPalettes {
    disp: ComObject,
}

impl IvgPalettes {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgPalette> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    /// 打开（COM 方法 `Open`）。
    pub fn open(&self, file_name: impl Into<String>) -> Option<IvgPalette> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp
            .invoke_method("Open", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    /// 打开固定（COM 方法 `OpenFixed`）。
    pub fn open_fixed(&self, palette_id: i32) -> Option<IvgPalette> {
        let args = vec![Variant::from_i64(palette_id as i64)];
        self.disp
            .invoke_method("OpenFixed", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }
}

// =============================================================
// IvgPalette
// =============================================================

/// CorelDRAW `IVGPaletteManager` 对象的后期绑定包装（调色板）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPalette {
    disp: ComObject,
}

impl IvgPalette {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property("Name", vec![arg]).is_ok()
    }

    /// 读取调色板标识（`PaletteID`）。
    pub fn palette_id(&self) -> Option<i64> { self.prop_i64("PaletteID") }
    /// 读取文件名称（`FileName`）。
    pub fn file_name(&self) -> Option<String> { self.prop_string("FileName") }
    /// 读取 COM 属性 `Identifier`。
    pub fn identifier(&self) -> Option<String> { self.prop_string("Identifier") }
    /// 是否空（`IsEmpty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }
    /// 读取锁定状态（`Locked`）。
    pub fn locked(&self) -> Option<bool> { self.prop_bool("Locked") }
    /// 是否默认值（`Default`）。
    pub fn is_default(&self) -> Option<bool> { self.prop_bool("Default") }
    /// 是否打开（`IsOpen`）。
    pub fn is_open(&self) -> Option<bool> { self.prop_bool("IsOpen") }

    /// 关闭（COM 方法 `Close`）。
    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    /// 打开（COM 方法 `Open`）。
    pub fn open(&self) -> bool {
        self.disp.invoke_method("Open", vec![]).is_ok()
    }

    /// 保存（COM 方法 `Save`）。
    pub fn save(&self) -> bool {
        self.disp.invoke_method("Save", vec![]).is_ok()
    }

    /// 生成默认值（COM 方法 `MakeDefault`）。
    pub fn make_default(&self) -> bool {
        self.disp.invoke_method("MakeDefault", vec![]).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> Option<bool> {
        self.disp.invoke_method("Delete", vec![]).ok()?.to_bool().ok()
    }

    /// `Index` 一 1-based。
    pub fn color(&self, index: i32) -> Option<IvgColor> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Color", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    /// 设置颜色（`put_Color`）。
    pub fn set_color(&self, index: i32, color: &IvgColor) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            color.as_variant(),
        ];
        self.disp.invoke_method("put_Color", args).is_ok()
    }

    /// 添加颜色（COM 方法 `AddColor`）。
    pub fn add_color(&self, color: &IvgColor) -> bool {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("AddColor", args).is_ok()
    }

    /// 插入颜色（COM 方法 `InsertColor`）。
    pub fn insert_color(&self, index: i32, color: &IvgColor) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            color.as_variant(),
        ];
        self.disp.invoke_method("InsertColor", args).is_ok()
    }

    /// 移除颜色（COM 方法 `RemoveColor`）。
    pub fn remove_color(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("RemoveColor", args).is_ok()
    }

    /// 读取颜色数量（`ColorCount`）。
    pub fn color_count(&self) -> Option<i64> {
        self.disp.get_property("ColorCount").ok()?.to_i64().ok()
    }

    /// 调用 COM 方法 `MatchColor`。
    pub fn match_color(&self, color: &IvgColor) -> Option<i64> {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("MatchColor", args).ok()?.to_i64().ok()
    }

    /// 查找颜色（COM 方法 `FindColor`）。
    pub fn find_color(&self, name: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("FindColor", args).ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgPaletteManager
// =============================================================

/// CorelDRAW `IVGPaletteManager` 对象的后期绑定包装（调色板管理器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPaletteManager {
    disp: ComObject,
}

impl IvgPaletteManager {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取调色板数量（`PaletteCount`）。
    pub fn palette_count(&self) -> Option<i64> {
        self.disp.get_property("PaletteCount").ok()?.to_i64().ok()
    }

    /// 读取调色板（`GetPalette`）。
    pub fn get_palette(&self, index_or_name: Variant) -> Option<IvgPalette> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("GetPalette", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    /// 读取默认值调色板（`DefaultPalette`）。
    pub fn default_palette(&self) -> Option<IvgPalette> {
        self.disp
            .get_property("DefaultPalette")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }

    /// 读取打开调色板集（`OpenPalettes`）。
    pub fn open_palettes(&self) -> Option<IvgPalettes> {
        self.disp
            .get_property("OpenPalettes")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalettes::new)
    }

    /// 加载全部调色板集（COM 方法 `LoadAllPalettes`）。
    pub fn load_all_palettes(&self) -> bool {
        self.disp.invoke_method("LoadAllPalettes", vec![]).is_ok()
    }

    /// 加载调色板（COM 方法 `LoadPalette`）。
    pub fn load_palette(&self, file_name: impl Into<String>) -> Option<IvgPalette> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp
            .invoke_method("LoadPalette", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPalette::new)
    }
}