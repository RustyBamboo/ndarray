
[[block]]
struct InArray1 {
    data: [[stride(4)]] array<f32>;
};

[[block]]
struct InArray2 {
    data: [[stride(4)]] array<f32>;
};

[[block]]
struct OutArray {
    data: [[stride(4)]] array<f32>;
};




[[group(0), binding(0)]]
var<storage> in_indices1: [[access(read_write)]] InArray1;

[[group(0), binding(1)]]
var<storage> in_indices2: [[access(read_write)]] InArray2;

[[group(0), binding(2)]]
var<storage> out_indices: [[access(read_write)]] OutArray;


[[stage(compute), workgroup_size(1)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    out_indices.data[global_id.x] = in_indices1.data[global_id.x] + in_indices2.data[global_id.x];
}
