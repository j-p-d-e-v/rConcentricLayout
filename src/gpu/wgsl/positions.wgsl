struct RingCapacity {
    index: u32,
    max_nodes: u32,
    radius: u32,
    range: array<u32,2>,
}
struct NodeId {
    id: u32
}
struct NormalizeValue {
    node_id: u32,
    total: f32
}
struct RingData {
    index: u32,
    radius: u32,
    angle_degree: f32,
    angle_radian: f32,
    cx: f32,
    cy: f32,
    x: f32,
    y: f32,
    node_id: u32
}
@group(0) @binding(0) var<storage,read> normalize_data: array<NormalizeValue>;
@group(0) @binding(1) var<storage,read> ring_capacity: array<RingCapacity>;
@group(0) @binding(2) var<storage,read> cx_cy: vec2<f32>;
@group(0) @binding(3) var<storage,read_write> result: array<RingData>;
const PI: f32 = 3.141592653589793;

@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) global_invocation_id: vec3<u32>
){
    let index = global_invocation_id.x;
    let capacity = ring_capacity[index];
    let cx = cx_cy.x;
    let cy = cx_cy.y;
    let ring_index = capacity.index;
    let ring_radius = capacity.radius;
    let start_index = capacity.range[0];
    let end_index = capacity.range[1];
    let max_nodes = capacity.max_nodes; //Total Nodes Per Index
    let step_angle = 360.0 / f32(max_nodes);
    var node_index:u32 = 0;

    for(var i = start_index; i < end_index; i++) {
        let angle_degree = f32(node_index) * step_angle;
        let angle_radian = angle_degree * (PI / 180.0 );
        let normalize_node = normalize_data[i];
        let node_id = normalize_node.node_id;
        let x = cx + f32(ring_radius) * cos(angle_radian);
        let y = cy + f32(ring_radius) * sin(angle_radian);
        result[i] = RingData(
            index,
            ring_radius,
            angle_degree,
            angle_radian,
            cx,
            cy,
            x,
            y,
            node_id);
        node_index++;
    }

}
