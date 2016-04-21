fn search(array: & [i32], to_find: i32) -> bool {
    //Set mid point to be middle of array
    let mid = array[array.len()/2];
    //If the middle is what we want, awesome, we found it
    if mid == to_find {
    	return true;
    } else {
        //If the array is length 1, welp we didnt find it, so return false.
        if array.len() == 1 {
            return false;
        }
        //Partition into into two arrays
    	let (lesser, greater) = array.split_at(array.len()/2);
        //Search the corret halfs
    	if mid > to_find {
    		return search(lesser, to_find);
    	} else {
    		return search(greater, to_find);
    	}
    }
}

fn main() {
    let x = [1,2,3,4];
    let y = 4;
    let z = search(&x,y);
    println!("{}", z);
}
