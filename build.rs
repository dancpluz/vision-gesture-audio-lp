fn main() {
    println!("cargo:rustc-link-search=native=C:/tools/opencv/build/x64/vc16/lib");
    println!("cargo:rustc-link-lib=opencv_world4110");
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=shell32");
}