use std::io::Result;

fn main() -> Result<()> {
    protobuf_codegen::Codegen::new()
        .pure()
        .cargo_out_dir("protos")
        .include("proto")
        .input("proto/grafbase/options.proto")
        .run_from_script();

    Ok(())
}
