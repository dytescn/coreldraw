//! `ICUIApplication` 鈥斺€?UI 搴旂敤

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::ui::{
    ICuiBitmapImage, ICuiDataContext, ICuiFrameWork, ICuiImageList,
    ICuiScreenRect, ICuiStatusText,
};

pub struct ICuiApplication {
    disp: ComObject,
}

impl ICuiApplication {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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
    // 鏁版嵁涓婁笅鏂?
    // ---------------------------------------------------------

    pub fn data_context(&self) -> Option<ICuiDataContext> {
        self.prop_dispatch("DataContext").map(ICuiDataContext::new)
    }

    // ---------------------------------------------------------
    // 妗嗘灦
    // ---------------------------------------------------------

    pub fn framework(&self) -> Option<ICuiFrameWork> {
        self.prop_dispatch("FrameWork").map(ICuiFrameWork::new)
    }

    // ---------------------------------------------------------
    // 宸ュ巶
    // ---------------------------------------------------------

    pub fn create_image_list(&self) -> Option<ICuiImageList> {
        self.disp
            .invoke_method("CreateImageList", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiImageList::new)
    }

    /// `ImageData` 鏄?`VARIANT`锛圫AFEARRAY / IStream锛夈€?
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

    pub fn create_status_text(&self) -> Option<ICuiStatusText> {
        self.disp
            .invoke_method("CreateStatusText", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiStatusText::new)
    }

    // ---------------------------------------------------------
    // 鏈湴鍖?
    // ---------------------------------------------------------

    pub fn load_localized_string(&self, guid: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(guid.into())];
        self.disp
            .invoke_method("LoadLocalizedString", args)
            .ok()?
            .to_string()
            .ok()
    }

    // ---------------------------------------------------------
    // 鏁版嵁婧?
    // ---------------------------------------------------------

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

    pub fn unregister_data_source(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("UnregisterDataSource", args)
            .ok()?
            .to_bool()
            .ok()
    }
}