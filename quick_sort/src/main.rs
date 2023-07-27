use rand::distributions::{Distribution, Uniform};
fn main() {
    let mut v = random_vector(10, 1000);
    println!("{:?}", v);
    quick_sort(&mut v);
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

fn quick_sort(slice: &mut [i32]) {
    if (slice.len() > 1) {
        let length: usize = slice.len();
        let pivot = partition(slice);
        quick_sort(&mut slice[0..pivot]);
        quick_sort(&mut slice[pivot + 1..length]);
    }
}

fn partition(array: &mut [i32]) -> usize {
    let range = Uniform::from(0..array.len() - 1); //for generating random number
    let mut rng = rand::thread_rng(); //for generating random number
    let mut pivot = range.sample(&mut rng);
    //swap pivot with last element
    array.swap(pivot, array.len() - 1);
    pivot = array.len() - 1;
    let mut a: usize = 0; //to store the index of element that is larger than pivot starting from left side of the array.
    let mut b: usize = array.len()-2; //to store the index of element that is smaller than pivot starting from the right side of the arra
    while a <= b {
        for i in 0..pivot {
            if array[i] > array[pivot] {
                a = i;
                break;
            }
        }
        for j in (0..pivot).rev() {
            if array[j] < array[pivot] {
                b = j;
                break;
            }
        }
        array.swap(a, b);
    }
    array.swap(a, pivot);
    pivot = a;
    return pivot;
}
