use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display

    // If you implement iterator trait:
    for val in &list {
       println!("{}", val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_clone() {
        let mut list: LinkedList<u32> = linked_list::LinkedList::new();
        for i in 1..10 {
            list.push_front(i);
        }
        let clone_list = list.clone();
        list.pop_front();
        assert_eq!(clone_list.get_size(), 9);
        assert_eq!(list.get_size(), 8);
    }

    #[test]
    fn test_partial_eq() {
        let mut list: LinkedList<u32> = linked_list::LinkedList::new();
        for i in 1..10 {
            list.push_front(i);
        }
        let mut other_list: LinkedList<u32> = linked_list::LinkedList::new();
        for i in 1..10 {
            other_list.push_front(i);
        }
        assert!(list == other_list);
    }

    #[test]
    fn test_not_partial_eq() {
        let mut list: LinkedList<u32> = linked_list::LinkedList::new();
        for i in 1..10 {
            list.push_front(i);
        }
        let mut other_list: LinkedList<u32> = linked_list::LinkedList::new();
        for i in 1..8 {
            other_list.push_front(i);
        }
        assert!(list != other_list);
    }

    #[test]
    fn test_compute_norm() {
        let mut list: LinkedList<f64> = linked_list::LinkedList::new();
        for i in 3..5 {
            list.push_front(i as f64);
        }
        assert_eq!(5.0, linked_list::ComputeNorm::compute_norm(&list));
    }
}
