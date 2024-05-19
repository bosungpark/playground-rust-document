enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn Rc_참조_카운트_스마트_포인터() {
    // 명시적으로 복수 소유권을 가능하게 하려면 러스트의 Rc<T> 타입을 이용해야 함

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
