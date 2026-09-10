fn main() {
    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <drm.h>\n#include <drm_mode.h>")
        .clang_arg("-I/usr/include/libdrm")
        .allowlist_type("DRM_.*|drm_.*")
        .allowlist_var("DRM_.*|drm_.*")
        .generate()
        .expect("Unable to generate drm bindings");

    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs")).unwrap();
}
