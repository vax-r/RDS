use std::{cell::RefCell, cmp::Ordering, rc::Rc};

type Link<T> = Option<Rc<RefCell<T>>>;
type ListCmpFunc = fn(&Rc<RefCell<ListHead>>, &Rc<RefCell<ListHead>>) -> Ordering;


pub fn cmp_func(n1: &Rc<RefCell<ListHead>>, n2: &Rc<RefCell<ListHead>>) -> Ordering {
    if n1.borrow().item < n2.borrow().item {
        Ordering::Less
    } else if n1.borrow().item > n2.borrow().item {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

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
    pub fn init_list_head(list: Rc<RefCell<Self>>) {
        list.borrow_mut().next = Some(Rc::clone(&list));
        list.borrow_mut().prev = Some(Rc::clone(&list));
    }


    /**
     * list_empty - tests whether a list is empty
     * @head: the list to test
     */
    #[allow(dead_code)]
    pub fn list_empty(head: Rc<RefCell<Self>>) -> bool {
        Rc::ptr_eq(head.borrow().next.as_ref().unwrap(), &head)
    }


    /**
     * Insert a new entry between two known consecutive entries.
     * 
     * This is only for internal list manipulation where we know
     * the prev/next entries already!
     */
    fn __list_add(new: Rc<RefCell<Self>>, prev: Rc<RefCell<Self>>, next: Rc<RefCell<Self>>) {
        next.borrow_mut().prev = Some(Rc::clone(&new));
        new.borrow_mut().next = Some(Rc::clone(&next));
        new.borrow_mut().prev = Some(Rc::clone(&prev));
        prev.borrow_mut().next = Some(Rc::clone(&new));
    }


    /* list_add - add a new entry
     * @new: new entry to be added
     * @head: list head to add it after
     * 
     * Insert a new entry after the specified head.
     * This is good for implementing stacks.
     */
    #[allow(dead_code)]
    pub fn list_add(new: Rc<RefCell<Self>>, head: Rc<RefCell<Self>>) {
        let next = head.borrow().next.as_ref().unwrap().clone();
        ListHead::__list_add(new.clone(), head.clone(), next);
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
    pub fn list_add_tail(new: Rc<RefCell<Self>>, head: Rc<RefCell<Self>>) {
        let prev: Rc<RefCell<ListHead>> = head.borrow().prev.as_ref().unwrap().clone();
        ListHead::__list_add(new.clone(), prev, head.clone());
    }


    /**
     * Delete a list entry by making the prev/next entries
     * point to each other.
     * 
     * This is only for internal list manipulation where we know
     * the prev/next entries already!
     */
    fn __list_del(prev: Rc<RefCell<Self>>, next: Rc<RefCell<Self>>) {
        next.borrow_mut().prev = Some(Rc::clone(&prev));
        prev.borrow_mut().next = Some(Rc::clone(&next));
    }


    fn __list_del_entry(entry: Rc<RefCell<Self>>) {
        if !ListHead::list_empty(entry.clone()) {
            ListHead::__list_del(entry.borrow().prev.clone().unwrap(), entry.borrow().next.clone().unwrap());
        }
    }


    /**
     * list_del_init - deletes entry from list and reinitialize it.
     * @entry: the element to delete from the list.
     */
    #[allow(dead_code)]
    pub fn list_del_init(entry: Rc<RefCell<Self>>) {
        ListHead::__list_del_entry(entry.clone());
        ListHead::init_list_head(entry.clone());
    }


    /**
     * list_replace - replace old entry by new one
     * @old : the element to be replaced
     * @new : the new element to insert
     * 
     * If @old was empty, it will be overwritten.
     */
    #[allow(dead_code)]
    pub fn list_replace(old: Rc<RefCell<Self>>, new: Rc<RefCell<Self>>) {
        new.borrow_mut().next = Some(Rc::clone(old.borrow().next.as_ref().unwrap()));
        new.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(&new));
        new.borrow_mut().prev = Some(Rc::clone(old.borrow().prev.as_ref().unwrap()));
        new.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&new));
    }


    /**
     * list_replace_init - replace old entry by new one and initialize the old one
     * @old : the element to be replaced
     * @new : the new element to insert
     * 
     * If @old was empty, it will be overwritten.
     */
    #[allow(dead_code)]
    pub fn list_replace_init(old: Rc<RefCell<Self>>, new: Rc<RefCell<Self>>) {
        ListHead::list_replace(old.clone(), new.clone());
        ListHead::init_list_head(old.clone());
    }


    /**
     * list_swap - replace entry1 with entry2 and re-add entry1 at entry2's position
     * @entry1: the location to place entry2
     * @entry2: the location to place entry1
     */
    #[allow(dead_code)]
    pub fn list_swap(entry1: Rc<RefCell<Self>>, entry2: Rc<RefCell<Self>>) {
        let mut pos = &entry2.borrow_mut().prev.as_ref().unwrap().clone();

        ListHead::list_del_init(entry2.clone());
        ListHead::list_replace(entry1.clone(), entry2.clone());

        if Rc::ptr_eq(&pos, &entry1) {
            pos = &entry2;
        }
        ListHead::list_add(entry1.clone(), pos.clone());
    }


    /**
     * list_move - delete from one list and add as another's head
     * @list: the entry to move
     * @head: the head that will precede our entry
     */
    #[allow(dead_code)]
    pub fn list_move(list: Rc<RefCell<Self>>, head: Rc<RefCell<Self>>) {
        ListHead::__list_del_entry(list.clone());
        ListHead::list_add(list.clone(), head.clone());
    }


    /**
     * list_move_tail - delete from one list and add as another's tail
     * @list: the entry to move
     * @head: the head that will follow our entry
     */
    #[allow(dead_code)]
    pub fn list_move_tail(list: Rc<RefCell<Self>>, head: Rc<RefCell<Self>>) {
        ListHead::__list_del_entry(list.clone());
        ListHead::list_add_tail(list.clone(), head.clone());
    }


    /* TODO: fix it */
    fn __list_splice(list: &Rc<RefCell<Self>>, prev: &Rc<RefCell<Self>>, next: &Rc<RefCell<Self>>) {
        list.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(prev));
        prev.borrow_mut().next = Some(Rc::clone(list.borrow().next.as_ref().unwrap()));

        list.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(next));
        next.borrow_mut().prev = Some(Rc::clone(list.borrow().prev.as_ref().unwrap()));
    }


    /**
     * list_splice - join two lists, this is designed for stacks
     * @list: the new list to add.
     * @head: the place to add it in the first list.
     */
    pub fn list_splice(list: &Rc<RefCell<Self>>, head: &Rc<RefCell<Self>>) {
        if !ListHead::list_empty(list.clone()) {
            ListHead::__list_splice(list, head, head.borrow().next.as_ref().unwrap());
        }
    }

    /**
     * list_splice_tail - join two lists, each list being a queue
     * @list: the new list to add
     * @head: the place to add it in the first list.
     */
    pub fn list_splice_tail(list: Rc<RefCell<Self>>, head: Rc<RefCell<Self>>) {
        if !ListHead::list_empty(list.clone()) {
            list.borrow().next.as_ref().unwrap().borrow_mut().prev = Some(Rc::clone(head.borrow().prev.as_ref().unwrap()));
            head.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(list.borrow().next.as_ref().unwrap()));

            list.borrow().prev.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(&head.clone()));
            head.borrow_mut().prev = Some(Rc::clone(list.borrow().prev.as_ref().unwrap()));
        }
    }


    pub fn list_show(head: Rc<RefCell<Self>>) {
        let mut current = Some(Rc::clone(&head.borrow().next.as_ref().unwrap()));
        while let Some(node) = current {
            print!("{} -> ", node.borrow().item);
            current = node.borrow().next.clone();
            
            if Rc::ptr_eq(&current.as_ref().unwrap(), &head) {
                break;
            }
        }
        println!("Finished");
    }

    
    /* TODO: Refactor in the future, too much duplicated code */
    pub fn merge(cmp: ListCmpFunc, mut a: Link<Self>, mut b: Link<Self>) -> Link<ListHead> {
        let head = Some(ListHead::new(0));

        let a_next = a.as_ref().unwrap().borrow().next.clone();
        let b_next = b.as_ref().unwrap().borrow().next.clone();
        ListHead::list_del_init(a.as_ref().unwrap().clone());
        ListHead::list_del_init(b.as_ref().unwrap().clone());
        a = a_next;
        b = b_next;

        loop {
            match (a.clone(), b.clone()) {
                (Some(a_node), Some(b_node )) => {
                    if cmp(&a_node, &b_node) != Ordering::Greater {
                        let a_next = a_node.borrow().next.clone();
                        ListHead::list_move_tail(a_node.clone(), head.as_ref().unwrap().clone());
                        if Rc::ptr_eq(&a_node, a_next.as_ref().unwrap()) {
                            a = None;
                        } else {
                            a = a_next;
                        }
                    } else {
                        let b_next = b_node.borrow().next.clone();
                        ListHead::list_move_tail(b_node.clone(), head.as_ref().unwrap().clone());
                        if Rc::ptr_eq(&b_node, b_next.as_ref().unwrap()) {
                            b = None;
                        } else {
                            b = b_next;
                        }
                    }
                }
                (Some(_), None) => {
                    let tmp = ListHead::new(-1);
                    ListHead::list_add_tail(tmp.clone(), a.as_ref().unwrap().clone());
                    ListHead::list_splice_tail(tmp.clone(), head.as_ref().unwrap().clone());
                    break;
                }
                (None, Some(_)) => {
                    let tmp = ListHead::new(-1);
                    ListHead::list_add_tail(tmp.clone(), b.as_ref().unwrap().clone());
                    ListHead::list_splice_tail(tmp.clone(), head.as_ref().unwrap().clone());
                    break;
                }
                (None, None) => break,
            }
        }

        head
    }


    pub fn list_sort(head: Rc<RefCell<Self>>, cmp: ListCmpFunc) -> Option<Rc<RefCell<ListHead>>> {
        let mut count = 0;

        let mut list = head.borrow().next.as_ref().unwrap().clone();
        let mut pending: Vec<Rc<RefCell<ListHead>>>= Vec::new();

        loop {
            let mut bits = count;

            while bits & 1 == 1 {
                bits >>= 1;
            }

            if bits != 0 {
                let a = pending.pop();
                let b = pending.pop();

                pending.push(ListHead::merge(cmp, b, a).unwrap().clone());
            }

            let list_next = list.borrow().next.as_ref().unwrap().clone();
            ListHead::list_del_init(list.clone());

            let tmp_head = ListHead::new(-1);
            ListHead::list_add_tail(list.clone(), tmp_head.clone());

            pending.push(tmp_head.clone());

            if Rc::ptr_eq(&head, &list_next) {
                break;
            }
            list = list_next;
            count += 1;
        }

        while pending.len() > 1 {
            let a = pending.pop();
            let b = pending.pop();

            pending.push(ListHead::merge(cmp, b, a).unwrap().clone());
        }

        pending.pop()
    }

}


/* Sections for Unit tests */
#[cfg(test)]
mod tests {
    use sorted_vec::SortedVec;
    use rand::Rng;

    use super::*;

    #[test]
    fn test_new() {
        let list = ListHead::new(1);

        assert_eq!(list.borrow().item, 1);
        assert!(ListHead::list_empty(list.clone()));
    }


    #[test]
    fn test_init_list_head() {
        let list =  Rc::new(RefCell::new(ListHead{
            item: 1,
            prev: None,
            next: None,
        }));

        ListHead::init_list_head(list.clone());

        assert_eq!(list.borrow().item, 1);
        assert!(ListHead::list_empty(list.clone()));
    }


    #[test]
    fn test_list_empty() {
        let list1 = ListHead::new(1);
        let list2 = ListHead::new(2);

        ListHead::list_add_tail(list2.clone(), ListHead::new(3));

        assert!(ListHead::list_empty(list1.clone()));
        assert!(!ListHead::list_empty(list2.clone()));
    }


    #[test]
    fn test_list_add() {
        let a = ListHead::new(0);
        let b = ListHead::new(0);
        let list = ListHead::new(0);

        ListHead::list_add(a.clone(), list.clone());
        ListHead::list_add(b.clone(), list.clone());

        assert!(Rc::ptr_eq(list.borrow().next.as_ref().unwrap(), &b));
        assert!(Rc::ptr_eq(b.borrow().prev.as_ref().unwrap(), &list));
        assert!(Rc::ptr_eq(b.borrow().next.as_ref().unwrap(), &a));
    }


    #[test]
    fn test_list_add_tail() {
        let a = ListHead::new(0);
        let b = ListHead::new(0);
        let list = ListHead::new(0);

        ListHead::list_add_tail(a.clone(), list.clone());
        ListHead::list_add_tail(b.clone(), list.clone());

        assert!(Rc::ptr_eq(list.borrow().next.as_ref().unwrap(), &a));
        assert!(Rc::ptr_eq(a.borrow().prev.as_ref().unwrap(), &list));
        assert!(Rc::ptr_eq(a.borrow().next.as_ref().unwrap(), &b));
    }


    #[test]
    fn test_list_del_init() {
        let a = ListHead::new(0);
        let b = ListHead::new(0);
        let list = ListHead::new(0);

        ListHead::list_add_tail(a.clone(), list.clone());
        ListHead::list_add_tail(b.clone(), list.clone());

        ListHead::list_del_init(a.clone());

        assert!(Rc::ptr_eq(list.borrow().next.as_ref().unwrap(), &b));
        assert!(Rc::ptr_eq(b.borrow().prev.as_ref().unwrap(), &list));
        assert!(ListHead::list_empty(a.clone()));
    }


    #[test]
    fn test_list_replace() {
        let a_old = ListHead::new(0);
        let a_new = ListHead::new(0);
        let b = ListHead::new(0);
        let list = ListHead::new(0);

        ListHead::list_add_tail(a_old.clone(), list.clone());
        ListHead::list_add_tail(b.clone(), list.clone());

        ListHead::list_replace(a_old.clone(), a_new.clone());

        assert!(Rc::ptr_eq(list.borrow().next.as_ref().unwrap(), &a_new));
        assert!(Rc::ptr_eq(b.borrow().prev.as_ref().unwrap(), &a_new));
        assert!(Rc::ptr_eq(a_new.borrow().next.as_ref().unwrap(), &b));
        assert!(Rc::ptr_eq(a_new.borrow().prev.as_ref().unwrap(), &list));
    }


    #[test]
    fn test_list_replace_init() {
        let a_old = ListHead::new(0);
        let a_new = ListHead::new(0);
        let b = ListHead::new(0);
        let list = ListHead::new(0);

        ListHead::list_add_tail(a_old.clone(), list.clone());
        ListHead::list_add_tail(b.clone(), list.clone());

        ListHead::list_replace_init(a_old.clone(), a_new.clone());

        assert!(Rc::ptr_eq(list.borrow().next.as_ref().unwrap(), &a_new));
        assert!(Rc::ptr_eq(b.borrow().prev.as_ref().unwrap(), &a_new));
        assert!(Rc::ptr_eq(a_new.borrow().next.as_ref().unwrap(), &b));
        assert!(Rc::ptr_eq(a_new.borrow().prev.as_ref().unwrap(), &list));

        assert!(ListHead::list_empty(a_old.clone()));
    }


    #[test]
    fn test_list_swap() {
        let a = ListHead::new(0);
        let b = ListHead::new(0);
        let list = ListHead::new(0);

        ListHead::list_add_tail(a.clone(), list.clone());
        ListHead::list_add_tail(b.clone(), list.clone());

        ListHead::list_swap(a.clone(), b.clone());

        assert!(Rc::ptr_eq(&b, list.borrow().next.as_ref().unwrap()));
        assert!(Rc::ptr_eq(&a, list.borrow().prev.as_ref().unwrap()));

        assert!(Rc::ptr_eq(&a, b.borrow().next.as_ref().unwrap()));
        assert!(Rc::ptr_eq(&list, b.borrow().prev.as_ref().unwrap()));

        assert!(Rc::ptr_eq(&list, a.borrow().next.as_ref().unwrap()));
        assert!(Rc::ptr_eq(&b, a.borrow().prev.as_ref().unwrap()));
    }


    #[test]
    fn test_list_move() {
        let a = ListHead::new(0);
        let b = ListHead::new(0);
        let list1 = ListHead::new(0);
        let list2 = ListHead::new(0);

        ListHead::list_add_tail(a.clone(), list1.clone());
        ListHead::list_add_tail(b.clone(), list2.clone());

        ListHead::list_move(a.clone(), list2.clone());

        assert!(ListHead::list_empty(list1.clone()));

        assert!(Rc::ptr_eq(&a, list2.borrow().next.as_ref().unwrap()));
        assert!(Rc::ptr_eq(&b, a.borrow().next.as_ref().unwrap()));
    }


    #[test]
    fn test_list_move_tail() {
        let a = ListHead::new(0);
        let b = ListHead::new(0);
        let list1 = ListHead::new(0);
        let list2 = ListHead::new(0);

        ListHead::list_add_tail(a.clone(), list1.clone());
        ListHead::list_add_tail(b.clone(), list2.clone());

        ListHead::list_move_tail(a.clone(), list2.clone());

        assert!(ListHead::list_empty(list1.clone()));
        assert!(Rc::ptr_eq(&b, list2.borrow().next.as_ref().unwrap()));
        assert!(Rc::ptr_eq(&a, b.borrow().next.as_ref().unwrap()));
    }


    #[test]
    fn test_list_sort() {
        let mut rng = rand::thread_rng();
        let mut map = SortedVec::new();
        let list = ListHead::new(-1);

        for _ in 0..100 {
            let num: i32 = rng.gen();
            map.insert(num);
            let node = ListHead::new(num);
            ListHead::list_add_tail(node.clone(), list.clone());
        }

        let sorted_list = ListHead::list_sort(list.clone(), cmp_func);


        let mut i: usize = 0;
        let mut current = sorted_list.as_ref().unwrap().borrow().next.clone();
        while let Some(node) = current {
            assert_eq!(map[i], node.borrow().item);

            current = node.borrow().next.clone();
            i += 1;
            if Rc::ptr_eq(current.as_ref().unwrap(), &sorted_list.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(i, 100);

        i = 99;
        current = sorted_list.as_ref().unwrap().borrow().prev.clone();
        while let Some(node) = current {
            assert_eq!(map[i], node.borrow().item);

            current = node.borrow().prev.clone();

            if i != 0 {
                i -= 1;
            }
            if Rc::ptr_eq(current.as_ref().unwrap(), &sorted_list.as_ref().unwrap()) {
                break;
            }
        }

        assert_eq!(i, 0);
    }
}
