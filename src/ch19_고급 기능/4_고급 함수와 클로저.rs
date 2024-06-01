fn 함수_포인터() {
    /*
    일반 함수를 함수에 전달할 수도 있다.
    fn 타입은 함수 포인터 (function pointer) 라고 한다.
    함수 포인터로 함수를 전달하면 함수를 다른 함수에 대한 인수로 사용할 수 있다.

    함수 포인터는 세 가지 클로저 트레이트 (Fn, FnMut, FnOnce) 을 모두 구현하므로,
    클로저를 기대하는 함수에 대한 인수로 함수 포인터를 언제나 전달할 수 있다.

    클로저가 아닌 fn만 허용하고 싶은 경우의 한 가지 예로는 클로저가 없는 외부 코드와 상호작용할 때
     */

    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

fn 클로저_반환하기() {
    /*
    대부분의 경우는 트레이트를 구현하는 구체적 타입을 함수의 반환 값으로 사용할 수 있다.
    하지만 클로저는 트레이트로 표현되고, 반환할 수 있는 구체적 타입이 없기 때문에 클로저를 직접 반환할 수는 없다.
    예를 들어 함수 포인터 fn은 반환 타입으로 사용될 수 없다.
     */

    //  컴파일 실패
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     // doesn't have a size known at compile-time
    //     |x| x + 1
    // }

    // 박스를 사용한 우회 방법
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
