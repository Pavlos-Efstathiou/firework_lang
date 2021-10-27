#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Types {
    String(String),
    Int(i128),
    Char(char),
    Boolean(bool),
}

#[derive(Debug)]
pub struct DefinitionMap {
    elems: Vec<(String, Types)>,
}

#[allow(dead_code)]
impl DefinitionMap {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: Types) {
        match self.get(key.to_owned()) {
            None => self.elems.push((key, value)),
            Some(_) => {}
        }
    }

    pub fn remove(&mut self, key: String) {
        self.elems.retain(|(x, _)| *x == key)
    }

    pub fn get(&self, key: String) -> Option<Types> {
        match self
            .elems
            .iter()
            .position(|(x, _)| *x == key)
            .unwrap_or(usize::MAX)
        {
            usize::MAX => None,
            x => Some(self.elems[x].1.clone()),
        }
    }

    pub fn len(&self) -> usize {
        self.elems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elems.is_empty()
    }
}

impl Default for DefinitionMap {
    fn default() -> Self {
        Self::new()
    }
}
