use crate::forms::validator::{ControlValidator, ControlValidatorResult, ValidatorMetadata};

pub const MIN_NAME: &str = "MIN";

pub struct Min {
    min_value: i32,
}

impl Min {
    pub fn new(min_value: i32) -> Self {
        Self { min_value }
    }
}

impl ControlValidator<i32> for Min {
    fn validate(&self, value: Option<i32>) -> ControlValidatorResult {
        if let Some(val) = value {
            if val.lt(&self.min_value) {
                let meta = ValidatorMetadata::new();
                meta.add(String::from("min_val"), self.min_value);
                meta.add(String::from("actual_value"), val);

                Err((String::from(MIN_NAME), meta))?
            }
        }

        Ok(())
    }
}
