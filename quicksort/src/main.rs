fn quicksort<E: Ord>(array: &mut [E]){
    if 1 < array.len() {
        //Set pivot, and set the high point of the array
        let (mut pivot, mut hi) = (0, array.len()-1);
        //Loop through the elements in the array
        for _ in 0..array.len()-1 {
            //Order them all based on the pivot
            if array[pivot] < array[pivot+1] {
                array.swap(pivot+1, hi);
                hi -= 1;
            } else {
                array.swap(pivot, pivot+1);
                pivot += 1;
            }
        }
        //Run quicksort recursively on both sides of the pivot
        quicksort(&mut array[..pivot]);
        quicksort(&mut array[pivot+1..]);
    }
}

fn main() {
	let mut to_sort = [6,8,1,4,2,0,9];
	quicksort(&mut to_sort); 
	println!("{:?}", to_sort);
}
