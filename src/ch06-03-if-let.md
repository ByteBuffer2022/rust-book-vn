## Quy trình kiểm soát ngắn gọn với `if let`

Cú pháp `if let` cho phép bạn kết hợp `if` và `let` thành một cách ít dài dòng hơn để xử lý các giá trị khớp với một mẫu trong khi bỏ qua phần còn lại. 
Xem xét chương trình trong Listing 6-6 phù hợp với một giá trị `Option<u8>` trong `config_max`
nhưng chỉ muốn thực thi mã nếu giá trị là trường hợp `Some`.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

<span class="caption">Listing 6-6: `match` chỉ quan tâm đến việc thực thi mã khi giá trị là
 `Some`</span>

Nếu giá trị là `Some`, chúng ta in ra giá trị `Some` bằng cách liên kết giá trị với biến `max` trong mẫu. 
Chúng ta không muốn làm bất cứ điều gì với giá trị `None`. Để đáp ứng biểu thức `match`, chúng ta phải thêm `_ =>
()` sau khi chỉ xử lý một trường hợp, đó là đoạn mã khó chịu để thêm vào.


Thay vào đó, chúng ta có thể viết điều này theo cách ngắn hơn bằng cách sử dụng `if let`. 
Đoạn mã sau hoạt động giống như `match` trong Listing 6-6:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

Cú pháp `if let` nhận một mẫu và một biểu thức được phân tách bằng dấu bằng. 
Nó hoạt động giống như cách một `match`mà trong đó biểu thức được đưa ra cho `match`
và mẫu là nhánh đầu tiên. Trong trường hợp này, mẫu là `Some(max)`, 
và giá trị `max` liên kết với giá trị bên trong `Some`. Sau đó chúng ta có thể sử dụng `max` 
trong phần thân của khối `if let` theo cách tương tự như chúng tôi đã sử dụng `max` in
tương ứng nhánh `match`. Đoạn mã trong khối `if let` sẽ không chạy nếu giá trị không khớp với mẫu.

Sử dụng `if let` có nghĩa là ít nhập hơn, ít thụt lề hơn và ít mã mẫu hơn.
Tuy nhiên, bạn sẽ matts đi sự kiểm tra đầy đủ mà một `match` được thực thi. Lựa chọn giữa `match` và `if let` 
phụ thuộc vào những gì bạn đang làm trong tình huống cụ thể của bạn và và liệu việc đạt được sự ngắn gọn có phải là một sự đánh đổi thích hợp để đánh mất việc kiểm tra đầy đủ hay không.

Nói cách khác, bạn có thể nghĩ về `if let` như là cú pháp ngọt ngào cho một `match` chạy mã khi giá trị khớp với một mẫu và sau đó bỏ qua tất cả các giá trị khác.

Chúng tôi có thể bao gồm một `else` với một `if let`. Khối code đó sẽ đi với `else` giống như khối mã sẽ đi với trường hợp `_` casetrong biểu thức 
`match` điều đó tương đương với `if let` và `else`. Nhớ lại định nghĩa enum `Coin` ở Listing 6-4, trường hợp `Quarter` cũng có giá trị là một
`UsState`. Nếu chúng ta muốn đếm tất cả các đồng tiền không phải đồng 25 xu mà chúng ta thấy đồng thời thông báo trạng thái của đồng 25 xu, chúng tôi có thể làm điều đó với một biểu thức `match` như thế này:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

Hoặc chúng ta có thể sử dụng một biểu thức `if let` và `else` như thế này:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

Nếu bạn gặp tình huống trong đó chương trình của bạn có logic quá dài dòng để diễn đạt bằng cách sử dụng
 `match`, nhớ rằng that `if let` cũng có trong hộp công cụ Rust của bạn.

## Tóm tắt

Bây giờ chúng ta đã đề cập đến cách sử dụng enum để tạo các loại tùy chỉnh có thể là một trong tập hợp các giá trị được liệt kê. 
Chúng tôi đã chỉ ra cách thư viện tiêu chuẩn `Option<T>`
loại giúp bạn sử dụng hệ thống loại để ngăn ngừa lỗi. 
Khi các giá trị enum có dữ liệu bên trong chúng, bạn có thể sử dụng `match` hoặc `if let` để trích xuất và sử dụng các giá trị đó, tùy thuộc vào số lượng trường hợp bạn cần xử lý.


Các chương trình Rust của bạn hiện có thể diễn đạt các khái niệm trong miền của bạn bằng cách sử dụng cấu trúc và enum.
Tạo các loại tùy chỉnh để sử dụng trong API của bạn đảm bảo an toàn cho loại: trình biên dịch sẽ đảm bảo các hàm của bạn chỉ nhận các giá trị kiểu mà mỗi hàm mong đợi.


Để cung cấp một API được tổ chức tốt cho người dùng của bạn, dễ sử dụng và chỉ hiển thị chính xác những gì người dùng của bạn sẽ cần, bây giờ hãy chuyển sang các mô-đun của Rust.

