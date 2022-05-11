fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word sẽ nhận được giá trị 5

    s.clear(); // ở đây chúng ta làm trống String, làm cho nó bằng ""

    // word vẫn có giá trị 5 ở đây, nhưng không còn String nào mà chúng ta có thể sử dụng
    // một cách có ý nghĩa với giá trị 5 này nên word bây giờ hoàn toàn không có giá trị sử dụng!
}
// ANCHOR_END: here
