## Packages and Crates

Các phần đầu tiên của module system chúng ta sẽ bao gồm packages và crates.

Một *package* là một hoặc nhiều crates cung cấp một loạt các chức năng. Một
package chứa một file *Cargo.toml* mô tả cách build các crate của package đó.

Một *crate* có thể là một binary crate hoặc một library crate. *Binary crates* là các chương trình bạn có thể biên dịch và thực thi để chạy,
chẳng hạn như một command-line program hoặc một server. Chúng ta cần có một function được gọi là `main`, function này
xác định điều gì xảy ra khi chạy thực thi. Tất cả các crates được tạo ra thường là binary crates.

*Library crates* không có `main` function, và chúng không được biên dịch để thực thi. 
Chúng định nghĩa các chức năng dự định để chia sẻ với nhiều projects.
Ví dụ, crate `rand` chúng tôi sử dụng trong [Chapter 2][rand]<!-- ignore--> cung cấp chức năng tạo các số ngẫu nhiên.

*crate root* là một file mà nguồn mà trình biên dịch Rust bắt đầu và tạo thành các root module
(chúng tôi sẽ giải thích về module sâu hơn trong phần [“Defining Modules to Control Scope and Privacy”][modules]<!-- ignore -->).

Một số quy tắc xác định những gì mà một package có thể chứa. Một package có thể chứa nhiều nhất một library crate. 
Nó có thể chứa bao nhiêu binary crates tùy thích, nhưng nó
phải chứa ít nhất một crate (hoặc là library hoặc binary).

Hãy xem điều gì xảy ra khi chúng ta tạo ra một package. Đầu tiên chúng ta gõ lệnh `cargo new`:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Khi chúng ta nhập lệnh, Cargo tạo một file *Cargo.toml* , cho chúng ta một
package. Nhìn vào nội dung của *Cargo.toml*, Không đề cập đến
*src/main.rs* bởi vì Cargo tuân theo một quy ước rằng *src/main.rs* là
crate root của một binary crate cùng tên với package. Hơn nữa, Cargo
biết rằng nếu thư mục package chứa *src/lib.rs*, package chứa
một library crate có cùng tên với package, và *src/lib.rs* là
crate root của nó. Cargo đưa các file crate root tới `rustc` để build the library
or binary.

Ở đây chúng ta có một package chỉ chứa *src/main.rs*, có nghĩa là nó chỉ
chứa một binary crate tên là `my-project`. Nếu một package chứa *src/main.rs*
và *src/lib.rs*, nó có 2 crates: một binary và một library, cả hai đều cùng tên với package. 
Một package có thể chứa nhiều binary crates bằng cách đặt các files trong thư mục *src/bin* : mỗi file sẽ là một binary crate riêng biệt.

[modules]: ch07-02-defining-modules-to-control-scope-and-privacy.html
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
