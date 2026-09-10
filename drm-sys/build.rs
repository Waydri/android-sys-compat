use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let mut builder = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <drm.h>\n#include <drm_mode.h>")
        .clang_arg("-I/usr/include/libdrm")
        .allowlist_type("DRM_.*|drm_.*")
        .allowlist_var("DRM_.*|drm_.*");

    if target_os == "android" {
        let clang_target = match target_arch.as_str() {
            "x86" => "i686-linux-android",
            "x86_64" => "x86_64-linux-android",
            "aarch64" => "aarch64-linux-android",
            "arm" => "armv7a-linux-androideabi",
            other => panic!("Unsupported Android arch: {}", other),
        };

        let ndk_home = env::var("ANDROID_NDK_HOME")
            .expect("ANDROID_NDK_HOME must be set");

        let sysroot = PathBuf::from(&ndk_home)
            .join("toolchains")
            .join("llvm")
            .join("prebuilt")
            .join("linux-x86_64")
            .join("sysroot");

        let arch_include = sysroot.join("usr").join("include").join(clang_target);

        builder = builder
            .clang_arg("-D__ANDROID__")
            .clang_arg("-D__linux__")
            .clang_arg("-target")
            .clang_arg(clang_target)
            .clang_arg(format!("--sysroot={}", sysroot.display()))
            .clang_arg(format!("-I{}", arch_include.display()));
    }

    let bindings = builder.generate().expect("Unable to generate drm bindings");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out.join("bindings.rs")).unwrap();
}
