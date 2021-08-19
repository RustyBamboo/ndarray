use crate::Array;
use crate::Dimension;
use crate::WgpuArray;
use crate::WgpuDevice;
use crate::WgpuRepr;
use std::marker::PhantomData;

impl<A, D> WgpuArray<'_, A, D>
where
    A: bytemuck::Pod + std::fmt::Debug,
    D: Dimension,
{
    pub fn into_cpu(self) -> Array<A, D> {
        // Get number of bytes
        let slice_size = self.data.len * std::mem::size_of::<A>();
        let size = slice_size as u64;

        // Create a CPU buffer to store result
        let staging_buffer = self.data.wgpu_device.create_staging_buffer(size);

        let mut encoder = self
            .data
            .wgpu_device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        encoder.copy_buffer_to_buffer(&self.data.storage_buffer, 0, &staging_buffer, 0, size);
        self.data.wgpu_device.queue.submit(Some(encoder.finish()));

        let buffer_slice = staging_buffer.slice(..);
        let buffer_future = buffer_slice.map_async(wgpu::MapMode::Read);
        self.data.wgpu_device.device.poll(wgpu::Maintain::Wait);
        if let Ok(()) = futures::executor::block_on(buffer_future) {
            let data = buffer_slice.get_mapped_range();
            let result: Vec<A> = data
                .chunks_exact(std::mem::size_of::<A>())
                .map(|b| *bytemuck::from_bytes::<A>(b))
                .collect();

            drop(data);
            staging_buffer.unmap();

            let array = unsafe { Array::<A, D>::from_shape_vec_unchecked(self.dim, result) };
            return array;
        }
        todo!()
    }
    
    

    pub fn get_wgpu_device(&self) -> &WgpuDevice {
        self.data.wgpu_device
    }
}


impl <A,D> Clone for WgpuArray<'_, A, D>
where
    A: bytemuck::Pod,
    D: Dimension,
{
    fn clone(&self) -> Self {
        WgpuArray {
            data: self.data.clone(),
            ptr: self.ptr,
            dim: self.dim.clone(),
            strides: self.strides.clone(),
        }
    }
}
