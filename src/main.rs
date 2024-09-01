use std::{cell::RefCell, rc::Rc};


type Link<T> = Option<Rc<RefCell<T>>>;

struct ListHead {
    next: Link<ListHead>,
    prev: Link<ListHead>,
    item: i32,
}

impl ListHead {
    #[allow(dead_code)]
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
     * init_list_head - Initialize a ListHead structure
     * @list: ListHead structure to be initialized.
     * 
     * Initializes the ListHead to point to itself. If it is a list header,
     * the result is an empty list.
     */
    #[allow(dead_code)]
    fn init_list_head(list: &Rc<RefCell<Self>>) {
        list.borrow_mut().next = Some(Rc::clone(list));
        list.borrow_mut().prev = Some(Rc::clone(list));
    }


    /**
     * list_empty - tests whether a list is empty
     * @head: the list to test
     */
    #[allow(dead_code)]
    fn list_empty(head: &Rc<RefCell<Self>>) -> bool {
        Rc::ptr_eq(head.borrow().next.as_ref().unwrap(), head)
    }


    /**
     * Insert a new entry between two known consecutive entries.
     * 
     * This is only for internal list manipulation where we know
     * the prev/next entries already!
     */
    fn __list_add(new: &Rc<RefCell<Self>>, prev: &Rc<RefCell<Self>>, next: &Rc<RefCell<Self>>) {
        next.borrow_mut().prev = Some(Rc::clone(new));
        new.borrow_mut().next = Some(Rc::clone(next));
        new.borrow_mut().prev = Some(Rc::clone(prev));
        prev.borrow_mut().next = Some(Rc::clone(new));
    }


    /* list_add - add a new entry
     * @new: new entry to be added
     * @head: list head to add it after
     * 
     * Insert a new entry after the specified head.
     * This is good for implementing stacks.
     */
    #[allow(dead_code)]
    fn list_add(new: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        let next = head.borrow().next.as_ref().unwrap().clone();
        ListHead::__list_add(new, head, &next);
    }


    /**
     * list_add_tail - add a new entry
     * @new: new entry to be added
     * @head: list head to add it before
     * 
     * Insert a new entry before the specified head.
     * This is useful for implementing queues.
     */
    #[allow(dead_code)]
    fn list_add_tail(new: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        let prev: Rc<RefCell<ListHead>> = head.borrow().prev.as_ref().unwrap().clone();
        ListHead::__list_add(new, &prev, head);
    }


    /**
     * Delete a list entry by making the prev/next entries
     * point to each other.
     * 
     * This is only for internal list manipulation where we know
     * the prev/next entries already!
     */
    fn __list_del(prev: &Rc<RefCell<Self>>, next: &Rc<RefCell<Self>>) {
        next.borrow_mut().prev = Some(Rc::clone(prev));
        prev.borrow_mut().next = Some(Rc::clone(next));
    }


    fn __list_del_entry(entry: &Rc<RefCell<Self>>) {
        ListHead::__list_del(entry.borrow().prev.as_ref().unwrap(), entry.borrow().next.as_ref().unwrap());
    }


    /**
     * list_del_init - deletes entry from list and reinitialize it.
     * @entry: the element to delete from the list.
     */
    #[allow(dead_code)]
    fn list_del_init(entry: &Rc<RefCell<Self>>) {
        ListHead::__list_del_entry(entry);
        ListHead::init_list_head(entry);
    }


    /**
     * list_replace - replace old entry by new one
     * @old : the element to be replaced
     * @new : the new element to insert
     * 
     * If @old was empty, it will be overwritten.
     */
    #[allow(dead_code)]
    fn list_replace(old: &Rc<RefCell<Self>>, new: &Rc<RefCell<Self>>) {
        new.borrow_mut().next = Some(Rc::clone(old.borrow().next.as_ref().unwrap()));
        new.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(new));
        new.borrow_mut().prev = Some(Rc::clone(old.borrow().prev.as_ref().unwrap()));
        new.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(new));
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
    ListHead::list_del_init(&second);
    ListHead::list_replace(&third, &second);

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
