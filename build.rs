use std::{env, path::PathBuf};

fn main() {
  let build_output_path = build_the_c_library();
  println!("build_output_path: {}", build_output_path.display());

  println!("cargo:rustc-link-search={}", build_output_path.join("lib").display());

  let target = env::var("TARGET").unwrap();
  println!("target:{target}");

  // STATIC LINK
  if target.contains("windows") {
    println!("cargo:rustc-link-lib=static=SDL3-static");
    for win_lib in [
      "user32", "gdi32", "winmm", "imm32", "ole32", "oleaut32", "version", "uuid", "advapi32",
      "setupapi", "shell32",
    ] {
      println!("cargo:rustc-link-lib={win_lib}");
    }
  } else {
    println!("cargo:rustc-link-lib=static=SDL3");
  }
}

/// Builds SDL3 via `cmake`.
///
/// * **Returns:** The `PathBuf` of where `cmake` put the build
fn build_the_c_library() -> PathBuf {
  let out_dir = env::var("OUT_DIR").unwrap();
  println!("out_dir:{out_dir}");

  let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  println!("cargo_manifest_dir:{cargo_manifest_dir}");

  let target = env::var("TARGET").unwrap();
  println!("target:{target}");

  let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
  println!("target_os:{target_os}");

  let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
  println!("target_arch:{target_arch}");

  let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
  println!("target_vendor:{target_vendor}");

  let mut cm = cmake::Config::new(std::path::Path::new(&cargo_manifest_dir).join("c-sdl3"));
  cm.profile("Release");
  cm.static_crt(true);
  cm.target(&target);
  cm.define("SDL_SHARED", "ON");
  cm.define("SDL_STATIC", "ON");
  // We need to set extra CMake options when building for Apple platforms.
  if target_vendor == "apple" {
    // CMake can handle the x86_64/aarch64 duality of Apple platforms, but
    // needs to be told which architecture(s) we want. Since rust doesn't
    // support fat binaries, we'll only set the one architecture
    // requested. See: https://github.com/rust-lang/cargo/issues/8875
    match target_arch.as_str() {
      "aarch64" => {
        cm.define("CMAKE_OSX_ARCHITECTURES", "arm64");
      }
      "x86_64" => {
        cm.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
      }
      arch => {
        println!("Unrecognized architecture for Apple platform `{arch}`, not setting `CMAKE_OSX_ARCHITECTURES`");
      }
    }
  }

  cm.build()
}
