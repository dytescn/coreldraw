//! `ICUIAutomation` / `ICUIControlData` —— UI 自动化
//!
//! 通过 GUID 访问工具条/ 按钮 / 菜单项，可编程地触发命令。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiAutomation
// =============================================================

/// CorelDRAW 对象的后期绑定包装（自动化）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiAutomation {
    disp: ComObject,
}

impl ICuiAutomation {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    // ---------------------------------------------------------
    // 工具条项
    // ---------------------------------------------------------

    /// 工具条上项数。
    pub fn get_num_items_on_bar(&self, bar_guid: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(bar_guid.into())];
        self.disp
            .invoke_method("GetNumItemsOnBar", args)
            .ok()?
            .to_i64()
            .ok()
    }

    /// 取第 index 项（1-based），返回 `(HasSubBar, GuidItem)`。
    /// IDispatch 后期绑定只能拿一个，这里返回 `GuidItem`。
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

    /// 读取 COM 属性 `GetItemInstanceHwnd`。
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

    /// 取子工具条 GUID。
    pub fn get_sub_bar(&self, bar_guid: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(bar_guid.into())];
        self.disp
            .invoke_method("GetSubBar", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 显示 / 隐藏工具条，返回原状态。
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

    /// 取按钟/ 菜单项的标题文字。
    pub fn get_caption_text(&self, item_guid: impl Into<String>) -> Option<String> {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp
            .invoke_method("GetCaptionText", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 触发命令。
    pub fn invoke(&self, item_guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp.invoke_method("Invoke", args).is_ok()
    }

    /// 触发菜单项。
    pub fn invoke_item(&self, item_guid: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp.invoke_method("InvokeItem", args).is_ok()
    }

    /// 触发对话框项。
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

    /// 是否启用。
    pub fn is_enabled(&self, item_guid: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(item_guid.into())];
        self.disp
            .invoke_method("IsEnabled", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 取项在屏幕上的矩形，返回 `(x, y, w, h, on_screen)`。
    /// IDispatch 后期绑定拿不全，只返回 `None`。
    pub fn get_item_screen_rect(
        &self,
        _parent_guid: impl Into<String>,
        _item_guid: impl Into<String>,
    ) -> Option<(i32, i32, i32, i32, bool)> {
        None
    }

    /// 取活动菜单项矩形。
    pub fn get_active_menu_item_screen_rect(
        &self,
        _item_index: i32,
    ) -> Option<(i32, i32, i32, i32, bool)> {
        None
    }

    /// 取活动菜单项的 GUID。
    pub fn get_active_menu_item_guid(&self, item_index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(item_index as i64)];
        self.disp
            .invoke_method("GetActiveMenuItemGuid", args)
            .ok()?
            .to_string()
            .ok()
    }

    // ---------------------------------------------------------
    // 控件数据
    // ---------------------------------------------------------

    /// 读取控件数据（`GetControlData`）。
    pub fn get_control_data(&self, guid: impl Into<String>) -> Option<ICuiControlData> {
        let args = vec![Variant::from_str(guid.into())];
        self.disp
            .invoke_method("GetControlData", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiControlData::new)
    }

    /// 读取控件数据扩展版（`GetControlDataEx`）。
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

/// CorelDRAW 对象的后期绑定包装（控件数据）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiControlData {
    disp: ComObject,
}

impl ICuiControlData {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 取控件某个属性的值。
    pub fn get_value(&self, property_name: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(property_name.into())];
        self.disp.invoke_method("GetValue", args).ok()
    }
}