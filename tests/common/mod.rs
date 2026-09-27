//! 测试公共辅助
//!
//! 所有测试文件都 `mod common;` 引入这里。

#![allow(dead_code)]

use cdrsdk::prelude::*;
use windows::Win32::System::Com::{
    CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED,
};

// =============================================================
// COM 初始化守卫
// =============================================================

/// RAII 守卫：构造时 `CoInitializeEx(STA)`，析构时 `CoUninitialize`。
pub struct ComGuard;

impl ComGuard {
    pub fn init() -> windows::core::Result<Self> {
        unsafe {
            let hr = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
            // S_OK / S_FALSE / RPC_E_CHANGED_MODE 都算可用
            if hr.is_err()
                && hr != windows::Win32::Foundation::RPC_E_CHANGED_MODE
            {
                return Err(hr.into());
            }
        }
        Ok(Self)
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

// =============================================================
// CorelDRAW 绑定
// =============================================================

/// 尝试绑定 CorelDRAW。
///
/// 顺序：
/// 1. 无版本 ProgID（`CorelDRAW.Application`）→ 默认安装版本
/// 2. 具体版本号（`CorelDRAW.Application.26` ~ `20`）
///
/// 成功时返回 `Some(app)`，失败返回 `None`（测试应该跳过而不是 fail）。
pub fn bind_app() -> Option<IvgApplication> {
    // 1. 无版本
    if let Some(app) = IvgApplication::new("") {
        eprintln!("[bind_app] via 'CorelDRAW.Application'");
        return Some(app);
    }
    // 2. 具体版本
    for v in &["26", "25", "24", "23", "22", "21", "20"] {
        if let Some(app) = IvgApplication::new(v) {
            eprintln!("[bind_app] via 'CorelDRAW.Application.{v}'");
            return Some(app);
        }
    }
    None
}

/// 绑定 CorelDRAW；失败则打印 skip 并 `return`。
///
/// 用法：
/// ```ignore
/// let app = require_app!();
/// ```
#[macro_export]
macro_rules! require_app {
    () => {
        match common::bind_app() {
            Some(a) => a,
            None => {
                eprintln!("skip: CorelDRAW not running or not installed");
                return;
            }
        }
    };
}