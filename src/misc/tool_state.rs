//! `IVGToolState` / `IVGToolStateAttributes` 鈥斺€?宸ュ叿鐘舵€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgPoint;

// =============================================================
// IvgToolState
// =============================================================

/// 宸ュ叿鐘舵€佹帴鍙ｃ€?
///
/// 鈿狅笍 杩欐槸涓€涓?*鍥炶皟鎺ュ彛**锛孋orelDRAW 浼氬湪榧犳爣浜嬩欢鏃惰皟浣犮€?
/// 瑕佸湪 Rust 渚х湡姝ｄ娇鐢紝闇€瑕佸疄鐜?`IDispatch` 骞舵敞鍐屻€?
/// 褰撳墠鍙彁渚?`On*` 瑙﹀彂鍖呰銆?
pub struct IvgToolState {
    disp: ComObject,
}

impl IvgToolState {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn is_drawing(&self) -> Option<bool> {
        self.disp.get_property("IsDrawing").ok()?.to_bool().ok()
    }

    pub fn on_start_state(&self, attrs: &IvgToolStateAttributes) -> bool {
        let args = vec![attrs.as_variant()];
        self.disp.invoke_method("OnStartState", args).is_ok()
    }

    pub fn on_exit_state(&self) -> bool {
        self.disp.invoke_method("OnExitState", vec![]).is_ok()
    }

    pub fn on_mouse_move(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnMouseMove", args).is_ok()
    }

    pub fn on_lbutton_down(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnLButtonDown", args).is_ok()
    }

    pub fn on_lbutton_up(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnLButtonUp", args).is_ok()
    }

    pub fn on_lbutton_dbl_click(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnLButtonDblClick", args).is_ok()
    }

    pub fn on_click(&self, pt: &IvgPoint) -> Option<bool> {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnClick", args).ok()?.to_bool().ok()
    }

    pub fn on_rbutton_down(&self, pt: &IvgPoint) -> Option<bool> {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnRButtonDown", args).ok()?.to_bool().ok()
    }

    pub fn on_rbutton_up(&self, pt: &IvgPoint) -> Option<bool> {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnRButtonUp", args).ok()?.to_bool().ok()
    }

    pub fn on_key_down(&self, key_code: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(key_code as i64)];
        self.disp.invoke_method("OnKeyDown", args).ok()?.to_bool().ok()
    }

    pub fn on_key_up(&self, key_code: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(key_code as i64)];
        self.disp.invoke_method("OnKeyUp", args).ok()?.to_bool().ok()
    }

    pub fn on_delete(&self) -> Option<bool> {
        self.disp.invoke_method("OnDelete", vec![]).ok()?.to_bool().ok()
    }

    pub fn on_abort(&self) -> bool {
        self.disp.invoke_method("OnAbort", vec![]).is_ok()
    }

    pub fn on_commit(&self, pt: &IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("OnCommit", args).is_ok()
    }

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

pub struct IvgToolStateAttributes {
    disp: ComObject,
}

impl IvgToolStateAttributes {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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

    // ---- 灞炴€?----

    pub fn property_bar_guid(&self) -> Option<String> { self.prop_string("PropertyBarGuid") }
    pub fn set_property_bar_guid(&self, v: impl Into<String>) -> bool {
        self.put_string("PropertyBarGuid", v)
    }

    pub fn context_menu_guid(&self) -> Option<String> { self.prop_string("ContextMenuGuid") }
    pub fn set_context_menu_guid(&self, v: impl Into<String>) -> bool {
        self.put_string("ContextMenuGuid", v)
    }

    pub fn use_tablet_pressure(&self) -> Option<bool> { self.prop_bool("UseTabletPressure") }
    pub fn set_use_tablet_pressure(&self, v: bool) -> bool {
        self.put_bool("UseTabletPressure", v)
    }

    pub fn allow_temp_pick_state(&self) -> Option<bool> { self.prop_bool("AllowTempPickState") }
    pub fn set_allow_temp_pick_state(&self, v: bool) -> bool {
        self.put_bool("AllowTempPickState", v)
    }

    pub fn allow_autopan(&self) -> Option<bool> { self.prop_bool("AllowAutopan") }
    pub fn set_allow_autopan(&self, v: bool) -> bool {
        self.put_bool("AllowAutopan", v)
    }

    pub fn allow_context_menu(&self) -> Option<bool> { self.prop_bool("AllowContextMenu") }
    pub fn set_allow_context_menu(&self, v: bool) -> bool {
        self.put_bool("AllowContextMenu", v)
    }

    pub fn can_update_selection_on_mouse_click(&self) -> Option<bool> {
        self.prop_bool("CanUpdateSelectionOnMouseClick")
    }
    pub fn set_can_update_selection_on_mouse_click(&self, v: bool) -> bool {
        self.put_bool("CanUpdateSelectionOnMouseClick", v)
    }

    pub fn deselect_on_lbutton_down(&self) -> Option<bool> {
        self.prop_bool("DeselectOnLButtonDown")
    }
    pub fn set_deselect_on_lbutton_down(&self, v: bool) -> bool {
        self.put_bool("DeselectOnLButtonDown", v)
    }

    pub fn enter_grace_state_on_lbutton_down(&self) -> Option<bool> {
        self.prop_bool("EnterGraceStateOnLButtonDown")
    }
    pub fn set_enter_grace_state_on_lbutton_down(&self, v: bool) -> bool {
        self.put_bool("EnterGraceStateOnLButtonDown", v)
    }

    pub fn current_pressure(&self) -> Option<f64> { self.prop_f64("CurrentPressure") }

    pub fn document(&self) -> Option<crate::document::IvgDocument> {
        self.disp
            .get_property("Document")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::document::IvgDocument::new)
    }

    // ---- 鏂规硶 ----

    pub fn set_status_info(&self, value: impl Into<String>) -> bool {
        self.put_string("StatusInfo", value)
    }

    /// `cdrCursorShape`
    pub fn set_cursor(&self, cursor_shape: i32) -> bool {
        let args = vec![Variant::from_i64(cursor_shape as i64)];
        self.disp.invoke_method("SetCursor", args).is_ok()
    }

    pub fn set_cursor_guid(&self, new_val: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(new_val.into())];
        self.disp.invoke_method("SetCursorGuid", args).is_ok()
    }

    pub fn set_state_hints_page(&self, new_val: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(new_val.into())];
        self.disp.invoke_method("SetStateHintsPage", args).is_ok()
    }

    pub fn start_timer(&self, timer_id: i32, time_to_tick: i32, one_time: bool) -> bool {
        let args = vec![
            Variant::from_i64(timer_id as i64),
            Variant::from_i64(time_to_tick as i64),
            Variant::from_bool(one_time),
        ];
        self.disp.invoke_method("StartTimer", args).is_ok()
    }

    pub fn stop_timer(&self, timer_id: i32) -> bool {
        let args = vec![Variant::from_i64(timer_id as i64)];
        self.disp.invoke_method("StopTimer", args).is_ok()
    }

    pub fn snap_mouse(&self, pt: &mut IvgPoint) -> bool {
        let args = vec![pt.as_variant()];
        self.disp.invoke_method("SnapMouse", args).is_ok()
    }

    pub fn anchored_snap_mouse(&self, pt: &mut IvgPoint, anchor: &IvgPoint) -> bool {
        let args = vec![pt.as_variant(), anchor.as_variant()];
        self.disp.invoke_method("AnchoredSnapMouse", args).is_ok()
    }

    pub fn constrain_mouse(&self, pt: &mut IvgPoint, anchor: &IvgPoint) -> bool {
        let args = vec![pt.as_variant(), anchor.as_variant()];
        self.disp.invoke_method("ConstrainMouse", args).is_ok()
    }

    pub fn is_key_down(&self, key_code: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(key_code as i64)];
        self.disp.invoke_method("IsKeyDown", args).ok()?.to_bool().ok()
    }

    pub fn exit_temporary_tool_state(&self) -> bool {
        self.disp
            .invoke_method("ExitTemporaryToolState", vec![])
            .is_ok()
    }

    pub fn set_focus(&self) -> bool {
        self.disp.invoke_method("SetFocus", vec![]).is_ok()
    }

    pub fn capture_mouse(&self) -> bool {
        self.disp.invoke_method("CaptureMouse", vec![]).is_ok()
    }

    pub fn release_mouse(&self) -> bool {
        self.disp.invoke_method("ReleaseMouse", vec![]).is_ok()
    }
}