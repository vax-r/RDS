pub mod list;
use list::{cmp_func, ListHead};

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let list_size = 100;
    let list = ListHead::new(-1);

    for _ in 0..list_size {
        let num = rng.gen();
        let node = ListHead::new(num);
        ListHead::list_add_tail(node.clone(), list.clone());
    }

    let sorted_list = ListHead::list_sort(list.clone(), cmp_func);

    ListHead::list_show(sorted_list.as_ref().unwrap().clone());
}
