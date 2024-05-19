
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn Drop_트레이트로_메모리_정리_코드_실행하기() {
    println!(
        "
        Drop 트레이트의 기능이 스마트 포인터를 구현할 때 거의 항상 이용
        Box<T>가 버려질 때는 이 박스가 가리키고 있는 힙 공간의 할당을 해제
        "
    );

    // 1. 직접 드랍하기
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.\n");

    // 2. 트레이트를 이용하기
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}