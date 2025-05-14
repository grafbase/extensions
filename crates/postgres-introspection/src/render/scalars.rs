use super::ast::{scalar::Scalar, schema::Schema};

pub(super) fn render(rendered: &mut Schema) {
    rendered.push_scalar({
        let mut scalar = Scalar::new("JSON");
        scalar.set_description("Arbitrary JSON object");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Bytes");
        scalar.set_description("Binary data type, represented as a string containing a hexadecimal value");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("BigInt");
        scalar.set_description("Big integer data type, represented as a string containing a numeric value");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Decimal");
        scalar.set_description(
            "Decimal data type with arbitrary precision, represented as a string containing a numeric value",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("UUID");
        scalar.set_description(
            "UUID data type represented as a string in the format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Date");
        scalar.set_description("Date data type represented as a string in ISO 8601 format (YYYY-MM-DD)");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Time");
        scalar.set_description("Time data type represented as a string in ISO 8601 format (HH:MM:SS or HH:MM:SS.sss)");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("TimeWithTimezone");
        scalar.set_description(
            "Time with time zone data type represented as a string in format (HH:MM:SS.sssZ or HH:MM:SS.sss+HH:MM)",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Timestamp");
        scalar.set_description(
            "Timestamp data type represented as a string in ISO 8601 format (YYYY-MM-DDTHH:MM:SS.sss)",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("DateTime");
        scalar
            .set_description("DateTime with time zone data type represented as a string in ISO 8601 format (YYYY-MM-DDTHH:MM:SS.sssZ or YYYY-MM-DDTHH:MM:SS.sss+HH:MM)");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Inet");
        scalar.set_description(
            "IPv4 or IPv6 network address represented as a string (e.g., '192.168.0.1' or '2001:db8::1')",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("CIDR");
        scalar.set_description(
            "IPv4 or IPv6 network address space represented as a string (e.g., '192.168.0.1/24' or '2001:db8::1/64')",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("MacAddr");
        scalar.set_description("MAC address data type represented as a string in the format 'XX:XX:XX:XX:XX:XX'");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("Money");
        scalar.set_description(
            "Currency amount data type represented as a string with a numeric value and optional currency symbol",
        );
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("BitString");
        scalar.set_description("Bit string data type represented as a string of 0s and 1s");
        scalar
    });

    rendered.push_scalar({
        let mut scalar = Scalar::new("XML");
        scalar.set_description("XML data type represented as a string");
        scalar
    });
}
