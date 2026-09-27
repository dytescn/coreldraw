//! `ICUIAutomation` / `ICUIControlData` 鈥斺€?UI 鑷姩鍖?
//!
//! 閫氳繃 GUID 璁块棶宸ュ叿鏉?/ 鎸夐挳 / 鑿滃崟椤癸紝鍙紪绋嬪湴瑙﹀彂鍛戒护銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiAutomation
// =============================================================

pub struct ICuiAutomation {
    disp: ComObject,
}

impl ICuiAutomation {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    // ---------------------------------------------------------
    // 宸ュ叿鏉￠」
    // ---------------------------------------------------------

    /// 宸ュ叿鏉′笂椤规暟銆?
    pub fn get_num_items_on_bar(&self, bar_guid: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(bar_guid.into())];
        self.disp
            .invoke_method("GetNumItemsOnBar", args)
            .ok()?
            .to_i64()
            .ok()
    }

    /// 鍙栫 index 椤癸紙1-based锛夛紝杩斿洖 `(HasSubBar, GuidItem)`銆?
    /// IDispatch 鍚庢湡缁戝畾鍙兘鎷夸竴涓紝杩欓噷杩斿洖 `GuidItem`銆?
    pub fn get_item(&self, bar_guid: impl Into<String>, index: i32) -> Option<String> {
        let args = vec![
            Variant::from_str(bar_guid.into()),
            Variant::from_i64(index as i64),
        ];
        self.disp
            .invoke_method("GetItem", args)
            .ok()?
            .to_string()
            .ok()
    }

    pub fn get_item_instance_hwnd(
        &self,
        parent_guid: impl Into<String>,
        item_guid: impl Into<String>,
    ) -> Option<i64> {
        let args = vec![
            Variant::from_str(parent_guid.into()),
            Variant::from_str(item_guid.into()),
        ];
        self.disp
            .invoke_method("GetItemInstanceHwnd", args)
            .ok()?
            .to_i64()
            .ok()
    }

    /// 鍙栧瓙宸ュ叿鏉?GUID銆?
    pub fn get_sub_bar(&self, bar_guid: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(bar_guid.into())];
        self.disp
            .invoke_method("GetSubBar", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 鏄剧ず / 闅愯棌宸ュ叿鏉★紝杩斿洖鍘熺姸鎬併€?
    pub fn show_bar(&self, bar_guid: impl Into<String>, show: bool) -> Option<bool> {
        let args = vec![
            Variant::from_str(bar_guid.into()),
            Variant::from_bool(show),
        ];
        self.disp
            .invoke_method("ShowBar", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 鍙栨寜閽?/ 鑿滃崟椤圭殑鏍囬鏂囧瓧銆?
    pub fn get_caption_text(&self, item_guid: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp
            .invoke_method("GetCaptionText", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 瑙﹀彂鍛戒护銆?
    pub fn invoke(&self, item_guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp.invoke_method("Invoke", args).is_ok()
    }

    /// 瑙﹀彂鑿滃崟椤广€?
    pub fn invoke_item(&self, item_guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp.invoke_method("InvokeItem", args).is_ok()
    }

    /// 瑙﹀彂瀵硅瘽妗嗛」銆?
    pub fn invoke_dialog_item(
        &self,
        dialog_guid: impl Into<String>,
        item_guid: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(dialog_guid.into()),
            Variant::from_str(item_guid.into()),
        ];
        self.disp.invoke_method("InvokeDialogItem", args).is_ok()
    }

    /// 鏄惁鍚敤銆?
    pub fn is_enabled(&self, item_guid: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp
            .invoke_method("IsEnabled", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 鍙栭」鍦ㄥ睆骞曚笂鐨勭煩褰紝杩斿洖 `(x, y, w, h, on_screen)`銆?
    /// IDispatch 鍚庢湡缁戝畾鎷夸笉鍏紝鍙繑鍥?`None`銆?
    pub fn get_item_screen_rect(
        &self,
        _parent_guid: impl Into<String>,
        _item_guid: impl Into<String>,
    ) -> Option<(i32, i32, i32, i32, bool)> {
        None
    }

    /// 鍙栨椿鍔ㄨ彍鍗曢」鐭╁舰銆?
    pub fn get_active_menu_item_screen_rect(
        &self,
        _item_index: i32,
    ) -> Option<(i32, i32, i32, i32, bool)> {
        None
    }

    /// 鍙栨椿鍔ㄨ彍鍗曢」鐨?GUID銆?
    pub fn get_active_menu_item_guid(&self, item_index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(item_index as i64)];
        self.disp
            .invoke_method("GetActiveMenuItemGuid", args)
            .ok()?
            .to_string()
            .ok()
    }

    // ---------------------------------------------------------
    // 鎺т欢鏁版嵁
    // ---------------------------------------------------------

    pub fn get_control_data(&self, guid: impl Into<String>) -> Option<ICuiControlData> {
        let args = vec![Variant::from_str(guid.into())];
        self.disp
            .invoke_method("GetControlData", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControlData::new)
    }

    pub fn get_control_data_ex(
        &self,
        parent_guid: impl Into<String>,
        guid: impl Into<String>,
    ) -> Option<ICuiControlData> {
        let args = vec![
            Variant::from_str(parent_guid.into()),
            Variant::from_str(guid.into()),
        ];
        self.disp
            .invoke_method("GetControlDataEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControlData::new)
    }
}

// =============================================================
// ICuiControlData
// =============================================================

pub struct ICuiControlData {
    disp: ComObject,
}

impl ICuiControlData {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 鍙栨帶浠舵煇涓睘鎬х殑鍊笺€?
    pub fn get_value(&self, property_name: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(property_name.into())];
        self.disp.invoke_method("GetValue", args).ok()
    }
}