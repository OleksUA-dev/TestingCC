use serde::{Deserialize, Serialize};
use sqlx::Type;

/// Supported data types for entity attributes
/// Aligned with ТЗ requirements: String, Text, Integer, Float, Decimal, Boolean, DateTime, Date, Lookup, Enum
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "data_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DataType {
    /// Short string (VARCHAR)
    String,
    /// Long text (TEXT)
    Text,
    /// Integer number (INT8)
    Integer,
    /// Floating point number (FLOAT8)
    Float,
    /// Precise decimal number (NUMERIC) - important for financial data
    Decimal,
    /// Boolean value (BOOL)
    Boolean,
    /// Date and time with timezone (TIMESTAMPTZ)
    DateTime,
    /// Date only (DATE)
    Date,
    /// Reference to another entity (Lookup/Foreign Key)
    Lookup,
    /// Enumeration - predefined set of values
    Enum,
}

impl DataType {
    /// Returns the corresponding PostgreSQL type for this data type
    pub fn to_pg_type(&self) -> &'static str {
        match self {
            DataType::String => "VARCHAR(255)",
            DataType::Text => "TEXT",
            DataType::Integer => "BIGINT",
            DataType::Float => "DOUBLE PRECISION",
            DataType::Decimal => "NUMERIC(19,4)", // Precision for financial calculations
            DataType::Boolean => "BOOLEAN",
            DataType::DateTime => "TIMESTAMPTZ",
            DataType::Date => "DATE",
            DataType::Lookup => "UUID", // References are stored as UUIDs
            DataType::Enum => "VARCHAR(255)", // Enum values stored as strings
        }
    }

    /// Checks if this data type requires additional metadata (e.g., referenced entity for Lookup)
    pub fn requires_metadata(&self) -> bool {
        matches!(self, DataType::Lookup | DataType::Enum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_type_pg_mapping() {
        assert_eq!(DataType::String.to_pg_type(), "VARCHAR(255)");
        assert_eq!(DataType::Decimal.to_pg_type(), "NUMERIC(19,4)");
        assert_eq!(DataType::Lookup.to_pg_type(), "UUID");
    }

    #[test]
    fn test_requires_metadata() {
        assert!(DataType::Lookup.requires_metadata());
        assert!(DataType::Enum.requires_metadata());
        assert!(!DataType::String.requires_metadata());
    }
}
