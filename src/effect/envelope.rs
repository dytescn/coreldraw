//! `IVGEffectEnvelope` —— 封套效果

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::shape::IvgShape;

/// CorelDRAW `IVGEffectEnvelope` 对象的后期绑定包装（效果封套）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectEnvelope {
    disp: ComObject,
}

impl IvgEffectEnvelope {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 容器曲线 ----

    /// 读取容器（`Container`）。
    pub fn container(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Container").map(IvgCurve::new)
    }

    /// 设置容器（`Container`）。
    pub fn set_container(&self, c: &IvgCurve) -> bool {
        let arg = c.as_variant();
        self.disp.set_property("Container", vec![arg]).is_ok()
    }

    // ---- 模式 ----

    /// `cdrEnvelopeMode`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }
    /// 设置模式（`Mode`）。
    pub fn set_mode(&self, v: i32) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property("Mode", vec![arg]).is_ok()
    }

    /// 读取保持线（`KeepLines`）。
    pub fn keep_lines(&self) -> Option<bool> { self.prop_bool("KeepLines") }
    /// 设置保持线（`KeepLines`）。
    pub fn set_keep_lines(&self, v: bool) -> bool { self.put_bool("KeepLines", v) }

    // ---- 操作 ----

    /// 从预设封套形状索引选择（`cdrEnvelopePreset`）。
    pub fn select(&self, preset_index: i32) -> bool {
        let args = vec![Variant::from_i64(preset_index as i64)];
        self.disp.invoke_method("Select", args).is_ok()
    }

    /// 复制（COM 方法 `CopyFrom`）。
    pub fn copy_from(&self, src: &IvgEffectEnvelope) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyFrom", args).is_ok()
    }

    /// 从图形形状创建封套。
    pub fn create_from(&self, shape: &IvgShape) -> Option<bool> {
        let args = vec![shape.as_variant()];
        self.disp
            .invoke_method("CreateFrom", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 从图形的轮廓曲线复制封套。
    pub fn copy_from_shape(
        &self,
        source: &IvgShape,
        mode: i32,
        keep_lines: bool,
        copy_mode: i32,
        corner_indices: Variant,
    ) -> Option<bool> {
        let args = vec![
            source.as_variant(),
            Variant::from_i64(mode as i64),
            Variant::from_bool(keep_lines),
            Variant::from_i64(copy_mode as i64),
            corner_indices,
        ];
        self.disp
            .invoke_method("CopyFromShape", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 从曲线复制封套。
    pub fn copy_from_curve(
        &self,
        source: &IvgCurve,
        mode: i32,
        keep_lines: bool,
        copy_mode: i32,
        corner_indices: Variant,
    ) -> Option<bool> {
        let args = vec![
            source.as_variant(),
            Variant::from_i64(mode as i64),
            Variant::from_bool(keep_lines),
            Variant::from_i64(copy_mode as i64),
            corner_indices,
        ];
        self.disp
            .invoke_method("CopyFromCurve", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---- 角点 ----

    /// 读取 COM 属性 `CornerIndices`。
    pub fn corner_indices(&self) -> Option<Variant> {
        self.disp.get_property("CornerIndices").ok()
    }

    /// 设置 COM 属性 `CornerIndices`。
    pub fn set_corner_indices(&self, v: Variant) -> bool {
        self.disp.set_property("CornerIndices", vec![v]).is_ok()
    }
}