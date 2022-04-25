fn main() {
    let s = String::from("hello");  // s đi vào scope

    takes_ownership(s);             // giá trị của s di chuyển vào hàm...
                                    // ... và như vậy không còn giá trị ở đây

    let x = 5;                      // x đi vào scope

    makes_copy(x);                  // x sẽ di chuyển vào hàm,
                                    // nhưng i32 là Copy, vì vậy vẫn hoàn toàn có thể
                                    // sử dụng x sau đó

} // Tại đây, x đi ra khỏi scope, sau đó là s. Nhưng vì giá trị của s đã được move, nên không có gì 
  // đặc biệt xảy ra.

fn takes_ownership(some_string: String) { // some_string đi vào scope
    println!("{}", some_string);
} // Tại đây, some_string đi ra khỏi scope và `drop` được gọi. Bộ nhớ được giải phóng.

fn makes_copy(some_integer: i32) { // some_integer đi vào scope
    println!("{}", some_integer);
} // Tại đây, some_integer đi ra khỏi scope. Không có gì đặc biệt xảy ra.
