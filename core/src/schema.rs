// JSON Schema validation for plugin entities
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub properties: Option<HashMap<String, Value>>,
    pub required: Option<Vec<String>>,
    pub additional_properties: Option<bool>,
}

pub struct SchemaValidator {
    schemas: HashMap<String, JsonSchema>,
}

impl SchemaValidator {
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    pub fn add_schema(&mut self, name: String, schema: JsonSchema) {
        self.schemas.insert(name, schema);
    }

    pub fn validate(&self, schema_name: &str, data: &Value) -> Result<(), ValidationError> {
        let schema = self.schemas.get(schema_name)
            .ok_or_else(|| ValidationError::SchemaNotFound(schema_name.to_string()))?;

        self.validate_against_schema(schema, data, &vec![])
    }

    fn validate_against_schema(
        &self,
        schema: &JsonSchema,
        data: &Value,
        path: &[String],
    ) -> Result<(), ValidationError> {
        // Basic type validation
        if let Some(schema_type) = &schema.schema_type {
            match schema_type.as_str() {
                "object" => {
                    if !data.is_object() {
                        return Err(ValidationError::TypeMismatch {
                            path: path.join("."),
                            expected: "object".to_string(),
                            actual: self.get_type_name(data),
                        });
                    }

                    // Validate required properties
                    if let Some(required) = &schema.required {
                        if let Some(obj) = data.as_object() {
                            for req_prop in required {
                                if !obj.contains_key(req_prop) {
                                    return Err(ValidationError::MissingRequiredProperty {
                                        path: path.join("."),
                                        property: req_prop.clone(),
                                    });
                                }
                            }
                        }
                    }

                    // Validate properties
                    if let Some(properties) = &schema.properties {
                        if let Some(obj) = data.as_object() {
                            for (prop_name, prop_schema) in properties {
                                if let Some(prop_value) = obj.get(prop_name) {
                                    let mut new_path = path.to_vec();
                                    new_path.push(prop_name.clone());

                                    // For now, just do basic validation
                                    // In a full implementation, you'd recursively validate
                                    if let Some(prop_type) = prop_schema.get("type") {
                                        if let Some(prop_type_str) = prop_type.as_str() {
                                            match prop_type_str {
                                                "string" => {
                                                    if !prop_value.is_string() {
                                                        return Err(ValidationError::TypeMismatch {
                                                            path: new_path.join("."),
                                                            expected: "string".to_string(),
                                                            actual: self.get_type_name(prop_value),
                                                        });
                                                    }
                                                }
                                                "number" => {
                                                    if !prop_value.is_number() {
                                                        return Err(ValidationError::TypeMismatch {
                                                            path: new_path.join("."),
                                                            expected: "number".to_string(),
                                                            actual: self.get_type_name(prop_value),
                                                        });
                                                    }
                                                }
                                                _ => {} // Other types not implemented yet
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "string" => {
                    if !data.is_string() {
                        return Err(ValidationError::TypeMismatch {
                            path: path.join("."),
                            expected: "string".to_string(),
                            actual: self.get_type_name(data),
                        });
                    }
                }
                "number" => {
                    if !data.is_number() {
                        return Err(ValidationError::TypeMismatch {
                            path: path.join("."),
                            expected: "number".to_string(),
                            actual: self.get_type_name(data),
                        });
                    }
                }
                _ => {} // Other types not implemented yet
            }
        }

        Ok(())
    }

    fn get_type_name(&self, value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        }
    }

    pub fn list_schemas(&self) -> Vec<&str> {
        self.schemas.keys().map(|s| s.as_str()).collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Schema '{0}' not found")]
    SchemaNotFound(String),

    #[error("Type mismatch at '{path}': expected {expected}, got {actual}")]
    TypeMismatch {
        path: String,
        expected: String,
        actual: String,
    },

    #[error("Missing required property '{property}' at '{path}'")]
    MissingRequiredProperty {
        path: String,
        property: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_basic_validation() {
        let mut validator = SchemaValidator::new();

        let schema = JsonSchema {
            schema: Some("http://json-schema.org/draft-07/schema#".to_string()),
            schema_type: Some("object".to_string()),
            properties: Some({
                let mut props = HashMap::new();
                props.insert("name".to_string(), json!({"type": "string"}));
                props.insert("age".to_string(), json!({"type": "number"}));
                props
            }),
            required: Some(vec!["name".to_string()]),
            additional_properties: Some(false),
        };

        validator.add_schema("person".to_string(), schema);

        // Valid data
        let valid_data = json!({
            "name": "John",
            "age": 30
        });
        assert!(validator.validate("person", &valid_data).is_ok());

        // Missing required property
        let invalid_data = json!({
            "age": 30
        });
        assert!(validator.validate("person", &invalid_data).is_err());

        // Wrong type
        let invalid_data2 = json!({
            "name": "John",
            "age": "thirty"
        });
        assert!(validator.validate("person", &invalid_data2).is_err());
    }

    #[test]
    fn test_schema_not_found() {
        let validator = SchemaValidator::new();
        let data = json!({"name": "test"});
        let result = validator.validate("nonexistent", &data);
        assert!(matches!(result, Err(ValidationError::SchemaNotFound(_))));
    }
}