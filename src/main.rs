pub mod list;
use list::{cmp_func, ListHead};

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
    ListHead::list_del_init(second.clone());
    ListHead::list_replace(third.clone(), second.clone());

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


    let list1 = ListHead::new(-1);
    let n1 = ListHead::new(1);
    let n2 = ListHead::new(2);
    let n3 = ListHead::new(3);

    ListHead::list_add_tail(n1.clone(), list1.clone());
    ListHead::list_add_tail(n2.clone(), list1.clone());
    ListHead::list_add_tail(n3.clone(), list1.clone());


    let list2 = ListHead::new(-1);
    let a1 = ListHead::new(10);
    let a2 = ListHead::new(11);
    let a3 = ListHead::new(12);

    ListHead::list_add_tail(a1.clone(), list2.clone());
    ListHead::list_add_tail(a2.clone(), list2.clone());
    ListHead::list_add_tail(a3.clone(), list2.clone());

    let list3 = ListHead::merge(cmp_func, Some(list1.clone()), Some(list2.clone()));

    let mut current = Some(Rc::clone(list3.as_ref().unwrap()));
    while let Some(node) = current {
        print!("{} -> ", node.borrow().item);
        current = node.borrow().next.clone();
        
        if Rc::ptr_eq(&current.as_ref().unwrap(), list3.as_ref().unwrap()) {
            break;
        }
    }
    println!("Finished");


    let list4 = ListHead::new(-1);
    let l1 = ListHead::new(10);
    let l2 = ListHead::new(1);
    let l3 = ListHead::new(22);

    ListHead::list_add_tail(l1.clone(), list4.clone());
    ListHead::list_add_tail(l2.clone(), list4.clone());
    ListHead::list_add_tail(l3.clone(), list4.clone());

    let list5 = ListHead::list_sort(list4.clone(), cmp_func);

    let mut current = Some(Rc::clone(list5.as_ref().unwrap()));
    while let Some(node) = current {
        print!("{} -> ", node.borrow().item);
        current = node.borrow().next.clone();
        
        if Rc::ptr_eq(&current.as_ref().unwrap(), list5.as_ref().unwrap()) {
            break;
        }
    }
    println!("Finished");

}
