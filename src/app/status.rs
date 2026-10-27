//! `IVGAppStatus` —— 应用状态 / 进度条

use wincom::{ComObject, Variant};

/// CorelDRAW `IVGAppStatus` 对象的后期绑定包装（应用状态）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgAppStatus {
    disp: ComObject,
}

impl IvgAppStatus {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: ComObject) -> Self {
        Self { disp }
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---- 属性 ----

    /// 读取进度（`Progress`）。
    pub fn progress(&self) -> Option<i64> {
        self.disp.get_property("Progress").ok()?.to_i64().ok()
    }

    /// 设置进度（`Progress`）。
    pub fn set_progress(&self, v: i32) -> bool {
        self.disp
            .set_property("Progress", vec![Variant::from_i64(v as i64)])
            .is_ok()
    }

    /// 读取 COM 属性 `Aborted`。
    pub fn aborted(&self) -> Option<bool> {
        self.disp.get_property("Aborted").ok()?.to_bool().ok()
    }

    // ---- 方法 ----

    /// 开始进度（COM 方法 `BeginProgress`）。
    pub fn begin_progress(&self, message: impl Into<String>, can_abort: bool) -> bool {
        let args = vec![
            Variant::from_str(&message.into()),
            Variant::from_bool(can_abort),
        ];
        self.disp.invoke_method("BeginProgress", args).is_ok()
    }

    /// 更新进度（COM 方法 `UpdateProgress`）。
    pub fn update_progress(&self, step: i32) -> bool {
        let args = vec![Variant::from_i64(step as i64)];
        self.disp.invoke_method("UpdateProgress", args).is_ok()
    }

    /// 设置进度消息（`SetProgressMessage`）。
    pub fn set_progress_message(&self, message: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(&message.into())];
        self.disp.invoke_method("SetProgressMessage", args).is_ok()
    }

    /// 结束进度（COM 方法 `EndProgress`）。
    pub fn end_progress(&self) -> bool {
        self.disp.invoke_method("EndProgress", vec![]).is_ok()
    }
}