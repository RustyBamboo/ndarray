#![feature(test)]

extern crate test;


macro_rules! mat_mul {
    ($modname:ident, $ty:ident, $(($name:ident, $m:expr, $n:expr, $k:expr))+) => {
        mod $modname {
            use test::{black_box, Bencher};
            use ndarray::{Array, WgpuDevice};
            $(
            #[bench]
            fn $name(bench: &mut Bencher)
            {
                let d = futures::executor::block_on(WgpuDevice::new()).unwrap();
                let a = Array::<$ty, _>::zeros(($m, $n)).into_wgpu(&d);
                let b = Array::<$ty, _>::zeros(($n, $k)).into_wgpu(&d);
                //TODO: WgpuRepr needs to implement Copy trait
                bench.iter(|| a.dot(&b).into_cpu());
            }
            )+
        }
    }
}

mat_mul! {mat_mul_wgpu_f32, f32,
    (m004, 4, 4, 4)
    (m007, 7, 7, 7)
    (m008, 8, 8, 8)
    (m012, 12, 12, 12)
    (m016, 16, 16, 16)
    (m032, 32, 32, 32)
    (m064, 64, 64, 64)
    (m127, 127, 127, 127)
    (mix16x4, 32, 4, 32)
    (mix32x2, 32, 2, 32)
    (mix10000, 128, 10000, 128)
}


