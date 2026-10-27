//! `IVGToolState` / `IVGToolStateAttributes` —— 工具状态

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgPoint;

// =============================================================
// IvgToolState
// =============================================================

/// 工具状态接口。
///
/// ⚠️ 这是一个**回调接口**，CorelDRAW 会在鼠标事件时调用你。
/// 要在 Rust 侧真正使用，需要实现 `IDispatch` 并注册。
/// 当前只提供 `On*` 触发包装。
pub struct IvgToolState {
    disp: ComObject,
}

impl IvgToolState {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取 COM 属性 `IsDrawing`。
    pub fn is_drawing(&self) -> Option<bool> {
        self.disp.get_property("IsDrawing").ok()?.to_bool().ok()
    }

    /// 于开始状态（COM 方法 `OnStartState`）。
    pub fn on_start_state(&self, attrs: &IvgToolStateAttributes) -> bool {
        let args = vec![attrs.as_variant()];
        self.disp.invoke_method("OnStartState", args).is_ok()
    }

    /// 调用 COM 方法 `OnExitState`。
    pub fn on_exit_state(&self) -> bool {
        self.disp.invoke_method("OnExitState", vec![]).is_ok()
    }

    /// 于鼠标移动（COM 方法 `OnMouseMove`）。
    pub fn on_mouse_move(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnMouseMove", args).is_ok()
    }

    /// 于左键向下（COM 方法 `OnLButtonDown`）。
    pub fn on_lbutton_down(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnLButtonDown", args).is_ok()
    }

    /// 调用 COM 方法 `OnLButtonUp`。
    pub fn on_lbutton_up(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnLButtonUp", args).is_ok()
    }

    /// 调用 COM 方法 `OnLButtonDblClick`。
    pub fn on_lbutton_dbl_click(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnLButtonDblClick", args).is_ok()
    }

    /// 于单击（COM 方法 `OnClick`）。
    pub fn on_click(&self, pt: &IvgPoint) -> Option<bool> {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnClick", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `OnRButtonDown`。
    pub fn on_rbutton_down(&self, pt: &IvgPoint) -> Option<bool> {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnRButtonDown", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `OnRButtonUp`。
    pub fn on_rbutton_up(&self, pt: &IvgPoint) -> Option<bool> {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnRButtonUp", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `OnKeyDown`。
    pub fn on_key_down(&self, key_code: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(key_code as i64)];
        self.disp.invoke_method("OnKeyDown", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `OnKeyUp`。
    pub fn on_key_up(&self, key_code: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(key_code as i64)];
        self.disp.invoke_method("OnKeyUp", args).ok()?.to_bool().ok()
    }

    /// 于删除（COM 方法 `OnDelete`）。
    pub fn on_delete(&self) -> Option<bool> {
        self.disp.invoke_method("OnDelete", vec![]).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `OnAbort`。
    pub fn on_abort(&self) -> bool {
        self.disp.invoke_method("OnAbort", vec![]).is_ok()
    }

    /// 于 commit（COM 方法 `OnCommit`）。
    pub fn on_commit(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnCommit", args).is_ok()
    }

    /// 调用 COM 方法 `OnTimer`。
    pub fn on_timer(&self, timer_id: i32, time_ellapsed: i32) -> bool {
        let args = vec![
            Variant::from_i64(timer_id as i64),
            Variant::from_i64(time_ellapsed as i64),
        ];
        self.disp.invoke_method("OnTimer", args).is_ok()
    }
}

// =============================================================
// IvgToolStateAttributes
// =============================================================

/// CorelDRAW `IVGToolState` 对象的后期绑定包装（工具状态属性）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgToolStateAttributes {
    disp: ComObject,
}

impl IvgToolStateAttributes {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 属性----

    /// 读取属性栏 GUID（`PropertyBarGuid`）。
    pub fn property_bar_guid(&self) -> Option<String> { self.prop_string("PropertyBarGuid") }
    /// 设置属性栏 GUID（`PropertyBarGuid`）。
    pub fn set_property_bar_guid(&self, v: impl Into<String>) -> bool {
        self.put_string("PropertyBarGuid", v)
    }

    /// 读取上下文菜单 GUID（`ContextMenuGuid`）。
    pub fn context_menu_guid(&self) -> Option<String> { self.prop_string("ContextMenuGuid") }
    /// 设置上下文菜单 GUID（`ContextMenuGuid`）。
    pub fn set_context_menu_guid(&self, v: impl Into<String>) -> bool {
        self.put_string("ContextMenuGuid", v)
    }

    /// 读取 COM 属性 `UseTabletPressure`。
    pub fn use_tablet_pressure(&self) -> Option<bool> { self.prop_bool("UseTabletPressure") }
    /// 设置 COM 属性 `UseTabletPressure`。
    pub fn set_use_tablet_pressure(&self, v: bool) -> bool {
        self.put_bool("UseTabletPressure", v)
    }

    /// 读取 COM 属性 `AllowTempPickState`。
    pub fn allow_temp_pick_state(&self) -> Option<bool> { self.prop_bool("AllowTempPickState") }
    /// 设置 COM 属性 `AllowTempPickState`。
    pub fn set_allow_temp_pick_state(&self, v: bool) -> bool {
        self.put_bool("AllowTempPickState", v)
    }

    /// 读取 COM 属性 `AllowAutopan`。
    pub fn allow_autopan(&self) -> Option<bool> { self.prop_bool("AllowAutopan") }
    /// 设置 COM 属性 `AllowAutopan`。
    pub fn set_allow_autopan(&self, v: bool) -> bool {
        self.put_bool("AllowAutopan", v)
    }

    /// 读取允许上下文菜单（`AllowContextMenu`）。
    pub fn allow_context_menu(&self) -> Option<bool> { self.prop_bool("AllowContextMenu") }
    /// 设置允许上下文菜单（`AllowContextMenu`）。
    pub fn set_allow_context_menu(&self, v: bool) -> bool {
        self.put_bool("AllowContextMenu", v)
    }

    /// 是否可以鼠标单击上的更新选择（`CanUpdateSelectionOnMouseClick`）。
    pub fn can_update_selection_on_mouse_click(&self) -> Option<bool> {
        self.prop_bool("CanUpdateSelectionOnMouseClick")
    }
    /// 设置是否可以鼠标单击上的更新选择（`CanUpdateSelectionOnMouseClick`）。
    pub fn set_can_update_selection_on_mouse_click(&self, v: bool) -> bool {
        self.put_bool("CanUpdateSelectionOnMouseClick", v)
    }

    /// 读取取消选择于左键向下（`DeselectOnLButtonDown`）。
    pub fn deselect_on_lbutton_down(&self) -> Option<bool> {
        self.prop_bool("DeselectOnLButtonDown")
    }
    /// 设置取消选择于左键向下（`DeselectOnLButtonDown`）。
    pub fn set_deselect_on_lbutton_down(&self, v: bool) -> bool {
        self.put_bool("DeselectOnLButtonDown", v)
    }

    /// 读取 COM 属性 `EnterGraceStateOnLButtonDown`。
    pub fn enter_grace_state_on_lbutton_down(&self) -> Option<bool> {
        self.prop_bool("EnterGraceStateOnLButtonDown")
    }
    /// 设置 COM 属性 `EnterGraceStateOnLButtonDown`。
    pub fn set_enter_grace_state_on_lbutton_down(&self, v: bool) -> bool {
        self.put_bool("EnterGraceStateOnLButtonDown", v)
    }

    /// 读取 COM 属性 `CurrentPressure`。
    pub fn current_pressure(&self) -> Option<f64> { self.prop_f64("CurrentPressure") }

    /// 读取文档（`Document`）。
    pub fn document(&self) -> Option<crate::document::IvgDocument> {
        self.disp
            .get_property("Document")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::document::IvgDocument::new)
    }

    // ---- 方法 ----

    /// 设置状态信息（`StatusInfo`）。
    pub fn set_status_info(&self, value: impl Into<String>) -> bool {
        self.put_string("StatusInfo", value)
    }

    /// `cdrCursorShape`
    pub fn set_cursor(&self, cursor_shape: i32) -> bool {
        let args = vec![Variant::from_i64(cursor_shape as i64)];
        self.disp.invoke_method("SetCursor", args).is_ok()
    }

    /// 设置光标 GUID（`SetCursorGuid`）。
    pub fn set_cursor_guid(&self, new_val: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(new_val.into())];
        self.disp.invoke_method("SetCursorGuid", args).is_ok()
    }

    /// 设置 COM 属性 `SetStateHintsPage`。
    pub fn set_state_hints_page(&self, new_val: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(new_val.into())];
        self.disp.invoke_method("SetStateHintsPage", args).is_ok()
    }

    /// 调用 COM 方法 `StartTimer`。
    pub fn start_timer(&self, timer_id: i32, time_to_tick: i32, one_time: bool) -> bool {
        let args = vec![
            Variant::from_i64(timer_id as i64),
            Variant::from_i64(time_to_tick as i64),
            Variant::from_bool(one_time),
        ];
        self.disp.invoke_method("StartTimer", args).is_ok()
    }

    /// 调用 COM 方法 `StopTimer`。
    pub fn stop_timer(&self, timer_id: i32) -> bool {
        let args = vec![Variant::from_i64(timer_id as i64)];
        self.disp.invoke_method("StopTimer", args).is_ok()
    }

    /// 捕捉鼠标（COM 方法 `SnapMouse`）。
    pub fn snap_mouse(&self, pt: &mut IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("SnapMouse", args).is_ok()
    }

    /// 调用 COM 方法 `AnchoredSnapMouse`。
    pub fn anchored_snap_mouse(&self, pt: &mut IvgPoint, anchor: &IvgPoint) -> bool {
        let args = vec![pt.as_variant(), anchor.as_variant()];
        self.disp.invoke_method("AnchoredSnapMouse", args).is_ok()
    }

    /// 调用 COM 方法 `ConstrainMouse`。
    pub fn constrain_mouse(&self, pt: &mut IvgPoint, anchor: &IvgPoint) -> bool {
        let args = vec![pt.as_variant(), anchor.as_variant()];
        self.disp.invoke_method("ConstrainMouse", args).is_ok()
    }

    /// 调用 COM 方法 `IsKeyDown`。
    pub fn is_key_down(&self, key_code: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(key_code as i64)];
        self.disp.invoke_method("IsKeyDown", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `ExitTemporaryToolState`。
    pub fn exit_temporary_tool_state(&self) -> bool {
        self.disp
            .invoke_method("ExitTemporaryToolState", vec![])
            .is_ok()
    }

    /// 设置 COM 属性 `SetFocus`。
    pub fn set_focus(&self) -> bool {
        self.disp.invoke_method("SetFocus", vec![]).is_ok()
    }

    /// 调用 COM 方法 `CaptureMouse`。
    pub fn capture_mouse(&self) -> bool {
        self.disp.invoke_method("CaptureMouse", vec![]).is_ok()
    }

    /// 调用 COM 方法 `ReleaseMouse`。
    pub fn release_mouse(&self) -> bool {
        self.disp.invoke_method("ReleaseMouse", vec![]).is_ok()
    }
}