pub type ScopeId = u32;

#[derive(Clone)]
pub struct Scope {
    ancestors: Vec<u32>,
}

impl Scope {
    pub fn new() -> Self {
        Self { ancestors: vec![0] }
    }
}

impl Scope {
    pub fn ancestors(&self) -> &[u32] {
        self.ancestors.as_ref()
    }

    pub fn id(&self) -> ScopeId {
        self.ancestors.last().copied().unwrap()
    }

    pub fn inherits(&self) -> Scope {
        let mut temp = self.ancestors.clone();
        let parent = temp.last().copied().unwrap();

        temp.push(parent + 1);

        Self { ancestors: temp }
    }
}
