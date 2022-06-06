pub mod sorting;


fn main() {
    let mut arr = vec![6, 5, 4, 1, 2, 3];
    // sorting::bubble_sort::bubble_sort(&mut arr);
    sorting::insertion_sort::insertion_sort(&mut arr);
    println!("{:?}", arr);
}
