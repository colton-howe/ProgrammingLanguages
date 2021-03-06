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

fn minimal_tree(tree: Vec<Node>) -> Vec<i32> {
	//Set up our vectors for storage of all the important information
	let mut forest = Vec::new();
	let mut edges = Vec::new();
	let mut touched = vec![false; tree.len()];
	//Scan the tree we are given and make a forest out of each node in our graph. Push the forest into our forest vector
	for x in &tree {
		let beginning_tree = Forest { tree: vec![x.num] , total: 0};
		forest.push(beginning_tree);
		for y in &x.connected {
			edges.push(y);
		}
	}
	let mut stop = false;
	while !stop{
		//Find the smallest weighted edge and get the index for it
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
		//Pop the smallest weighted edge off the edges stack
		let focus_edge = edges.remove(index);
		let mut index1 = 0;
		let mut index2 = 0;
		let mut count = 0;
		//Look at the edges start and end points. Compare those points to the forests in the forests vector to find the
		//nodes we are looking for
		for x in &forest {
			let start_index = x.tree.iter().position(|&x| x == focus_edge.start);
			if start_index.is_some() {
				index1 = count;
			}
			let end_index = x.tree.iter().position(|&x| x == focus_edge.end);
			if end_index.is_some() {
				index2 = count;
			}
			count = count + 1;
		}
		//If the two nodes are not part of the same forest, merge the two forests into one forest and put it back
		//on the stack.
		if index1 != index2 {
			let mut forest1 = Forest {tree: vec![], total: 0};
			let mut forest2 = Forest {tree: vec![], total: 0};
			if index1 > index2 {
				forest1 = forest.remove(index1);
				forest2 = forest.remove(index2);
			} else {
				forest2 = forest.remove(index2);
				forest1 = forest.remove(index1);
			}
			forest1.tree.append(&mut forest2.tree);
			let new_forest = Forest { tree: forest1.tree, total: forest1.total + forest2.total + focus_edge.weight };
			for x in &new_forest.tree {
				let temp = x-1;
				let index = temp as usize;
				touched[index] = true;
			}
			forest.push(new_forest);
		}
		//First stop condition - Every node has been touched
		let mut check = true;
		for x in &touched {
			if !x {
				check = false;
			}
		}
		//Second stop condition - All edges have been looked at.
		if edges.is_empty(){
			stop = true;
		} else if check {
			stop = true;
		} 
	}
	//Returns final weights. Is a vector incase some parts of the graph are detached from others.
	let mut final_weights = Vec::new();
	for x in &forest {
		final_weights.push(x.total);
	}
	final_weights
}

fn main() {
	//Main function to make a group a small graph and test the function
	let edge1 = Edge { start: 1, end: 2, weight: 4 };
	let edge2 = Edge { start: 2, end: 3, weight: 2 };
	let edge3 = Edge { start: 3, end: 4, weight: 1 };
	let edge4 = Edge { start: 4, end: 1, weight: 6 };
	let edge5 = Edge { start: 3, end: 1, weight: 3 };
   	let node1 = Node { num: 1, connected: vec![edge1] };
   	let node2 = Node { num: 2, connected: vec![edge2] };
   	let node3 = Node { num: 3, connected: vec![edge3] };
   	let node4 = Node { num: 4, connected: vec![edge4] };
   	let tree = vec![node1, node2, node3, node4];
   	let result = minimal_tree(tree);
   	for x in &result {
   		   	println!("Tree Weight = {}",x);
   	}
}
