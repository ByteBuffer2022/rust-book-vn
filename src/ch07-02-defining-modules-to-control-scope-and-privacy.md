## Defining Modules to Control Scope and Privacy

Trong phần này , chúng ta sẽ nói về modules và các phần khác của module system,
cụ thể là *paths* cho phép bạn đặt tên cho các mục; từ khóa `use` đưa ra một
đường dẫn tới scope; và từ khóa `pub` làm cho các mục public. Chúng ta sẽ thảo luận
về từ khóa `as` , external packages, và toán tử toàn cục.

Đầu tiên, chúng ta sẽ bắt đầu với một danh sách các quy tắc để dễ dàng tham khảo.Khi bạn tổ chức code của mình trong tương lai.
Sau đó, chúng tôi sẽ giải thích chi tiết từng quy tắc.

### Modules Quick Reference

Đây là cách mà modules, paths, từ khóa `use`, và từ khóa `pub` hoạt động trong
trình biên dịch, và làm thế nào hầu hết các developer tổ chức code của họ. 
Chúng ta sẽ trải qua các ví dụ về từng quy tắc này, nhưng đây là một nơi tuyệt vời
để tìm kiếm trong tương lai như một lời nhắc nhở về cách các module hoạt động.

- **Bắt đầu từ crate root**: Khi biên dịch một crate, Trình biên dịch sẽ nhìn vào file crate root đầu tiên
(thường là *src/lib.rs* cho một library crate hoặc *src/main.rs* cho một binary crate).
- **Khai báo modules**: Trong file crate root,bạn có thể khai báo một module mới 
  có tên gọi là “garden”, với cú pháp `mod garden;`. Trình biên dịch sẽ tìm kiếm code
  bên trong module tại đây:
  - Trong cùng 1 dòng, trực tiếp viết `mod garden`, trong dấu ngoặc nhọn thay vì dấu chấm phẩy. 
  - Trong file *src/garden.rs*
  - Trong file *src/garden/mod.rs*
- **Khái báo submodules**: Trong bất kì file nào khác crate root, được
  biên dịch như là một phần của crate (ví dụ, *src/garden.rs*), bạn cần khai báo
  submodules (ví dụ, `mod vegetables;`). Trình biên dịch sẽ tìm kiếm
  trong dòng code submodules ở những nơi trong thư mục được đặt tên theo
  module cha:
  - Trong cùng 1 dòng,trực tiếp viết `mod vegetables`, trong dấu ngoặc nhọn thay vì dấu chấm phẩy.
  - Trong file *src/garden/vegetables.rs*
  - Trong file *src/garden/vegetables/mod.rs*
- **Đừng dẫn đến code trong modules**: Khi một module đang được biên dịch là một phần 
  crate của bạn, bạn có thể tham khảo code trong module đó (ví dụ, một loại `Asparagus`(măng tây)
  trong module garden) từ bất cứ nơi nào khác trong crate này bằng cách sử dụng đường dẫn
  `crate::garden::vegetables::Asparagus` miễn là các quy tắc bảo mật cho phép.
- **Private vs public**: Code trong một module là private từ các modules cha theo mặc định.
  Để làm cho một module public, khai báo nới với từ khóa `pub mod`
  thay vì `mod` là tốt nhất, sử dụng `pub` trước khi khai báo.
- **Từ khóa`use` **: Trong phạm vi, từ khóa `use` Tạo các lối tắt 
  cho các mục để giảm sự lặp lại của các đường dẫn dài. Trong bất kỳ phạm vi nào có thể tham khảo
  `crate::garden::vegetables::Asparagus`, bạn có thể tạo một lối tắt với  `use
  crate::garden::vegetables::Asparagus;` và sau đó chỉ cần viết `Asparagus`
  để sử dụng loại đó trong phạm vi.

Đây là một binary crate được đặt tên là  `backyard` minh họa cho những quy tắc này.Thư mục của crates,
cũng được đặt tên là `backyard`, chứa các file và thư mục này:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

File crate root, trong trường hợp này *src/main.rs*, chứa:

<span class="filename">Filename: src/main.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

The `pub mod garden;` means the compiler includes the code it finds in
*src/garden.rs*, which is:

<span class="filename">Filename: src/garden.rs</span>

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

Và `pub mod vegetables;` có nghĩa là code trong *src/garden/vegetables.rs* cũng được bao gồm:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```

Bây giờ, hãy cùng tìm hiểu chi tiết về các quy tắc này và chứng minh chúng trong thực tế!

### Grouping Related Code in Modules

*Modules* hãy tổ chức code của bạn trong một crate thành các group để dễ dàng đọc và sử dụng lại.
Các module cũng kiểm soát *privacy* của các mục, đó là một mục có thể sử dụng
bới code bên ngoài (*public*) hoặc  là một triển khai nội bộ
chi tiết và không có sẵn để sử dụng bên ngoài(*private*).

Ví dụ: hãy viết một library crate cung cấp chức năng của một nhà hàng.
Chúng tôi xác định chữ kí hàm nhưng phần thân để trống
để tập trung vào việc tổ chức code, thay vì thực sự thực hiện một nhà hàng trong code.
Chữ ký của một hàm mô tả:
  - tên của nó
  - đối số của nó
  - kết quả của nó
  - trong trường hợp của các chức năng chung, các tham số chung của nó, với các giới hạn cụ thể có khả năng
  Ví dụ: nếu bạn xác định:
  ```text
  fn hello(s: &str) {
   println!("Hello {}", s);
  }
  ```
  - Chữ ký hàm là fn hello(&str).

Trong ngành nhà hàng, Một số phần của một nhà hàng được gọi là
*Trước nhà* và những cái khác như là  *Sau nhà*. Phía trước nhà là nơi khách hàng
;Đây là nơi khách hàng ngồi, servers nhận đơn đặt hàng và thanh toán,
và người pha chế làm đồ uống.Phía sau nhà là nơi  các đầu bếp 
làm việc trong bếp,máy rửa chén rửa chén,và các nhà quản lý làm công việc hành chính
Để cấu trúc crate cũng giống như cách mà nhà hàng hoạt động, Chúng ta có thể tổ chức các functions 
vào các modules lồng nhau.Tạo mới một library đặt tên là`restaurant` bằng cách chạy `cargo new --lib restaurant`; 
Sau đó đặt mã vào Listing 7-1 vào *src/lib.rs* để xác định một số modules and chữ kí hàm.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-01/src/lib.rs}}
```

<span class="caption">Listing 7-1: Một `front_of_house` module chứa các module khác sau đó chứa các function</span>

Chúng ta định nghĩa một module với từ khóa `mod` và sau đó chỉ định tên module 
(trong trường hợp này là `front_of_house`)và đặt dấu ngoặc nhọn bao quanh phần thân của module. 
Bên trong module, chúng ta cần có các module khác, trong trường hợp này
là với các module `hosting` và `serving`. Các module cũng có thể chứa
các định nghĩa cho những mục khác, như là structs, enums, constants, traits, hoặc như
trong Listing 7-1—functions.

Bằng cách sử dụng các module, chúng tôi có thể nhóm các định nghĩa có liên quan lại với nhau 
và đặt tên cho lý do tại sao chúng có liên quan.Các lập trình viên sử dụng code này sẽ có thời gian
dễ dàng hơn trong việc tìm các định nghĩa mà họ muốn sử dụng vì họ có thể điều hướng code dựa trên các nhóm 
thay vì phải đọc qua tất cả các định nghĩa.
Các lập trình viên thêm chức năng mới vào mã này sẽ biết nơi đặt code để giữ cho chương trình có tổ chức.

Trước đó, chúng tôi đã đề cập rằng *src/main.rs* và *src/lib.rs* được gọi là crate
roots. Lý do cho tên của chúng là vì nội dung của một trong hai tệp này tạo thành một module có tên `crate` 
ở cấu trúc root crate module, được gọi là *module tree*.

Listing 7-2 hiển thị cây module cho cấu trúc trong Listing 7-1.

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

<span class="caption">Listing 7-2: Cây module cho code trong Listing 7-1</span>

Cây này cho thấy cách một số module lồng vào nhau(Ví dụ,`hosting` lồng bên trong `front_of_house`). 
Cây cũng cho thấy rằng một số module là *anh chị em* với nhau,nghĩa là chúng được xác định trong cùng một module
(`hosting` và `serving` được xác định trong `front_of_house`).Tiếp tục phép ẩn dụ về gia đình, 
Nếu module A được chứa bên trong module B, chúng tôi nói rằng module A là *con* của module B và module B là *cha* của module
Lưu ý rằng toàn bộ cây module được bắt nguồn từ module ngầm định có tên `crate`.

Cây module có thể nhắc bạn về cây thư mục của hệ thống tệp trên máy tính của bạn; 
đây là một so sánh rất phù hợp! Cũng giống như các thư mục trong hệ thống tệp, bạn sử dụng các module để tổ chức mã của mình.
Và cũng giống như các tệp trong thư mục, chúng ta cần một cách để tìm các module của mình.
