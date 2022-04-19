## Hello, World!

Sau khi cài đặt Rust, hãy viết chương trình Rust đầu tiên hiển thị `Hello, world!`

### Tạo một Project Directory

Bạn sẽ bắt đầu bằng tạo một thư mục để lưu trữ Rust code. Bạn có thể lưu trữ nó ở bất kì đâu,
tuy nhiên cho các bài tập trong sách này, bạn nên để đường dẫn ở thư mục home.

Mở terminal và nhập những lệnh sau để tạo thư mục *projects* và thư mục “Hello, world!” bên
trong thư mục *projects*

Với Linux, macOS, và PowerShell trên Windows:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Với Windows CMD:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Viết và chạy một chương trình Rust

Tiếp theo, tạo một file tên *main.rs*. Rust file luôn kết thúc với đuôi *.rs*. Nếu có hay từ
trong tên file, hãy thêm dấu gạch dưới để tách nó. Ví dụ, dùng *hello_world.rs* thay vì *helloworld.rs*.

Bây giờ mở file *main.rs* và nhập đoạn code ở Mục 1-1.

<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("Hello, world!");
}
```

<span class="caption">Mục 1-1: Chương trình in ra `Hello, world!`</span>

Lưu file và mở terminal. Trên Linux hoặc macOS, nhập lệnh sau để chạy file:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Trên Windows, nhập lệnh `.\main.exe` thay vì `./main`:

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

Dòng `Hello, world!` sẽ được in ra ở terminal.
Nếu bạn không thấy hãy tham gia nhóm facebook [Cộng đồng Rust Việt Nam][facebook] để được mọi người giúp đỡ.

[facebook]: https://www.facebook.com/groups/546307380433651

### Phân tích một chương trình Rust

Hãy xem lại chi tiết chương trình “Hello, world!”. Đây là phần đầu tiên:

```rust
fn main() {

}
```

Dòng code khai báo một hàm trong Rust. Hàm `main` là hàm đặc biệt: nó luôn là đoạn code chạy đầu tiên trong tất cả
chương trình Rust. Dòng đầu khai báo một hàm tên `main`, nó không có tham số truyền vào và không trả về giá trị.
Nếu có tham số truyền vào, hãy đặt nó trong `()`.

Thân hàm được viết trong cặp ngoặc `{}`. Bạn nên đặt dấu mở ngoặc trên cùng dòng với tên hàm
và có dấu cách ở giữa

Nếu bạn muốn format code theo phong cách chuẩn, bạn có thể sử dụng tool format code `rustfmt`.
Tool này đã được cài đặt cùng với `rustc`.

Trong hàm `main` có dòng code:

```rust
    println!("Hello, world!");
```

Dòng này in ra một đoạn text trên màn hình. Có 4 điều cần lưu ý:

Thứ nhất, Rust style là thụt lề với bốn dấu cách, không phải là một tab

Thứ hai, `println!` gọi một Rust macro. Nếu gọi một hàm sẽ là `println` (không có `!`).
Chúng ta sẽ bàn luận chi tiết về Rust macros trong Chương 19.
Bây giờ, bạn chỉ cần biết là sử dụng `!` nghĩa là bạn đang gọi một macro thay vì một hàm thông thường.

Thứ ba, bạn thấy chuỗi kí tự `"Hello, world!"`. Chúng ta sẽ truyền chuỗi này như một tham số vào macro `println!`,
và đoạn kí tự sẽ được in ra màn hình.

Thứ tư, chúng ta kết thúc dòng với dấu chấm phẩy (`;`), nó chỉ ra một expression kết thúc. Hầu hết các dòng code Rust
đều kết thúc với dấu chấm phẩy

### Biên dịch và Chạy chương trình là hai bước riêng

Bạn vừa mới chạy một chương trình vừa tạo, hãy xem từng bước

Trước khi chạy một chương trình Rust, bạn phải biên dịch bằng Rust compiler với lệnh `rustc`. Ví dụ:

```console
$ rustc main.rs
```

Sau khi biên dịch thành công, Rust sinh ra một file binary có thể thực thi

Trên Linux, macOS, và PowerShell trên Windows, bạn có thể thấy file bằng lệnh `ls`.

Trên Linux và macOS, bạn sẽ thấy 2 file:

```console
$ ls
main  main.rs
```

Trên Windows bạn sẽ thấy 3 files:

```cmd
> dir
main.exe
main.pdb
main.rs
```

Nó bao gồm file source với đuôi *.rs*, file thực thi(*main.exe* trên Windows, *main* trên Linux và MacOS).
Và khi sử dụng Windows, có thêm một file bao gồm thông tin debug với đuôi *.pdb*.
Bạn chạy file *main* hoặc *main.exe*:

```console
$ ./main # hoặc .\main.exe trên Windows
```

Nếu *main.rs* là chương trình “Hello, world!”, terminal sẽ in ra `Hello, world!`

Rust là ngôn ngữ đi trước thời đại, bạn
có thể biên dịch 1 chương trình và đưa file thực thi cho người khác, và họ có thể chạy nó mà không
cần cài đặt Rust. Nếu bạn đưa người khác một file *.rb*, *.py*, hoặc
*.js* họ cần cài Ruby, Python, JavaScript để chạy nó. Đánh đổi lại, các ngôn ngữ đó
biên dịch và chạy chỉ với 1 câu lệnh. 

Biên dịch với `rustc` chỉ dành cho chương trình đơn giản. Khi project phát triển, sẽ rất phức tạp khi quản lý các options.
Cargo tool sẽ giúp ta việc đó.

