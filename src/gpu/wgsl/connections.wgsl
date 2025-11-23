struct Node {
    id: array<u32,32>,
    label: array<u32,32>
}
struct Edge {
    id: array<u32,32>,
    source_node: array<u32,32>,
    target_node: array<u32,32>
}
struct Connection {
    node_id: array<u32,32>,
    total: u32
}

@group(0) @binding(0) var<storage, read> nodes: array<Node>;
@group(0) @binding(1) var<storage, read> edges: array<Edge>;
@group(0) @binding(2) var<storage, read_write> connections: array<Connection>;


@compute @workgroup_size(64)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
    @builtin(local_invocation_id) local_id: vec3<u32>
){
    let gid:u32 = global_id.x;
    let lid:u32 = local_id.x;
    let wg_size: u32 = 64;
    let index = gid * wg_size + lid;
}
