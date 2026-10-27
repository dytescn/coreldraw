//! `IVGSnapPoint` 一时—— 抓取点
//!
//! - [`IvgSnapPoint`]          : 通用抓取点
//! - [`IvgSnapPoints`]         : 集合
//! - [`IvgSnapPointRange`]     : 范围
//! - [`IvgUserSnapPoint`]      : 用户自定义抓取点
//! - [`IvgObjectSnapPoint`]    : 对象抓取点（中点、端点、圆心……）
//! - [`IvgBBoxSnapPoint`]      : 包围盒角点
//! - [`IvgEdgeSnapPoint`]      : 边缘抓取点

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgNode;
use crate::shape::IvgShape;

// =============================================================
// IvgSnapPoint
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（捕捉点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSnapPoint {
    disp: ComObject,
}

impl IvgSnapPoint {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 位置 ----

    /// 读取位置 X 坐标（`PositionX`）。
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    /// 设置位置 X 坐标（`PositionX`）。
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    /// 读取位置 Y 坐标（`PositionY`）。
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    /// 设置位置 Y 坐标（`PositionY`）。
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    /// 读取位置。
    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.position_x()?, self.position_y()?))
    }

    /// 设置位置。
    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_position_x(x) && self.set_position_y(y)
    }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    // ---- 类型 / 方向 ----

    /// `cdrPointType`
    pub fn point_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 读取方向（`Direction`）。
    pub fn direction(&self) -> Option<f64> { self.prop_f64("Direction") }
    /// 设置方向（`Direction`）。
    pub fn set_direction(&self, v: f64) -> bool { self.put_f64("Direction", v) }

    /// 读取 COM 属性 `UsesDirection`。
    pub fn uses_direction(&self) -> Option<bool> { self.prop_bool("UsesDirection") }
    /// 设置 COM 属性 `UsesDirection`。
    pub fn set_uses_direction(&self, v: bool) -> bool { self.put_bool("UsesDirection", v) }

    /// 是否可选择（`IsSelectable`）。
    pub fn is_selectable(&self) -> Option<bool> { self.prop_bool("IsSelectable") }
    /// 读取 COM 属性 `IsDeletable`。
    pub fn is_deletable(&self) -> Option<bool> { self.prop_bool("IsDeletable") }
    /// 读取 COM 属性 `IsMovable`。
    pub fn is_movable(&self) -> Option<bool> { self.prop_bool("IsMovable") }
    /// 是否可以更改方向（`CanChangeDirection`）。
    pub fn can_change_direction(&self) -> Option<bool> {
        self.prop_bool("CanChangeDirection")
    }

    /// 读取已选中（`Selected`）。
    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    /// 设置已选中（`Selected`）。
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    // ---- 关联 ----

    /// 读取节点（`Node`）。
    pub fn node(&self) -> Option<IvgNode> {
        self.prop_dispatch("Node").map(IvgNode::new)
    }

    /// 读取图形（`Shape`）。
    pub fn shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("Shape").map(IvgShape::new)
    }

    /// 引用数据（GUID 字符串，唯一定位一个抓取点）。
    pub fn reference_data(&self) -> Option<String> {
        self.prop_string("ReferenceData")
    }

    // ---- 子类视图（按类型取） ----

    /// 读取用户（`User`）。
    pub fn user(&self) -> Option<IvgUserSnapPoint> {
        self.prop_dispatch("User").map(IvgUserSnapPoint::new)
    }

    /// 读取对象（`Object`）。
    pub fn object(&self) -> Option<IvgObjectSnapPoint> {
        self.prop_dispatch("Object").map(IvgObjectSnapPoint::new)
    }

    /// 读取 COM 属性 `BBox`。
    pub fn bbox(&self) -> Option<IvgBBoxSnapPoint> {
        self.prop_dispatch("BBox").map(IvgBBoxSnapPoint::new)
    }

    /// 读取边缘（`Edge`）。
    pub fn edge(&self) -> Option<IvgEdgeSnapPoint> {
        self.prop_dispatch("Edge").map(IvgEdgeSnapPoint::new)
    }

    // ---- 操作 ----

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}

// =============================================================
// IvgUserSnapPoint
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（用户捕捉点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgUserSnapPoint {
    disp: ComObject,
}

impl IvgUserSnapPoint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取标识（`ID`）。
    pub fn id(&self) -> Option<String> { self.prop_string("ID") }

    /// 读取自动捕捉（`AutoSnap`）。
    pub fn auto_snap(&self) -> Option<bool> { self.prop_bool("AutoSnap") }
    /// 设置自动捕捉（`AutoSnap`）。
    pub fn set_auto_snap(&self, v: bool) -> bool { self.put_bool("AutoSnap", v) }
}

// =============================================================
// IvgObjectSnapPoint
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（对象捕捉点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgObjectSnapPoint {
    disp: ComObject,
}

impl IvgObjectSnapPoint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// `cdrObjectSnapPointType`
    pub fn snap_type(&self) -> Option<i64> {
        self.disp.get_property("Type").ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgBBoxSnapPoint
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（b 框捕捉点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgBBoxSnapPoint {
    disp: ComObject,
}

impl IvgBBoxSnapPoint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// `cdrReferencePoint`
    pub fn snap_type(&self) -> Option<i64> {
        self.disp.get_property("Type").ok()?.to_i64().ok()
    }
}

// =============================================================
// IvgEdgeSnapPoint
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（边缘捕捉点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEdgeSnapPoint {
    disp: ComObject,
}

impl IvgEdgeSnapPoint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    /// 读取线段索引（`SegmentIndex`）。
    pub fn segment_index(&self) -> Option<i64> { self.prop_i64("SegmentIndex") }
    /// 读取线段偏移（`SegmentOffset`）。
    pub fn segment_offset(&self) -> Option<f64> { self.prop_f64("SegmentOffset") }
}

// =============================================================
// IvgSnapPoints
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（捕捉点集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSnapPoints {
    disp: ComObject,
}

impl IvgSnapPoints {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgSnapPoint> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    // ---- 分类获取 ----

    /// 用户（COM 方法 `User`）。
    pub fn user(&self, id: impl Into<String>) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_str(id.into())];
        self.disp
            .invoke_method("User", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// `cdrReferencePoint`
    pub fn bbox(&self, ref_point: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(ref_point as i64)];
        self.disp
            .invoke_method("BBox", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// `cdrObjectSnapPointType`
    pub fn object(&self, snap_type: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(snap_type as i64)];
        self.disp
            .invoke_method("Object", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 边缘（COM 方法 `Edge`）。
    pub fn edge(&self, segment_index: i32, segment_offset: f64) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_i64(segment_index as i64),
            Variant::from_f64(segment_offset),
        ];
        self.disp
            .invoke_method("Edge", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 自动捕捉点（COM 方法 `Auto`）。
    pub fn auto_snap_point(&self) -> Option<IvgSnapPoint> {
        self.disp
            .invoke_method("Auto", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 查找最近（COM 方法 `FindClosest`）。
    pub fn find_closest(
        &self,
        type_set: i32,
        x: f64,
        y: f64,
    ) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_i64(type_set as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp
            .invoke_method("FindClosest", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    // ---- 范围 ----

    /// 按引用数组取范围。
    pub fn range(&self, references: Variant) -> Option<IvgSnapPointRange> {
        let args = vec![references];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPointRange::new)
    }

    /// 读取选择（`Selection`）。
    pub fn selection(&self) -> Option<IvgSnapPointRange> {
        self.prop_dispatch("Selection").map(IvgSnapPointRange::new)
    }

    /// 读取全部范围（`All`）。
    pub fn all_range(&self) -> Option<IvgSnapPointRange> {
        self.prop_dispatch("All").map(IvgSnapPointRange::new)
    }

    /// 清除选择（COM 方法 `ClearSelection`）。
    pub fn clear_selection(&self) -> bool {
        self.disp.invoke_method("ClearSelection", vec![]).is_ok()
    }

    // ---- 添加用户抓取点----

    /// 添加用户捕捉点（COM 方法 `AddUserSnapPoint`）。
    pub fn add_user_snap_point(
        &self,
        x: f64,
        y: f64,
        direction: f64,
        use_direction: bool,
    ) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(direction),
            Variant::from_bool(use_direction),
        ];
        self.disp
            .invoke_method("AddUserSnapPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 添加用户捕捉点扩展版（COM 方法 `AddUserSnapPointEx`）。
    pub fn add_user_snap_point_ex(
        &self,
        id: impl Into<String>,
        x: f64,
        y: f64,
        direction: f64,
        use_direction: bool,
    ) -> Option<IvgSnapPoint> {
        let args = vec![
            Variant::from_str(id.into()),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(direction),
            Variant::from_bool(use_direction),
        ];
        self.disp
            .invoke_method("AddUserSnapPointEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }
}

// =============================================================
// IvgSnapPointRange
// =============================================================

/// CorelDRAW `IVGSnapPoint` 对象的后期绑定包装（捕捉点范围）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSnapPointRange {
    disp: ComObject,
}

impl IvgSnapPointRange {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgSnapPoint> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, sp: &IvgSnapPoint) -> Option<bool> {
        let args = vec![sp.as_variant()];
        self.disp.invoke_method("Add", args).ok()?.to_bool().ok()
    }

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).ok()?.to_bool().ok()
    }

    /// 移除参考（COM 方法 `RemoveByReference`）。
    pub fn remove_by_reference(&self, reference_data: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(reference_data.into())];
        self.disp
            .invoke_method("RemoveByReference", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 查找（COM 方法 `Find`）。
    pub fn find(&self, reference_data: impl Into<String>) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_str(reference_data.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    /// 添加选择（COM 方法 `AddToSelection`）。
    pub fn add_to_selection(&self) -> bool {
        self.disp.invoke_method("AddToSelection", vec![]).is_ok()
    }

    /// 移除选择（COM 方法 `RemoveFromSelection`）。
    pub fn remove_from_selection(&self) -> bool {
        self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok()
    }

    /// 更改方向（COM 方法 `ChangeDirection`）。
    pub fn change_direction(&self, direction: f64, uses_direction: i32) -> bool {
        let args = vec![
            Variant::from_f64(direction),
            Variant::from_i64(uses_direction as i64),
        ];
        self.disp.invoke_method("ChangeDirection", args).is_ok()
    }

    /// 设置自动捕捉（`SetAutoSnap`）。
    pub fn set_auto_snap(&self, v: bool) -> bool {
        let args = vec![Variant::from_bool(v)];
        self.disp.invoke_method("SetAutoSnap", args).is_ok()
    }
}