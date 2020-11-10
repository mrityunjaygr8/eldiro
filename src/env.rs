use crate::val::Val;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub(crate) struct Env {
    bindings: HashMap<String, Val>
}

impl Env {
    pub(crate) fn store_bindings(&mut self, name: String, value: Val) {
        self.bindings.insert(name, value);
    }
}