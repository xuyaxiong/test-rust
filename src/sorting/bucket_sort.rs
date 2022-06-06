use crate::sorting::insertion_sort::insertion_sort;

pub fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }
    let max = arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];
    for x in arr {
        buckets[len * *x / max].push(*x);
    }

    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
    }

    let mut result = vec![];
    for bucket in buckets {
        for x in bucket {
            result.push(x);
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use crate::sorting::bucket_sort::bucket_sort;

    #[test]
    fn basic() {
        let mut arr = vec![4, 5, 6, 1, 2, 3];
        let res = bucket_sort(&mut arr);
        println!("{:?}", res);
    }
}