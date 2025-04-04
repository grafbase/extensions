use super::ast::{scalar::Scalar, schema::Schema};

pub(super) fn render(rendered: &mut Schema) {
    rendered.push_scalar({
        let mut scalar = Scalar::new("JSON");
        scalar.set_description("JSON data type");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Bytes");
        scalar.set_description("Binary data type");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("BigInt");
        scalar.set_description("Big integer data type");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Decimal");
        scalar.set_description("Decimal data type");
        scalar
    });
}
