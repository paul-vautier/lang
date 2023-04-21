use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

pub struct Env {
    enclosing: Option<Rc<RefCell<Env>>>,
    values: HashMap<String, Value>,
}

impl Env {
    fn new(enclosing: Env) -> Env {
        Env {
            enclosing: Some(Rc::new(RefCell::new(enclosing))),
            values: HashMap::new(),
        }
    }

    fn assign(&mut self, id: String, value: Value) -> bool {
        if self.values.contains_key(&id) {
            self.values.insert(id, value);
            return true;
        }
        self.enclosing
            .as_mut()
            .map_or(false, |env| env.borrow_mut().assign(id, value))
    }
}

pub enum Value {
    None,
    Integer(f64),
    String(String),
}
