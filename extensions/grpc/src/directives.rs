#[derive(Debug, serde::Deserialize)]
pub(crate) struct GrpcMethod {
    pub(crate) service: String,
    pub(crate) method: String,
    // Note: there is an `input` field. It is left out to be serialized separately, directly into a protocol buffer. The service and method fields are required for that last step.
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoMessages {
    pub(crate) definitions: Vec<ProtoMessageDefinition>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoMessageDefinition {
    pub(crate) name: String,
    pub(crate) fields: Vec<ProtoField>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoField {
    pub(crate) name: String,
    pub(crate) number: u32,
    #[serde(default)]
    pub(crate) repeated: bool,
    pub(crate) r#type: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoServices {
    pub(crate) definitions: Vec<ProtoServiceDefinition>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoServiceDefinition {
    pub(crate) name: String,
    pub(crate) methods: Vec<ProtoMethodDefinition>,
}

impl ProtoServiceDefinition {
    pub(crate) fn get_method(&self, name: &str) -> Option<&ProtoMethodDefinition> {
        self.methods.iter().find(|m| m.name == name)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProtoMethodDefinition {
    pub(crate) name: String,
    pub(crate) input_type: String,
    pub(crate) output_type: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoEnums {
    pub(crate) definitions: Vec<ProtoEnumDefinition>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoEnumDefinition {
    pub(crate) name: String,
    pub(crate) values: Vec<ProtoEnumValueDefinition>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ProtoEnumValueDefinition {
    pub(crate) name: String,
    pub(crate) number: u32,
}
