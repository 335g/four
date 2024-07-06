trait Parameter {
    /// AllowedPattern
    fn allowed_pattern(&self) -> Option<String> {
        None
    }

    /// AllowedValues
    fn allowed_values(&self) -> Option<Vec<String>> {
        None
    }

    /// ConstraintDescription
    fn constraint_description(&self) -> Option<String> {
        None
    }

    /// Default
    fn default(&self) -> Option<String> {
        None
    }

    /// Description
    fn description(&self) -> Option<String> {
        None
    }

    /// MaxLength
    fn max_length(&self) -> Option<usize> {
        None
    }

    /// MinLength
    fn min_length(&self) -> Option<usize> {
        None
    }

    fn no_echo(&self) -> Option<bool> {
        None
    }
}

impl<T> Parameter for T {}

pub trait StringParameter: Parameter {
    fn value(&self) -> &str;
}
