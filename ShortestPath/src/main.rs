struct Node {
	num: i32, 
	connected: Vec<i32>,
}

fn shortest_path(tree: Vec<Node>, start_index: usize, end_index: usize) -> i32 {
	let mut distance = vec![std::i32::MAX; tree.len()];
	distance[start_index] = 0;
	let test = start_index as i32;
	let mut unvisited = Vec::new();
	for x in &tree {
		if x.num != test+1 {
			unvisited.push(x.num);
		}
	}
	let mut count = 1;
	let mut current = &tree[start_index];
	let mut done = false;
	let mut stack = Vec::new();
	while !done {
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
		let index = unvisited.iter().position(|&x| x == current.num);
		unvisited.remove(index.unwrap());
		let end = end_index as i32;
		if !unvisited.contains(&end) {
			done = true;
		}
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
		let temp = stack.pop().unwrap();
		let next_index = *temp as usize;
		current = &tree[next_index];
	} 
	distance[end_index]
}


fn main(){
	let node1 = Node { num: 1, connected: vec![2] };
	let node2 = Node { num: 2, connected: vec![3] };
	let node3 = Node { num: 3, connected: vec![2] };
	let tree = vec![node1, node2, node3];
	let results = shortest_path(tree, 0, 2);
	println!("Shortest Path is of a distance: {}",results);
}