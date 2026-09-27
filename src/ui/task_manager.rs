//! `ICUITaskManager` / `ICUITask` / `ICUIBackgroundTask` 绛?鈥斺€?浠诲姟绠＄悊
//!
//! 鈿狅笍 `ICUITask` / `ICUIBackgroundTask` / `ICUIRunningTask` 鏄?*鍥炶皟鎺ュ彛**锛?
//! CorelDRAW 鍦?UI 绾跨▼ / 鍚庡彴绾跨▼璋冪敤浣犮€俁ust 渚ц鐪熸浣跨敤闇€瀹炵幇
//! `IDispatch` 骞舵敞鍐岋紱褰撳墠鍙毚闇茶Е鍙戝寘瑁呫€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiTaskManager
// =============================================================

pub struct ICuiTaskManager {
    disp: ComObject,
}

impl ICuiTaskManager {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 鍦?UI 绾跨▼杩愯浠诲姟銆?
    pub fn run_on_ui_thread(&self, task: &ICuiTask) -> bool {
        let args = vec![task.as_variant()];
        self.disp.invoke_method("RunOnUIThread", args).is_ok()
    }

    /// `cuiTaskPriority` 瑙?`enums::ui`銆?
    pub fn run_in_background(
        &self,
        priority: i32,
        task: &ICuiBackgroundTask,
    ) -> Option<ICuiRunningBackgroundTask> {
        let args = vec![
            Variant::from_i64(priority as i64),
            task.as_variant(),
        ];
        self.disp
            .invoke_method("RunInBackground", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiRunningBackgroundTask::new)
    }
}

// =============================================================
// ICuiTask
// =============================================================

pub struct ICuiTask {
    disp: ComObject,
}

impl ICuiTask {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn run_task(&self) -> bool {
        self.disp.invoke_method("RunTask", vec![]).is_ok()
    }
}

// =============================================================
// ICuiBackgroundTask
// =============================================================

pub struct ICuiBackgroundTask {
    disp: ComObject,
}

impl ICuiBackgroundTask {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    pub fn finalize_task(&self) -> bool {
        self.disp.invoke_method("FinalizeTask", vec![]).is_ok()
    }

    pub fn free_task(&self) -> bool {
        self.disp.invoke_method("FreeTask", vec![]).is_ok()
    }

    pub fn quit_task(&self) -> bool {
        self.disp.invoke_method("QuitTask", vec![]).is_ok()
    }
}

// =============================================================
// ICuiRunningTask
// =============================================================

pub struct ICuiRunningTask {
    disp: ComObject,
}

impl ICuiRunningTask {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn try_abort(&self) -> bool {
        self.disp.invoke_method("TryAbort", vec![]).is_ok()
    }
}

// =============================================================
// ICuiRunningBackgroundTask
// =============================================================

pub struct ICuiRunningBackgroundTask {
    disp: ComObject,
}

impl ICuiRunningBackgroundTask {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 闃诲绛夊緟浠诲姟瀹屾垚銆?
    pub fn wait_until_done(&self) -> bool {
        self.disp.invoke_method("WaitUntilDone", vec![]).is_ok()
    }

    /// `cuiTaskPriority`銆?
    pub fn reprioritize(&self, priority: i32) -> bool {
        let args = vec![Variant::from_i64(priority as i64)];
        self.disp.invoke_method("Reprioritize", args).is_ok()
    }

    /// 濡傛灉宸插畬鎴愶紝閲婃斁銆?
    pub fn finalize_if_done(&self) -> Option<bool> {
        self.disp
            .invoke_method("FinalizeIfDone", vec![])
            .ok()?
            .to_bool()
            .ok()
    }

    /// 缁ф壙鑷?`ICuiRunningTask`銆?
    pub fn try_abort(&self) -> bool {
        self.disp.invoke_method("TryAbort", vec![]).is_ok()
    }
}