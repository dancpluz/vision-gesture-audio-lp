fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    // Detectar sistema operacional
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    
    match target_os.as_str() {
        "windows" => {
            // Windows: tentar múltiplos caminhos
            let possible_paths = [
                "C:\\tools\\opencv\\build\\x64\\vc16\\lib",
                "C:\\opencv\\build\\x64\\vc16\\lib",
                &std::env::var("OPENCV_LINK_PATHS").unwrap_or_default(),
                "C:\\Program Files\\OpenCV\\lib",
            ];
            
            for path in &possible_paths {
                if !path.is_empty() && std::path::Path::new(path).exists() {
                    println!("cargo:rustc-link-search=native={}", path);
                    println!("cargo:rustc-link-lib=opencv_world4110");
                    break;
                }
            }
            
            // Bibliotecas do Windows necessárias
            println!("cargo:rustc-link-lib=kernel32");
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=shell32");
        }
        
        "linux" => {
            // Linux: usar pkg-config
            if let Ok(output) = std::process::Command::new("pkg-config")
                .args(&["--libs", "--cflags", "opencv4"])
                .output()
            {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for flag in output_str.split_whitespace() {
                        if flag.starts_with("-L") {
                            let lib_path = &flag[2..];
                            println!("cargo:rustc-link-search=native={}", lib_path);
                        } else if flag.starts_with("-l") {
                            let lib_name = &flag[2..];
                            println!("cargo:rustc-link-lib={}", lib_name);
                        }
                    }
                    return;
                }
            }
            
            // Fallback para caminhos comuns do Linux
            let linux_paths = [
                "/usr/lib/x86_64-linux-gnu",
                "/usr/local/lib",
                "/usr/lib",
                "/lib/x86_64-linux-gnu",
            ];
            
            for path in &linux_paths {
                if std::path::Path::new(path).exists() {
                    println!("cargo:rustc-link-search=native={}", path);
                }
            }
            
            // Bibliotecas OpenCV comuns no Linux
            let linux_libs = [
                "opencv_core",
                "opencv_highgui",
                "opencv_imgproc",
                "opencv_videoio",
                "opencv_imgcodecs",
            ];
            
            for lib in &linux_libs {
                println!("cargo:rustc-link-lib={}", lib);
            }
        }
        
        _ => {
            // Outros sistemas
            println!("cargo:warning=OS não suportado: {}", target_os);
        }
    }
}