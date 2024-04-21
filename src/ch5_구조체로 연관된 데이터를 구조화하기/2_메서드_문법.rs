// fn main() {
//     메서드_정의하기();
// }

struct 구조체 {
    이름: String,
}

// self의 소유권을 가져올 수도,
// 지금처럼 self를 불변으로 빌려올 수도,
// 가변으로 빌려올 수도 있다.
impl 구조체 {
    // 게터
    fn 이름(&self) {
        print!("안녕하세요 저는 {} 입니다.", self.이름)
    }

    fn 인사(&self) {
        print!("안녕하세요 저는 {} 입니다.", self.이름)
    }

    fn 동명이인(&self, other: &구조체) {
        print!("{}", self.이름 == other.이름)
    }

    // impl 블록 내에 구현된 모든 함수를 연관 함수 (associated function) 라고 함
    fn new(name: String) -> Self {
        Self { 이름: name }
    }
}

fn 메서드_정의하기() {
    let 인스턴스 = 구조체::new("구조체".to_string());

    인스턴스.이름(); // 안녕하세요 저는 구조체 입니다.
    인스턴스.인사(); // 안녕하세요 저는 구조체 입니다.

    let 인스턴스2 = 구조체::new("팔조체".to_string());

    인스턴스.동명이인(&인스턴스2); // false
}
