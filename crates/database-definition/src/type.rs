use std::fmt::Debug;

use grafbase_sdk::host_io::postgres::types::PgType;

use super::{EnumId, EnumWalker};

#[derive(Clone, Copy, PartialEq)]
pub enum DatabaseType<'a> {
    Scalar(ScalarType),
    Enum(EnumWalker<'a>),
}

impl Debug for DatabaseType<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Scalar(scalar) => scalar.fmt(f),
            DatabaseType::Enum(_) => f.debug_struct("EnumWalker").finish(),
        }
    }
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

    pub fn from_db_to_client_cast(&self) -> Option<&'static str> {
        match self {
            DatabaseType::Scalar(scalar) => scalar.from_db_to_client_cast(),
            DatabaseType::Enum(_) => None,
        }
    }

    pub fn enum_database_name(&'a self) -> Option<&'a str> {
        match self {
            DatabaseType::Scalar(_) => None,
            DatabaseType::Enum(enum_type) => Some(enum_type.database_name()),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseType::Scalar(scalar) => match scalar.kind {
                ScalarKind::Smallint => "SMALLINT",
                ScalarKind::Integer => "INTEGER",
                ScalarKind::Int => "INT",
                ScalarKind::Bigint => "BIGINT",
                ScalarKind::Decimal => "DECIMAL",
                ScalarKind::Numeric => "NUMERIC",
                ScalarKind::Real => "REAL",
                ScalarKind::DoublePrecision => "DOUBLE_PRECISION",
                ScalarKind::Smallserial => "SMALLSERIAL",
                ScalarKind::Serial => "SERIAL",
                ScalarKind::Bigserial => "BIGSERIAL",
                ScalarKind::Varchar => "VARCHAR",
                ScalarKind::Char => "CHAR",
                ScalarKind::Text => "TEXT",
                ScalarKind::Bytea => "BYTEA",
                ScalarKind::Timestamp => "TIMESTAMP",
                ScalarKind::Timestamptz => "TIMESTAMPTZ",
                ScalarKind::Date => "DATE",
                ScalarKind::Time => "TIME",
                ScalarKind::Timetz => "TIMETZ",
                ScalarKind::Interval => "INTERVAL",
                ScalarKind::Boolean => "BOOLEAN",
                ScalarKind::Enum => "ENUM",
                ScalarKind::Point => "POINT",
                ScalarKind::Line => "LINE",
                ScalarKind::Lseg => "LSEG",
                ScalarKind::Box => "BOX",
                ScalarKind::Path => "PATH",
                ScalarKind::Polygon => "POLYGON",
                ScalarKind::Circle => "CIRCLE",
                ScalarKind::Cidr => "CIDR",
                ScalarKind::Inet => "INET",
                ScalarKind::Macaddr => "MACADDR",
                ScalarKind::Macaddr8 => "MACADDR8",
                ScalarKind::Bit => "BIT",
                ScalarKind::BitVarying => "BIT VARYING",
                ScalarKind::Varbit => "VARBIT",
                ScalarKind::Tsvector => "TSVECTOR",
                ScalarKind::Tsquery => "TSQUERY",
                ScalarKind::Uuid => "UUID",
                ScalarKind::Xml => "XML",
                ScalarKind::Json => "JSON",
                ScalarKind::Jsonb => "JSONB",
                ScalarKind::Array => "ARRAY",
                ScalarKind::Composite => "COMPOSITE",
                ScalarKind::Int4range => "INT4RANGE",
                ScalarKind::Int8range => "INT8RANGE",
                ScalarKind::Numrange => "NUMRANGE",
                ScalarKind::Tsrange => "TSRANGE",
                ScalarKind::Tstzrange => "TSTZRANGE",
                ScalarKind::Daterange => "DATERANGE",
                ScalarKind::Domain => "DOMAIN",
                ScalarKind::Oid => "OID",
                ScalarKind::Regproc => "REGPROC",
                ScalarKind::Regprocedure => "REGPROCEDURE",
                ScalarKind::Regoper => "REGOPER",
                ScalarKind::Regoperator => "REGOPERATOR",
                ScalarKind::Regclass => "REGCLASS",
                ScalarKind::Regtype => "REGTYPE",
                ScalarKind::Regrole => "REGROLE",
                ScalarKind::Regnamespace => "REGNAMESPACE",
                ScalarKind::Regconfig => "REGCONFIG",
                ScalarKind::Regdictionary => "REGDICTIONARY",
                ScalarKind::PgLsn => "PG_LSN",
                ScalarKind::Money => "MONEY",
            },
            DatabaseType::Enum(_) => "ENUM",
        }
    }
}

impl From<DatabaseType<'_>> for PgType {
    fn from(value: DatabaseType<'_>) -> Self {
        match value {
            DatabaseType::Scalar(scalar) => match scalar.kind {
                ScalarKind::Smallint => PgType::Int16,
                ScalarKind::Integer => PgType::Int32,
                ScalarKind::Int => PgType::Int32,
                ScalarKind::Bigint => PgType::Int64,
                ScalarKind::Decimal => PgType::Decimal,
                ScalarKind::Numeric => PgType::Numeric,
                ScalarKind::Real => PgType::Float32,
                ScalarKind::DoublePrecision => PgType::Float64,
                ScalarKind::Smallserial => PgType::Int16,
                ScalarKind::Serial => PgType::Int32,
                ScalarKind::Bigserial => PgType::Int64,
                ScalarKind::Varchar => PgType::String,
                ScalarKind::Char => PgType::String,
                ScalarKind::Text => PgType::String,
                ScalarKind::Bytea => PgType::Bytes,
                ScalarKind::Timestamp => PgType::Timestamp,
                ScalarKind::Timestamptz => PgType::Timestamptz,
                ScalarKind::Date => PgType::Date,
                ScalarKind::Time => PgType::Time,
                ScalarKind::Timetz => PgType::Timetz,
                ScalarKind::Interval => PgType::Interval,
                ScalarKind::Boolean => PgType::Boolean,
                ScalarKind::Enum => PgType::String,
                ScalarKind::Point => PgType::Point,
                ScalarKind::Line => PgType::String,
                ScalarKind::Lseg => PgType::String,
                ScalarKind::Box => PgType::String,
                ScalarKind::Path => PgType::String,
                ScalarKind::Polygon => PgType::String,
                ScalarKind::Circle => PgType::String,
                ScalarKind::Cidr => PgType::Cidr,
                ScalarKind::Inet => PgType::Inet,
                ScalarKind::Macaddr => PgType::Macaddr,
                ScalarKind::Macaddr8 => PgType::Macaddr8,
                ScalarKind::Bit => PgType::Bit,
                ScalarKind::BitVarying => PgType::Varbit,
                ScalarKind::Varbit => PgType::Varbit,
                ScalarKind::Tsvector => PgType::String,
                ScalarKind::Tsquery => PgType::String,
                ScalarKind::Uuid => PgType::Uuid,
                ScalarKind::Xml => PgType::Xml,
                ScalarKind::Json => PgType::Json,
                ScalarKind::Jsonb => PgType::Jsonb,
                ScalarKind::Array => PgType::String,
                ScalarKind::Composite => PgType::String,
                ScalarKind::Int4range => PgType::String,
                ScalarKind::Int8range => PgType::String,
                ScalarKind::Numrange => PgType::String,
                ScalarKind::Tsrange => PgType::String,
                ScalarKind::Tstzrange => PgType::String,
                ScalarKind::Daterange => PgType::String,
                ScalarKind::Domain => PgType::String,
                ScalarKind::Oid => PgType::Oid,
                ScalarKind::Regproc => PgType::String,
                ScalarKind::Regprocedure => PgType::String,
                ScalarKind::Regoper => PgType::String,
                ScalarKind::Regoperator => PgType::String,
                ScalarKind::Regclass => PgType::String,
                ScalarKind::Regtype => PgType::String,
                ScalarKind::Regrole => PgType::String,
                ScalarKind::Regnamespace => PgType::String,
                ScalarKind::Regconfig => PgType::String,
                ScalarKind::Regdictionary => PgType::String,
                ScalarKind::PgLsn => PgType::String,
                ScalarKind::Money => PgType::Money,
            },
            DatabaseType::Enum(_) => PgType::String,
        }
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

impl EnumType {
    pub fn new(id: EnumId, is_array: bool) -> Self {
        EnumType { id, is_array }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScalarType {
    pub kind: ScalarKind,
    pub is_array: bool,
}

impl ScalarType {
    pub fn new(kind: ScalarKind, is_array: bool) -> Self {
        ScalarType { kind, is_array }
    }
}

impl ScalarType {
    pub fn is_binary(self) -> bool {
        matches!(self.kind, ScalarKind::Bytea)
    }

    pub fn is_json(&self) -> bool {
        matches!(self.kind, ScalarKind::Json)
    }

    pub fn is_jsonb(&self) -> bool {
        matches!(self.kind, ScalarKind::Jsonb)
    }

    pub fn client_type(self) -> Option<&'static str> {
        use ScalarKind::*;

        let type_name = match self.kind {
            Xml => "XML",
            Cidr => "CIDR",
            Macaddr | Macaddr8 => "MacAddr",
            Bit | Varbit => "BitString",
            Char | Text | Varchar => "String",
            Inet => "Inet",
            Date => "Date",
            Time => "Time",
            Timetz => "TimeWithTimezone",
            Timestamp => "Timestamp",
            Timestamptz => "DateTime",
            Uuid => "UUID",
            Oid | Bigint | Bigserial => "BigInt",
            Interval => "String",
            Money => "Money",
            Decimal | Numeric => "Decimal",
            Smallserial | Serial | Smallint | Int | Integer => "Int",
            Json | Jsonb => "JSON",
            Real | DoublePrecision => "Float",
            Boolean => "Boolean",
            Bytea => "Bytes",
            _ => return None,
        };

        Some(type_name)
    }

    /// Returns the PostgreSQL type name for explicit casting when reading from the database, if necessary.
    ///
    /// Some PostgreSQL types (like `Int64`) might need to be cast to a string representation (`TEXT`)
    /// when being fetched from the database to ensure compatibility with client-side languages
    /// (e.g., JavaScript, which has limitations with full 64-bit integers).
    /// This method provides the target type name for such casts.
    ///
    /// # Returns
    ///
    /// - `Some(&'static str)` containing the target type name string (e.g., "TEXT") if casting is needed.
    /// - `None` if the type does not require explicit casting when reading.
    pub fn from_db_to_client_cast(&self) -> Option<&'static str> {
        match self.kind {
            ScalarKind::Bigint | ScalarKind::Oid | ScalarKind::Numeric | ScalarKind::Decimal if self.is_array => {
                Some("TEXT[]")
            }
            ScalarKind::Bigint | ScalarKind::Oid | ScalarKind::Numeric | ScalarKind::Decimal => Some("TEXT"),
            _ => None,
        }
    }
}

/// Postgres data types supported for column definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ScalarKind {
    /// 2-byte signed integer, range: -32768 to +32767
    Smallint,
    /// 4-byte signed integer, range: -2147483648 to +2147483647
    Integer,
    /// Alias for INTEGER
    Int,
    /// 8-byte signed integer, range: -9223372036854775808 to +9223372036854775807
    Bigint,
    /// Exact numeric with selectable precision
    Decimal,
    /// Alias for DECIMAL
    Numeric,
    /// 4-byte floating-point number
    Real,
    /// 8-byte floating-point number
    DoublePrecision,
    /// 2-byte autoincrementing integer
    Smallserial,
    /// 4-byte autoincrementing integer
    Serial,
    /// 8-byte autoincrementing integer
    Bigserial,
    /// Variable-length character string with limit
    Varchar,
    /// Fixed-length character string, blank padded
    Char,
    /// Variable unlimited length character string
    Text,
    /// Binary data ("byte array")
    Bytea,
    /// Date and time (without time zone)
    Timestamp,
    /// Date and time with time zone
    Timestamptz,
    /// Calendar date (year, month, day)
    Date,
    /// Time of day (without time zone)
    Time,
    /// Time of day with time zone
    Timetz,
    /// Time interval
    Interval,
    /// Logical Boolean (true/false)
    Boolean,
    /// User-defined enumerated type
    Enum,
    /// Geometric point on a plane
    Point,
    /// Infinite geometric line
    Line,
    /// Geometric line segment
    Lseg,
    /// Rectangular geometric box
    Box,
    /// Geometric path
    Path,
    /// Geometric polygon
    Polygon,
    /// Geometric circle
    Circle,
    /// IPv4 or IPv6 network address
    Cidr,
    /// IPv4 or IPv6 host address
    Inet,
    /// MAC address (6 bytes)
    Macaddr,
    /// MAC address (8 bytes, EUI-64 format)
    Macaddr8,
    /// Fixed-length bit string
    Bit,
    /// Variable-length bit string
    BitVarying,
    /// Alias for BIT_VARYING
    Varbit,
    /// Text search document
    Tsvector,
    /// Text search query
    Tsquery,
    /// Universally unique identifier
    Uuid,
    /// XML data
    Xml,
    /// Textual JSON data
    Json,
    /// Binary JSON data, decomposed
    Jsonb,
    /// Array of data type
    Array,
    /// User-defined composite type
    Composite,
    /// Range of integers (4-byte)
    Int4range,
    /// Range of integers (8-byte)
    Int8range,
    /// Range of numeric values
    Numrange,
    /// Range of timestamp without time zone
    Tsrange,
    /// Range of timestamp with time zone
    Tstzrange,
    /// Range of dates
    Daterange,
    /// User-defined domain type
    Domain,
    /// Object identifier
    Oid,
    /// Function name
    Regproc,
    /// Function with argument types
    Regprocedure,
    /// Operator name
    Regoper,
    /// Operator with argument types
    Regoperator,
    /// Relation name
    Regclass,
    /// Data type name
    Regtype,
    /// Role name
    Regrole,
    /// Schema name
    Regnamespace,
    /// Text search configuration
    Regconfig,
    /// Text search dictionary
    Regdictionary,
    /// Postgres Log Sequence Number
    PgLsn,
    /// Currency amount
    Money,
}

impl From<u32> for ScalarType {
    fn from(value: u32) -> Self {
        let (kind, is_array) = match value {
            16 => (ScalarKind::Boolean, false),
            17 => (ScalarKind::Bytea, false),
            18 => (ScalarKind::Char, false),
            19 => (ScalarKind::Varchar, false),
            20 => (ScalarKind::Bigint, false),
            21 => (ScalarKind::Smallint, false),
            22 => (ScalarKind::Smallint, true),
            23 => (ScalarKind::Int, false),
            25 => (ScalarKind::Text, false),
            26 => (ScalarKind::Oid, false),
            30 => (ScalarKind::Oid, true),
            114 => (ScalarKind::Json, false),
            142 => (ScalarKind::Xml, false),
            143 => (ScalarKind::Xml, true),
            199 => (ScalarKind::Json, true),
            600 => (ScalarKind::Point, false),
            601 => (ScalarKind::Lseg, false),
            602 => (ScalarKind::Path, false),
            603 => (ScalarKind::Box, false),
            604 => (ScalarKind::Polygon, false),
            628 => (ScalarKind::Line, false),
            629 => (ScalarKind::Line, true),
            650 => (ScalarKind::Cidr, false),
            651 => (ScalarKind::Cidr, true),
            700 => (ScalarKind::Real, false),
            701 => (ScalarKind::DoublePrecision, false),
            705 => (ScalarKind::Text, false), // Unknown type, default to Text
            718 => (ScalarKind::Circle, false),
            719 => (ScalarKind::Circle, true),
            774 => (ScalarKind::Macaddr8, false),
            775 => (ScalarKind::Macaddr8, true),
            790 => (ScalarKind::Money, false), // Money
            791 => (ScalarKind::Money, true),  // Money array
            829 => (ScalarKind::Macaddr, false),
            869 => (ScalarKind::Inet, false),
            1000 => (ScalarKind::Boolean, true),
            1001 => (ScalarKind::Bytea, true),
            1002 => (ScalarKind::Char, true),
            1003 => (ScalarKind::Text, true),     // Name array
            1005 => (ScalarKind::Smallint, true), // Int2 array
            1006 => (ScalarKind::Smallint, true), // Int2Vector array
            1007 => (ScalarKind::Int, true),      // Int4 array
            1008 => (ScalarKind::Regproc, true),
            1009 => (ScalarKind::Text, true),
            1010 => (ScalarKind::Oid, true),  // Tid array
            1011 => (ScalarKind::Oid, true),  // Xid array
            1012 => (ScalarKind::Oid, true),  // Cid array
            1013 => (ScalarKind::Oid, true),  // OidVector array
            1014 => (ScalarKind::Char, true), // Bpchar array
            1015 => (ScalarKind::Varchar, true),
            1016 => (ScalarKind::Bigint, true), // Int8 array
            1017 => (ScalarKind::Point, true),
            1018 => (ScalarKind::Lseg, true),
            1019 => (ScalarKind::Path, true),
            1020 => (ScalarKind::Box, true),
            1021 => (ScalarKind::Real, true),            // Float4 array
            1022 => (ScalarKind::DoublePrecision, true), // Float8 array
            1027 => (ScalarKind::Polygon, true),
            1028 => (ScalarKind::Oid, true),
            1033 => (ScalarKind::Oid, false), // Aclitem
            1034 => (ScalarKind::Oid, true),  // Aclitem array
            1040 => (ScalarKind::Macaddr, true),
            1041 => (ScalarKind::Inet, true),
            1042 => (ScalarKind::Char, false), // Bpchar
            1043 => (ScalarKind::Varchar, false),
            1082 => (ScalarKind::Date, false),
            1083 => (ScalarKind::Time, false),
            1114 => (ScalarKind::Timestamp, false),
            1115 => (ScalarKind::Timestamp, true),
            1182 => (ScalarKind::Date, true),
            1183 => (ScalarKind::Time, true),
            1184 => (ScalarKind::Timestamptz, false),
            1185 => (ScalarKind::Timestamptz, true),
            1186 => (ScalarKind::Interval, false),
            1187 => (ScalarKind::Interval, true),
            1231 => (ScalarKind::Numeric, true),
            1263 => (ScalarKind::Text, true), // Cstring array
            1266 => (ScalarKind::Timetz, false),
            1270 => (ScalarKind::Timetz, true),
            1560 => (ScalarKind::Bit, false),
            1561 => (ScalarKind::Bit, true),
            1562 => (ScalarKind::Varbit, false),
            1563 => (ScalarKind::Varbit, true),
            1700 => (ScalarKind::Numeric, false),
            1790 => (ScalarKind::Text, false), // Refcursor
            2201 => (ScalarKind::Text, true),  // Refcursor array
            2202 => (ScalarKind::Regprocedure, false),
            2203 => (ScalarKind::Regoper, false),
            2204 => (ScalarKind::Regoperator, false),
            2205 => (ScalarKind::Regclass, false),
            2206 => (ScalarKind::Regtype, false),
            2207 => (ScalarKind::Regprocedure, true),
            2208 => (ScalarKind::Regoper, true),
            2209 => (ScalarKind::Regoperator, true),
            2210 => (ScalarKind::Regclass, true),
            2211 => (ScalarKind::Regtype, true),
            2249 => (ScalarKind::Text, false),  // Record
            2275 => (ScalarKind::Text, false),  // Cstring
            2276 => (ScalarKind::Text, false),  // Any
            2277 => (ScalarKind::Array, false), // Anyarray
            2278 => (ScalarKind::Text, false),  // Void
            2279 => (ScalarKind::Text, false),  // Trigger
            2280 => (ScalarKind::Text, false),  // Language handler
            2281 => (ScalarKind::Text, false),  // Internal
            2283 => (ScalarKind::Text, false),  // Anyelement
            2287 => (ScalarKind::Text, true),   // Record array
            2776 => (ScalarKind::Text, false),  // Anynonarray
            2949 => (ScalarKind::Text, true),   // TxidSnapshot array
            2950 => (ScalarKind::Uuid, false),
            2951 => (ScalarKind::Uuid, true),
            2970 => (ScalarKind::Text, false), // TxidSnapshot
            3115 => (ScalarKind::Text, false), // FdwHandler
            3220 => (ScalarKind::PgLsn, false),
            3221 => (ScalarKind::PgLsn, true),
            3310 => (ScalarKind::Text, false), // TsmHandler
            3361 => (ScalarKind::Text, false), // PgNdistinct
            3402 => (ScalarKind::Text, false), // PgDependencies
            3500 => (ScalarKind::Enum, false), // Anyenum
            3614 => (ScalarKind::Tsvector, false),
            3615 => (ScalarKind::Tsquery, false),
            3642 => (ScalarKind::Tsvector, false), // GtsVector
            3643 => (ScalarKind::Tsvector, true),
            3644 => (ScalarKind::Tsvector, true), // GtsVector array
            3645 => (ScalarKind::Tsquery, true),
            3734 => (ScalarKind::Regconfig, false),
            3735 => (ScalarKind::Regconfig, true),
            3769 => (ScalarKind::Regdictionary, false),
            3770 => (ScalarKind::Regdictionary, true),
            3802 => (ScalarKind::Jsonb, false),
            3807 => (ScalarKind::Jsonb, true),
            3831 => (ScalarKind::Text, false), // AnyRange
            3838 => (ScalarKind::Text, false), // EventTrigger
            3904 => (ScalarKind::Int4range, false),
            3905 => (ScalarKind::Int4range, true),
            3906 => (ScalarKind::Numrange, false),
            3907 => (ScalarKind::Numrange, true),
            3908 => (ScalarKind::Tsrange, false),
            3909 => (ScalarKind::Tsrange, true),
            3910 => (ScalarKind::Tstzrange, false),
            3911 => (ScalarKind::Tstzrange, true),
            3912 => (ScalarKind::Daterange, false),
            3913 => (ScalarKind::Daterange, true),
            3926 => (ScalarKind::Int8range, false),
            3927 => (ScalarKind::Int8range, true),
            4072 => (ScalarKind::Text, false), // Jsonpath
            4073 => (ScalarKind::Text, true),  // Jsonpath array
            4989 => (ScalarKind::Regnamespace, false),
            4090 => (ScalarKind::Regnamespace, true),
            4096 => (ScalarKind::Regrole, false),
            4097 => (ScalarKind::Regrole, true),
            4191 => (ScalarKind::Text, false), // Regcollation
            4192 => (ScalarKind::Text, true),  // Regcollation array
            4451 => (ScalarKind::Text, false), // Int4multiRange
            4532 => (ScalarKind::Text, false), // NummultiRange
            4533 => (ScalarKind::Text, false), // TsmultiRange
            4534 => (ScalarKind::Text, false), // TstzmultiRange
            4535 => (ScalarKind::Text, false), // DatemultiRange
            4536 => (ScalarKind::Text, false), // Int8multiRange
            4537 => (ScalarKind::Text, false), // AnymultiRange
            4538 => (ScalarKind::Text, false), // AnycompatiblemultiRange
            4600 => (ScalarKind::Text, false), // PgBrinBloomSummary
            4601 => (ScalarKind::Text, false), // PgBrinMinmaxMultiSummary
            5017 => (ScalarKind::Text, false), // PgMcvList
            5038 => (ScalarKind::Text, false), // PgSnapshot
            5039 => (ScalarKind::Text, true),  // PgSnapshot array
            5069 => (ScalarKind::Text, false), // Xid8
            5077 => (ScalarKind::Text, false), // Anycompatible
            5078 => (ScalarKind::Text, false), // Anycompatiblearray
            5079 => (ScalarKind::Text, false), // Anycompatiblenonarray
            5080 => (ScalarKind::Text, false), // AnycompatibleRange
            6150 => (ScalarKind::Text, true),  // Int4multiRange array
            6151 => (ScalarKind::Text, true),  // NummultiRange array
            6152 => (ScalarKind::Text, true),  // TsmultiRange array
            6153 => (ScalarKind::Text, true),  // TstzmultiRange array
            6155 => (ScalarKind::Text, true),  // DatemultiRange array
            6157 => (ScalarKind::Text, true),  // Int8multiRange array
            _ => (ScalarKind::Text, false),    // Default to Text for unknown types
        };

        Self { kind, is_array }
    }
}
