fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // biến r1 và r2 sẽ không được sử dụng sau thời điểm này 

    let r3 = &mut s; // no problem
    println!("{}", r3);
    // ANCHOR_END: here
}
