pub trait BoolConversion {
    fn to_bool(&self) -> Option<bool>;
}

impl BoolConversion for str {
    fn to_bool(&self) -> Option<bool> {
        return match self {
            "true" | "1" => Some(true),
            "false" | "0" => Some(false),
            &_ => None,
        };
    }
}
