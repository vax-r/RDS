pub mod list;
use list::ListHead;

use std::rc::Rc;


fn main() {
    let first = ListHead::new(1);
    let second = ListHead::new(2);
    let third = ListHead::new(3);

    if ListHead::list_empty(first.clone()) {
        println!("List empty!")
    }

    ListHead::list_add(second.clone(),first.clone());
    ListHead::list_add_tail(third.clone(), first.clone());
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
