fn main() {}

#[test]
fn 테스트1() {
    assert_eq!(1, 1);
}

#[test]
fn 테스트2() {
    assert_eq!(1, 1);
}

#[test]
#[ignore]
fn 무시할_테스트1() {
    assert_eq!(1, 1);
}