fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 đi ra khỏi scope tại đây, vì vậy chúng ta có thể tạo một 
      //tham chiếu mới mà không gặp vấn đề gì.

    let r2 = &mut s;
    // ANCHOR_END: here
}
