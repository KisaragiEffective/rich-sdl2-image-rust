fn main() {
    use git2::Repository;
    use std::env;
    use std::path::PathBuf;
    use std::process;

    let root_dir = env::var("OUT_DIR").expect("OUT_DIR not found");
    let root = PathBuf::from(&root_dir);

    let sdl_dir = root.join("SDL2");
    let _ = Repository::clone("https://github.com/libsdl-org/SDL", &sdl_dir);
    let _ = process::Command::new(sdl_dir.join("configure"))
        .arg(format!("--prefix={}", root_dir))
        .current_dir(&sdl_dir)
        .output()
        .expect("failed to configure");
    let _ = process::Command::new("make")
        .arg("-j4")
        .current_dir(&sdl_dir)
        .output()
        .expect("failed to make");
    let _ = process::Command::new("make")
        .arg("install")
        .current_dir(&sdl_dir)
        .output()
        .expect("failed to install");

    let sdl_image_dir = root.join("SDL2_image");
    let _ = Repository::clone("https://github.com/libsdl-org/SDL_image", &sdl_image_dir);
    let _ = process::Command::new(sdl_image_dir.join("configure"))
        .arg(format!("--prefix={}", root_dir))
        .current_dir(&sdl_image_dir)
        .env("SDL2_DIR", &sdl_dir)
        .output()
        .expect("failed to configure");
    let _ = process::Command::new("make")
        .arg("-j4")
        .current_dir(&sdl_image_dir)
        .output()
        .expect("failed to make");

    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=SDL2_image");
    println!(
        "cargo:rustc-link-search={}",
        root.join("SDL2/build/.libs").as_path().to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search={}",
        root.join("SDL2_image/.libs").as_path().to_string_lossy()
    );
    println!(
        "cargo:rustc-link-search={}",
        root.join("lib").as_path().to_string_lossy()
    );
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(&format!("-I{}/SDL2/include", root_dir))
        .clang_arg(&format!("-I{}/SDL2_image", root_dir))
        .allowlist_function("IMG_.*")
        .allowlist_function("SDL_FreeSurface")
        .allowlist_function("SDL_RWFromFile")
        .allowlist_type("IMG_.*")
        .allowlist_var("IMG_.*")
        .generate_comments(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("bindgen builder was invalid");

    bindings
        .write_to_file(root.join("bind.rs"))
        .expect("`src` directory not found");
}
