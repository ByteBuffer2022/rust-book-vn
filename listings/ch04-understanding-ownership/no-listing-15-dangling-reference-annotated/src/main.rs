fn main() {
    let reference_to_nothing = dangle();
}

// ANCHOR: here
fn dangle() -> &String { // trả về một tham chiếu đến một String

    let s = String::from("hello"); // s là một String mới

    &s // chúng ta trả về một tham chiếu đến  String, s
} // Tại đây, s đi ra khỏi scope, và bị dropped. Bộ nhớ của nó đã bị mất đi.
  // Nguy hiểm!
// ANCHOR_END: here
