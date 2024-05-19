use std::sync::mpsc;
use std::thread;

fn 메시지_패싱을_사용하여_스레드_간_데이터_전송하기() {
    println!(
        "
    메시지 패싱:
    스레드들 혹은 액터들이 서로 데이터를 담은 메시지를 보내서 통신하는 것
    메모리를 공유하여 통신하는 대신, 통신하여 메모리를 공유하는 방식

    러스트의 표준 라이브러리는 한 채널이 값을 생산하는 송신 단말을 여러 개 가질 수 있지만 
    값을 소비하는 수신 단말은 단 하나만 가질 수 있다(mpsc: multiple producer, single consumer).

    ㄴ> 아이디어가 참 단순하면서 재미있는듯
    "
    );

    let (송신자, 수신자) = mpsc::channel();

    let handle = thread::spawn(move || {
        let 메시지 = String::from("안녕?");

        송신자.send(메시지).unwrap();
    });

    // 1. recv: 메인 스레드의 실행을 블록
    let 응답 = 수신자.recv().unwrap();
    println!("{}", 응답); // 안녕?

    // 2. try_recv: 블록없이 즉시 실행
    // handle.join().unwrap();

    // let 응답 = 수신자.try_recv().unwrap();
    // println!("{}", 응답); // 안녕?
}

fn 채널과_소유권_이동() {
    println!(
        "
        대략 소유권 이동 없이 값을 채널에 이동 시킬 경우, 값의 변경 등으로 인해 의도치 않은 동작을 할 수 있다는 내용.
    "
    );
}

use std::time::Duration;

fn 여러_값_보내기와_수신자가_기다리는지_알아보기() {
    let (송신자, 수신자) = mpsc::channel();

    thread::spawn(move || {
        let 메시지들 = vec![
            String::from("1번 메시지"),
            String::from("2번 메시지"),
            String::from("3번 메시지"),
        ];

        for 메시지 in 메시지들 {
            송신자.send(메시지).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 이터레이터처럼 쓸 수도 있다!
    for 메시지 in 수신자 {
        println!("{}", 메시지);
    }
    // 1번 메시지
    // 2번 메시지
    // 3번 메시지
}

fn 송신자를_복제하여_여러_생산자_만들기() {
    let (송신자, 수신자) = mpsc::channel();

    let 송신자_복제 = 송신자.clone();

    thread::spawn(move || {
        let 메시지들 = vec![
            String::from("1번 메시지"),
            String::from("2번 메시지"),
            String::from("3번 메시지"),
        ];

        for 메시지 in 메시지들 {
            송신자.send(메시지).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let 메시지들 = vec![
            String::from("1번 복제 메시지"),
            String::from("2번 복제 메시지"),
            String::from("3번 복제 메시지"),
        ];

        for 메시지 in 메시지들 {
            송신자_복제.send(메시지).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for 메시지 in 수신자 {
        println!("{}", 메시지);
    }
    // 1번 메시지
    // 1번 복제 메시지
    // 2번 복제 메시지
    // 2번 메시지
    // 3번 복제 메시지
    // 3번 메시지
}
