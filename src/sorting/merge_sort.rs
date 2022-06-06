// 归并排序
fn merge<T: Ord + Copy>(arr: &mut [T], mid: usize) {
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();
    let mut l = 0;
    let mut r = 0;
    for v in arr {
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}

pub fn merge_sort<T>(arr: &mut [T])
    where T: Ord + Copy {
    if arr.len() > 1 {
        let mid = arr.len() / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
}

#[cfg(test)]
mod tests {
    use crate::sorting::merge_sort::merge_sort;

    #[test]
    fn basic() {
        let mut arr = vec![4, 5, 6, 1, 2, 3];
        merge_sort(&mut arr);
        println!("{:?}", arr);
    }
}