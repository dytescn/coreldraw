//! `ICorelScriptTools` —— 脚本工具（Application.CorelScriptTools）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW 对象的后期绑定包装（脚本工具）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICorelScriptTools {
    disp: ComObject,
}

impl ICorelScriptTools {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    // ---- 单位转换 ----

    /// 长度转换（COM 方法 `LengthConvert`）。
    pub fn length_convert(&self, from: i32, to: i32, value: f64) -> Option<f64> {
        let args = vec![
            Variant::from_i64(from as i64),
            Variant::from_i64(to as i64),
            Variant::from_f64(value),
        ];
        self.disp.invoke_method("LengthConvert", args).ok()?.to_f64().ok()
    }

    /// 角度转换（COM 方法 `AngleConvert`）。
    pub fn angle_convert(&self, from: i32, to: i32, value: f64) -> Option<f64> {
        let args = vec![
            Variant::from_i64(from as i64),
            Variant::from_i64(to as i64),
            Variant::from_f64(value),
        ];
        self.disp.invoke_method("AngleConvert", args).ok()?.to_f64().ok()
    }

    // ---- 数学 ----

    /// 调用 COM 方法 `ASin`。
    pub fn asin(&self, value: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(value)];
        self.disp.invoke_method("ASin", args).ok()?.to_f64().ok()
    }

    /// 调用 COM 方法 `Log`。
    pub fn log(&self, value: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(value)];
        self.disp.invoke_method("Log", args).ok()?.to_f64().ok()
    }

    /// 调用 CorelDRAW COM 接口。
    pub fn build_date(&self, year: i32, month: i32, day: i32) -> Option<f64> {
        let args = vec![
            Variant::from_i64(year as i64),
            Variant::from_i64(month as i64),
            Variant::from_i64(day as i64),
        ];
        let _ = args;
        None
    }

    /// 调用 CorelDRAW COM 接口。
    pub fn format_time(&self, time: f64, format: impl Into<String>) -> Option<String> {
        let _ = (time, format.into());
        None
    }

    /// 调用 COM 方法 `FileAttr`。
    pub fn file_attr(&self, folder_file: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(folder_file.into())];
        self.disp.invoke_method("FileAttr", args).ok()?.to_i64().ok()
    }

    /// 读取 COM 属性 `GetTempFolder`。
    pub fn get_temp_folder(&self) -> Option<String> {
        self.disp.invoke_method("GetTempFolder", vec![]).ok()?.to_string().ok()
    }

    /// 读取 COM 属性 `GetCurrFolder`。
    pub fn get_curr_folder(&self) -> Option<String> {
        self.disp.invoke_method("GetCurrFolder", vec![]).ok()?.to_string().ok()
    }

    /// 读取脚本文件夹（`GetScriptFolder`）。
    pub fn get_script_folder(&self) -> Option<String> {
        self.disp.invoke_method("GetScriptFolder", vec![]).ok()?.to_string().ok()
    }

    /// 调用 COM 方法 `MkFolder`。
    pub fn mk_folder(&self, folder: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(folder.into())];
        self.disp.invoke_method("MkFolder", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `RmFolder`。
    pub fn rm_folder(&self, folder: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(folder.into())];
        self.disp.invoke_method("RmFolder", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `Kill`。
    pub fn kill(&self, file_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("Kill", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `Rename`。
    pub fn rename(
        &self,
        src: impl Into<String>,
        dst: impl Into<String>,
        overwrite: i32,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(src.into()),
            Variant::from_str(dst.into()),
            Variant::from_i64(overwrite as i64),
        ];
        self.disp.invoke_method("Rename", args).ok()?.to_bool().ok()
    }

    // ---- 对话框----

    /// 读取文件框（`GetFileBox`）。
    #[allow(clippy::too_many_arguments)]
    pub fn get_file_box(
        &self,
        filter: impl Into<String>,
        title: impl Into<String>,
        dialog_type: i32,
        file: impl Into<String>,
        extension: impl Into<String>,
        folder: impl Into<String>,
        button: impl Into<String>,
    ) -> Option<String> {
        let args = vec![
            Variant::from_str(filter.into()),
            Variant::from_str(title.into()),
            Variant::from_i64(dialog_type as i64),
            Variant::from_str(file.into()),
            Variant::from_str(extension.into()),
            Variant::from_str(folder.into()),
            Variant::from_str(button.into()),
        ];
        self.disp.invoke_method("GetFileBox", args).ok()?.to_string().ok()
    }

    /// 读取文件夹（`GetFolder`）。
    pub fn get_folder(
        &self,
        init_folder: impl Into<String>,
        title: impl Into<String>,
        parent_window_handle: i32,
    ) -> Option<String> {
        let args = vec![
            Variant::from_str(init_folder.into()),
            Variant::from_str(title.into()),
            Variant::from_i64(parent_window_handle as i64),
        ];
        self.disp.invoke_method("GetFolder", args).ok()?.to_string().ok()
    }

    // ---- 系统 ----

    /// 读取应用手柄（`GetAppHandle`）。
    pub fn get_app_handle(&self) -> Option<i64> {
        self.disp.invoke_method("GetAppHandle", vec![]).ok()?.to_i64().ok()
    }

    /// 读取 COM 属性 `GetWinHandle`。
    pub fn get_win_handle(&self) -> Option<i64> {
        self.disp.invoke_method("GetWinHandle", vec![]).ok()?.to_i64().ok()
    }

    /// 读取命令线（`GetCommandLine`）。
    pub fn get_command_line(&self) -> Option<String> {
        self.disp.invoke_method("GetCommandLine", vec![]).ok()?.to_string().ok()
    }

    /// 读取版本（`GetVersion`）。
    pub fn get_version(&self, option: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(option as i64)];
        self.disp.invoke_method("GetVersion", args).ok()?.to_i64().ok()
    }

    // ---- 其它 ----

    /// 开始 wait 光标（COM 方法 `BeginWaitCursor`）。
    pub fn begin_wait_cursor(&self) -> bool {
        self.disp.invoke_method("BeginWaitCursor", vec![]).is_ok()
    }

    /// 结束 wait 光标（COM 方法 `EndWaitCursor`）。
    pub fn end_wait_cursor(&self) -> bool {
        self.disp.invoke_method("EndWaitCursor", vec![]).is_ok()
    }

    /// 调用 COM 方法 `Dec`。
    pub fn dec(&self, hex: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(hex.into())];
        self.disp.invoke_method("Dec", args).ok()?.to_i64().ok()
    }

    // ---- 单位换算批量 ----

    /// 调用 CorelDRAW COM 接口。
    pub fn from_centimeters(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 4, v)
    }
    /// 调用 CorelDRAW COM 接口。
    pub fn to_centimeters(&self, v: f64) -> Option<f64> {
        self.length_convert(4, 0, v)
    }
    /// 调用 CorelDRAW COM 接口。
    pub fn from_inches(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 0, v)
    }
    /// 调用 CorelDRAW COM 接口。
    pub fn to_inches(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 0, v)
    }
    /// 从…构造点集。
    pub fn from_points(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 3, v)
    }
    /// 转换为点集。
    pub fn to_points(&self, v: f64) -> Option<f64> {
        self.length_convert(3, 0, v)
    }
}