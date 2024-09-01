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

    /**
     * list_empty - tests whether a list is empty
     * @head: the list to test
     */
    fn list_empty(head: &Rc<RefCell<Self>>) -> bool {
        Rc::ptr_eq(head.borrow().next.as_ref().unwrap(), head)
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


    /**
     * list_add_tail - add a new entry
     * @new: new entry to be added
     * @head: list head to add it before
     * 
     * Insert a new entry before the specified head.
     * This is useful for implementing queues.
     */
    fn list_add_tail(new: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        let prev: Rc<RefCell<ListHead>> = head.borrow().prev.as_ref().unwrap().clone();

        new.borrow_mut().prev = Some(Rc::clone(&prev));
        new.borrow_mut().next = Some(Rc::clone(head));
        head.borrow_mut().prev = Some(Rc::clone(new));
        prev.borrow_mut().next = Some(Rc::clone(new));
    }

}


fn main() {
    let first = ListHead::new(1);
    let second = ListHead::new(2);
    let third = ListHead::new(3);

    if ListHead::list_empty(&first) {
        println!("List empty!")
    }

    ListHead::list_add(&second,&first);
    ListHead::list_add_tail(&third, &first);

    /* Print the linked list forward */
    let mut current = Some(Rc::clone(&first));
    while let Some(node) = current {
        print!("{} -> ", node.borrow().item);
        current = node.borrow().next.clone();
        
        if Rc::ptr_eq(&current.as_ref().unwrap(), &first) {
            break;
        }
    }
    println!("Finished");
}
