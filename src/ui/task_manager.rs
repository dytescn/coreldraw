//! `ICUITaskManager` / `ICUITask` / `ICUIBackgroundTask` 等—— 任务管理
//!
//! ⚠️ `ICUITask` / `ICUIBackgroundTask` / `ICUIRunningTask` 是*回调接口**：
//! CorelDRAW 在 UI 线程 / 后台线程调用你。Rust 侧要真正使用需实现
//! `IDispatch` 并注册；当前只暴露触发包装。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiTaskManager
// =============================================================

/// CorelDRAW 对象的后期绑定包装（任务管理器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiTaskManager {
    disp: ComObject,
}

impl ICuiTaskManager {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 在 UI 线程运行任务。
    pub fn run_on_ui_thread(&self, task: &ICuiTask) -> bool {
        let args = vec![task.as_variant()];
        self.disp.invoke_method("RunOnUIThread", args).is_ok()
    }

    /// `cuiTaskPriority` 见 `enums::ui`。
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

/// CorelDRAW 对象的后期绑定包装（任务）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiTask {
    disp: ComObject,
}

impl ICuiTask {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 运行任务（COM 方法 `RunTask`）。
    pub fn run_task(&self) -> bool {
        self.disp.invoke_method("RunTask", vec![]).is_ok()
    }
}

// =============================================================
// ICuiBackgroundTask
// =============================================================

/// CorelDRAW 对象的后期绑定包装（背景任务）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiBackgroundTask {
    disp: ComObject,
}

impl ICuiBackgroundTask {
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

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// 调用 COM 方法 `FinalizeTask`。
    pub fn finalize_task(&self) -> bool {
        self.disp.invoke_method("FinalizeTask", vec![]).is_ok()
    }

    /// 调用 COM 方法 `FreeTask`。
    pub fn free_task(&self) -> bool {
        self.disp.invoke_method("FreeTask", vec![]).is_ok()
    }

    /// 调用 COM 方法 `QuitTask`。
    pub fn quit_task(&self) -> bool {
        self.disp.invoke_method("QuitTask", vec![]).is_ok()
    }
}

// =============================================================
// ICuiRunningTask
// =============================================================

/// CorelDRAW 对象的后期绑定包装（运行中的任务）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiRunningTask {
    disp: ComObject,
}

impl ICuiRunningTask {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 调用 COM 方法 `TryAbort`。
    pub fn try_abort(&self) -> bool {
        self.disp.invoke_method("TryAbort", vec![]).is_ok()
    }
}

// =============================================================
// ICuiRunningBackgroundTask
// =============================================================

/// CorelDRAW 对象的后期绑定包装（运行中的背景任务）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiRunningBackgroundTask {
    disp: ComObject,
}

impl ICuiRunningBackgroundTask {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 阻塞等待任务完成。
    pub fn wait_until_done(&self) -> bool {
        self.disp.invoke_method("WaitUntilDone", vec![]).is_ok()
    }

    /// `cuiTaskPriority`。
    pub fn reprioritize(&self, priority: i32) -> bool {
        let args = vec![Variant::from_i64(priority as i64)];
        self.disp.invoke_method("Reprioritize", args).is_ok()
    }

    /// 如果已完成，释放。
    pub fn finalize_if_done(&self) -> Option<bool> {
        self.disp
            .invoke_method("FinalizeIfDone", vec![])
            .ok()?
            .to_bool()
            .ok()
    }

    /// 继承自 `ICuiRunningTask`。
    pub fn try_abort(&self) -> bool {
        self.disp.invoke_method("TryAbort", vec![]).is_ok()
    }
}