fn search(array: & [i32], to_find: i32) -> i32 {
    let mid = array[array.len()/2];
    if mid == to_find {
    	return to_find;
    } else {
    	let (lesser, greater) = array.split_at(array.len()/2);
    	if mid > to_find {
    		search(lesser, to_find);
    	} else {
    		search(greater, to_find);
    	}
    }
}

fn main() {
    
}
