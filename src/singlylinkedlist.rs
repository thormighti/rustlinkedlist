// Lets try to implement a linkedlist, which is a Ds that contains data and keep track of
// the next by a pointer to the next data. Point to none of np other is available

// lets begin with a node struct

use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;
type Singlelinkedlist = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
pub struct Node{
    data: i32,
    next: Singlelinkedlist
}

//can add at both ends for a linkedlist
#[derive(Debug)]
pub struct SinglyLinkedList{
   pub head: Singlelinkedlist,
    tail: Singlelinkedlist,
    lens: u32
}

impl Node{
   pub fn new(value:i32) -> Rc<RefCell<Node>>{
        Rc::new(RefCell::new(Node{
            data:value,
            next:None
        }))

    }
}
impl SinglyLinkedList{
    //lets create an emptylist
   pub fn new_mt() -> SinglyLinkedList{
        SinglyLinkedList{
            head:None,
            tail:None,
            lens:0
        }
    }

  pub  fn push(&mut self, value:i32){
        let new = Node::new(value);
        match self.tail.take(){
            Some(previous) => previous.borrow_mut().next = Some(Rc::clone(&new)),
            None => self.head = Some(Rc::clone(&new))
        };
        self.lens += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<i32>{
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take(){
                self.head = Some(next);
            }
            else {
                self.tail.take();
            }

            self.lens -= 1;
            Rc::try_unwrap(head).ok().expect("A fail").into_inner().data
        })

    }
    //check if list is empty
    pub fn is_empty(&self) -> bool{
        self.lens==0
    }

    pub fn length(&self) -> u32{
        self.lens
    }

    //leta take a peek at the  head data

    pub fn peek(&mut self) -> Option<i32>{
        self.head.take().map(| node| Rc::try_unwrap(node).ok().expect("no val").into_inner().data )
    }
}