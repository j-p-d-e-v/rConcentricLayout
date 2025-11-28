struct RingData {
    index: u32,
    node_id: u32,
}
struct NormalizeValue {
    node_id: u32,
    total: f32
}

@group(0) @binding(0) var<storage,read> normalize_data: array<NormalizeValue>;
@group(0) @binding(1) var<storage,read> highest_normalize_value: f32;
@group(0) @binding(2) var<storage,read_write> ring_data: array<RingData>;


@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
) {
    let index = global_invocation_id.x;
    let normalize_item = normalize_data[index];
    let normalize_total = normalize_item.total;
    let node_id = normalize_item.node_id;
    let ring_index = u32(floor((highest_normalize_value - normalize_total) * 2.0));
    ring_data[index] = RingData(ring_index,node_id);
}
