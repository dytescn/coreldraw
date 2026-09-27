

use wincom::Variant;

pub struct PropertyEntry {
    pub name: String,
    pub id: i32,
    pub value: Option<Variant>,
}


impl PropertyEntry {
    pub fn new(name: impl Into<String>, id: i32) -> Self {
        Self {
            name: name.into(),
            id,
            value: None,
        }
    }

    pub fn with_value(mut self, v: Variant) -> Self {
        self.value = Some(v);
        self
    }
}

#[derive(Default)]
pub struct PropertyList {
    entries: Vec<PropertyEntry>,
}
impl PropertyList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, entry: PropertyEntry) {
        self.entries.push(entry);
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, PropertyEntry> {
        self.entries.iter()
    }

    pub fn find(&self, name: &str, id: i32) -> Option<&PropertyEntry> {
        self.entries.iter().find(|e| e.name == name && e.id == id)
    }

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

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}