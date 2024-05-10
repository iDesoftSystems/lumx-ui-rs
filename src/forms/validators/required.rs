use crate::forms::validator::{ControlValidator, ControlValidatorResult, ValidatorMetadata};

pub const REQUIRED_NAME: &str = "REQUIRED";

pub struct Required;

impl Required {
    pub fn new() -> Self {
        Self {}
    }
}

impl<V> ControlValidator<V> for Required
where
    V: 'static,
{
    fn validate(&self, value: Option<V>) -> ControlValidatorResult {
        if value.is_none() {
            Err((String::from(REQUIRED_NAME), ValidatorMetadata::new()))?
        }

        Ok(())
    }
}
