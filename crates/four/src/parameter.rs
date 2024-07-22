use std::marker::PhantomData;

use serde::{ser::SerializeMap, Serialize};
use thiserror::Error;

use crate::logical_id::{LogicalId, LogicalIdentified};

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ParameterType {
    String,
    Number,
}

#[derive(Debug, Clone)]
pub struct Parameter<T> {
    allowed_pattern: Option<String>,
    allowed_values: Option<Vec<T>>,
    constraint_description: Option<String>,
    default: Option<T>,
    description: Option<String>,
    max_length: Option<usize>,
    min_length: Option<usize>,
    max_value: Option<f64>,
    min_value: Option<f64>,
    no_echo: Option<bool>,
    r#type: ParameterType,
    logical_id: LogicalId,
}

impl<T> Parameter<T> {
    pub fn string() -> StringParameterBuilder<T> {
        StringParameterBuilder::new()
    }

    pub fn number() -> NumberParameterBuilder<T> {
        NumberParameterBuilder::new()
    }
}

impl<T> Serialize for Parameter<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = self.allowed_pattern.as_ref().map(|_| 1).unwrap_or_default()
            + self.allowed_values.as_ref().map(|_| 1).unwrap_or_default()
            + self
                .constraint_description
                .as_ref()
                .map(|_| 1)
                .unwrap_or_default()
            + self.default.as_ref().map(|_| 1).unwrap_or_default()
            + self.description.as_ref().map(|_| 1).unwrap_or_default()
            + self.max_length.as_ref().map(|_| 1).unwrap_or_default()
            + self.min_length.as_ref().map(|_| 1).unwrap_or_default()
            + self.max_value.as_ref().map(|_| 1).unwrap_or_default()
            + self.min_value.as_ref().map(|_| 1).unwrap_or_default()
            + self.no_echo.as_ref().map(|_| 1).unwrap_or_default()
            + 1;
        let mut map = serializer.serialize_map(Some(len))?;

        if let Some(allowed_pattern) = self.allowed_pattern.as_ref() {
            map.serialize_entry("AllowedPattern", allowed_pattern)?;
        }
        if let Some(allowed_values) = self.allowed_values.as_ref() {
            map.serialize_entry("AllowedValues", allowed_values)?;
        }
        if let Some(constraint_description) = self.constraint_description.as_ref() {
            map.serialize_entry("ConstraintDescription", constraint_description)?;
        }
        if let Some(default) = &self.default {
            map.serialize_entry("Default", default)?;
        }
        if let Some(description) = self.description.as_ref() {
            map.serialize_entry("Description", description)?;
        }
        if let Some(max_length) = self.max_length {
            map.serialize_entry("MaxLength", &max_length)?;
        }
        if let Some(min_length) = self.min_length {
            map.serialize_entry("MinLength", &min_length)?;
        }
        if let Some(max_value) = self.max_value {
            map.serialize_entry("MaxValue", &max_value)?;
        }
        if let Some(min_value) = self.min_value {
            map.serialize_entry("MinValue", &min_value)?;
        }
        if let Some(no_echo) = self.no_echo {
            map.serialize_entry("NoEcho", &no_echo)?;
        }

        map.serialize_entry("Type", &self.r#type)?;

        map.end()
    }
}

impl<T> LogicalIdentified for Parameter<T> {
    fn logical_id(&self) -> &LogicalId {
        &self.logical_id
    }
}

pub struct StringParameterBuilder<T> {
    allowed_pattern: Option<String>,
    allowed_values: Option<Vec<String>>,
    constraint_description: Option<String>,
    default: Option<String>,
    description: Option<String>,
    max_length: Option<usize>,
    min_length: Option<usize>,
    no_echo: Option<bool>,
    r#type: ParameterType,
    _ghost: PhantomData<T>,
}

impl<T> StringParameterBuilder<T> {
    fn new() -> Self {
        Self {
            allowed_pattern: None,
            allowed_values: None,
            constraint_description: None,
            default: None,
            description: None,
            max_length: None,
            min_length: None,
            no_echo: None,
            r#type: ParameterType::String,
            _ghost: PhantomData,
        }
    }
    pub fn allowed_pattern(mut self, x: String) -> Self {
        self.allowed_pattern = Some(x);
        self
    }

    pub fn allowed_values(mut self, x: Vec<String>) -> Self {
        self.allowed_values = Some(x);
        self
    }

    pub fn constraint_description(mut self, x: String) -> Self {
        self.constraint_description = Some(x);
        self
    }

    pub fn default(mut self, x: String) -> Self {
        self.default = Some(x);
        self
    }

    pub fn description(mut self, x: String) -> Self {
        self.description = Some(x);
        self
    }

    pub fn max_length(mut self, x: usize) -> Self {
        self.max_length = Some(x);
        self
    }

    pub fn min_length(mut self, x: usize) -> Self {
        self.min_length = Some(x);
        self
    }

    pub fn no_echo(mut self, x: bool) -> Self {
        self.no_echo = Some(x);
        self
    }
}

impl StringParameterBuilder<String> {
    pub fn build(self, logical_id: LogicalId) -> Result<Parameter<String>, ParameterError> {
        match (&self.min_length, &self.max_length) {
            (Some(min_length), Some(max_length)) if min_length > max_length => {
                return Err(ParameterError::MinMaxLengthInverersed(
                    *min_length,
                    *max_length,
                ))
            }
            _ => {}
        };

        Ok(Parameter {
            allowed_pattern: self.allowed_pattern,
            allowed_values: self.allowed_values,
            constraint_description: self.constraint_description,
            default: self.default,
            description: self.description,
            max_length: self.max_length,
            min_length: self.min_length,
            max_value: None,
            min_value: None,
            no_echo: self.no_echo,
            logical_id,
            r#type: self.r#type,
        })
    }
}

pub struct NumberParameterBuilder<T> {
    allowed_pattern: Option<String>,
    allowed_values: Option<Vec<f64>>,
    constraint_description: Option<String>,
    default: Option<f64>,
    description: Option<String>,
    max_value: Option<f64>,
    min_value: Option<f64>,
    no_echo: Option<bool>,
    r#type: ParameterType,
    _ghost: PhantomData<T>,
}

impl<T> NumberParameterBuilder<T> {
    fn new() -> Self {
        Self {
            allowed_pattern: None,
            allowed_values: None,
            constraint_description: None,
            default: None,
            description: None,
            max_value: None,
            min_value: None,
            no_echo: None,
            r#type: ParameterType::String,
            _ghost: PhantomData,
        }
    }
    pub fn allowed_pattern(mut self, x: String) -> Self {
        self.allowed_pattern = Some(x);
        self
    }

    pub fn allowed_values(mut self, x: Vec<f64>) -> Self {
        self.allowed_values = Some(x);
        self
    }

    pub fn constraint_description(mut self, x: String) -> Self {
        self.constraint_description = Some(x);
        self
    }

    pub fn default(mut self, x: f64) -> Self {
        self.default = Some(x);
        self
    }

    pub fn description(mut self, x: String) -> Self {
        self.description = Some(x);
        self
    }

    pub fn max_value(mut self, x: f64) -> Self {
        self.max_value = Some(x);
        self
    }

    pub fn min_value(mut self, x: f64) -> Self {
        self.min_value = Some(x);
        self
    }

    pub fn no_echo(mut self, x: bool) -> Self {
        self.no_echo = Some(x);
        self
    }
}

impl NumberParameterBuilder<f64> {
    pub fn build(self, logical_id: LogicalId) -> Result<Parameter<f64>, ParameterError> {
        match (&self.min_value, &self.max_value) {
            (Some(min_value), Some(max_value)) if min_value > max_value => {
                return Err(ParameterError::MinMaxValueInversed(*min_value, *max_value))
            }
            _ => {}
        };

        Ok(Parameter {
            allowed_pattern: self.allowed_pattern,
            allowed_values: self.allowed_values,
            constraint_description: self.constraint_description,
            default: self.default,
            description: self.description,
            max_length: None,
            min_length: None,
            max_value: self.max_value,
            min_value: self.min_value,
            no_echo: self.no_echo,
            logical_id,
            r#type: self.r#type,
        })
    }
}

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("min_length({0}) is bigger than max_length({1})")]
    MinMaxLengthInverersed(usize, usize),

    #[error("min_value({0}) is bigger than max_value({1})")]
    MinMaxValueInversed(f64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_param1() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string().build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_string_param2() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string()
            .default("default-value".to_string())
            .build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"Default":"default-value","Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_string_param3() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string()
            .allowed_pattern("allowed".to_string())
            .build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"AllowedPattern":"allowed","Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_string_param4() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string()
            .allowed_values(vec!["abc".to_string()])
            .build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"AllowedValues":["abc"],"Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_string_param5() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string()
            .constraint_description("description1".to_string())
            .build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"ConstraintDescription":"description1","Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_string_param6() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string().max_length(10).build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"MaxLength":10,"Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_string_param7() {
        let param_id = LogicalId::try_from("param-id").unwrap();
        let param = Parameter::string().min_length(10).build(param_id);
        match param {
            Ok(x) => {
                let s = serde_json::to_string(&x).unwrap();
                let rhs = r#"{"MinLength":10,"Type":"String"}"#;
                assert_eq!(s, rhs);
            }
            Err(_) => assert!(false),
        }
    }
}
