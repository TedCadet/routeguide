fn main() {
    tonic_build::compile_protos("proto/route_guide.proto")
        .unwrap_or_else(|err| panic!("Failed to compile protos {:?}", err));
}
