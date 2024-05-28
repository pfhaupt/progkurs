struct Graph {
    edges: Vec<Edge>,
    nodes: Vec<Node>,
}
struct Edge {
    start: Node,
    end: Node
}
struct Node {
    id: usize,
    edges: Vec<Edge>,
}