use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    let kernel_path = "kernels/solver.cu";
    let ptx_path = out_dir.join("solver.ptx");
    
    println!("cargo:rerun-if-changed={}", kernel_path);
    
    // Detect GPU architecture or use environment variable, default to sm_75
    let arch = env::var("CUDA_ARCH").unwrap_or_else(|_| {
        // Try to detect GPU architecture using nvidia-smi
        if let Ok(output) = Command::new("nvidia-smi")
            .args(["--query-gpu=compute_cap", "--format=csv,noheader"])
            .output()
        {
            if output.status.success() {
                let cap = String::from_utf8_lossy(&output.stdout);
                let cap = cap.trim().lines().next().unwrap_or("7.5");
                let sm = cap.replace(".", "");
                return format!("sm_{}", sm);
            }
        }
        "sm_75".to_string()
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
