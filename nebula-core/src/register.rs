use std::collections::HashMap;

use crate::scope::Scope;

pub struct Entry<A> {
    _scope: Scope,
    _name: String,
    value: A,
}

pub struct Register<A> {
    inner: HashMap<String, Entry<A>>,
}

impl<A> Default for Register<A> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<A> Register<A> {
    pub fn register(&mut self, scope: &Scope, name: &str, value: A) -> bool {
        let id = generate_id(scope, name);

        self.inner
            .insert(
                id,
                Entry {
                    _scope: scope.clone(),
                    _name: name.to_string(),
                    value,
                },
            )
            .is_none()
    }

    pub fn lookup(&self, scope: &Scope, name: &str) -> Option<&A> {
        let id = generate_id(scope, name);

        self.inner.get(&id).map(|e| &e.value)
    }

    pub fn remove(&mut self, scope: &Scope, name: &str) {
        let id = generate_id(scope, name);
        self.inner.remove(&id);
    }
}

fn generate_id(scope: &Scope, name: &str) -> String {
    format!("{}:{}", scope.id(), name)
}
