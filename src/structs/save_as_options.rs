//! `IVGStructSaveAsOptions` —— 另存为选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStructSaveAsOptions` 对象的后期绑定包装（保存为选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructSaveAsOptions {
    disp: ComObject,
}

impl IvgStructSaveAsOptions {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// `cdrFilter` ——文件类型。
    pub fn filter(&self) -> Option<i64> { self.prop_i64("Filter") }
    /// 设置过滤器（`Filter`）。
    pub fn set_filter(&self, v: i32) -> bool { self.put_i64("Filter", v as i64) }

    /// `cdrFileVersion` ——文件版本。
    pub fn version(&self) -> Option<i64> { self.prop_i64("Version") }
    /// 设置版本（`Version`）。
    pub fn set_version(&self, v: i32) -> bool { self.put_i64("Version", v as i64) }

    /// `cdrThumbnailSize` ——缩略图大小。
    pub fn thumbnail_size(&self) -> Option<i64> { self.prop_i64("ThumbnailSize") }
    /// 设置 COM 属性 `ThumbnailSize`。
    pub fn set_thumbnail_size(&self, v: i32) -> bool {
        self.put_i64("ThumbnailSize", v as i64)
    }

    /// `cdrExportRange` ——导出范围。
    pub fn range(&self) -> Option<i64> { self.prop_i64("Range") }
    /// 设置范围（`Range`）。
    pub fn set_range(&self, v: i32) -> bool { self.put_i64("Range", v as i64) }

    /// 读取覆盖（`Overwrite`）。
    pub fn overwrite(&self) -> Option<bool> { self.prop_bool("Overwrite") }
    /// 设置覆盖（`Overwrite`）。
    pub fn set_overwrite(&self, v: bool) -> bool { self.put_bool("Overwrite", v) }

    /// 读取嵌入 ICC 配置文件（`EmbedICCProfile`）。
    pub fn embed_icc_profile(&self) -> Option<bool> { self.prop_bool("EmbedICCProfile") }
    /// 设置嵌入 ICC 配置文件（`EmbedICCProfile`）。
    pub fn set_embed_icc_profile(&self, v: bool) -> bool {
        self.put_bool("EmbedICCProfile", v)
    }

    /// 读取 COM 属性 `EmbedVBAProject`。
    pub fn embed_vba_project(&self) -> Option<bool> { self.prop_bool("EmbedVBAProject") }
    /// 设置 COM 属性 `EmbedVBAProject`。
    pub fn set_embed_vba_project(&self, v: bool) -> bool {
        self.put_bool("EmbedVBAProject", v)
    }

    /// 读取 COM 属性 `IncludeCMXData`。
    pub fn include_cmx_data(&self) -> Option<bool> { self.prop_bool("IncludeCMXData") }
    /// 设置 COM 属性 `IncludeCMXData`。
    pub fn set_include_cmx_data(&self, v: bool) -> bool {
        self.put_bool("IncludeCMXData", v)
    }

    /// 读取 COM 属性 `KeepAppearance`。
    pub fn keep_appearance(&self) -> Option<bool> { self.prop_bool("KeepAppearance") }
    /// 设置 COM 属性 `KeepAppearance`。
    pub fn set_keep_appearance(&self, v: bool) -> bool {
        self.put_bool("KeepAppearance", v)
    }
}