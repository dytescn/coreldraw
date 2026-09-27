
/// 浜嬩欢 Cookie锛坄AdviseEvents` 杩斿洖鍊硷級銆?
pub type EventCookie = i32;

/// Rust 渚у缓璁殑浜嬩欢鍥炶皟 trait锛堢敤鎴峰疄鐜帮紝SDK 璐熻矗妗ユ帴鍒?COM锛?
///
/// 瀵瑰簲 `IVGApplicationEvents` 閲岀殑鏂规硶锛?
/// ```text
/// DocumentOpen(Doc, FileName)
/// DocumentNew(Doc, FromTemplate, Template, IncludeGraphics)
/// DocumentClose(Doc)
/// DocumentBeforeSave(Doc, SaveAs, FileName)
/// DocumentAfterSave(Doc, SaveAs, FileName)
/// DocumentBeforePrint(Doc)
/// DocumentAfterPrint(Doc)
/// DocumentBeforeExport(Doc, FileName, Filter, SaveBitmap)
/// DocumentAfterExport(Doc, FileName, Filter, SaveBitmap)
/// WindowActivate(Doc, Window)
/// WindowDeactivate(Doc, Window)
/// SelectionChange()
/// Start()
/// Quit()
/// OnPluginCommand(CommandID)
/// OnUpdatePluginCommand(CommandID, Enabled, Checked)
/// OnApplicationEvent(EventName, Parameters)
/// ```
pub trait ApplicationEvents {
    fn on_start(&self) {}
    fn on_quit(&self) {}
    fn on_selection_change(&self) {}
    fn on_plugin_command(&self, _command_id: &str) {}
    fn on_application_event(&self, _event_name: &str) {}
}

// TODO: 鐢?`windows::Win32::System::Com` 瀹炵幇 `IDispatch`锛?
//       鍦?`Invoke` 閲屾牴鎹?`disp_id` 鍒嗗彂鍒?`ApplicationEvents` 鐨勬柟娉曘€?
//       鐒跺悗鍦?`IvgApplication` 涓婂姞锛?
//         pub fn advise_events(&self, sink: &impl ApplicationEvents) -> Option<EventCookie>
//         pub fn unadvise_events(&self, cookie: EventCookie) -> bool