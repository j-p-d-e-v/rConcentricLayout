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
@group(0) @binding(3) var<storage, read_write> min_max: array<u32,2>;

@compute
@workgroup_size(64)
fn get_connections(
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

@compute
@workgroup_size(64)
fn get_min(
    @builtin(global_invocation_id) global_id: vec3<u32>,
){
    let wg_size: u32 = 64;
    let index = global_id.x;
    let connection = connections[index];
    let total = connection.total;

    if(index == 0){
        min_max[0] = total;
    }
    else {
        if(total < min_max[0]) {
            min_max[0] = total;
        }
    }

}

@compute
@workgroup_size(64)
fn get_max(
    @builtin(global_invocation_id) global_id: vec3<u32>,
){
    let wg_size: u32 = 64;
    let index = global_id.x;
    let connection = connections[index];
    let total = connection.total;

    if(index == 0){
        min_max[1] = total;
    }
    else {
        if(total > min_max[1]) {
            min_max[1] = total;
        }
    }

}
