fn main() {
    let s1 = gives_ownership();         // gives_ownership di chuyển giá trị trả về của nó
                                        // vào s1

    let s2 = String::from("hello");     // s2 đi vào scope

    let s3 = takes_and_gives_back(s2);  // s2 is được di chuyển vào
                                        // takes_and_gives_back, và 
                                        // giá trị trả về của nó được di chuyển vào s3
} // Tại đây, s3 đi ra khỏi scope và bị drop. s2 đã bị di chuyển, vậy nên không có gì
  // xảy ra. s1 đi ra khỏi scope và bị drop.

fn gives_ownership() -> String {             // gives_ownership sẽ di chuyển 
                                             // giá trị trả về của nó nó vào hàm gọi nó
                                             
    let some_string = String::from("yours"); // some_string đi vào scope

    some_string                              // some_string được trả về và
                                             // chuyển sang hàm gọi
                          
}

// Hàm này nhận một a String trả về một string
fn takes_and_gives_back(a_string: String) -> String { // a_string đi vào
                                                      // scope

    a_string  // a_string được trả về và chuyển sang hàm gọi
}
