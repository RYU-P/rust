use rand::distributions::{Distribution, Uniform};

fn main() {
    let v = random_vector(10, 2);
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

fn selection_sort(v: &mut Vec<i32>, size: usize) {

}

fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return
    }
    let mid = len / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let aux = arr.to_vec();

    merge(, right, arr);
}

fn merge(left: &[i32], right: &[i32], arr: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }
}
