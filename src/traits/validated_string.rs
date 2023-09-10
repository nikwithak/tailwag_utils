use std::{ops::Deref, sync::Arc};

pub trait Validate {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

struct ValidatedString<F: Fn(&str) -> Result<(), String>> {
    value: Arc<String>,
    validate: F,
}

impl<F: Fn(&str) -> Result<(), String>> ValidatedString<F> {
    // pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
    //     Self::validate(&value);
    //     let identifier = Self {
    //         value: Arc::new(value.into()),
    //         validate: todo!(),
    //     };
    //     match identifier.validate() {
    //         Ok(()) => Ok(identifier),
    //         Err(e) => Err(e),
    //     }
    // }
    // pub fn new<S: Into<String>>(value: S) -> Result<Self, String> {
    //     Self::validate(&value);
    //     let identifier = Self {
    //         value: Arc::new(value.into()),
    //         validate: todo!(),
    //     };
    //     match identifier.validate() {
    //         Ok(()) => Ok(identifier),
    //         Err(e) => Err(e),
    //     }
    // }
}

impl<F: Fn(&str) -> Result<(), String>> Deref for ValidatedString<F> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
