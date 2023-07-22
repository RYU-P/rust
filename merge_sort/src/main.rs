use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut v = random_vector(1000, 1000);
    println!("{:?}", v);
    
    merge_sort(&mut v);
    println!("{:?}", v);
}

fn random_vector(size: usize, max: i32) -> Vec<i32> {
    let range = Uniform::from(0..max);
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(range.sample(&mut rng));
    }
    v
}

fn sum_slice(s: &[i32]) -> i32 {
    s.iter().sum()
}

fn linear_search(s: &[i32], target: i32) -> usize {
    s.iter()
        .position(|&num| num == target)
        .expect("no element that exists that equal target")
}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    let mut buffer = vec![0; len];
    merge_sort_helper(arr, &mut buffer);
}

fn merge_sort_helper(arr: &mut [i32], buffer: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;

    merge_sort_helper(&mut arr[0..mid], &mut buffer[0..mid]);
    merge_sort_helper(&mut arr[mid..], &mut buffer[mid..]);

    merge(&arr[0..mid], &arr[mid..], &mut buffer[..len]);

    arr.copy_from_slice(&buffer[..len]);
}

fn merge(left: &[i32], right: &[i32], result: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i];
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}


