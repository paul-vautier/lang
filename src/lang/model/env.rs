use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::lang::{InterpError, InterpErrorReason, Value};

pub struct Env {
    enclosing: Option<Rc<RefCell<Env>>>,
    values: HashMap<String, Value>,
}

impl Env {
    pub fn enclosing(enclosing: Env) -> Env {
        Env {
            enclosing: Some(Rc::new(RefCell::new(enclosing))),
            values: HashMap::new(),
        }
    }
    pub fn new() -> Env {
        Env {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, id: String, value: Value) -> bool {
        if self.values.contains_key(&id) {
            self.values.insert(id, value);
            return true;
        }
        self.enclosing
            .as_mut()
            .map_or(false, |env| env.borrow_mut().assign(id, value))
    }

    pub fn get_ident_value(&self, id: &String) -> Result<Value, InterpError> {
        self.values.get(id).map_or(
            Err(InterpError::value(InterpErrorReason::UndeclaredVar(
                id.clone(),
            ))),
            |v| Ok(v.clone()),
        )
    }
}
