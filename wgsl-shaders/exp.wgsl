
[[block]]
struct Array {
    data: [[stride(4)]] array<f32>;
};

[[group(0), binding(0)]]
var<storage> in_indices1: [[access(read_write)]] Array;

[[group(0), binding(1)]]
var<storage> out_indices: [[access(read_write)]] Array;

var pade_consts : array<f32, 6> = array<f32, 6>(1.0, 0.5, 0.11111111, 0.013888889, 0.0009920635, 0.0000330688);

fn pade_exp(x: f32) -> f32 {
   var n: f32 = pade_consts[0] + pade_consts[1] * x + pade_consts[2] * x * x + pade_consts[3] * x * x * x + pade_consts[4] * x * x * x * x + pade_consts[5] * x * x * x * x * x;

   var d: f32 = pade_consts[0] - pade_consts[1] * x + pade_consts[2] * x * x - pade_consts[3] * x * x * x + pade_consts[4] * x * x * x * x - pade_consts[5] * x * x * x * x * x;

   return n / d;

}

[[stage(compute), workgroup_size(16)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    out_indices.data[global_id.x] = pade_exp(in_indices1.data[global_id.x]);
}
