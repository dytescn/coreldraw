//! `ICorelScriptTools` 鈥斺€?鑴氭湰宸ュ叿锛圓pplication.CorelScriptTools锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct ICorelScriptTools {
    disp: ComObject,
}

impl ICorelScriptTools {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    // ---- 鍗曚綅杞崲 ----

    pub fn length_convert(&self, from: i32, to: i32, value: f64) -> Option<f64> {
        let args = vec![
            Variant::from_i64(from as i64),
            Variant::from_i64(to as i64),
            Variant::from_f64(value),
        ];
        self.disp.invoke_method("LengthConvert", args).ok()?.to_f64().ok()
    }

    pub fn angle_convert(&self, from: i32, to: i32, value: f64) -> Option<f64> {
        let args = vec![
            Variant::from_i64(from as i64),
            Variant::from_i64(to as i64),
            Variant::from_f64(value),
        ];
        self.disp.invoke_method("AngleConvert", args).ok()?.to_f64().ok()
    }

    // ---- 鏁板 ----

    pub fn asin(&self, value: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(value)];
        self.disp.invoke_method("ASin", args).ok()?.to_f64().ok()
    }

    pub fn log(&self, value: f64) -> Option<f64> {
        let args = vec![Variant::from_f64(value)];
        self.disp.invoke_method("Log", args).ok()?.to_f64().ok()
    }

    pub fn build_date(&self, year: i32, month: i32, day: i32) -> Option<f64> {
        let args = vec![
            Variant::from_i64(year as i64),
            Variant::from_i64(month as i64),
            Variant::from_i64(day as i64),
        ];
        let _ = args;
        None
    }

    pub fn format_time(&self, time: f64, format: impl Into<String>) -> Option<String> {
        let _ = (time, format.into());
        None
    }

    pub fn file_attr(&self, folder_file: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(folder_file.into())];
        self.disp.invoke_method("FileAttr", args).ok()?.to_i64().ok()
    }

    pub fn get_temp_folder(&self) -> Option<String> {
        self.disp.invoke_method("GetTempFolder", vec![]).ok()?.to_string().ok()
    }

    pub fn get_curr_folder(&self) -> Option<String> {
        self.disp.invoke_method("GetCurrFolder", vec![]).ok()?.to_string().ok()
    }

    pub fn get_script_folder(&self) -> Option<String> {
        self.disp.invoke_method("GetScriptFolder", vec![]).ok()?.to_string().ok()
    }

    pub fn mk_folder(&self, folder: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(folder.into())];
        self.disp.invoke_method("MkFolder", args).ok()?.to_bool().ok()
    }

    pub fn rm_folder(&self, folder: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(folder.into())];
        self.disp.invoke_method("RmFolder", args).ok()?.to_bool().ok()
    }

    pub fn kill(&self, file_name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file_name.into())];
        self.disp.invoke_method("Kill", args).ok()?.to_bool().ok()
    }

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

    // ---- 瀵硅瘽妗?----

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

    // ---- 绯荤粺 ----

    pub fn get_app_handle(&self) -> Option<i64> {
        self.disp.invoke_method("GetAppHandle", vec![]).ok()?.to_i64().ok()
    }

    pub fn get_win_handle(&self) -> Option<i64> {
        self.disp.invoke_method("GetWinHandle", vec![]).ok()?.to_i64().ok()
    }

    pub fn get_command_line(&self) -> Option<String> {
        self.disp.invoke_method("GetCommandLine", vec![]).ok()?.to_string().ok()
    }

    pub fn get_version(&self, option: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(option as i64)];
        self.disp.invoke_method("GetVersion", args).ok()?.to_i64().ok()
    }

    // ---- 鍏跺畠 ----

    pub fn begin_wait_cursor(&self) -> bool {
        self.disp.invoke_method("BeginWaitCursor", vec![]).is_ok()
    }

    pub fn end_wait_cursor(&self) -> bool {
        self.disp.invoke_method("EndWaitCursor", vec![]).is_ok()
    }

    pub fn dec(&self, hex: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(hex.into())];
        self.disp.invoke_method("Dec", args).ok()?.to_i64().ok()
    }

    // ---- 鍗曚綅鎹㈢畻鎵归噺 ----

    pub fn from_centimeters(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 4, v)
    }
    pub fn to_centimeters(&self, v: f64) -> Option<f64> {
        self.length_convert(4, 0, v)
    }
    pub fn from_inches(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 0, v)
    }
    pub fn to_inches(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 0, v)
    }
    pub fn from_points(&self, v: f64) -> Option<f64> {
        self.length_convert(0, 3, v)
    }
    pub fn to_points(&self, v: f64) -> Option<f64> {
        self.length_convert(3, 0, v)
    }
}