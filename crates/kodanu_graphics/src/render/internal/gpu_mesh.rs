use crate::Mesh;

use wgpu::{Buffer as WgpuBuffer, BufferUsages as WgpuBufferUsages, Device as WgpuDevice};

use wgpu::util::{BufferInitDescriptor as WgpuBufferInitDescriptor, DeviceExt as WgpuDeviceExt};

use bytemuck::cast_slice;

pub(crate) struct GpuMesh {
    vertex_buffer: WgpuBuffer,
    index_buffer: WgpuBuffer,
    index_count: u32,
}

impl GpuMesh {
    pub fn new(device: &WgpuDevice, mesh: &Mesh) -> Self {
        let vertex_buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(mesh.vertices()),
            usage: WgpuBufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&WgpuBufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: cast_slice(mesh.indices()),
            usage: WgpuBufferUsages::INDEX,
        });

        let index_count = mesh.indices().len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
        }
    }
}

impl GpuMesh {
    pub fn vertex_buffer(&self) -> &WgpuBuffer {
        &self.vertex_buffer
    }

    pub fn index_buffer(&self) -> &WgpuBuffer {
        &self.index_buffer
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }
}
