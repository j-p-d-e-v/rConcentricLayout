
@group(0) @binding(0) var<storage, read> nodes: array<u32>;
@group(0) @binding(1) var<storage, read> edges: array<array<u32,2>>;
@group(0) @binding(2) var<storage, read_write> connections: array<array<u32,2>>;


@compute
@workgroup_size(64,1,1)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
){
    let wg_size: u32 = 64;
    let index = global_id.x;

    let node: u32 = nodes[index];
    let total_edges = arrayLength(&edges);
    var total_connections: u32 = 0;
    for(var i = 0u; i < total_edges; i++) {
        let edge: array<u32,2> = edges[i];
        if (edge[0] == node | edge[1] == node) {
            total_connections += 1;
        }
    }
    connections[index][0] = node;
    connections[index][1] = total_connections;
}
