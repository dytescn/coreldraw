//! 单位换算（纯 Rust，不走 COM）
//!
//! CorelDRAW 的 `Application.ConvertUnits` 走的是 COM 出参，
//! `IDispatch` 后期绑定拿不到，所以这里用本地换算实现。
//!
//! 所有换算都走 inch 中转：`value * from_to_inch / to_to_inch`。

use crate::enums::page::cdrUnit;

/// 每个单位相对 inch 的换算因子（`1 unit = X inch`）。
fn to_inch_factor(u: cdrUnit) -> Option<f64> {
    use cdrUnit::*;
    Some(match u {
        Inch       => 1.0,
        Millimeter => 1.0 / 25.4,
        Centimeter => 1.0 / 2.54,
        Point      => 1.0 / 72.0,
        Pica       => 1.0 / 6.0,
        Meter      => 1.0 / 0.0254,
        Kilometer  => 1.0 / 0.0000254,
        Foot       => 12.0,
        Yard       => 36.0,
        Mile       => 63360.0,
        // Cicero / Didot / Pixel / Agate 等按需补
        _ => return None,
    })
}

pub fn convert(value: f64, from: cdrUnit, to: cdrUnit) -> Option<f64> {
    let f = to_inch_factor(from)?;
    let t = to_inch_factor(to)?;
    let raw = value * f / t;
    // 对浮点噪声做一次截断：保留 12 位有效数字
    Some(round_to_significant(raw, 12))
}

fn round_to_significant(x: f64, digits: i32) -> f64 {
    if x == 0.0 || !x.is_finite() {
        return x;
    }
    let magnitude = x.abs().log10().floor() as i32;
    let factor = 10f64.powi(digits - magnitude - 1);
    (x * factor).round() / factor
}

/// 用 `cdrUnit` 的 i32 值直接换算（方便 `IvgApplication` 调用）。
pub fn convert_by_i32(value: f64, from: i32, to: i32) -> Option<f64> {
    let f = cdrUnit::from_i32(from)?;
    let t = cdrUnit::from_i32(to)?;
    convert(value, f, t)
}