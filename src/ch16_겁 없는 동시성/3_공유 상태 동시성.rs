
// use std::rc::Rc;
// Rc<T>는 스레드를 교차하면서 공유하기에는 안전하지 않다.
// Rc<T>가 참조 카운트를 관리할 때, 각 clone 호출마다 카운트에 더하고 각 클론이 버려질 때 카운트에서 제거한다.
// 하지만 다른 스레드에 의해 카운트가 변경될 수 있다.
use std::sync::{Arc, Mutex};
use std::thread;

fn 공유_상태_동시성() {
    println!(
        "
    동시성을 다루는 또 하나의 방법은 여러 스레드가 동일한 공유 데이터에 접근하는 것

    메시지 패싱은 단일소유권에 가깝다면
    공유 메모리 동시성은 복수 소유권과 비슷하다.

    뮤텍스:
    
    뮤텍스는 상호 배제 (mutual exclusion) 의 줄임말
    뮤텍스 내부의 데이터에 접근하려면 스레드는 먼저 뮤텍스의 락을 획득해야 함
    데이터 사용이 끝나면 반납해야 다른 스레드가 락을 얻을 수 있음

    단, 당연하게도 Mutex<T>가 모든 종류의 논리 에러를 예방하지는 못한다.
    데드락 등의 상황이 발생할 수 있다.
    "
    );

    // 1. 뮤텍스란?
    let m = Mutex::new(5);
    // Mutex<T>는 스마트 포인터 이다.
    // 스코프 밖으로 벗어났을 때 자동으로 락을 해제(Drop)하기 때문에 실수의 여지가 없다.
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m); // Mutex { data: 6, poisoned: false, .. }

    // 2. 여러 스레드 사이에서 Mutex<T> 공유하기
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // counter는 불변이지만 내부값에 대한 가변 참조자를 가지고 올 수 있음
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap()); // Result: 10
}
