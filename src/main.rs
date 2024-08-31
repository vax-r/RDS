use std::{cell::RefCell, rc::Rc};


type Link<T> = Option<Rc<RefCell<T>>>;

struct ListHead {
    next: Link<ListHead>,
    prev: Link<ListHead>,
    item: i32,
}

impl ListHead {
    fn new(num: i32) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(ListHead{
            item: num,
            prev: None,
            next: None,
        }));

        node.borrow_mut().prev = Some(Rc::clone(&node));
        node.borrow_mut().next = Some(Rc::clone(&node));
        
        node
    }

    /* list_add - add a new entry
     * @new: new entry to be added
     * @head: list head to add it after
     * 
     * Insert a new entry after the specified head.
     * This is good for implementing stacks.
     */
    fn list_add(new: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        let next = head.borrow().next.as_ref().unwrap().clone();

        new.borrow_mut().prev = Some(Rc::clone(head));
        new.borrow_mut().next = Some(Rc::clone(&next));
        next.borrow_mut().prev = Some(Rc::clone(new));
        head.borrow_mut().next = Some(Rc::clone(new));
    }

    fn next_node_value(&self) {
        match &self.next {
            Some(node) => {
                println!("{} -> {}", self.item, node.borrow().item);
            },
            None => {},
        }
    }

    fn prev_node_value(&self) {
        match &self.prev {
            Some(node) => {
                println!("{} -> {}", self.item, node.borrow().item);
            },
            None => {},
        }
    }
}


fn main() {
    let first = ListHead::new(1);
    let second = ListHead::new(2);
    
    first.borrow().next_node_value();
    first.borrow().prev_node_value();
    second.borrow().next_node_value();
    second.borrow().prev_node_value();

    println!("After insertion");
    ListHead::list_add(&second,&first);
    first.borrow().next_node_value();
    first.borrow().prev_node_value();
    second.borrow().next_node_value();
    second.borrow().prev_node_value();
}
