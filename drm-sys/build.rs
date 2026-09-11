use std::env;
use std::path::PathBuf;

const HEADER: &str = r#"
#include <drm.h>
#include <drm_mode.h>

#undef DRM_MODE_PROP_SIGNED_RANGE
const unsigned int DRM_MODE_PROP_SIGNED_RANGE = (1 << 6);

#undef DRM_MODE_PROP_OBJECT
const unsigned int DRM_MODE_PROP_OBJECT = (1 << 8);

#undef DRM_PLANE_TYPE_OVERLAY
const unsigned int DRM_PLANE_TYPE_OVERLAY = 0;

#undef DRM_PLANE_TYPE_PRIMARY
const unsigned int DRM_PLANE_TYPE_PRIMARY = 1;

#undef DRM_PLANE_TYPE_CURSOR
const unsigned int DRM_PLANE_TYPE_CURSOR = 2;
"#;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let mut builder = bindgen::Builder::default()
        .header_contents("bindings.h", HEADER)
        .ctypes_prefix("libc")
        .prepend_enum_name(false)
        .layout_tests(false)
        .generate_comments(false)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_hash(true)
        .derive_eq(true)
        .allowlist_recursively(true)
        .blocklist_type("drm_control_DRM_ADD_COMMAND")
        .allowlist_type("DRM_.*|drm_.*")
        .allowlist_var("DRM_.*|drm_.*")
        .constified_enum_module("drm_control_.*")
        .constified_enum_module("drm_buf_desc_.*")
        .constified_enum_module("drm_map_type")
        .constified_enum_module("drm_map_flags")
        .constified_enum_module("drm_stat_type")
        .constified_enum_module("drm_lock_flags")
        .constified_enum_module("drm_dma_flags")
        .constified_enum_module("drm_ctx_flags")
        .constified_enum_module("drm_drawable_info_type_t")
        .constified_enum_module("drm_vblank_seq_type")
        .constified_enum_module("drm_mode_subconnector")
        .clang_arg("-I/usr/include/libdrm");

    if target_os == "android" {
        let clang_target = match target_arch.as_str() {
            "x86" => "i686-linux-android",
            "x86_64" => "x86_64-linux-android",
            "aarch64" => "aarch64-linux-android",
            "arm" => "armv7a-linux-androideabi",
            other => panic!("Unsupported Android arch: {}", other),
        };

        let ndk_home = env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME must be set");
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
