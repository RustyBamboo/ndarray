use ndarray::Array;
use ndarray::WgpuDevice;

#[test]
fn test_wgpu() {
    let d = futures::executor::block_on(WgpuDevice::new()).unwrap();
    let a_cpu: Array<f32, _> = Array::ones((5, 5)) * 2.;
    let b_cpu: Array<f32, _> = Array::ones((5, 5)) * 3.;
    let c_cpu: Array<f32, _> = Array::ones((5, 5)) * 6.;

    let a_gpu = a_cpu.into_wgpu(&d);
    let b_gpu = b_cpu.into_wgpu(&d);
    let c_gpu = c_cpu.into_wgpu(&d);

    let x_gpu = a_gpu + b_gpu;
    let y_gpu = x_gpu - c_gpu;

    let y_cpu = y_gpu.into_cpu();

    assert_eq!(y_cpu, Array::<f32, _>::ones((5, 5)) * -1.);
}
