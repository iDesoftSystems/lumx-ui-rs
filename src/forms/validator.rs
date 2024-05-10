use std::{cell::RefCell, collections::HashMap};

pub type ValidatorKey = String;
pub type ControlValidatorResult = Result<(), (ValidatorKey, ValidatorMetadata)>;

pub trait ControlValidator<T> {
    fn validate(&self, value: Option<T>) -> ControlValidatorResult;
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ValidatorMetadata {
    meta: RefCell<HashMap<String, i32>>,
}

impl ValidatorMetadata {
    pub fn new() -> Self {
        Self {
            meta: RefCell::new(HashMap::new()),
        }
    }

    pub fn add(&self, key: String, value: i32) {
        self.meta.borrow_mut().insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        self.meta
            .borrow()
            .get(key)
            .and_then(|val| Some(val.clone()))
    }
}
