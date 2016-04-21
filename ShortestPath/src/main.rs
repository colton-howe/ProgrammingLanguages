struct Node {
	num: i32, 
	connected: Vec<i32>,
}

fn shortest_path(tree: Vec<Node>, start_index: usize, end_index: usize) -> i32 {
	//Make a distance vector, setting all distances to the max.
	let mut distance = vec![std::i32::MAX; tree.len()];
	//Set distance of the starting node to 0
	distance[start_index] = 0;
	let test = start_index as i32;
	let mut unvisited = Vec::new();
	//If the node is not the starting node, push it onto the unvisited vector.
	for x in &tree {
		if x.num != test+1 {
			unvisited.push(x.num);
		}
	}
	let mut count = 1;
	//Set the current node to be the starting node
	let mut current = &tree[start_index];
	let mut done = false;
	//Make a stack to handle the next nodes to check
	let mut stack = Vec::new();
	while !done {
		//For our current node, if the connected nodes are unvisited, push them onto the stack and update their distance
		for x in &current.connected {
			let temp = x-1;
			let check_index = temp as usize;
			if unvisited.contains(x) {
				if count < distance[check_index] {
					distance[check_index] = count;
					stack.push(x);
				}
			}	
		}
		//Finds the index of the current node, then removes it from the unvisited vector.
		let index = unvisited.iter().position(|&x| x == current.num);
		if index.is_some() {
			unvisited.remove(index.unwrap());
		}
		let end = end_index as i32;
		//First stop condition - reached the final point
		if !unvisited.contains(&end) {
			done = true;
		}
		//Second stop condition - Check if unreachable
		let mut unvisited_check = true;
		for x in &unvisited {
			let temp = x-1;
			let check_index = temp as usize; 
			if distance[check_index] != std::i32::MAX {
				unvisited_check = false;
			}
		}
		if unvisited_check {
			done = true;
		}
		count = count +1;
		//If the stack isn't empty, go to the next node on the stack and repeat distance checking.
		if !stack.is_empty(){
			let temp = stack.pop().unwrap()-1;
			let next_index = temp as usize;
			current = &tree[next_index];
		} else { //Third stop condition - No more nodes to check
			done = true;
		}
	} 
	distance[end_index]
}


fn main(){
	//Set up graph and test function
	let node1 = Node { num: 1, connected: vec![6,2] };
	let node2 = Node { num: 2, connected: vec![3] };
	let node3 = Node { num: 3, connected: vec![4] };
	let node4 = Node { num: 4, connected: vec![5] };
	let node5 = Node { num: 5, connected: vec![6] };
	let node6 = Node { num: 6, connected: vec![1] };
	let tree = vec![node1, node2, node3, node4, node5, node6];
	let results = shortest_path(tree, 0, 5);
	println!("Shortest Path is of a distance: {}",results);
}