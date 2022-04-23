fn main() {
    // ANCHOR: here
    {                      // s không hợp lệ ở đây, nó chưa được khai báo
        let s = "hello";   // s có giá trị từ thời điểm này trở đi

        // nơi làm những thứ với s
    }                      // scope này hiện đã kết thúc và s không còn hợp lệ
    // ANCHOR_END: here
}