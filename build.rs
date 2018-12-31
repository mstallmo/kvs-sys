use bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=static=producer");
    println!("cargo:rustc-link-search=native=amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-native-build");

    //TODO: handle building the library on each platform
    //TODO: handle compilation of kinesis-video-pic on Mac vs Linux vs Windows
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include")
        .clang_arg("-I./amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-pic/src/client/include")
        .clang_arg("-I./amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-pic/src/common/include")
        .clang_arg("-I./amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-pic/src/utils/include")
        .clang_arg("-I./amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-pic/src/mkvgen/include")
        .clang_arg("-I./amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-pic/src/view/include")
        .clang_arg("-I./amazon-kinesis-video-streams-producer-sdk-cpp/kinesis-video-pic/src/heap/include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("could not write bindings");
}
