# Các khái niệm lập trình chung 

Chương này bao quát các khái niệm xuất hiện trong hầu hết mọi ngôn ngữ lập trình
và cách chúng hoạt động trong Rust. Nhiều ngôn ngữ lập trình có nhiều điểm chung
cốt lõi. Không có khái niệm nào trong chương này là duy nhất đối với Rust, nhưng
chúng ta sẽ thảo luận về chúng trong ngữ cảnh của Rust và giải thích các quy ước
xung quanh việc sử dụng các khái niệm này.

Cụ thể, bạn sẽ tìm hiểu về các biến (variables), kiểu dữ liệu cơ bản, hàm (functions), 
comments và control flow. Những nền tảng này sẽ có trong mọi chương trình Rust và học 
chúng sớm cho bạn một nền tảng cốt lõi vừng chắc để bắt đầu.

> #### Từ khóa
>
> Ngôn ngữ Rust có một tập hợp các từ khóa *(keywords)* chỉ dành riêng cho 
> ngôn ngữ đó, giống như các ngôn ngữ khác. Hãy nhớ rằng bạn không thể sử dụng
> những từ này cho tên biến hoặc tên hàm. Phần lớn từ khóa có ý nghĩa đặc biệt
> và bạn sẽ sử dụng chúng để thực hiện các tác vụ khác nhau trong chương trình Rust
> của mình; một số ít không có chức năng hiện tại liên kết với chúng nhưng đã được
> dành riêng cho chức năng có thể được thêm vào Rust trong tương lai. Bạn có thể
> tìm thấy danh sách các từ khóa trong [Phụ lục A][appendix_a]<!-- ignore -->.

[appendix_a]: appendix-01-keywords.md
