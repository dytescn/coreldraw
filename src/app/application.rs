//! `IVGApplication` —— CorelDRAW 应用对象
//!
//! 通过 ProgID `CorelDRAW.Application.{version}` 创建。
//!
//! ```no_run
//! use cdrsdk::prelude::*;                          // → 改 coreldraw → cdrsdk
//!
//! let app = IvgApplication::new("26").expect("CorelDRAW 未启动");
//! println!("version = {:?}", app.version());
//! ```

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::document::{IvgDocument, IvgDocuments};
use crate::geometry::{IvgRect, IvgSnapPoint};
use crate::layer::IvgLayer;
use crate::page::{IvgPage, IvgPageSizes};
use crate::shape::{IvgShape, IvgShapeRange};
use crate::structs::{
    IvgStructAlignProperties, IvgStructCreateOptions, IvgStructExportOptions,
    IvgStructFontProperties, IvgStructHyphenationSettings, IvgStructImportOptions,
    IvgStructOpenOptions, IvgStructPaletteOptions, IvgStructPasteOptions,
    IvgStructSaveAsOptions, IvgStructSpaceProperties,
};
use crate::types::{progid_application, APP_INTER_IID};
use crate::view::{IvgWindow, IvgWindows};

// =============================================================
// IvgApplication
// =============================================================

/// CorelDRAW `IVGApplication` 对象的后期绑定包装（应用）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgApplication {
    disp: ComObject,
}

impl IvgApplication {
    // ---------------------------------------------------------
    // 构造
    // ---------------------------------------------------------

    /// 通过版本号创建实例，例如 `"24.0"`（CorelDRAW 2022）。
    ///
    /// 失败时返回 `None`（未安装 / 未启动/ ProgID 不匹配）。
    pub fn new(version: &str) -> Option<Self> {
        ComObject::new_from_name(&progid_application(version), APP_INTER_IID)
            .ok()
            .map(|disp| Self { disp })
    }

    /// 与 [`new`](Self::new) 相同，但返回 `Result`。
    pub fn try_new(version: &str) -> wincom::Result<Self> {
        ComObject::new_from_name(&progid_application(version), APP_INTER_IID)
            .map(|disp| Self { disp })
    }

    // ---------------------------------------------------------
    // 通用工具
    // ---------------------------------------------------------

    /// 读取一个字符串属性；属性不存在或类型不匹配时返回 `None`。
    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    /// 读取一个 `i64` 属性。
    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }


    /// 读取一个 `bool` 属性。
    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    /// 读取一个属性并轨 `IDispatch`。
    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp
            .get_property(name)
            .ok()?
            .to_idispatch().ok()
    }

    /// 统一写一个属性。
    fn put(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 属性：版本 & 路径
    // ---------------------------------------------------------

    /// 读取版本（`version`）。
    pub fn version(&self) -> Option<String> {
        self.prop_string("version")
    }

    /// 读取 COM 属性 `versionmajor`。
    pub fn version_major(&self) -> Option<i64> {
        self.prop_i64("versionmajor")
    }

    /// 读取 COM 属性 `versionminor`。
    pub fn version_minor(&self) -> Option<i64> {
        self.prop_i64("versionminor")
    }

    /// 读取 COM 属性 `versionbuild`。
    pub fn version_build(&self) -> Option<i64> {
        self.prop_i64("versionbuild")
    }

    /// 读取路径（`Path`）。
    pub fn path(&self) -> Option<String> {
        self.prop_string("Path")
    }

    /// 读取 COM 属性 `ConfigPath`。
    pub fn config_path(&self) -> Option<String> {
        self.prop_string("ConfigPath")
    }

    /// 读取 COM 属性 `SetupPath`。
    pub fn setup_path(&self) -> Option<String> {
        self.prop_string("SetupPath")
    }

    /// 读取 COM 属性 `ProgramPath`。
    pub fn program_path(&self) -> Option<String> {
        self.prop_string("ProgramPath")
    }

    /// 读取 COM 属性 `AddonPath`。
    pub fn addon_path(&self) -> Option<String> {
        self.prop_string("AddonPath")
    }

    /// 读取用户数据路径（`UserDataPath`）。
    pub fn user_data_path(&self) -> Option<String> {
        self.prop_string("UserDataPath")
    }

    /// 读取用户工作区路径（`UserWorkspacePath`）。
    pub fn user_workspace_path(&self) -> Option<String> {
        self.prop_string("UserWorkspacePath")
    }

    /// 读取语言路径（`LanguagePath`）。
    pub fn language_path(&self) -> Option<String> {
        self.prop_string("LanguagePath")
    }

    /// 读取 COM 属性 `HelpFile`。
    pub fn help_file(&self) -> Option<String> {
        self.prop_string("HelpFile")
    }

    // ---------------------------------------------------------
    // 属性：窗口可见性
    // ---------------------------------------------------------

    /// 读取可见性（`Visible`）。
    pub fn visible(&self) -> Option<bool> {
        self.prop_bool("Visible")
    }

    /// 设置可见性。
    pub fn set_visible(&self, v: bool) -> bool {
        self.put("Visible", Variant::from_bool(v))
    }

    // ---------------------------------------------------------
    // 文档
    // ---------------------------------------------------------

    /// 读取文档集（`Documents`）。
    pub fn documents(&self) -> Option<IvgDocuments> {
        self.prop_dispatch("Documents").map(IvgDocuments::new)
    }

    /// 读取活动文档（`ActiveDocument`）。
    pub fn active_document(&self) -> Option<IvgDocument> {
        self.prop_dispatch("ActiveDocument").map(IvgDocument::new)
    }

    /// 打开文档，返回是否成功。
    pub fn open_document(&self, src: impl Into<String>) -> bool {
        let args = vec![
            Variant::from_str(&src.into()),
            Variant::from_i64(0),
        ];
        self.disp.invoke_method("OpenDocument", args).is_ok()
    }

    /// 打开文档（带 `IVGStructOpenOptions`）。
    pub fn open_document_ex(
        &self,
        src: impl Into<String>,
        options: &IvgStructOpenOptions,
    ) -> Option<IvgDocument> {
        let args = vec![
            Variant::from_str(&src.into()),
            options.as_variant(),
        ];
        self.disp
            .invoke_method("OpenDocumentEx", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgDocument::new)
    }

    /// 新建空文档。
    pub fn create_document(&self) -> Option<IvgDocument> {
        self.disp
            .invoke_method("CreateDocument", vec![])
            .ok()?
            .to_idispatch().ok()
            .map(IvgDocument::new)
    }

    /// 新建文档（带 `IVGStructCreateOptions`）。
    pub fn create_document_ex(
        &self,
        options: &IvgStructCreateOptions,
    ) -> Option<IvgDocument> {
        let args = vec![options.as_variant()];
        self.disp
            .invoke_method("CreateDocumentEx", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgDocument::new)
    }

    /// 从模板新建文档。
    pub fn create_document_from_template(
        &self,
        template: impl Into<String>,
        include_graphics: bool,
    ) -> Option<IvgDocument> {
        let args = vec![
            Variant::from_str(&template.into()),
            Variant::from_bool(include_graphics),
        ];
        self.disp
            .invoke_method("CreateDocumentFromTemplate", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgDocument::new)
    }

    // ---------------------------------------------------------
    // 页面 / 图层 / 选择
    // ---------------------------------------------------------

    /// 读取活动页面（`ActivePage`）。
    pub fn active_page(&self) -> Option<IvgPage> {
        self.prop_dispatch("ActivePage").map(IvgPage::new)
    }

    /// 读取活动图层（`ActiveLayer`）。
    pub fn active_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveLayer").map(IvgLayer::new)
    }

    /// 读取活动图形（`ActiveShape`）。
    pub fn active_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ActiveShape").map(IvgShape::new)
    }

    /// 读取活动选择范围（`ActiveSelectionRange`）。
    pub fn active_selection_range(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("ActiveSelectionRange").map(IvgShapeRange::new)
    }

    /// 读取 COM 属性 `PageSizes`。
    pub fn page_sizes(&self) -> Option<IvgPageSizes> {
        self.prop_dispatch("PageSizes").map(IvgPageSizes::new)
    }

    // ---------------------------------------------------------
    // 窗口 / 视图
    // ---------------------------------------------------------

    /// 读取活动窗口（`ActiveWindow`）。
    pub fn active_window(&self) -> Option<IvgWindow> {
        self.prop_dispatch("ActiveWindow").map(IvgWindow::new)
    }

    /// 读取窗口集（`Windows`）。
    pub fn windows(&self) -> Option<IvgWindows> {
        self.prop_dispatch("Windows").map(IvgWindows::new)
    }

    // ---------------------------------------------------------
    // 颜色工厂
    // ---------------------------------------------------------

    /// 创建 RGB 颜色（COM 方法 `CreateRGBColor`）。
    pub fn create_rgb_color(&self, r: i32, g: i32, b: i32) -> Option<IvgColor> {
        let args = vec![
            Variant::from_i64(r as i64),
            Variant::from_i64(g as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp
            .invoke_method("CreateRGBColor", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgColor::new)
    }

    /// 创建 CMYK 颜色（COM 方法 `CreateCMYKColor`）。
    pub fn create_cmyk_color(
        &self,
        c: i32,
        m: i32,
        y: i32,
        k: i32,
    ) -> Option<IvgColor> {
        let args = vec![
            Variant::from_i64(c as i64),
            Variant::from_i64(m as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(k as i64),
        ];
        self.disp
            .invoke_method("CreateCMYKColor", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgColor::new)
    }

    /// 创建灰度颜色（COM 方法 `CreateGrayColor`）。
    pub fn create_gray_color(&self, gray: i32) -> Option<IvgColor> {
        let args = vec![Variant::from_i64(gray as i64)];
        self.disp
            .invoke_method("CreateGrayColor", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgColor::new)
    }

    /// 创建颜色（COM 方法 `CreateColor`）。
    pub fn create_color(&self, color_str: impl Into<String>) -> Option<IvgColor> {
        let args = vec![Variant::from_str(&color_str.into())];
        self.disp
            .invoke_method("CreateColor", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgColor::new)
    }


    /// 创建导出选项集。
    pub fn create_export_options(&self) -> Option<IvgStructExportOptions> {
        self.invoke_factory(
            "CreateStructExportOptions",
            IvgStructExportOptions::from_disp,
        )
    }

    /// 创建保存 as 选项集。
    pub fn create_save_as_options(&self) -> Option<IvgStructSaveAsOptions> {
        self.invoke_factory(
            "CreateStructSaveAsOptions",
            IvgStructSaveAsOptions::from_disp,
        )
    }

    /// 创建导入选项集。
    pub fn create_import_options(&self) -> Option<IvgStructImportOptions> {
        self.invoke_factory(
            "CreateStructImportOptions",
            IvgStructImportOptions::from_disp,
        )
    }

    /// 创建调色板选项集。
    pub fn create_palette_options(&self) -> Option<IvgStructPaletteOptions> {
        self.invoke_factory(
            "CreateStructPaletteOptions",
            IvgStructPaletteOptions::from_disp,
        )
    }

    /// 创建打开选项集。
    pub fn create_open_options(&self) -> Option<IvgStructOpenOptions> {
        self.invoke_factory(
            "CreateStructOpenOptions",
            IvgStructOpenOptions::from_disp,
        )
    }

    /// 创建创建选项集。
    pub fn create_create_options(&self) -> Option<IvgStructCreateOptions> {
        self.invoke_factory(
            "CreateStructCreateOptions",
            IvgStructCreateOptions::from_disp,
        )
    }

    /// 创建 paste 选项集。
    pub fn create_paste_options(&self) -> Option<IvgStructPasteOptions> {
        self.invoke_factory(
            "CreateStructPasteOptions",
            IvgStructPasteOptions::from_disp,
        )
    }

    /// 创建字体属性集。
    pub fn create_font_properties(&self) -> Option<IvgStructFontProperties> {
        self.invoke_factory(
            "CreateStructFontProperties",
            IvgStructFontProperties::from_disp,
        )
    }

    /// 创建对齐属性集。
    pub fn create_align_properties(&self) -> Option<IvgStructAlignProperties> {
        self.invoke_factory(
            "CreateStructAlignProperties",
            IvgStructAlignProperties::from_disp,
        )
    }

    /// 创建间距属性集。
    pub fn create_space_properties(&self) -> Option<IvgStructSpaceProperties> {
        self.invoke_factory(
            "CreateStructSpaceProperties",
            IvgStructSpaceProperties::from_disp,
        )
    }

    /// 创建断字设置集。
    pub fn create_hyphenation_settings(
        &self,
    ) -> Option<IvgStructHyphenationSettings> {
        self.invoke_factory(
            "CreateStructHyphenationSettings",
            IvgStructHyphenationSettings::from_disp,
        )
    }

    /// 通用工厂调用：`invoke_method(name, []) → IDispatch → T`
    fn invoke_factory<T>(
        &self,
        method: &str,
        from_disp: fn(IDispatch) -> T,
    ) -> Option<T> {
        self.disp
            .invoke_method(method, vec![])
            .ok()?
            .to_idispatch().ok()
            .map(from_disp)
    }

    // ---------------------------------------------------------
    // 几何工厂
    // ---------------------------------------------------------

    /// 创建矩形（COM 方法 `CreateRect`）。
    pub fn create_rect(
        &self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
    ) -> Option<IvgRect> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp
            .invoke_method("CreateRect", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgRect::new)
    }

    /// 创建捕捉点（COM 方法 `CreateSnapPoint`）。
    pub fn create_snap_point(&self, x: f64, y: f64) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_f64(x), Variant::from_f64(y)];
        self.disp
            .invoke_method("CreateSnapPoint", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgSnapPoint::new)
    }

    // ---------------------------------------------------------
    // 单位
    // ---------------------------------------------------------

    /// 读取单位（`Unit`）。
    pub fn unit(&self) -> Option<i64> {
        self.prop_i64("Unit")
    }

    /// 设置单位。
    pub fn set_unit(&self, unit: i32) -> bool {
        self.put("Unit", Variant::from_i64(unit as i64))
    }

    /// 转换单位。
    pub fn convert_units(&self, value: f64, from: i32, to: i32) -> Option<f64> {
        crate::geometry::units::convert_by_i32(value, from, to)
    }

    // ---------------------------------------------------------
    // 脚本 & 其它
    // ---------------------------------------------------------

    /// `Evaluate("...")` ——执行 CorelDRAW 表达式。
    pub fn evaluate(&self, expr: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(&expr.into())];
        self.disp.invoke_method("Evaluate", args).ok()
    }

    /// 刷新（COM 方法 `Refresh`）。
    pub fn refresh(&self) -> bool {
        self.disp.invoke_method("Refresh", vec![]).is_ok()
    }

    /// 退出 CorelDRAW。
    pub fn quit(&self) -> bool {
        self.disp.invoke_method("Quit", vec![]).is_ok()
    }

    /// 从…构造分发。
    pub fn from_dispatch(disp: IDispatch) -> Self {
        Self {
            disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH),
        }
    }
      /// 别名：兼容旧代码里用的 `from_disp`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::from_dispatch(disp)
    }

    /// 允许下游直接拿到 COM 对象。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }
}