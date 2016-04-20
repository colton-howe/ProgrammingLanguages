struct Node {
	num: i32, 
	connected: Vec<Edge>,
}

struct Edge {
	start: i32,
	end: i32,
	weight: i32,
}

struct Forest {
	tree: Vec<i32>,
	total: i32,
}

fn minimal_tree(tree: Vec<Node>) -> i32 {
	let mut forest = Vec::new();
	let mut edges = Vec::new();
	let mut touched = vec![false; tree.len()];
	for x in &tree {
		let beginning_tree = Forest { tree: vec![x.num] , total: 0};
		forest.push(beginning_tree);
		for y in &x.connected {
			edges.push(y);
		}
	}
	let mut stop = false;
	while !stop{
		let mut check = true;
		for x in &touched {
			if !x {
				check = false;
			}
		}
		if edges.is_empty(){
			stop = true;
		} else if check {
			stop = true;
		} else {
			let mut minimum_weight = std::i32::MAX;
			let mut index = 0;
			let mut count = 0;
			for x in &edges {
				if x.weight < minimum_weight {
					minimum_weight = x.weight;
					index = count;
				}
				count = count + 1;
			}
			let focus_edge = edges.remove(index);
			for x in &forest {
				
			}
		}
	}
	0
}

fn main() {
	let edge1 = Edge { start: 1, end: 2, weight: 4 };
   	let node1 = Node { num: 1, connected: vec![edge1] };
   	let edge2 = Edge { start: 2, end: 3, weight: 2 };
   	let node2 = Node { num: 2, connected: vec![edge2] };
   	let edge3 = Edge { start: 3, end: 1, weight: 1 };
   	let node3 = Node { num: 3, connected: vec![edge3] };
   	let result = minimal_tree(vec![node1, node2, node3]);
   	println!("{}",result);
}
