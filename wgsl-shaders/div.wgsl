
[[block]]
struct Array {
    data: [[stride(4)]] array<f32>;
};


[[group(0), binding(0)]]
var<storage> in_indices1: [[access(read_write)]] Array;

[[group(0), binding(1)]]
var<storage> in_indices2: [[access(read_write)]] Array;

[[group(0), binding(2)]]
var<storage> out_indices: [[access(read_write)]] Array;


[[stage(compute), workgroup_size(16)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    out_indices.data[global_id.x] = in_indices1.data[global_id.x] / in_indices2.data[global_id.x];
}
