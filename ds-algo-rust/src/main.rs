mod vector;
use vector::Vector;

fn main() {
    let mut vec1: Vector<i32> = Vector::new();
    vec1.push(5);
    vec1.push(5);
    vec1.push(5);

    vec1.insert(1, 4);
    vec1.remove(0);

    for elem in vec1 {
        print!("elem is {}", elem);
    }
}
