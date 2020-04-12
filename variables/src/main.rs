fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // const MAX_POINT: u32 = 100_000;

    // println!("{}", MAX_POINT);

    // let x = 5;

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The value of y is: {}", y);
    
    // let s = String::from("hello");  // s가 스코프 안으로 들어왔습니다.

    // takes_ownership(s);             // s의 값이 함수 안으로 이동했습니다...
    //                                 // ... 그리고 이제 더이상 유효하지 않습니다.
    // let x = 5;                      // x가 스코프 안으로 들어왔습니다.

    // makes_copy(x);                  // x가 함수 안으로 이동했습니다만,
    //                                 // i32는 Copy가 되므로, x를 이후에 계속
    //                                 // 사용해도 됩니다.
    
    // let s1 = String::from("hello");

    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);

    // let mut s = String::from("hello");

    // let r1 = &s; // 문제 없음
    // let r2 = &s; // 문제 없음
    // let r3 = &mut s; // 큰 문제

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

} // 여기서 x는 스코프 밖으로 나가고, s도 그 후 나갑니다. 하지만 s는 이미 이동되었으므로,
  // 별다른 일이 발생하지 않습니다.

// fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어왔습니다.
//     println!("{}", some_string);
// } // 여기서 some_string이 스코프 밖으로 벗어났고 `drop`이 호출됩니다. 메모리는
//   // 해제되었습니다.

// fn makes_copy(some_integer: i32) { // some_integer이 스코프 안으로 들어왔습니다.
//     println!("{}", some_integer);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }