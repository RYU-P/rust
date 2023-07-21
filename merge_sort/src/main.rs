use rand::Rng;

fn main() {
    let v = random_vector(10, 2);
    println!("{:?}", v);
}

fn random_vector(size: usize, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut v = Vec::with_capacity(size);
    for _ in 0..size {
        v.push(rng.gen_range(0..max));
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

fn selection_sort(v: &mut Vec<i32>, size: usize) {}
