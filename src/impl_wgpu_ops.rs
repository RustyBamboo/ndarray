use crate::{Dim, Dimension};
use crate::WgpuArray;
use crate::WgpuRepr;

use std::borrow::Cow;
use std::marker::PhantomData;
use std::ptr::NonNull;

macro_rules! impl_binary_op(
    ($trt:ident, $operator:tt, $mth:ident, $iop:tt, $shader:expr, $doc:expr) => (

impl<'d, A, D> $trt<WgpuArray<'d, A, D>> for WgpuArray<'d, A, D>
where
    A: bytemuck::Pod + std::fmt::Debug + Default,
    D: Dimension,
{
    type Output = WgpuArray<'d, A, D>;
    fn $mth(self, rhs: WgpuArray<A, D>) -> Self::Output
    {
        self.$mth(&rhs)
    }
}

impl<'a, 'd,  A, D> $trt<WgpuArray<'_, A, D>> for &'a WgpuArray<'d, A, D>
where
    A: bytemuck::Pod + std::fmt::Debug + Default,
    D: Dimension,
{
    type Output = WgpuArray<'d, A, D>;
    fn $mth(self, rhs: WgpuArray<A,D>) -> Self::Output
    {
        let cs_module =
            self.data
                .wgpu_device
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!($shader))),
                    flags: wgpu::ShaderFlags::empty()
                });
        let storage_buffer = self.data.wgpu_device.create_storage_buffer(vec![A::default(); self.data.len].as_slice());
        let compute_pipeline =
            self
                .data
                .wgpu_device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &cs_module,
                    entry_point: "main",
                });
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self
            .data
            .wgpu_device
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: rhs.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: storage_buffer.as_entire_binding(),
                    },
                ],
            });
        let mut encoder = self
            .data
            .wgpu_device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("add");
            cpass.dispatch(self.data.len as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }


        // Submits command encoder for processing
        self.data.wgpu_device.queue.submit(Some(encoder.finish()));
        let data: WgpuRepr<A> = WgpuRepr {
            wgpu_device: self.data.wgpu_device,
            storage_buffer,
            len: self.data.len,
            life: PhantomData
        };
        let array = WgpuArray {
            data,
            ptr: NonNull::dangling(), // Hack. There is nothing to point to
            dim: self.dim.clone(),
            strides: self.strides.clone(),
        };
        array
    }

}

impl<'a, 'd,  A, D> $trt<&'a WgpuArray<'_, A, D>> for &'a WgpuArray<'d, A, D>
where
    A: bytemuck::Pod + std::fmt::Debug + Default,
    D: Dimension,
{
    type Output = WgpuArray<'d, A, D>;
    fn $mth(self, rhs: &'a WgpuArray<A,D>) -> Self::Output
    {
        let cs_module =
            self.data
                .wgpu_device
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!($shader))),
                    flags: wgpu::ShaderFlags::empty()
                });
        let storage_buffer = self.data.wgpu_device.create_storage_buffer(vec![A::default(); self.data.len].as_slice());
        let compute_pipeline =
            self
                .data
                .wgpu_device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &cs_module,
                    entry_point: "main",
                });
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self
            .data
            .wgpu_device
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: rhs.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: storage_buffer.as_entire_binding(),
                    },
                ],
            });
        let mut encoder = self
            .data
            .wgpu_device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("add");
            cpass.dispatch(self.data.len as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }


        // Submits command encoder for processing
        self.data.wgpu_device.queue.submit(Some(encoder.finish()));
        let data: WgpuRepr<A> = WgpuRepr {
            wgpu_device: self.data.wgpu_device,
            storage_buffer,
            len: self.data.len,
            life: PhantomData
        };
        let array = WgpuArray {
            data,
            ptr: NonNull::dangling(), // Hack. There is nothing to point to
            dim: self.dim.clone(),
            strides: self.strides.clone(),
        };
        array
    }

}

impl<'a, 'd,  A, D> $trt<&'a WgpuArray<'_, A, D>> for WgpuArray<'d, A, D>
where
    A: bytemuck::Pod + std::fmt::Debug + Default,
    D: Dimension,
{
    type Output = WgpuArray<'d, A, D>;
    fn $mth(self, rhs: &WgpuArray<A,D>) -> Self::Output
    {
        let cs_module =
            self.data
                .wgpu_device
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!($shader))),
                    flags: wgpu::ShaderFlags::empty()
                });
        let storage_buffer = self.data.wgpu_device.create_storage_buffer(vec![A::default(); self.data.len].as_slice());
        let compute_pipeline =
            self
                .data
                .wgpu_device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &cs_module,
                    entry_point: "main",
                });
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self
            .data
            .wgpu_device
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: rhs.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: storage_buffer.as_entire_binding(),
                    },
                ],
            });
        let mut encoder = self
            .data
            .wgpu_device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("add");
            cpass.dispatch(self.data.len as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }


        // Submits command encoder for processing
        self.data.wgpu_device.queue.submit(Some(encoder.finish()));
        let data: WgpuRepr<A> = WgpuRepr {
            wgpu_device: self.data.wgpu_device,
            storage_buffer,
            len: self.data.len,
            life: PhantomData
        };
        let array = WgpuArray {
            data,
            ptr: NonNull::dangling(), // Hack. There is nothing to point to
            dim: self.dim.clone(),
            strides: self.strides.clone(),
        };
        array
    }
}


    );
);

use std::ops::*;
impl_binary_op!(Add, +, add, +=, "../wgsl-shaders/add.wgsl", "addition");
impl_binary_op!(Sub, -, sub, -=, "../wgsl-shaders/sub.wgsl", "subtraction");
impl_binary_op!(Mul, *, mul, *=, "../wgsl-shaders/mul.wgsl", "multiplication");
impl_binary_op!(Div, /, div, /=, "../wgsl-shaders/div.wgsl", "division");


use crate::Ix2;

impl<'a, 'd,  A> WgpuArray<'d, A, Ix2>
where
    A: bytemuck::Pod + std::fmt::Debug + Default,
{
    pub fn dot(self, rhs: &WgpuArray<A,Ix2>) -> WgpuArray<'d, A,Ix2>
    {
        let cs_module =
            self.data
                .wgpu_device
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../wgsl-shaders/dot.wgsl"))),
                    flags: wgpu::ShaderFlags::empty()
                });

        let result_shape = [self.shape()[0], rhs.shape()[1]];
        let len = result_shape[0] * result_shape[1];

        let mut stride: Vec<u32> = [self.strides(), rhs.strides()].concat().iter().map(|&x| x as u32).collect();
        let mut sizes_buffer: Vec<u32> = [self.shape(), rhs.shape()].concat().iter().map(|&x| x as u32).collect();

        sizes_buffer.append(&mut stride);
        let sizes_buffer = self.data.wgpu_device.create_storage_buffer(sizes_buffer.as_slice());


        let storage_buffer = self.data.wgpu_device.create_storage_buffer(vec![A::default(); len].as_slice());
        let compute_pipeline =
            self
                .data
                .wgpu_device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &cs_module,
                    entry_point: "main",
                });


        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self
            .data
            .wgpu_device
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: rhs.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: sizes_buffer.as_entire_binding(),
                    },

                ],
            });
        let mut encoder = self
            .data
            .wgpu_device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("add");
            let dispatch = [(result_shape[0] + 8 - 1) / 8, (result_shape[1] + 8 - 1 ) / 8 ];
            cpass.dispatch(dispatch[0] as u32, dispatch[1] as u32, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }


        // Submits command encoder for processing
        self.data.wgpu_device.queue.submit(Some(encoder.finish()));
        let data: WgpuRepr<A> = WgpuRepr {
            wgpu_device: self.data.wgpu_device,
            storage_buffer,
            len,
            life: PhantomData
        };
        let array = WgpuArray {
            data,
            ptr: NonNull::dangling(), // Hack. There is nothing to point to
            dim: Dim(result_shape),
            strides: self.strides.clone(),
        };
        array
    }
}
impl<'d, A, D> WgpuArray<'d, A, D>
where
    A: bytemuck::Pod + std::fmt::Debug + Default,
    D: Dimension,
{
    pub fn exp(self) -> WgpuArray<'d, A, D>
    {
        let cs_module =
            self.data
                .wgpu_device
                .device
                .create_shader_module(&wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("../wgsl-shaders/exp.wgsl"))),
                    flags: wgpu::ShaderFlags::empty()
                });
        let storage_buffer = self.data.wgpu_device.create_storage_buffer(vec![A::default(); self.data.len].as_slice());
        let compute_pipeline =
            self
                .data
                .wgpu_device
                .device
                .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: None,
                    layout: None,
                    module: &cs_module,
                    entry_point: "main",
                });
        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group = self
            .data
            .wgpu_device
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: self.data.storage_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: storage_buffer.as_entire_binding(),
                    },
                ],
            });
        let mut encoder = self
            .data
            .wgpu_device
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            cpass.insert_debug_marker("add");
            cpass.dispatch(self.data.len as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
        }


        // Submits command encoder for processing
        self.data.wgpu_device.queue.submit(Some(encoder.finish()));
        let data: WgpuRepr<A> = WgpuRepr {
            wgpu_device: self.data.wgpu_device,
            storage_buffer,
            len: self.data.len,
            life: PhantomData
        };
        let array = WgpuArray {
            data,
            ptr: NonNull::dangling(), // Hack. There is nothing to point to
            dim: self.dim.clone(),
            strides: self.strides.clone(),
        };
        array
    }

}
