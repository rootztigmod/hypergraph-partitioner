use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    let kernel_path = "kernels/solver.cu";
    let ptx_path = out_dir.join("solver.ptx");
    
    println!("cargo:rerun-if-changed={}", kernel_path);
    
    // Use sm_75 (Turing) for broad compatibility across RTX 20/30/40/50 series.
    // Override with CUDA_ARCH environment variable if needed.
    let arch = env::var("CUDA_ARCH").unwrap_or_else(|_| "sm_75".to_string());
    
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
