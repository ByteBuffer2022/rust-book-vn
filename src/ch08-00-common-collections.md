# Common Collections

Thư viện tiêu chuẩn của Rust bao gồm một số cấu trúc dữ liệu rất hữu ích được gọi là
*collections*. Hầu hết các kiểu dữ liệu khác đại diện cho một giá trị cụ thể, nhưng các collections 
có thể chứa nhiều giá trị. Không giống như các loại array và tuple
được tích hợp sẵn,dữ liệu mà các collectionsnày trỏ tới được lưu trữ trên heap, 
có nghĩa là lượng dữ liệu không cần biết tại thời điểm biên dịch và có thể tăng lên hoặc thu nhỏ
khi chương trình chạy.Mỗi loại collections có tính năng và costs khác nhau,và chọn một tính năng
thích hợp cho tình huống hiện tại của bạn là một tính năng bạn sẽ phát triển theo thời gian. 
Trong chương này, chúng ta sẽ thảo luận về ba collections được sử dụng rất thường xuyên trong các chương trình Rust:

* Một *vector* cho phép bạn lưu trữ một số lượng giá trị bên cạnh nhau.
* Một *string* là một tập hợp các ký tự. Trước đây chúng ta đã đề cập đến kiểu `String`, nhưng trong chương này chúng ta sẽ nói sâu hơn về nó.
* Một *hash map* cho phép bạn liên kết giá trị với một khóa cụ thể. Đó là một triển khai cụ thể của cấu trúc dữ liệu chung hơn gọi là một *map*.

Để tìm hiểu về các loại collections khác được cung cấp bởi thư viện tiêu chuẩn, hãy xem [the documentation][collections].

Chúng ta sẽ thảo luận về cách tạo và cập nhật vectors, strings, and hash maps, cũng như những gì làm cho mỗi thứ trở nên đặc biệt.

[collections]: ../std/collections/index.html
