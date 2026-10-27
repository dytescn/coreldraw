

use wincom::Variant;

/// CorelDRAW 对象的后期绑定包装（属性条目）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct PropertyEntry {
    pub name: String,
    pub id: i32,
    pub value: Option<Variant>,
}


impl PropertyEntry {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(name: impl Into<String>, id: i32) -> Self {
        Self {
            name: name.into(),
            id,
            value: None,
        }
    }

    /// 值。
    pub fn with_value(mut self, v: Variant) -> Self {
        self.value = Some(v);
        self
    }
}

/// CorelDRAW 对象的后期绑定包装（属性列表）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
#[derive(Default)]
pub struct PropertyList {
    entries: Vec<PropertyEntry>,
}
impl PropertyList {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加。
    pub fn add(&mut self, entry: PropertyEntry) {
        self.entries.push(entry);
    }

    /// 调用 CorelDRAW COM 接口。
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// 是否空。
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 调用 CorelDRAW COM 接口。
    pub fn iter(&self) -> std::slice::Iter<'_, PropertyEntry> {
        self.entries.iter()
    }

    /// 查找。
    pub fn find(&self, name: &str, id: i32) -> Option<&PropertyEntry> {
        self.entries.iter().find(|e| e.name == name && e.id == id)
    }

    /// 移除。
    pub fn remove(&mut self, name: &str, id: i32) -> bool {
        if let Some(pos) = self
            .entries
            .iter()
            .position(|e| e.name == name && e.id == id)
        {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    /// 清空全部内容。
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}