//Playing around with enum type for this search
enum SearchResult {
    Found(i32),
    NotFound
}

//Some renaming of stuff for easier use
use SearchResult::*;

fn linear_search(items: &[i32], target: i32) -> SearchResult {
    //Go till we find the item
    for (idx, item) in items.iter().enumerate() {
        if *item == target {
            return Found(idx as i32);
        }
    }
    NotFound
}

fn main() {
    let items = [1, 3, 5, 7, 12, 454];
    if let SearchResult::Found(n) = linear_search(&items, 12) {
        println!("Index of Item: {}", n);
    } else {
        println!("Not found.");
    }
}
