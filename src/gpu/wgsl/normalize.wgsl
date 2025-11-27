struct NormalizedValue {
    node_id: u32,
    total: f32
}
struct NodeConnectionValue {
    node_id: u32,
    total: u32
}

@group(0) @binding(0) var<storage, read> node_connections: array<NodeConnectionValue>;
@group(0) @binding(1) var<storage, read> min_max_degree: array<u32,2>;
@group(0) @binding(2) var<storage, read_write> normalized_values: array<NormalizedValue>;

@compute
@workgroup_size(64,1,1)
fn main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
){
    let min_degree = f32(min_max_degree[0]);
    let max_degree = f32(min_max_degree[1]);
    let index = global_invocation_id.x;
    let item = node_connections[index];
    let node_id = item.node_id;
    let total = item.total;
    let normalized_value: f32 = (f32(total) - min_degree) / (max_degree - min_degree);
    normalized_values[index] = NormalizedValue(node_id,normalized_value);
}
