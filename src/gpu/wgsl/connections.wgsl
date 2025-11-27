struct NodeValue {
    node_id: u32,
    total: u32,
}

struct Edge {
    source_node: u32,
    target_node: u32
}

@group(0) @binding(0) var<storage, read> nodes: array<u32>;
@group(0) @binding(1) var<storage, read> edges: array<Edge>;
@group(0) @binding(2) var<storage, read_write> connections: array<NodeValue>;


@compute
@workgroup_size(64)
fn main(
    @builtin(global_invocation_id) global_id: vec3<u32>,
){
    let wg_size: u32 = 64;
    let index = global_id.x;

    let nodes_length = arrayLength(&nodes);

    let node: u32 = nodes[index];
    let total_edges = arrayLength(&edges);
    var total_connections: u32 = 0;
    for(var i = 0u; i < total_edges; i++) {
        let edge: Edge = edges[i];
        if (edge.source_node == node || edge.target_node == node) {
            total_connections += 1;
        }
    }
    connections[index] = NodeValue(node,total_connections);
}
