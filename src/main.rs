mod singlylinkedlist;
use singlylinkedlist::{Node, SinglyLinkedList};

fn main() {
    let mut list = SinglyLinkedList::new_mt();
    list.push(98);
    list.push(90);
    list.push(100);

    println!("{:?}", list);
    let first_item = list.pop().unwrap();
    println!(" first item in queue : {}", first_item);

    println!("peek at the top {:?}", list.peek());

    println!("{}", list.is_empty());
}

// println!("Head :{:?} , Head value:", list.head.as_ref());