struct NormalizedValue {
    node_id: u32,
    value: f32
}
struct NodeConnectionValue {
    node_id: u32,
    total: u32
}

@group(0) @binding(0) var<storage, read> node_connections: array<NodeConnectionValue>;
@group(0) @binding(1) var<storage, read> min_max_degree: array<u32,2>;
@group(0) @binding(2) var<storage, read_write> normalized_values: array<NormalizedValue>;
@group(0) @binding(3) var<uniform> sort_toggle: u32;

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

@compute
@workgroup_size(64)
fn sort(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
){
    let left_index = global_invocation_id.x;
    let right_index = left_index + 1;
    let total_values = arrayLength(&normalized_values);
    if(right_index < total_values){
        var left_item =  normalized_values[left_index];
        var right_item = normalized_values[right_index];
        if((sort_toggle == 0 && left_index % 2 == 0) || (sort_toggle == 1 && left_index % 2 == 1)){
            if(left_item.value < right_item.value) {
                normalized_values[right_index] = left_item;
                normalized_values[left_index] = right_item;
            }
        }
    }
}
