#[derive(Debug, Copy, Clone)]
pub enum Types<'a> {
    String(&'a str),
    Int(i128),
    Char(char),
    Boolean(bool),
}

pub struct DefinitionMap<'a> {
    elems: Vec<(String, Types<'a>)>,
}

impl<'a> DefinitionMap<'a> {
    pub fn new() -> Self {
        Self { elems: Vec::new() }
    }

    pub fn insert(&mut self, key: String, value: Types<'a>) {
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
}
