use wgpu::BindGroupLayout as WgpuBindGroupLayout;

use std::any::Any;

pub trait BindGroupLayout: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;

    fn get(&self) -> &WgpuBindGroupLayout;

    fn group(&self) -> u32;
}
