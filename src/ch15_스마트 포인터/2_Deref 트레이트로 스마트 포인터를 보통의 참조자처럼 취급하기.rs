
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn Deref_트레이트로_스마트_포인터를_보통의_참조자처럼_취급하기() {
    // Deref 트레이트를 구현하면 역참조 연산자 (dereference operator) * 동작의 커스터마이징을 가능

    // 포인터를 따라가서 값 얻기
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Box<T>를 참조자처럼 사용하기
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // 자체 스마트 포인터 정의하기
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
