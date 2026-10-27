pub type EventCookie = i32;

/// Rust 侧文档事件回调 trait。
///
/// 用户实现这个 trait，SDK 通过 `IDispatch` 桥接到 COM。
pub trait DocumentEvents {
    fn on_open(&self) {}
    fn on_close(&self) {}
    fn on_before_save(&self, _save_as: bool, _file_name: &str) {}
    fn on_after_save(&self, _save_as: bool, _file_name: &str) {}
    fn on_before_print(&self) {}
    fn on_after_print(&self) {}
    fn on_before_export(&self, _file_name: &str, _filter: i32, _save_bitmap: bool) {}
    fn on_after_export(&self, _file_name: &str, _filter: i32, _save_bitmap: bool) {}
    fn on_selection_change(&self) {}
    fn on_layer_create(&self) {}
    fn on_layer_delete(&self, _count: i32) {}
    fn on_layer_activate(&self) {}
    fn on_layer_change(&self) {}
    fn on_page_create(&self) {}
    fn on_page_delete(&self, _count: i32) {}
    fn on_page_activate(&self) {}
    fn on_page_change(&self) {}
    fn on_shape_create(&self) {}
    fn on_shape_delete(&self, _count: i32) {}
    fn on_shape_move(&self) {}
    fn on_shape_transform(&self) {}
    fn on_shape_distort(&self) {}
    fn on_shape_change(&self, _scope: i32) {}
    fn on_query_close(&self) -> bool { true }
    fn on_query_save(&self) -> bool { true }
    fn on_query_print(&self) -> bool { true }
    fn on_query_export(&self) -> bool { true }
}

// TODO: 实现 `IDispatch` 桥接。
//       `Invoke` 里根据 `disp_id` 分发到上面的回调。
//       然后场 `IvgDocument` 上加：
//         pub fn advise_events(&self, sink: &impl DocumentEvents) -> Option<EventCookie>
//         pub fn unadvise_events(&self, cookie: EventCookie) -> bool