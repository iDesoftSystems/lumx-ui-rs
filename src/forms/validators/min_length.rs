use crate::forms::validator::{ControlValidator, ControlValidatorResult, ValidatorMetadata};

pub const MIN_LENGTH_NAME: &str = "MIN_LENGTH";

pub struct MinLength {
    min_length: i32,
}

impl MinLength {
    pub fn new(min_length: i32) -> Self {
        Self { min_length }
    }
}

impl ControlValidator<String> for MinLength {
    fn validate(&self, value: Option<String>) -> ControlValidatorResult {
        if let Some(val) = value {
            let val_len = val.trim().len() as i32;

            if val_len.lt(&self.min_length) {
                let meta = ValidatorMetadata::new();
                meta.add(String::from("min_length"), self.min_length);
                meta.add(String::from("actual_length"), val_len);

                Err((String::from(MIN_LENGTH_NAME), meta))?
            }
        }

        Ok(())
    }
}
