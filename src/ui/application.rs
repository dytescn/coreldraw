//! `ICUIApplication` —— UI 应用

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiBitmapImage, ICuiDataContext, ICuiFrameWork, ICuiImageList,
    ICuiScreenRect, ICuiStatusText,
};

/// CorelDRAW 对象的后期绑定包装（应用）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiApplication {
    disp: ComObject,
}

impl ICuiApplication {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    // fn prop_string(&self, name: &str) -> Option<String> {
    //     self.disp.get_property(name).ok()?.to_string().ok()
    // }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 数据上下文
    // ---------------------------------------------------------

    /// 读取数据上下文（`DataContext`）。
    pub fn data_context(&self) -> Option<ICuiDataContext> {
        self.prop_dispatch("DataContext").map(ICuiDataContext::new)
    }

    // ---------------------------------------------------------
    // 框架
    // ---------------------------------------------------------

    /// 读取 COM 属性 `FrameWork`。
    pub fn framework(&self) -> Option<ICuiFrameWork> {
        self.prop_dispatch("FrameWork").map(ICuiFrameWork::new)
    }

    // ---------------------------------------------------------
    // 工厂
    // ---------------------------------------------------------

    /// 创建图像列表（COM 方法 `CreateImageList`）。
    pub fn create_image_list(&self) -> Option<ICuiImageList> {
        self.disp
            .invoke_method("CreateImageList", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiImageList::new)
    }

    /// `ImageData` 是 `VARIANT`（SAFEARRAY / IStream）。
    pub fn create_bitmap_image(
        &self,
        image_data: Variant,
        max_size: i32,
    ) -> Option<ICuiBitmapImage> {
        let args = vec![
            image_data,
            Variant::from_i64(max_size as i64),
        ];
        self.disp
            .invoke_method("CreateBitmapImage", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiBitmapImage::new)
    }

    /// 创建屏幕矩形（COM 方法 `CreateScreenRect`）。
    pub fn create_screen_rect(
        &self,
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    ) -> Option<ICuiScreenRect> {
        let args = vec![
            Variant::from_i64(left as i64),
            Variant::from_i64(top as i64),
            Variant::from_i64(width as i64),
            Variant::from_i64(height as i64),
        ];
        self.disp
            .invoke_method("CreateScreenRect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }

    /// 创建状态文本（COM 方法 `CreateStatusText`）。
    pub fn create_status_text(&self) -> Option<ICuiStatusText> {
        self.disp
            .invoke_method("CreateStatusText", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiStatusText::new)
    }

    // ---------------------------------------------------------
    // 本地区
    // ---------------------------------------------------------

    /// 调用 COM 方法 `LoadLocalizedString`。
    pub fn load_localized_string(&self, guid: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(guid.into())];
        self.disp
            .invoke_method("LoadLocalizedString", args)
            .ok()?
            .to_string()
            .ok()
    }

    // ---------------------------------------------------------
    // 数据源
    // ---------------------------------------------------------

    /// 注册数据源（COM 方法 `RegisterDataSource`）。
    pub fn register_data_source(
        &self,
        name: impl Into<String>,
        factory: IDispatch,
        category_list: impl Into<String>,
        auto_create_instance: bool,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_dispatch(&factory),
            Variant::from_str(category_list.into()),
            Variant::from_bool(auto_create_instance),
        ];
        self.disp
            .invoke_method("RegisterDataSource", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 注销数据源（COM 方法 `UnregisterDataSource`）。
    pub fn unregister_data_source(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("UnregisterDataSource", args)
            .ok()?
            .to_bool()
            .ok()
    }
}