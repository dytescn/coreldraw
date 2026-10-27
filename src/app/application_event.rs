
/// 事件 Cookie（`AdviseEvents` 返回值）。
pub type EventCookie = i32;

/// Rust 侧建议的事件回调 trait（用户实现，SDK 负责桥接到 COM）
///
/// 对应 `IVGApplicationEvents` 里的方法：
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

// TODO: 由 `windows::Win32::System::Com` 实现 `IDispatch`：
//       在 `Invoke` 里根据 `disp_id` 分发到 `ApplicationEvents` 的方法。
//       然后场 `IvgApplication` 上加：
//         pub fn advise_events(&self, sink: &impl ApplicationEvents) -> Option<EventCookie>
//         pub fn unadvise_events(&self, cookie: EventCookie) -> bool