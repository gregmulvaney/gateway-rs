fn main() {
    tonic_build::compile_protos("../proto/gateway.proto").expect("Error compiling proto files");
}
