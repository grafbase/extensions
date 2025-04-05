use super::{EnumId, EnumWalker};

#[derive(Clone, Copy, PartialEq)]
pub enum DatabaseType<'a> {
    Scalar(ScalarType),
    Enum(EnumWalker<'a>),
}

impl<'a> DatabaseType<'a> {
    pub fn is_enum(self) -> bool {
        matches!(self, DatabaseType::Enum(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, DatabaseType::Scalar(scalar) if scalar.is_binary())
    }

    pub fn is_json(&self) -> bool {
        matches!(self, DatabaseType::Scalar(scalar) if scalar.is_json())
    }

    pub fn is_jsonb(&self) -> bool {
        matches!(self, DatabaseType::Scalar(scalar) if scalar.is_jsonb())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnType {
    Scalar(ScalarType),
    Enum(EnumType),
}

impl ColumnType {
    pub fn is_array(self) -> bool {
        match self {
            ColumnType::Scalar(scalar_type) => scalar_type.is_array,
            ColumnType::Enum(r#enum) => r#enum.is_array,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EnumType {
    pub id: EnumId,
    pub is_array: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarType {
    pub kind: ScalarKind,
    pub is_array: bool,
}

impl ScalarType {
    pub fn is_binary(self) -> bool {
        matches!(self.kind, ScalarKind::BYTEA)
    }

    pub fn is_json(&self) -> bool {
        matches!(self.kind, ScalarKind::JSON)
    }

    pub fn is_jsonb(&self) -> bool {
        matches!(self.kind, ScalarKind::JSONB)
    }
}

/// Postgres data types supported for column definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[allow(non_camel_case_types)]
pub enum ScalarKind {
    /// 2-byte signed integer, range: -32768 to +32767
    SMALLINT,
    /// 4-byte signed integer, range: -2147483648 to +2147483647
    INTEGER,
    /// Alias for INTEGER
    INT,
    /// 8-byte signed integer, range: -9223372036854775808 to +9223372036854775807
    BIGINT,
    /// Exact numeric with selectable precision
    DECIMAL,
    /// Alias for DECIMAL
    NUMERIC,
    /// 4-byte floating-point number
    REAL,
    /// 8-byte floating-point number
    DOUBLE_PRECISION,
    /// 2-byte autoincrementing integer
    SMALLSERIAL,
    /// 4-byte autoincrementing integer
    SERIAL,
    /// 8-byte autoincrementing integer
    BIGSERIAL,
    /// Variable-length character string with limit
    VARCHAR,
    /// Fixed-length character string, blank padded
    CHAR,
    /// Variable unlimited length character string
    TEXT,
    /// Binary data ("byte array")
    BYTEA,
    /// Date and time (without time zone)
    TIMESTAMP,
    /// Date and time with time zone
    TIMESTAMPTZ,
    /// Calendar date (year, month, day)
    DATE,
    /// Time of day (without time zone)
    TIME,
    /// Time of day with time zone
    TIMETZ,
    /// Time interval
    INTERVAL,
    /// Logical Boolean (true/false)
    BOOLEAN,
    /// User-defined enumerated type
    ENUM,
    /// Geometric point on a plane
    POINT,
    /// Infinite geometric line
    LINE,
    /// Geometric line segment
    LSEG,
    /// Rectangular geometric box
    BOX,
    /// Geometric path
    PATH,
    /// Geometric polygon
    POLYGON,
    /// Geometric circle
    CIRCLE,
    /// IPv4 or IPv6 network address
    CIDR,
    /// IPv4 or IPv6 host address
    INET,
    /// MAC address (6 bytes)
    MACADDR,
    /// MAC address (8 bytes, EUI-64 format)
    MACADDR8,
    /// Fixed-length bit string
    BIT,
    /// Variable-length bit string
    BIT_VARYING,
    /// Alias for BIT_VARYING
    VARBIT,
    /// Text search document
    TSVECTOR,
    /// Text search query
    TSQUERY,
    /// Universally unique identifier
    UUID,
    /// XML data
    XML,
    /// Textual JSON data
    JSON,
    /// Binary JSON data, decomposed
    JSONB,
    /// Array of data type
    ARRAY,
    /// User-defined composite type
    COMPOSITE,
    /// Range of integers (4-byte)
    INT4RANGE,
    /// Range of integers (8-byte)
    INT8RANGE,
    /// Range of numeric values
    NUMRANGE,
    /// Range of timestamp without time zone
    TSRANGE,
    /// Range of timestamp with time zone
    TSTZRANGE,
    /// Range of dates
    DATERANGE,
    /// User-defined domain type
    DOMAIN,
    /// Object identifier
    OID,
    /// Function name
    REGPROC,
    /// Function with argument types
    REGPROCEDURE,
    /// Operator name
    REGOPER,
    /// Operator with argument types
    REGOPERATOR,
    /// Relation name
    REGCLASS,
    /// Data type name
    REGTYPE,
    /// Role name
    REGROLE,
    /// Schema name
    REGNAMESPACE,
    /// Text search configuration
    REGCONFIG,
    /// Text search dictionary
    REGDICTIONARY,
    /// Postgres Log Sequence Number
    PG_LSN,
}
