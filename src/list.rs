use std::{cell::RefCell, rc::Rc};

type Link<T> = Option<Rc<RefCell<T>>>;

/*TODO: struct members should be private */
pub struct ListHead {
    pub next: Link<ListHead>,
    pub prev: Link<ListHead>,
    pub item: i32,
}

impl ListHead {
    #[allow(dead_code)]
    pub fn new(num: i32) -> Rc<RefCell<Self>> {
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
    pub fn init_list_head(list: &Rc<RefCell<Self>>) {
        list.borrow_mut().next = Some(Rc::clone(list));
        list.borrow_mut().prev = Some(Rc::clone(list));
    }


    /**
     * list_empty - tests whether a list is empty
     * @head: the list to test
     */
    #[allow(dead_code)]
    pub fn list_empty(head: &Rc<RefCell<Self>>) -> bool {
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
    pub fn list_add(new: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
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
    pub fn list_add_tail(new: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
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
    pub fn list_del_init(entry: &Rc<RefCell<Self>>) {
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
    pub fn list_replace(old: &Rc<RefCell<Self>>, new: &Rc<RefCell<Self>>) {
        new.borrow_mut().next = Some(Rc::clone(old.borrow().next.as_ref().unwrap()));
        new.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(new));
        new.borrow_mut().prev = Some(Rc::clone(old.borrow().prev.as_ref().unwrap()));
        new.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(new));
    }


    /**
     * list_replace_init - replace old entry by new one and initialize the old one
     * @old : the element to be replaced
     * @new : the new element to insert
     * 
     * If @old was empty, it will be overwritten.
     */
    #[allow(dead_code)]
    pub fn list_replace_init(old: &Rc<RefCell<Self>>, new: &Rc<RefCell<Self>>) {
        ListHead::list_replace(old, new);
        ListHead::init_list_head(old);
    }


    /**
     * list_swap - replace entry1 with entry2 and re-add entry1 at entry2's position
     * @entry1: the location to place entry2
     * @entry2: the location to place entry1
     */
    #[allow(dead_code)]
    pub fn list_swap(entry1: &Rc<RefCell<Self>>, entry2: &Rc<RefCell<Self>>) {
        let mut pos = &entry2.borrow_mut().prev.as_ref().unwrap().clone();

        ListHead::list_del_init(entry2);
        ListHead::list_replace(entry1, entry2);

        if Rc::ptr_eq(&pos, &entry1) {
            pos = entry2;
        }
        ListHead::list_add(entry1, &pos);
    }


    /**
     * list_move - delete from one list and add as another's head
     * @list: the entry to move
     * @head: the head that will precede our entry
     */
    #[allow(dead_code)]
    pub fn list_move(list: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        ListHead::__list_del_entry(list);
        ListHead::list_add(list, head);
    }


    /**
     * list_move_tail - delete from one list and add as another's tail
     * @list: the entry to move
     * @head: the head that will follow our entry
     */
    #[allow(dead_code)]
    pub fn list_move_tail(list: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        ListHead::__list_del_entry(list);
        ListHead::list_add_tail(list, head);
    }

}


/* Sections for Unit tests */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = ListHead::new(1);

        assert_eq!(list.borrow().item, 1);
        assert!(ListHead::list_empty(&list));
    }


    #[test]
    fn test_init_list_head() {
        let list =  Rc::new(RefCell::new(ListHead{
            item: 1,
            prev: None,
            next: None,
        }));

        ListHead::init_list_head(&list);

        assert_eq!(list.borrow().item, 1);
        assert!(ListHead::list_empty(&list));
    }


    #[test]
    fn test_list_empty() {
        let list1 = ListHead::new(1);
        let list2 = ListHead::new(2);

        ListHead::list_add_tail(&list2, &ListHead::new(3));

        assert!(ListHead::list_empty(&list1));
        assert!(!ListHead::list_empty(&list2));
    }
}
