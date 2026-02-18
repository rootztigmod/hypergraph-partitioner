use anyhow::{anyhow, Result};
use cudarc::driver::{CudaContext, CudaModule, CudaStream};
use cudarc::nvrtc::Ptx;
use cudarc::runtime::result::device::get_device_prop;
use cudarc::runtime::sys::cudaDeviceProp;
use std::sync::Arc;

pub struct GpuContext {
    #[allow(dead_code)]
    pub ctx: Arc<CudaContext>,
    pub stream: Arc<CudaStream>,
    pub module: Arc<CudaModule>,
    pub prop: cudaDeviceProp,
}

impl GpuContext {
    pub fn new() -> Result<Self> {
        let num_gpus = CudaContext::device_count().map_err(|e| anyhow!("Failed to get device count: {:?}", e))?;
        if num_gpus == 0 {
            return Err(anyhow!("No CUDA devices found"));
        }
        
        let ctx = CudaContext::new(0).map_err(|e| anyhow!("Failed to create CUDA context: {:?}", e))?;
        
        let ptx_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/solver.ptx"));
        let ptx = Ptx::from_src(std::str::from_utf8(ptx_bytes).map_err(|e| anyhow!("Invalid PTX: {}", e))?);
        
        let module = ctx.load_module(ptx).map_err(|e| anyhow!("Failed to load PTX module: {:?}", e))?;
        let stream = ctx.default_stream();
        let prop = get_device_prop(0).map_err(|e| anyhow!("Failed to get device properties: {:?}", e))?;
        
        Ok(Self {
            ctx,
            stream,
            module,
            prop,
        })
    }
}
