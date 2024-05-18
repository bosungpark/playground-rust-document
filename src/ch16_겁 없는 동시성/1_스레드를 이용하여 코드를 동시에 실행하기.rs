use std::thread;
use std::time::Duration;

fn spawn으로_새로운_스레드_생성하기() {
    thread::spawn(|| {
        println!("스레드에서 실행된 클로저");
    });

    thread::sleep(Duration::from_millis(1));
}

fn join_핸들을_사용하여_모든_스레드가_끝날_때까지_기다리기() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("스레드에서 실행된 클로저 {i}");
        }
    });

    // 핸들에 대한 스레드가 종료될 때까지 현재 실행 중인 스레드를 블록
    handle.join().unwrap();
}

fn 스레드에_move_클로저_사용하기() {
    // move 클로저는 thread::spawn에 넘겨지는 클로저와 함께 자주 사용된다.
    // 클로저가 환경으로부터 사용하는 값의 소유권을 갖게 되어 소유권이 이동이 가능하기 때문

    let v = vec![1, 2, 3];

    // 1. 실패
    // let handle = thread::spawn(|| {
    //     println!("{:?}", v);
    // });

    // ㄴ> 러스트는 생성된 스레드가 얼마나 오랫동안 실행될지 알 수 없다.
    // ㄴ> v에 대한 참조자가 항상 유효할 것인지 알지 못한다.
    // ㄴ> 예를 들어 중간에 drop(v); 을 한다고 생각해보자,,,

    // 2. 성공
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // ㄴ> 소유권 이동!

    handle.join().unwrap();
}
