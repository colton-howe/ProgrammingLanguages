use std::any::*;

struct Node {
	num: i32, 
	edges: Option<Vec<Edge>>,
}

struct Edge {
	distance: i32,
	start: Node,
	end: Node,
}

fn shortest_path(tree: Option<Vec<Node>>, source: Node, target: Node) -> i32 {
	1
}


fn main(){
	let x = Node {}
}