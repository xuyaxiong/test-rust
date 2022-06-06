// 冒泡排序
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descending() {
        let mut ve1 = vec![4, 5, 6, 1, 2, 3];
        bubble_sort(&mut ve1);
        for i in 0..ve1.len() - 1 {
            assert!(ve1[i] <= ve1[i + 1]);
        }
        println!("{:?}", ve1);
    }
}