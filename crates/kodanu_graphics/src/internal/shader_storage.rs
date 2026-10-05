#![allow(dead_code)]

use wgpu::{
    Device as WgpuDevice, ShaderModule as WgpuShaderModule,
    ShaderModuleDescriptor as WgpuShaderModuleDescriptor, ShaderSource as WgpuShaderSource,
};

pub struct ShaderStorage;

impl ShaderStorage {
    pub fn vertex(device: &WgpuDevice) -> WgpuShaderModule {
        device.create_shader_module(WgpuShaderModuleDescriptor {
            label: Some("Vertex Shader"),
            source: WgpuShaderSource::Wgsl(include_str!("../wgsl/mesh.vert.wgsl").into()),
        })
    }

    pub fn fragment(device: &WgpuDevice) -> WgpuShaderModule {
        device.create_shader_module(WgpuShaderModuleDescriptor {
            label: Some("Fragment Shader"),
            source: WgpuShaderSource::Wgsl(include_str!("../wgsl/mesh.frag.wgsl").into()),
        })
    }

    pub fn normal(device: &WgpuDevice) -> WgpuShaderModule {
        device.create_shader_module(WgpuShaderModuleDescriptor {
            label: Some("Normal Shader"),
            source: WgpuShaderSource::Wgsl(include_str!("../wgsl/mesh.normal.wgsl").into()),
        })
    }
}
