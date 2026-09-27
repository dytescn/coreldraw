//! `IVGPostScriptFill` 鈥斺€?PostScript 濉厖
//! `IVGPSScreenOptions` 鈥斺€?PostScript 灞忓箷閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPostScriptFill
// =============================================================

pub struct IvgPostScriptFill {
    disp: ComObject,
}

impl IvgPostScriptFill {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    /// PostScript 濉厖鍚嶃€?
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// PostScript 濉厖绱㈠紩銆?
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 璇?鍐欏崟涓睘鎬э紙0..4 鎴?0..N锛夈€?
    pub fn get_property_at(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Properties", args).ok()?.to_i64().ok()
    }

    pub fn set_property_at(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_Properties", args).is_ok()
    }

    /// 閫夋嫨 PostScript 濉厖銆?
    pub fn select(&self, index_or_name: Variant) -> bool {
        let args = vec![index_or_name];
        self.disp.invoke_method("Select", args).is_ok()
    }

    /// 涓€娆¤缃?5 涓睘鎬с€?
    pub fn set_properties(
        &self,
        p1: i32,
        p2: i32,
        p3: i32,
        p4: i32,
        p5: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(p1 as i64),
            Variant::from_i64(p2 as i64),
            Variant::from_i64(p3 as i64),
            Variant::from_i64(p4 as i64),
            Variant::from_i64(p5 as i64),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }
}

// =============================================================
// IvgPSScreenOptions
// =============================================================

pub struct IvgPSScreenOptions {
    disp: ComObject,
}

impl IvgPSScreenOptions {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    pub fn frequency(&self) -> Option<i64> { self.prop_i64("Frequency") }
    pub fn set_frequency(&self, v: i32) -> bool { self.put_i64("Frequency", v as i64) }

    /// 鎸夊悕绉伴€夋嫨銆?
    pub fn select(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("Select", args).ok()?.to_bool().ok()
    }

    /// 涓€娆℃€ц缃弬鏁般€?
    pub fn set_properties(
        &self,
        index_or_name: Variant,
        angle: f64,
        frequency: i32,
    ) -> Option<bool> {
        let args = vec![
            index_or_name,
            Variant::from_f64(angle),
            Variant::from_i64(frequency as i64),
        ];
        self.disp
            .invoke_method("SetProperties", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 閫氳繃绱㈠紩鑾峰彇鍚嶇О銆?
    pub fn name_by_index(&self, index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("NameByIndex", args).ok()?.to_string().ok()
    }

    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }
}