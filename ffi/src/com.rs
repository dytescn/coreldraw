//! COM 初始化与 CorelDRAW 绑定。
//!
//! cyctron 的 libffi 调用是同步的、固定在主线程上执行的，因此这里用
//! `thread_local!` 缓存 COM 初始化状态和 `IvgApplication`：
//!
//! - 首次调用 `CoInitializeEx(STA)`，之后复用；
//! - 已绑定的 CorelDRAW 实例跨请求复用，避免每次都重新激活 COM 服务器；
//! - 缓存对象失效（CorelDRAW 被关闭）时自动重新绑定。

use std::cell::{Cell, RefCell};

use cdrsdk::prelude::IvgApplication;
use windows::Win32::Foundation::RPC_E_CHANGED_MODE;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};

use crate::json::{ApiError, ApiResult};

thread_local! {
    static COM_READY: Cell<bool> = const { Cell::new(false) };
    static APP: RefCell<Option<IvgApplication>> = const { RefCell::new(None) };
}

/// 主线程只需初始化一次；MTA 线程（`RPC_E_CHANGED_MODE`）也算可用。
fn ensure_com() -> Result<(), ApiError> {
    COM_READY.with(|ready| {
        if ready.get() {
            return Ok(());
        }
        unsafe {
            let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            if hr.is_err() && hr != RPC_E_CHANGED_MODE {
                return Err(ApiError::new(
                    "com_init_failed",
                    format!("CoInitializeEx failed: {hr:?}"),
                ));
            }
        }
        ready.set(true);
        Ok(())
    })
}

/// 绑定 CorelDRAW：先试默认 ProgID，再按版本号 26..20 依次尝试。
pub fn bind_app() -> Option<IvgApplication> {
    if let Some(app) = IvgApplication::new("") {
        return Some(app);
    }
    for version in ["26", "25", "24", "23", "22", "21", "20"] {
        if let Some(app) = IvgApplication::new(version) {
            return Some(app);
        }
    }
    None
}

/// 在当前线程的 CorelDRAW 实例上执行一段逻辑。
pub fn with_app(f: impl FnOnce(&IvgApplication) -> ApiResult) -> ApiResult {
    ensure_com()?;

    APP.with(|slot| {
        let alive = slot
            .borrow()
            .as_ref()
            .map(|app| app.version().is_some())
            .unwrap_or(false);

        if !alive {
            let app = bind_app().ok_or_else(|| {
                ApiError::new(
                    "coreldraw_unavailable",
                    "CorelDRAW is not running and could not be started",
                )
            })?;
            *slot.borrow_mut() = Some(app);
        }

        let guard = slot.borrow();
        let app = guard.as_ref().expect("app bound above");
        f(app)
    })
}
