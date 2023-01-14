use cmake::Config;
use std::env;
use std::path::Path;

fn main() {
    let static_link_libs = [ "glfw3" ];
    let dynamic_link_libs = [ "X11"];
    let project_name = "division_engine_c";
    let out_dir = env::var("OUT_DIR").unwrap();

    Config::new(project_name)
        .no_build_target(true)
        .out_dir(&out_dir)
        .build();

    let build_path = Path::new(&out_dir).join("build");
    println!("cargo:rustc-link-search=native={}", build_path.to_str().unwrap());

    println!("cargo:rustc-link-lib=static={}", project_name);
    for lib_name in static_link_libs {
        println!("cargo:rustc-link-lib=static={}", lib_name);
    }

    for lib_name in dynamic_link_libs {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }
}