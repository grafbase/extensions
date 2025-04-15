use super::FieldType;
use std::collections::HashMap;

pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) ty: FieldType,
    pub(crate) number: u32,
    pub(crate) repeated: bool,
}

pub(crate) struct Message {
    pub(crate) name: String,
    pub(crate) fields: HashMap<String, Field>,
}

pub(crate) struct EnumDefinition {
    pub(crate) name: String,
    pub(crate) values: Vec<EnumValueDefinition>,
}

pub(crate) struct EnumValueDefinition {
    pub(crate) name: String,
    pub(crate) number: u32,
}

pub(crate) type Service = crate::directives::ProtoServiceDefinition;
