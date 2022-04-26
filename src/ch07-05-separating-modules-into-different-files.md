## Tách các module thành các file khác nhau

Cho đến nay, tất cả các ví dụ trong chương này đều xác định nhiều module trong một file.
Khi các module trở nên lớn, bạn có thể muốn chuyển định nghĩa của chúng sang một file riêng biệt 
để làm cho code điều hướng dễ dàng hơn.

Ví dụ: hãy bắt đầu từ code trong Listing 7-17 và trích xuất các module vào các file thay vì xác định tất cả các module trong file crate root.
Trong trường hợp này,file crate root là *src/lib.rs*, nhưng quy trình này cũng hoạt động với các binary crate có file crate root là *src/main.rs*.

Đầu tiên, chúng tôi sẽ giải nén module `front_of_house` vào file nó sở hữu. Xóa code bên trong dấu ngoặc nhọn cho module `front_of_house`,
chỉ để lại khai báo` mod front_of_house; `, để *src/lib.rs* chứa code được hiển thị trong Listing 7-21. 
Lưu ý rằng điều này sẽ không được biên dịch cho đến khi chúng tôi tạo tệp *src/front_of_house.rs* trong Listing 7-22.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

<span class="caption">Listing 7-21: Khai báo module `front_of_house` có phần thân sẽ nằm trong *src/front_of_house.rs*</span>

Tiếp theo, đặt code nằm trong dấu ngoặc nhọn vào một tệp mới có tên
*src/front_of_house.rs*, như được hiển thị trong Listing 7-22. 
Trình biên dịch biết tìm kiếm trong file này vì khai báo module mà nó tìm thấy trong crate root với tên `front_of_house`.

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

<span class="caption">Listing 7-22: Các định nghĩa bên trong module `front_of_house`
trong *src/front_of_house.rs*</span>

Lưu ý rằng bạn chỉ cần tải nội dung của file bằng cách sử dụng `mod`
khai báo một lần ở đâu đó trong cây module của bạn. Sau khi trình biên dịch biết file 
là một phần của dự án (và biết mã nằm ở đâu trong cây module vì nơi bạn đã đặt câu lệnh `mod`),
các file khác trong dự án của bạn nên tham chiếu đến code trong file đó bằng cách sử dụng 
đường dẫn đến nơi nó được khai báo như được đề cập trong phần [“Paths for Referring to an Item in the Module
Tree”][paths]<!-- ignore -->. Nói cách khác, `mod` *không* phải là một hoạt động “bao gồm ”mà các ngôn ngữ lập trình khác có.

Tiếp theo, chúng tôi cũng sẽ giải nén module `hosting` vào file nó sở hữu. Quá trình này 
hơi khác một chút vì `hosting` là một module con của` front_of_house`, không phải của root module. 
Tệp cho `hosting` sẽ nằm trong một thư mục được đặt tên cho vị trí của nó trong cây module.

Để bắt đầu di chuyển `hosting`, chúng tôi thay đổi *src/front_of_house.rs* để chỉ chứa phần khai báo của module` hosting`:

<span class="filename">Filename: src/front_of_house.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

Sau đó, chúng tôi tạo một thư mục *src/front_of_house* và một file *src/front_of_house/hosting.rs* để
chứa các định nghĩa được thực hiện trong module `hosting`:

<span class="filename">Filename: src/front_of_house/hosting.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

Thay vào đó, nếu chúng tôi đặt *hosting.rs* trong thư mục *src*, trình biên dịch sẽ mong đợi code 
đó nằm trong module `hosting` được khai báo trong crate root, không phải là phần tử con của module` front_of_house`. 
Các quy tắc mà trình biên dịch tuân theo để biết những file nào cần tìm cho code của module có nghĩa 
là các thư mục và tệp phù hợp hơn với cây module.

> ### Đường dẫn tệp thay thế
>
> Phần này bao gồm các đường dẫn file khuôn mẫu nhất mà trình biên dịch Rust sử dụng;
> nhưng một đường dẫn file cũ hơn cũng vẫn được hỗ trợ.
>
> Đối với module có tên `front_of_house` được khai báo trong crate root, trình biên dịch
> sẽ tìm thấy code của module trong:
>
> * *src/front_of_house.rs* (những gì chúng tôi đã đề cập)
> * *src/front_of_house/mod.rs* (đường dẫn cũ hơn, vẫn được hỗ trợ)
>
> Đối với một module có tên là `hosting` là một module con của` front_of_house`,
> trình biên dịch sẽ tìm kiếm code của module trong:
>
> * *src/front_of_house/hosting.rs* (những gì chúng tôi đã đề cậ)
> * *src/front_of_house/hosting/mod.rs* (đường dẫn cũ hơn, vẫn được hỗ trợ)
>
> Nếu bạn sử dụng cả hai cho cùng một module, bạn sẽ gặp lỗi trình biên dịch. Sử dụng
> cho phép các kiểu khác nhau cho các module khác nhau trong cùng một dự án, nhưng
> có thể gây nhầm lẫn cho những người đang điều hướng dự án của bạn.
>
> Nhược điểm chính của kiểu sử dụng file có tên *mod.rs* là
> dự án có thể kết thúc với nhiều file có tên *mod.rs*, điều này có thể gây nhầm lẫn
> khi bạn mở chúng trong editor của mình cùng một lúc.

Việc chuyển code của từng module sang một file riêng biệt hiện đã hoàn tất và cây module vẫn giữ nguyên. 
Các lệnh gọi hàm trong `eat_at_restaurant` sẽ hoạt động mà không có bất kỳ sửa đổi nào, mặc dù các định nghĩa nằm trong các file khác nhau. 
Kỹ thuật này cho phép bạn di chuyển các module sang các file mới khi chúng phát triển về kích thước.

Lưu ý rằng câu lệnh `pub use crate :: front_of_house :: hosting` trong *src/lib.rs* cũng không thay đổi, cũng như` use` không có bất kỳ 
tác động nào đến những file nào được biên dịch như một phần của crate. Từ khóa `mod` khai báo các module và Rust tìm kiếm trong một file 
có cùng tên với module cho code đi vào module đó.

## Tổng kết

Rust cho phép bạn chia một package thành nhiều crate và một crate thành các module để bạn có thể tham chiếu 
đến các item được xác định trong một module từ một module khác. Bạn có thể làm điều này bằng cách chỉ định 
các đường dẫn tuyệt đối hoặc tương đối. Các đường dẫn này có thể được đưa vào scope bằng câu lệnh `use` để bạn có thể 
sử dụng đường dẫn ngắn hơn cho nhiều mục đích sử dụng trong scope đó. Code module là private theo mặc định, nhưng bạn có thể 
đặt định nghĩa ở chế độ public bằng cách thêm từ khóa `pub`.

Trong chương tiếp theo, chúng ta sẽ xem xét một số cấu trúc dữ liệu thu thập trong thư viện chuẩn mà bạn có thể sử dụng trong code được sắp xếp gọn gàng của mình.

[paths]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
