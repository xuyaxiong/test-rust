// 选择排序
fn selection_sort<T: Ord>(arr: &mut [T]) {
    for left in 0..arr.len() {
        let mut smallest = left;
        for right in (left + 1)..arr.len() {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(left, smallest);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::selection_sort::selection_sort;

    #[test]
    fn basic() {
        let mut arr = vec![4, 5, 6, 1, 2, 3];
        selection_sort(&mut arr);
        println!("{:?}", arr);
    }
}