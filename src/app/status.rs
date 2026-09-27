//! `IVGAppStatus` —— 应用状态 / 进度条

use wincom::{ComObject, Variant};

pub struct IvgAppStatus {
    disp: ComObject,
}

impl IvgAppStatus {
    pub fn new(disp: ComObject) -> Self {
        Self { disp }
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---- 属性 ----

    pub fn progress(&self) -> Option<i64> {
        self.disp.get_property("Progress").ok()?.to_i64().ok()
    }

    pub fn set_progress(&self, v: i32) -> bool {
        self.disp
            .set_property("Progress", vec![Variant::from_i64(v as i64)])
            .is_ok()
    }

    pub fn aborted(&self) -> Option<bool> {
        self.disp.get_property("Aborted").ok()?.to_bool().ok()
    }

    // ---- 方法 ----

    pub fn begin_progress(&self, message: impl Into<String>, can_abort: bool) -> bool {
        let args = vec![
            Variant::from_str(&message.into()),
            Variant::from_bool(can_abort),
        ];
        self.disp.invoke_method("BeginProgress", args).is_ok()
    }

    pub fn update_progress(&self, step: i32) -> bool {
        let args = vec![Variant::from_i64(step as i64)];
        self.disp.invoke_method("UpdateProgress", args).is_ok()
    }

    pub fn set_progress_message(&self, message: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(&message.into())];
        self.disp.invoke_method("SetProgressMessage", args).is_ok()
    }

    pub fn end_progress(&self) -> bool {
        self.disp.invoke_method("EndProgress", vec![]).is_ok()
    }
}