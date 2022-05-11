fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// ANCHOR: here
fn calculate_length(s: &String) -> usize { //s là một tham chiếu đến một String
    s.len()
} // Tại đây, s đi ra khỏi scope. Nhưng vì nó không có quyền sở hữu của String nó đề cập đến, 
  // nên không có gì xảy ra.
// ANCHOR_END: here
