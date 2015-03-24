use std::process::{Command, Stdio};
use std::fs;
use std::path::Path;

trait PlatformArgs {
    fn platform_args<'a>(&'a mut self) -> &'a mut Self;
}

impl PlatformArgs for Command {
    #[cfg(not(windows))]
    fn platform_args<'a>(&'a mut self) -> &'a mut Command {
        self
    }
    
    #[cfg(all(windows, target_pointer_width = "64"))]
    fn platform_args<'a>(&'a mut self) -> &'a mut Command {
        self.arg("-G").arg("Visual Studio 12 Win64")
    }

    #[cfg(all(windows, target_pointer_width = "32"))]
    fn platform_args<'a>(&'a mut self) -> &'a mut Command {
        self.arg("-G").arg("Visual Studio 12")
    }
}

#[cfg(not(windows))]
fn export() {
    let lib = Path::new(env!("OUT_DIR")).join("output");
    println!("cargo:rustc-flags=-L {} -l ovr:dylib", lib.as_str().expect("Invalid path string"));
}

#[cfg(windows)]
fn export() {
    // ovr.dll must move - otherwise ld will deal poorly with the ambiguity over both
    // .lib and .dll files, and mess up the IAT
    fs::copy(&Path::new(env!("OUT_DIR")).join("output").join("ovr.dll"),
             &Path::new(env!("OUT_DIR")).join("ovr.dll"))
        .ok().expect("failed to move ovr.dll");
    println!("cargo:rustc-flags=-L {} -l ovr:dylib", env!("OUT_DIR"));
}

fn main() {
    Command::new("cmake")
        .platform_args()
        .arg("-DBUILD_SHARED_LIBS=TRUE")
        .arg("-DOCULUS_BUILD_SAMPLES=FALSE")
        .arg(env!("CARGO_MANIFEST_DIR"))
        .current_dir(&Path::new(env!("OUT_DIR")))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .ok().expect("CMake failed for Oculus SDK");

    Command::new("cmake")
        .arg("--build")
        .arg(&Path::new(env!("OUT_DIR")))
        .arg("--config").arg("Release")
        .arg("--clean-first")
        .current_dir(&Path::new(env!("OUT_DIR")))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .ok().expect("CMake --build failed for Oculus SDK");

    export();
}
