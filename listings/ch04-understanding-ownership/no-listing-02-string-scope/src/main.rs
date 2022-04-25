fn main() {
    // ANCHOR: here
    {
        let s = String::from("hello"); // s có giá trị từ thời điểm này trở đi

        // do stuff with s
    }                                  // phạm vi này hiện đã kết thúc và không  
                                       // còn giá trị
    // ANCHOR_END: here
}
