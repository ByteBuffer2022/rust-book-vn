fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() nối một ký tự vào một chuỗi

    println!("{}", s); // Điều này sẽ in ra `hello, world! '
                       // ANCHOR_END: here
}
