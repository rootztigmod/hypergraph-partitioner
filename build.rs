use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    let kernel_path = "kernels/solver.cu";
    let ptx_path = out_dir.join("solver.ptx");
    
    println!("cargo:rerun-if-changed={}", kernel_path);
    
    // Detect GPU architecture or use environment variable, default to sm_89
    // Note: sm_120 (Blackwell) requires CUDA 12.8+, so we cap at sm_89 for compatibility
    let arch = env::var("CUDA_ARCH").unwrap_or_else(|_| {
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=compute_cap", "--format=csv,noheader"])
            .output()
        {
            if output.status.success() {
                let cap = String::from_utf8_lossy(&output.stdout);
                let cap = cap.trim().lines().next().unwrap_or("8.9");
                let major_minor: Vec<&str> = cap.split('.').collect();
                if major_minor.len() == 2 {
                    let major: u32 = major_minor[0].parse().unwrap_or(8);
                    let minor: u32 = major_minor[1].parse().unwrap_or(9);
                    // Cap at sm_89 (Ada) for CUDA 12.0-12.7 compatibility
                    if major > 8 || (major == 8 && minor > 9) {
                        return "sm_89".to_string();
                    }
                    return format!("sm_{}{}", major, minor);
                }
            }
        }
        "sm_89".to_string()
    });
    
    println!("cargo:warning=Compiling CUDA kernels for {}", arch);
    
    let status = Command::new("nvcc")
        .args([
            "-ptx",
            &format!("-arch={}", arch),
            "-O3",
            "--use_fast_math",
            "-o",
            ptx_path.to_str().unwrap(),
            kernel_path,
        ])
        .status()
        .expect("Failed to run nvcc. Make sure CUDA toolkit is installed.");
    
    if !status.success() {
        panic!("nvcc failed to compile {}", kernel_path);
    }
    
    println!("cargo:rustc-env=OUT_DIR={}", out_dir.display());
}
