mod vector;
use vector::Vector;
mod linked_list;
use linked_list::List;

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

    let mut ll: List<i32> = List::new();
    ll.push(5);
}
