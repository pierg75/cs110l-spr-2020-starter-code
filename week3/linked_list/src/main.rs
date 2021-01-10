use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<String> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    let vec_string = vec![String::from("test"),
                          String::from("test2"),
                          String::from("test3"),];
    for i in &vec_string {
        list.push_front(i.to_owned());
    }
    println!("list of Strings");
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display



    let mut list_vec: LinkedList<Vec<String>> = LinkedList::new();
    assert!(list_vec.is_empty());
    assert_eq!(list_vec.get_size(), 0);
    let vec_string1 = vec![String::from("test"),
                          String::from("test2"),
                          String::from("test3"),];
    let vec_string2 = vec![String::from("test"),
                          String::from("test2"),
                          String::from("test3"),];
    let vec_string3 = vec![String::from("test"),
                          String::from("test2"),
                          String::from("test3"),];
    list_vec.push_front(vec_string1);
    list_vec.push_front(vec_string2);
    list_vec.push_front(vec_string3);
    println!("list of Vec<Strings>");
    println!("{:?}", list_vec);
    println!("list size: {:?}", list_vec.get_size());
    println!("top element: {:?}", list_vec.pop_front().unwrap());
    println!("{:?}", list_vec);
    println!("size: {}", list_vec.get_size());
    println!("{:?}", list_vec); // ToString impl for anything impl Display
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}
}
