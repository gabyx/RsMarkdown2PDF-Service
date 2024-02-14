use crate::string::BoolConversion;
use std::env;

#[derive(Debug)]
pub struct KeyVal<T> {
    key: &'static str,
    value: T,
}

impl KeyVal<String> {
    pub fn bool(&self) -> KeyVal<bool> {
        match self.value.to_bool() {
            Some(value) => KeyVal::<_> {
                key: self.key,
                value,
            },
            None => panic!("Cannot convert env. var '{}'", self.key),
        }
    }
}

impl<T> KeyVal<T>
where
    T: Default,
{
    pub fn take(&mut self) -> T {
        // The same as Optional does,
        // moving out the value.
        // The memory replace here is similar to `std::move` in C++.
        // https://stackoverflow.com/a/31308299/293195
        return std::mem::take(&mut self.value);
    }
}

pub fn get_env_var(key: &'static str) -> KeyVal<String> {
    return KeyVal {
        key,
        value: env::var(key).unwrap_or_else(|_| panic!("Expect environment variable '{}' to be defined.",
            key)),
    };
}
