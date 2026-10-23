//! libary 插件的 JSON 输入/输出约定。
//!
//! - 输入：`const char *json_in`，可能为 `NULL`、空串或非法 JSON。
//! - 输出：64 KB 缓冲区（与 cyctron `PLUGIN_JSON_BUF_SIZE` 一致），
//!   必须写入以 `\0` 结尾的紧凑 JSON。
//! - 返回值：始终返回 0（业务错误放在 JSON 的 `ok:false` 里），
//!   只有输出指针为空时才返回 -1。

use serde_json::{json, Value};
use std::ffi::c_char;
use std::os::raw::c_int;

pub const OUT_BUF_SIZE: usize = 65536;

/// 业务错误。`code` 供 TS 测试断言，`message` 供人阅读。
#[derive(Debug)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.into(),
        }
    }
}

pub type ApiResult = Result<Value, ApiError>;

pub fn missing(key: &str) -> ApiError {
    ApiError::new("missing_param", format!("missing required parameter: {key}"))
}

pub fn invalid(key: &str, expect: &str) -> ApiError {
    ApiError::new(
        "invalid_param",
        format!("parameter {key} must be {expect}"),
    )
}

// ---------------------------------------------------------------
// 输入
// ---------------------------------------------------------------

/// 读取输入 JSON；`NULL` / 空串 / 非法 JSON 一律视为 `Value::Null`。
pub fn read_input(json_in: *const c_char) -> Value {
    if json_in.is_null() {
        return Value::Null;
    }
    let s = unsafe { std::ffi::CStr::from_ptr(json_in) }.to_string_lossy();
    let s = s.trim();
    if s.is_empty() {
        return Value::Null;
    }
    serde_json::from_str(s).unwrap_or(Value::Null)
}

// ---------------------------------------------------------------
// 参数提取
// ---------------------------------------------------------------

pub fn get_str(input: &Value, key: &str) -> Result<String, ApiError> {
    match input.get(key).and_then(|v| v.as_str()) {
        Some(s) => Ok(s.to_string()),
        None => Err(missing(key)),
    }
}

pub fn opt_str(input: &Value, key: &str) -> Option<String> {
    input.get(key).and_then(|v| v.as_str()).map(|s| s.to_string())
}

pub fn opt_i64(input: &Value, key: &str) -> Option<i64> {
    input.get(key).and_then(|v| match v {
        Value::Number(n) => n.as_i64(),
        Value::String(s) => s.parse::<i64>().ok(),
        _ => None,
    })
}

pub fn get_i32(input: &Value, key: &str) -> Result<i32, ApiError> {
    opt_i64(input, key)
        .map(|v| v as i32)
        .ok_or_else(|| missing(key))
}

pub fn opt_i32(input: &Value, key: &str) -> Option<i32> {
    opt_i64(input, key).map(|v| v as i32)
}

pub fn get_f64(input: &Value, key: &str) -> Result<f64, ApiError> {
    match input.get(key) {
        Some(Value::Number(n)) => n.as_f64().ok_or_else(|| invalid(key, "a number")),
        Some(Value::String(s)) => s
            .parse::<f64>()
            .map_err(|_| invalid(key, "a number")),
        _ => Err(missing(key)),
    }
}

pub fn opt_f64(input: &Value, key: &str) -> Option<f64> {
    match input.get(key) {
        Some(Value::Number(n)) => n.as_f64(),
        Some(Value::String(s)) => s.parse::<f64>().ok(),
        _ => None,
    }
}

pub fn opt_bool(input: &Value, key: &str) -> Option<bool> {
    input.get(key).and_then(|v| v.as_bool())
}

// ---------------------------------------------------------------
// 输出
// ---------------------------------------------------------------

fn write_str(json_out: *mut c_char, s: &str) -> c_int {
    if json_out.is_null() {
        return -1;
    }
    let bytes = s.as_bytes();
    let n = bytes.len().min(OUT_BUF_SIZE - 1);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr(), json_out as *mut u8, n);
        *json_out.add(n) = 0;
    }
    0
}

/// 输出统一信封：
///
/// 成功 `{"ok":true,"method":"POST","interface":"app_info","data":{...}}`；
/// 失败 `{"ok":false,"method":"POST","interface":"app_info","error":"code","message":"..."}`。
pub fn write_result(json_out: *mut c_char, interface: &str, result: ApiResult) -> c_int {
    let payload = match result {
        Ok(data) => json!({
            "ok": true,
            "method": "POST",
            "interface": interface,
            "data": data,
        }),
        Err(e) => json!({
            "ok": false,
            "method": "POST",
            "interface": interface,
            "error": e.code,
            "message": e.message,
        }),
    };
    write_str(json_out, &payload.to_string())
}
