use std::{cell::RefCell, collections::HashMap, ops::Not};

use leptos::{create_rw_signal, RwSignal, SignalGetUntracked, SignalSet};

use super::validator::{ControlValidator, ValidatorMetadata};

pub trait AbstractFormControl<T>
where
    T: Clone + 'static,
{
    fn update_and_validity(&self);
    fn set_value(&self, new_value: Option<T>);
    fn has_err(&self, name: &str) -> bool;
    fn err(&self, name: &str) -> Option<ValidatorMetadata>;
    fn has_errs(&self) -> bool;
    fn is_valid(&self) -> bool;
}

pub struct FormControl<T>
where
    T: Clone + 'static,
{
    pub value: RwSignal<Option<T>>,
    /// keep validations errors with their metadata
    errs: RefCell<HashMap<String, ValidatorMetadata>>,
    validators: Vec<Box<dyn ControlValidator<T>>>,
}

impl<T> FormControl<T>
where
    T: Clone + 'static,
{
    pub fn new(default_value: Option<T>, validators: Vec<Box<dyn ControlValidator<T>>>) -> Self {
        let form_ctrl = Self {
            value: create_rw_signal(default_value),
            errs: RefCell::new(HashMap::new()),
            validators,
        };
        form_ctrl.update_and_validity();

        form_ctrl
    }
}

impl<T> AbstractFormControl<T> for FormControl<T>
where
    T: Clone + 'static,
{
    fn update_and_validity(&self) {
        // clear the preceding errors
        self.errs.borrow_mut().clear();

        for validator in self.validators.iter() {
            if let Some(form_value) = self.value.try_get_untracked() {
                if let Err(errs) = validator.validate(form_value) {
                    self.errs.borrow_mut().insert(errs.0, errs.1);
                }
            }
        }
    }

    fn set_value(&self, new_value: Option<T>) {
        self.value.set(new_value);
        self.update_and_validity();
    }

    fn has_err(&self, name: &str) -> bool {
        self.errs.borrow().contains_key(name)
    }

    fn err(&self, name: &str) -> Option<ValidatorMetadata> {
        self.errs
            .borrow()
            .get(name)
            .and_then(|val| Some(val.clone()))
    }

    fn has_errs(&self) -> bool {
        self.errs.borrow().is_empty().not()
    }

    fn is_valid(&self) -> bool {
        self.errs.borrow().is_empty()
    }
}

#[cfg(test)]
mod test {

    mod text_control {

        mod text_control_default {
            use leptos::create_runtime;

            use crate::forms::control::{AbstractFormControl, FormControl};

            #[test]
            fn it_create_a_text_control() {
                let runtime = create_runtime();

                let form_control: FormControl<String> =
                    FormControl::new(Some(String::new()), vec![]);

                assert_eq!(form_control.is_valid(), true);
                assert_eq!(form_control.has_errs(), false);

                runtime.dispose();
            }
        }

        mod text_control_validators {

            use leptos::create_runtime;

            use crate::forms::{
                control::{AbstractFormControl, FormControl},
                validator::ValidatorMetadata,
                validators::min_length::{MinLength, MIN_LENGTH_NAME},
            };

            #[test]
            fn it_create_a_text_control_with_validators() {
                let runtime = create_runtime();

                let form_control: FormControl<String> =
                    FormControl::new(Some(String::new()), vec![Box::new(MinLength::new(1))]);

                assert_eq!(form_control.is_valid(), false);
                assert_eq!(form_control.has_errs(), true);
                assert_eq!(form_control.has_err(MIN_LENGTH_NAME), true);

                let expected_meta = ValidatorMetadata::new();
                expected_meta.add(String::from("min_length"), 1);
                expected_meta.add(String::from("actual_length"), 0);
                assert_eq!(form_control.err(MIN_LENGTH_NAME), Some(expected_meta));

                runtime.dispose();
            }
        }
    }

    mod option_control {

        use leptos::create_runtime;

        use crate::forms::{
            control::{AbstractFormControl, FormControl},
            validators::{
                min::Min,
                required::{Required, REQUIRED_NAME},
            },
        };

        #[test]
        fn it_create_an_option_control() {
            let runtime = create_runtime();

            let form_control: FormControl<i32> = FormControl::new(None, vec![]);

            assert_eq!(form_control.is_valid(), true);
            assert_eq!(form_control.has_errs(), false);

            runtime.dispose();
        }

        #[test]
        fn it_create_an_option_with_validators() {
            let runtime = create_runtime();

            let form_control: FormControl<i32> =
                FormControl::new(None, vec![Box::new(Required::new()), Box::new(Min::new(5))]);

            assert_eq!(form_control.is_valid(), false);
            assert_eq!(form_control.has_errs(), true);
            assert_eq!(form_control.has_err(REQUIRED_NAME), true);

            runtime.dispose();
        }
    }
}
