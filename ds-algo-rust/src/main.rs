mod vector;
use vector::Vector;

fn main() {
    let mut vec1: Vector<i32> = Vector::new();
    vec1.push(5);
    print!("my vector {}", vec1);
}
