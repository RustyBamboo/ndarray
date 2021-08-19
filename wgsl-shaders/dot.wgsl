[[block]]
struct Matrix {
  numbers: array<f32>;
};

[[block]]
struct Sizes {
  size0: vec2<u32>;
  size1: vec2<u32>;

  stride0: vec2<u32>;
  stride1: vec2<u32>;
};

[[group(0), binding(0)]] var<storage> firstMatrix : [[access(read_write)]] Matrix;
[[group(0), binding(1)]] var<storage> secondMatrix : [[access(read_write)]] Matrix;
[[group(0), binding(2)]] var<storage> resultMatrix : [[access(read_write)]] Matrix;
[[group(0), binding(3)]] var<storage> matrixSizes : [[access(read_write)]] Sizes;

[[stage(compute), workgroup_size(8, 8, 0)]]
fn main([[builtin(global_invocation_id)]] global_id : vec3<u32>) {
  if (global_id.x >= matrixSizes.size0.x || global_id.y >= matrixSizes.size1.y) {
    return;
  }
  let resultCell : vec2<u32> = vec2<u32>(global_id.x, global_id.y);
  var result : f32 = 0.0;
  for (var i : u32 = 0u; i < matrixSizes.size0.y; i = i + 1u) {
    let a : u32 = i*matrixSizes.stride0.y + resultCell.x * matrixSizes.stride0.x;
    let b : u32 = resultCell.y * matrixSizes.stride1.y + i * matrixSizes.stride1.x;
    result = result + firstMatrix.numbers[a] * secondMatrix.numbers[b];
  }

  let index : u32 = resultCell.y + resultCell.x * matrixSizes.size1.y;
  resultMatrix.numbers[index] = result;
}
