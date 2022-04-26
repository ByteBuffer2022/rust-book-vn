# Các khái niệm lập trình chung

Chương này bao gồm các khái niệm xuất hiện hầu hết trong mọi ngôn ngữ lập trình
và cách chúng hoạt động trong Rust. Nhiều ngôn ngữ lập trình có nhiều điểm chung
trong cố lõi của chúng. Không có khái niệm nào trong chương trình là duy nhất
đối với Rust nhưng chúng ta sẽ thảo luận về chúng trong ngữ cảnh Rust và giải
thích các quy ước xung quanh việc sử dụng các khái niệm này.

Cụ thể, bạn sẽ tìm hiểu về các biến, các kiểu dữ liệu cơ bản, hàm, comment và control flow.
Những nền tảng này sẽ có mặt trong mọi chương trình Rust và học chúng sớm sẽ cung cấp cho
bạn một nền tảng cốt lõi vững chắc.

> #### Từ khóa
>
> Ngôn ngữ Rust có một tập hợp các *từ khóa (keywords)* chỉ dành riêng cho ngôn
> ngữ đó, giống như nhiều ngôn ngữ khác. Hãy nhớ rằng bạn không thể sử dụng các
> từ khóa này để đặt tên biến hay tên hàm. Hầu hết từ khóa có ý nghĩa đặc biệt
> và bạn sẽ sử dụng chúng để thực hiện nhiều tác vụ khác nhau trong chương trình
> Rust; một vài tự khóa hiện tại chưa có chức năng đi kèm với chúng nhưng đã
> được dành riêng cho các chức năng có thể được thêm vào Rust trong tương lai.
> Bạn có thể tìm thấy danh sách các từ khóa trong [Phụ lục A][appendix_a]<!-- ignore -->.

[appendix_a]: appendix-01-keywords.md
