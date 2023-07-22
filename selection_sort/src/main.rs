use rand::distributions::{Distribution, Uniform};
fn main() {
    let mut v = random_vector(100, 100);
    println!("{:?}", v);
    selection_sort(&mut v, 100);
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

fn selection_sort(v: &mut [i32], size: usize) {
    let mut min = 0;
    let mut min_index: usize;
    for i in 0..size {
        min_index = i;
        for j in i + 1..size {
            if v[j] < v[min_index] {
                min = v[j];
                min_index = j;
            }
        }
        v.swap(i, min_index);
    }
}
