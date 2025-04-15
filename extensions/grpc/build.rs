// This is for tests only.
fn main() {
    tonic_build::configure()
        .type_attribute("routeguide.Point", "#[derive(Hash)]")
        .compile_protos(&["tests/proto/routeguide/route_guide.proto"], &["proto"])
        .unwrap();
}
