## Hello, Cargo!

Cargo là hệ thống build và quản lý thư viện của Rust. Hầu hết Rust coder - Rustaceans đều sử
dụng Cargo để quản lý project của họ bởi vì Cargo xử lý nhiều tác vụ cho bạn ví dụ như: build code,
tải thư viện, và build những thư viện đó. (Chúng ta gọi những thư viện mà code của bạn cần là những dependency.)

Những chương trình Rust đơn giản nhất, như ví dụ bạn vừa viết chẳng hạn, không có dependency nào. Nên nếu chúng
ta build chương trình “Hello, world!” với Cargo, Cargo sẽ chỉ giúp chúng ta build code.
Khi bạn viết những chương trình phức tạp hơn, bạn sẽ cần thêm các dependency. Và nếu bạn bắt đầu
chương trình với Cargo, việc thêm các dependency sẽ cực kì dễ dàng.

Bởi vì phần lớn các dự án Rust sử dụng Cargo, phần còn lại của cuốn sách này giả định rằng bạn cũng đang sử dụng Cargo.
Cargo được cài đặt cùng với Rust nếu bạn sử dụng bộ cài chính thức đã được nêu ở phần [“Cài đặt”][installation]. 
Nếu bạn đã cài Rust theo những cách khác, hãy kiểm tra xem Cargo đã được cài đặt hay chưa bằng cách nhập câu lệnh sau trong terminal:

```console
$ cargo --version
```

Nếu bạn nhìn thấy số phiên bản, tức là bạn đã cài Cargo! Nếu bạn nhìn thấy lỗi như `command not found` thì
phiên bản Rust bạn đã cài đặt không bao gồm Cargo. 

### Tạo một project với Cargo

Chúng ta hãy tạo một project mới bằng Cargo và xem nó khác như thế nào so với project “Hello, world!” trước.
Vào thư mục projects của bạn (bất cứ thư mục nào mà bạn muốn lưu trữ code). Sau đó chạy những lệnh sau:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

Câu lệnh đầu tiên tạo một thư mục mới là *hello_cargo*. Chúng ta đã đặt tên project của chúng ta là *hello_cargo*,
và Cargo sẽ tạo những file mặc định trong thư mục trên.

Vào thư mục *hello_cargo* và xem các file được tạo. Bạn sẽ thấy Cargo tạo ra hai file và một thư mục: file *Cargo.toml*
và thư mục *src* có file *main.rs* bên trong.

Cargo cũng tạo một Git repository cùng với file *.gitignore*. Nếu bạn chạy `cargo new` trong một thư mục đã có Git repository
nó sẽ không tạo Git repository mới. Nếu bạn vẫn muốn tạo Git repository mới hãy dùng `cargo new --vcs=git`.

> Note: Git là hệ thống quản lý phiên bản phổ biến. Bạn có thể dùng hệ thống quản lý phiên bản khác
> với cờ `--vcs`. Sử dụng `cargo new --help` để xem những options có thể chọn.

Mở file *Cargo.toml*.  Nó sẽ giống với code ở Listing 1-2.

<span class="filename">Filename: Cargo.toml</span>

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

<span class="caption">Listing 1-2: Nội dung file *Cargo.toml* được tạo bởi `cargo
new`</span>

File này được viết theo chuẩn [*TOML*](https://toml.io)<!-- ignore --> (*Tom’s Obvious,
Minimal Language*), theo định dạng cấu hình của Cargo

Dòng đầu tiên, `[package]`, là tiêu đề của một đoạn thể hiện rằng những khai báo sau đó đang cấu hình một package.
Chúng ta sẽ thêm nhiều thông tin khác cũng như các đoạn khác vào file này.

Ba dòng tiếp theo đặt thông tin cấu hình Cargo cần để biên dịch chương trình: tên, phiên bản, và phiên bản(`edition`) Rust cần dùng
Chúng ta sẽ nói về `edition` trong phần [Appendix E][appendix-e]<!-- ignore -->.

Dòng cuối, `[dependencies]`, là bắt đầu của một đoạn liệt kê các dependency của project.
Trong Rust, các package được coi như các *crate*. Chúng ta không cần bất kỳ crate nào cho project này, 
nhưng chúng ta sẽ cần trong project đầu tiên ở Chương 2, nên chũng ta sẽ sử dùng đoạn khai báo dependencies này sau.

Giờ mở file *src/main.rs* và xem thử:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo đã tạo ra một chương trình “Hello, world!” cho bạn, giống như cái chúng ta đã viện trong Listing 1-1! 
Cho đến giờ, điểm khác biệt giữa project trước và project Cargo tạo ra là Cargo đặt code bên trong thư mục *src*,
và chúng ta có một file cấu hình *Cargo.toml* ở thư mục ngoài cùng.

Cargo yêu cầu những file mã nguồn của bạn nằm trong thư mục *src*. Thư mục ngoài cùng của project chỉ để file
README, thông tin bản quyền, các file cấu hình, và những thứ không liên quan tới code. 
Việc sử dụng Cargo giúp bạn tổ chức project khoa học hơn.

Nếu bạn đã tạp một project mà không sử dụng Cargo, như project “Hello, world!”,
bạn có thể chuyển nó thành một project sử dụng Cargo. Chuyển code vào trong thư mục *src* và tạo một
file *Cargo.toml* phù hợp.

### Build và chạy một Cargo Project

Giờ hãy nhìn vào sự khác biệt khi bạn build và chạy chương trình “Hello, world!” với Cargo! Từ thư mục *hello_cargo* của bạn,
build project bằng lệnh sau:

```console
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

Câu lệnh trên tạo một file thực thi trong *target/debug/hello_cargo* (hoặc *target\debug\hello_cargo.exe* trên Windows) 
thay vì thư mục hiện tại của bạn. Bạn có thể chạy file thực thi với lệnh sau:

```console
$ ./target/debug/hello_cargo # hoặc .\target\debug\hello_cargo.exe trên Windows
Hello, world!
```

Nếu mọi việc thực hiện đúng, `Hello, world!` sẽ được in ra terminal.
Việc chạy `cargo build` lần đầu tiên cũng làm Cargo tạo ra một file mới ở thư mục ngoài: *Cargo.lock*.
File này chứa thông tin phiên bản của các dependency trong project của bạn.
Project này không có dependency nên file này hầu như không có thông tin gì cả. 
Bạn không cần phải sửa đổi gì ở file này; Cargo sẽ quản lý nội dung của nó cho bạn.

Chúng ta vừa mới build một project với `cargo build` và chạy nó với `./target/debug/hello_cargo`,
nhưng chúng ta cũng có thể sử dụng `cargo run` để biên dịch và chạy trong một lệnh.

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Lưu ý rằng lần này chúng ta không nhìn thấy output chỉ ra rằng Cargo đang biên dịc `hello_cargo`.
Cargo thấy rằng các file không thay đổi nên nó chỉ thực thi file nhị phân. 
Nếu bạn đã sửa source code, Cargo sẽ build lại project trước khi chạy nó và bạn sẽ nhìn thấy output này:

```console
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo cung cấp một câu lệnh gọi là `cargo check`. Câu lệnh này nhanh chóng kiểm tra code của bạn để chắc chắn rằng nó biên dịch được nhưng không tạo ra file thực thi:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Sao bạn lại không muốn file thực thi? Thông thường, `cargo check` nhanh hơn rất nhiều so với `cargo build`, 
bởi vì nó bỏ qua bước tạo file thực thi. Nếu bạn liên tục kiểm tra code của bạn trong khi viết, sử dụng `cargo check` sẽ tăng tốc quá trình làm việc!
Như vậy, nhiều Rustacean chạy `cargo check` thường xuyên khi họ viết chương trình để đảm bảo nó biên dịch được.
Sau đó họ chạy cargo build khi họ sẵn sàng sử dụng file thực thi.

Hãy cùng điểm những gì chúng ta đã học được về Cargo cho đến lúc này:

* Chúng ta có thể tạo project với `cargo new`.
* Chúng ta có thể build một project với `cargo build`.
* Chúng ta có thể build và chạy một project trong một bước bằng `cargo run`.
* Chúng ta có thể build project mà không sinh ra file thực thi để kiểm tra lỗi compile bằng `cargo check`.
* Thay vì lưu kết quả build trong cùng thư mục với code, Cargo lưu nó trong thư mục target/debug.

Một ưu điểm nữa của Cargo là các câu lệnh của nó giống nhau trên các hệ điều hành.
Nên từ bây giờ chúng ta sẽ không đưa ra những chỉ dẫn riêng cho Linux và macOS với Windows nữa.

### Build bản phát hành

Khi project của bạn sẵn sàng để phát hành, bạn có thể sử dụng `cargo build --release` để biên dịch nó một các tối ưu.
Câu lệnh này sẽ tạo một file thực thi trong *target/release* thay vì *target/debug*.
Cách này làm code của bạn chạy nhanh hơn, nhưng sẽ biên dịch lâu hơn. 
Đây là lý do vì sao có hai cấu hình khác nhau: một cho việc phát triển, khi mà bạn muốn build lại nhanh chóng và thường xuyên,
và một cái khác cho việc build ra chương trình cuối cùng, thứ mà bạn sẽ đưa cho người dùng,
nó sẽ không cần build lại nhiều lần và sẽ chạy nhanh nhất có thể. Nếu bạn đang benchmark thời gian chạy của code,
hãy chắc chắn rằng chạy `cargo build --release` và benchmark với file thực thi trong *target/release*.

### Cargo as Convention

Với những project đơn giản, Cargo không có giá trị nhiều so với sử dụng `rustc`, nhưng nó sẽ chứng minh giá
trị của nó khi chương trình của bạn phức tạp hơn. Với những project phức tạp bao gồm nhiều crate, dùng cargo sẽ
thuận tiện hơn rất nhiều.

Mặc dù project `hello_cargo` rất đơn giản, nhưng nó đã sử dụng những công cụ mà bạn sẽ sử dụng sau 
này trong công việc của bạn. Thực tế, để tham gia vào project Rust, bạn có thể sử dụng những lệnh sau để clone 
code từ git và build nó

```console
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Xem thêm [tài liệu Cargo] để biết nhiều thông tin hơn.

[tài liệu Cargo]: https://doc.rust-lang.org/cargo/

## Tổng kết

Bạn đã có một khởi đầu rất tuyệt trong hành trình đến với Rust! Trong chương này, bạn đã học được cách:

* Cài đặt phiên bản ổn định mới nhất của Rust bằng cách sử dụng `rustup`
* Viết và chạy một chương trình “Hello, world!” với `rustc`
* Tạo và chạy một project mới bằng Cargo

Đây là thời điểm rất tốt để build một chương trình lớn hơn để làm quen với việc đọc và viết code Rust. 
Vì thế, trong chương 2, chúng ta sẽ build một chương trình trò chơi đoán số.
Nếu bạn muốn bắt đầu học về các khái niệm lập trình phổ biến trong Rust, bạn có thể đọc Chương 3 và quay lại Chương 2 sau.

[installation]: ch01-01-installation.html#installation
[appendix-e]: appendix-05-editions.html
