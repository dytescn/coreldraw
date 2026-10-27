//! `IVGStructColorConversionOptions` —— 颜色转换选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColorManagementPolicy;

/// CorelDRAW `IVGStructColorConversionOptions` 对象的后期绑定包装（颜色转换选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructColorConversionOptions {
    disp: ComObject,
}

impl IvgStructColorConversionOptions {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 颜色策略 ----

    /// 读取颜色策略（`ColorPolicy`）。
    pub fn color_policy(&self) -> Option<IvgColorManagementPolicy> {
        self.prop_dispatch("ColorPolicy").map(IvgColorManagementPolicy::new)
    }

    // ---- 源/ 目标 profile ----

    /// 读取源颜色配置文件列表（`SourceColorProfileList`）。
    pub fn source_color_profile_list(&self) -> Option<String> {
        self.prop_string("SourceColorProfileList")
    }
    /// 设置源颜色配置文件列表（`SourceColorProfileList`）。
    pub fn set_source_color_profile_list(&self, v: impl Into<String>) -> bool {
        self.put_string("SourceColorProfileList", v)
    }

    /// 读取目标颜色配置文件列表（`TargetColorProfileList`）。
    pub fn target_color_profile_list(&self) -> Option<String> {
        self.prop_string("TargetColorProfileList")
    }
    /// 设置目标颜色配置文件列表（`TargetColorProfileList`）。
    pub fn set_target_color_profile_list(&self, v: impl Into<String>) -> bool {
        self.put_string("TargetColorProfileList", v)
    }

    // ---- 转换回调 ----

    /// `IColorConversionHandler` 的 dispatch（用户实现）。
    pub fn color_conversion_handler(&self) -> Option<IDispatch> {
        self.prop_dispatch("ColorConversionHandler")
    }

    /// 设置颜色转换处理器（`ColorConversionHandler`）。
    pub fn set_color_conversion_handler(&self, h: IDispatch) -> bool {
        let arg = Variant::from_dispatch(&h);
        self.disp.set_property("ColorConversionHandler", vec![arg]).is_ok()
    }
}