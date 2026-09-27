//! `IVGApplication` —— CorelDRAW 应用对象
//!
//! 通过 ProgID `CorelDRAW.Application.{version}` 创建。
//!
//! ```no_run
//! use cdrsdk::prelude::*;                          // ← 改 coreldraw → cdrsdk
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

pub struct IvgApplication {
    disp: ComObject,
}

impl IvgApplication {
    // ---------------------------------------------------------
    // 鏋勯€?
    // ---------------------------------------------------------

    /// 閫氳繃鐗堟湰鍙峰垱寤哄疄渚嬶紝渚嬪 `"24.0"`锛圕orelDRAW 2022锛夈€?
    ///
    /// 澶辫触鏃惰繑鍥?`None`锛堟湭瀹夎 / 鏈惎鍔?/ ProgID 涓嶅尮閰嶏級銆?
    pub fn new(version: &str) -> Option<Self> {
        ComObject::new_from_name(&progid_application(version), APP_INTER_IID)
            .ok()
            .map(|disp| Self { disp })
    }

    /// 涓?[`new`](Self::new) 鍚岋紝浣嗚繑鍥?`Result`銆?
    pub fn try_new(version: &str) -> wincom::Result<Self> {
        ComObject::new_from_name(&progid_application(version), APP_INTER_IID)
            .map(|disp| Self { disp })
    }

    // ---------------------------------------------------------
    // 閫氱敤宸ュ叿
    // ---------------------------------------------------------

    /// 璇诲彇涓€涓瓧绗︿覆灞炴€э紱灞炴€т笉瀛樺湪鎴栫被鍨嬩笉鍖归厤鏃惰繑鍥?`None`銆?
    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    /// 璇诲彇涓€涓?`i64` 灞炴€с€?
    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }


    /// 璇诲彇涓€涓?`bool` 灞炴€с€?
    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    /// 璇诲彇涓€涓睘鎬у苟杞?`IDispatch`銆?
    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp
            .get_property(name)
            .ok()?
            .to_idispatch().ok()
    }

    /// 缁熶竴鍐欎竴涓睘鎬с€?
    fn put(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 灞炴€э細鐗堟湰 & 璺緞
    // ---------------------------------------------------------

    pub fn version(&self) -> Option<String> {
        self.prop_string("version")
    }

    pub fn version_major(&self) -> Option<i64> {
        self.prop_i64("versionmajor")
    }

    pub fn version_minor(&self) -> Option<i64> {
        self.prop_i64("versionminor")
    }

    pub fn version_build(&self) -> Option<i64> {
        self.prop_i64("versionbuild")
    }

    pub fn path(&self) -> Option<String> {
        self.prop_string("Path")
    }

    pub fn config_path(&self) -> Option<String> {
        self.prop_string("ConfigPath")
    }

    pub fn setup_path(&self) -> Option<String> {
        self.prop_string("SetupPath")
    }

    pub fn program_path(&self) -> Option<String> {
        self.prop_string("ProgramPath")
    }

    pub fn addon_path(&self) -> Option<String> {
        self.prop_string("AddonPath")
    }

    pub fn user_data_path(&self) -> Option<String> {
        self.prop_string("UserDataPath")
    }

    pub fn user_workspace_path(&self) -> Option<String> {
        self.prop_string("UserWorkspacePath")
    }

    pub fn language_path(&self) -> Option<String> {
        self.prop_string("LanguagePath")
    }

    pub fn help_file(&self) -> Option<String> {
        self.prop_string("HelpFile")
    }

    // ---------------------------------------------------------
    // 灞炴€э細绐楀彛鍙鎬?
    // ---------------------------------------------------------

    pub fn visible(&self) -> Option<bool> {
        self.prop_bool("Visible")
    }

    pub fn set_visible(&self, v: bool) -> bool {
        self.put("Visible", Variant::from_bool(v))
    }

    // ---------------------------------------------------------
    // 鏂囨。
    // ---------------------------------------------------------

    pub fn documents(&self) -> Option<IvgDocuments> {
        self.prop_dispatch("Documents").map(IvgDocuments::new)
    }

    pub fn active_document(&self) -> Option<IvgDocument> {
        self.prop_dispatch("ActiveDocument").map(IvgDocument::new)
    }

    /// 鎵撳紑鏂囨。锛岃繑鍥炴槸鍚︽垚鍔熴€?
    pub fn open_document(&self, src: impl Into<String>) -> bool {
        let args = vec![
            Variant::from_str(&src.into()),
            Variant::from_i64(0),
        ];
        self.disp.invoke_method("OpenDocument", args).is_ok()
    }

    /// 鎵撳紑鏂囨。锛堝甫 `IVGStructOpenOptions`锛夈€?
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

    /// 鏂板缓绌烘枃妗ｃ€?
    pub fn create_document(&self) -> Option<IvgDocument> {
        self.disp
            .invoke_method("CreateDocument", vec![])
            .ok()?
            .to_idispatch().ok()
            .map(IvgDocument::new)
    }

    /// 鏂板缓鏂囨。锛堝甫 `IVGStructCreateOptions`锛夈€?
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

    /// 浠庢ā鏉挎柊寤烘枃妗ｃ€?
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
    // 椤甸潰 / 鍥惧眰 / 閫夋嫨
    // ---------------------------------------------------------

    pub fn active_page(&self) -> Option<IvgPage> {
        self.prop_dispatch("ActivePage").map(IvgPage::new)
    }

    pub fn active_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveLayer").map(IvgLayer::new)
    }

    pub fn active_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ActiveShape").map(IvgShape::new)
    }

    pub fn active_selection_range(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("ActiveSelectionRange").map(IvgShapeRange::new)
    }

    pub fn page_sizes(&self) -> Option<IvgPageSizes> {
        self.prop_dispatch("PageSizes").map(IvgPageSizes::new)
    }

    // ---------------------------------------------------------
    // 绐楀彛 / 瑙嗗浘
    // ---------------------------------------------------------

    pub fn active_window(&self) -> Option<IvgWindow> {
        self.prop_dispatch("ActiveWindow").map(IvgWindow::new)
    }

    pub fn windows(&self) -> Option<IvgWindows> {
        self.prop_dispatch("Windows").map(IvgWindows::new)
    }

    // ---------------------------------------------------------
    // 棰滆壊宸ュ巶
    // ---------------------------------------------------------

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

    pub fn create_gray_color(&self, gray: i32) -> Option<IvgColor> {
        let args = vec![Variant::from_i64(gray as i64)];
        self.disp
            .invoke_method("CreateGrayColor", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgColor::new)
    }

    pub fn create_color(&self, color_str: impl Into<String>) -> Option<IvgColor> {
        let args = vec![Variant::from_str(&color_str.into())];
        self.disp
            .invoke_method("CreateColor", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgColor::new)
    }


    pub fn create_export_options(&self) -> Option<IvgStructExportOptions> {
        self.invoke_factory(
            "CreateStructExportOptions",
            IvgStructExportOptions::from_disp,
        )
    }

    pub fn create_save_as_options(&self) -> Option<IvgStructSaveAsOptions> {
        self.invoke_factory(
            "CreateStructSaveAsOptions",
            IvgStructSaveAsOptions::from_disp,
        )
    }

    pub fn create_import_options(&self) -> Option<IvgStructImportOptions> {
        self.invoke_factory(
            "CreateStructImportOptions",
            IvgStructImportOptions::from_disp,
        )
    }

    pub fn create_palette_options(&self) -> Option<IvgStructPaletteOptions> {
        self.invoke_factory(
            "CreateStructPaletteOptions",
            IvgStructPaletteOptions::from_disp,
        )
    }

    pub fn create_open_options(&self) -> Option<IvgStructOpenOptions> {
        self.invoke_factory(
            "CreateStructOpenOptions",
            IvgStructOpenOptions::from_disp,
        )
    }

    pub fn create_create_options(&self) -> Option<IvgStructCreateOptions> {
        self.invoke_factory(
            "CreateStructCreateOptions",
            IvgStructCreateOptions::from_disp,
        )
    }

    pub fn create_paste_options(&self) -> Option<IvgStructPasteOptions> {
        self.invoke_factory(
            "CreateStructPasteOptions",
            IvgStructPasteOptions::from_disp,
        )
    }

    pub fn create_font_properties(&self) -> Option<IvgStructFontProperties> {
        self.invoke_factory(
            "CreateStructFontProperties",
            IvgStructFontProperties::from_disp,
        )
    }

    pub fn create_align_properties(&self) -> Option<IvgStructAlignProperties> {
        self.invoke_factory(
            "CreateStructAlignProperties",
            IvgStructAlignProperties::from_disp,
        )
    }

    pub fn create_space_properties(&self) -> Option<IvgStructSpaceProperties> {
        self.invoke_factory(
            "CreateStructSpaceProperties",
            IvgStructSpaceProperties::from_disp,
        )
    }

    pub fn create_hyphenation_settings(
        &self,
    ) -> Option<IvgStructHyphenationSettings> {
        self.invoke_factory(
            "CreateStructHyphenationSettings",
            IvgStructHyphenationSettings::from_disp,
        )
    }

    /// 閫氱敤宸ュ巶璋冪敤锛歚invoke_method(name, []) 鈫?IDispatch 鈫?T`
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
    // 鍑犱綍宸ュ巶
    // ---------------------------------------------------------

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

    pub fn create_snap_point(&self, x: f64, y: f64) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_f64(x), Variant::from_f64(y)];
        self.disp
            .invoke_method("CreateSnapPoint", args)
            .ok()?
            .to_idispatch().ok()
            .map(IvgSnapPoint::new)
    }

    // ---------------------------------------------------------
    // 鍗曚綅
    // ---------------------------------------------------------

    pub fn unit(&self) -> Option<i64> {
        self.prop_i64("Unit")
    }

    pub fn set_unit(&self, unit: i32) -> bool {
        self.put("Unit", Variant::from_i64(unit as i64))
    }

    pub fn convert_units(&self, value: f64, from: i32, to: i32) -> Option<f64> {
        crate::geometry::units::convert_by_i32(value, from, to)
    }

    // ---------------------------------------------------------
    // 鑴氭湰 & 鍏跺畠
    // ---------------------------------------------------------

    /// `Evaluate("...")` 鈥斺€?鎵ц CorelDRAW 琛ㄨ揪寮忋€?
    pub fn evaluate(&self, expr: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(&expr.into())];
        self.disp.invoke_method("Evaluate", args).ok()
    }

    pub fn refresh(&self) -> bool {
        self.disp.invoke_method("Refresh", vec![]).is_ok()
    }

    /// 閫€鍑?CorelDRAW銆?
    pub fn quit(&self) -> bool {
        self.disp.invoke_method("Quit", vec![]).is_ok()
    }

    pub fn from_dispatch(disp: IDispatch) -> Self {
        Self {
            disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH),
        }
    }
      /// 别名：兼容旧代码里用的 `from_disp`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::from_dispatch(disp)
    }

    /// 鍏佽涓嬫父鐩存帴鎷垮埌 COM 瀵硅薄銆?
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }
}