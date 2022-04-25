# Lập trình Trò chơi đoán số

Bây giờ, hãy nhảy vào Rust bằng cách bắt tay vào làm một dự án cùng nhau! Chương này sẽ giới thiệu một vài ý tưởng Rust thông thường, Bạn sẽ học về `let`, `match`, các phương thức, các hàm liên quan, sử dụng các external crates, và còn nhiều hơn nữa! Trong chương tiếp theo, chúng ta sẽ khám phá những ý tưởng này chi tiết hơn. Trong chương này, bạn sẽ luyện tập được những nguyên tắc cơ bản.

Chúng ta sẽ triển khai một vài vấn đề lập trình cơ bản dành cho người mới: một trò chơi đoán số. Đây là cách nó thực hiện: Chương trình này sẽ khởi tạo một số nguyên ngẫu nhiên trong khoảng 1 đến 100. Nó sẽ yêu cầu người chơi phải đoán. Sau khi người chơi nhập số, chương trình sẽ hiển thị rằng: dự đoán của người chơi quá thấp hoặc quá cao. Nếu dự đoán chính xác, chương trình sẽ in ra thông báo chúc mừng và thoát ra.

## Cấu hình một dự án mới

Để cấu hình một dự án mới, đến thư mục *project*, nơi mà được bạn tạo ra trong chương 1 và tạo mới một dự án mới sử dụng Cargo, ví dụ như sau:

```console
$ cargo new guessing_game
$ cd guessing_game
```

Câu lệnh đầu tiên, `cargo new`, đặt tên cho dự án này, (`guessing_game`) là thông số đầu tiên. Câu lệnh tiếp theo để thay đổi thư mục, đưa bạn tới thư mục của dự án.

Nhìn vào file *Cargo.toml* đã được khởi tạo trước đó.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new --name guessing_game no-listing-01-cargo-new
cd no-listing-01-cargo-new
cargo run
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Như bạn đã thấy trong Chương 1, `cargo new` khởi tạo chương trình "Hello, world!" cho bạn. Kiểm tra file *src/main.rs*:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Bây giờ, hãy cùng biên dịch chương trình "Hello, world!" và chạy nó, sử dụng câu lệnh `cargo run`:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Câu lệnh `run` sẽ có ích khi bạn cần lặp lại nhanh chóng trong một dự án, cũng như cách mà chúng ra thực hiện trong trò chơi này, nhanh chóng kiểm tra từng lần lặp lại trước khi chuyển tới bước tiếp theo.

Mở lại file *src/main.rs*. Bạn sẽ phải viết lại tất cả code trong file này.

## Tiến hành dự đoán

Phần đầu tiên của trò chơi dự đoán này là sẽ hỏi người chơi nhập đầu vào, xử lý đầu vào đó, và kiểm tra đầu vào đó có trong định dạng được kì vọng hay không. Để bắt đầu, chúng ta sẽ cho người chơi nhập vào một đầu vào dự đoán. Nhập đoạn code trong Listing 2-1 vào trong file *src/main.rs*.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption">Listing 2-1: Đoạn mã nhận dự đoán của người dùng và in nó ra</span>

Đoạn code này hàm chứa rất nhiều thông tin, vì vậy hãy cùng nhau giải thích nó từng dòng một. Để lựa chọn đầu vào của người dùng và in kết quả đầu ra, ta cần mang thư viện `io` input/output vào trong phạm vi này. Thư viện `io` từ thư viện tiêu chuẩn, được viết tới là `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Trong mặc định, Rust có một tập các items định nghĩa các thư viện tiêu chuẩn được mang vào trong phạm vi của mỗi dự án. Tập này được gọi là *prelude*, và bạn có thể nhìn thấy tất cả chúng trong [tài liệu về thư viện tiêu chuẩn][prelude].

Nếu kiểu thư viện bạn muốn không ở trong phần dạo đầu, bạn cần mang nó vào trong một phạm vi rõ ràng với statement `use`. Sử dụng thư viện `std::id` cung cấp cho bạn một vài tính năng hữu dụng, bao gồm khả năng chấp nhận đầu vào của người dùng.

Như bạn từng thấy trong Chương 1, hàm `main` sẽ là điểm vào trong chương trình.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

Cú pháp `fn` khai báo hàm mới, dấu ngoặc đơn, `()`, biểu thị rằng nó không có tham số, và dấu ngoặc nhọn, `{` bắt đầu phần thân của một hàm.

Như bạn đã từng học trong Chương 1, `println!` cũng là một macro in ra chuỗi ngoài màn hình.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Đoạn code này in ra một lời nhắc đây là trò chơi gì và yêu cầu nhập đầu vào từ người dùng.

### Lưu giữ giá trị với biến.

Tiếp theo, chúng ta sẽ tạo một *biến* để lưu trữ giá trị từ đầu vào của người dùng, như sau:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Bây giờ chương trình đang dần thú vị hơn! Có rất nhiều điều xảy ra trong dòng này. Chúng ta sẽ sử dụng câu lệnh `let` để tạo giá trị. Sau đây là một ví dụ:

```rust,ignore
let apples = 5;
```

Dòng code này tạo mới một biến được đặt tên là `apples` và gắn nó với giá trị là 5. Trong Rust, các biến đều mắc định là *bất biến*, nghĩa là một khi chúng được đưa một giá trị nhất định, giá trị này sẽ không thay đổi. Chúng ta sẽ bàn đến ý tưởng này chi tiết hơn trong phần ["Các biến và tính bất biến"][variables-and-mutability]<!-- ignore --> ở chương 3. Để làm cho một biến có thể biến đổi được, ta thêm `mut` trước tên biến: 

```rust,ignore
let apples = 5; // immutable
let mut bananas = 5; // mutable
```

> Ghi chú: Cú pháp `\\` bắt đầu một bình luận kéo dài đến cuối dòng. Rust sẽ bỏ qua tất cả ở trong phần bình luận này. Chúng ta sẽ bàn đến phần bình luận này chi tiết hơn ở trong [Chương 3][comments]<!-- ignore -->.

Trở về chương trình trò chơi đoán số, bạn biết rằng `let mut guess` sẽ giới thiệu biến khả biến được đặt tên là `guess`. Dấu bằng (`=`) nói với Rust rằng chúng ta muốn gắn gì đó vào biến. Bên phải của dấu bằng là giá trị mà `guess` được gắn vào, thứ là kết quả của việc gọi `String::new`, một hàm trả về một instance của `String`. [`String`][string]<!-- ignore --> là kiểu dữ liệu được cung cấp bởi thư viện tiêu chuẩn, nó có thể phát triển được, UTF-8 encoded bit hoặc text.

Cú pháp `::` trong dòng `::new` chỉ ra rằng `new` có liên quá đến hàm của kiểu dữ liệu `String`. Một *hàm liên quan* là một hàm được triển khai thực hiện trên một kiểu, trong trường hợp này là `String`. Hàm `new` này tạo ra một chuỗi mới và trống. Bạn sẽ tìm được hàm `new` trong nhiều kiểu dữ liệu, bởi vì nó là cái tên phổ biến cho hàm tạo ra giá trị mới, hoặc đại loại vậy.

Về cụ thể, dòng `let mut guess = String::new()` đã tạo ra một biến khả biến, nó được gắn liền với một chuỗi mới, trống và là instance của `String`. Phù!

### Nhận dữ liệu đầu vào từ người dùng.

Quay lại với việc chúng ta đã bao gồm chức năng đầu vào/đầu ra từ một thư viện tiêu chuẩn với `use std::io;` trong dòng đầu tiên của chương trình. Bây giờ chúng ta sẽ gọi hàm `stdin` từ module `io`, thứ mà cho phép chúng ta xử lý đầu vào được nhập từ người dùng: 

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Nếu chúng ta chưa bổ sung thư viện `io` với `use std::io` ở phần đầu của chương trình, chúng ta vẫn có thể sử dụng hàm bằng cách viết hàm gọi, ví dụ `std::io::stdin`. Hàm `stdin` trả về instance của [`std::io::Stdin`][iostdin]<!-- ignore -->, thứ mà đại diện xử lý đầu vào tiêu chuẩn cho terminal của bạn.

Tiếp theo, dòng `.read_line(&mut guess)` gọi đến phương thức [`read_line`][read_line]<!--ignore --> trên chuẩn xử lý đầu vào để lấy đầu vào được nhập từ người dùng. Chúng ta cũng đi qua `&mut gues` như là một tham số để `read_line` để chỉ ra rằng chuỗi nào lưu trữ đầu vào của người dùng. Công việc đầy đủ của `read_line` là nhận lấy bất kỳ kiểu dữ liệu tiêu chuẩn nào của người dùng và nối chúng thành một chuỗi (mà không hề ghi đè nội dung), vì vậy trước khi bỏ qua chuỗi đó như là một tham số. Chuỗi tham số này cần để trở nên khả biến vì vậy phương thức có thể thay đổi nội dung chuỗi.

Ký hiệu `&` chỉ ra rằng tham số này là một *tham chiếu*, thứ mà chỉ ra cho bạn cách để nhiều đoạn code của bạn cùng truy cập vào một dùng dữ liệu mà không cần sao chiếu dữ liệu đó vào bộ nhớ nhiều lần. Tham chiếu là một tính năng phức tạo, nhưng và nó là một trong những điểm điểm mạnh của Rust, vô cùng an toàn và dễ dàng để sử dụng tham chiếu trong Rust. Bạn không cần phải biết quá nhiều chi tiết để hoàn thiện chương trình. Bây giờ, tất cả những gì bạn cần phải biết là nó giống với biến, tham chiếu là bất biến trong mặc định. Vì vậy, bạn cần phải ghi `&mut guess` thay vì `&guess` để làm nó khả biến. (Chương 4 sẽ giải thích tham chiếu rõ ràng hơn)

Chúng ta vẫn sẽ làm việc trong dòng code này. Giờ đây, ta bàn tiếp về dòng code thứ 3 trong đoạn đó, nhưng ghi chú rằng nó vẫn là một phần của dòng code logic. Phần tiếp theo là phương thức này:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Chúng ta có thể ghi dòng code như sau:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Mặc dù vậy, một dòng quá dài thì vô cùng khó để đọc, vì vậy cách tốt nhất là chia nó ra. Thông thường, sẽ khôn ngoan hơn nếu khai báo một dòng code và một khoảng trắng để giúp chia một dòng dài khi bạn gọi một phương thức với cú pháp `.method_name()`. Bây giờ chúng ta sẽ thảo luận dòng code này làm gì.

Như đã đề cập trước đó, `read_line` đưa vào bất kỳ dữ liệu đầu vào nào được người dùng nhập vào, nhưng cũng đồng thời trả về giá trị `Result`. [`Result`][result]<!--ignore --> là một [*enumeration*][enums]<!-- ignore -->, thường xuyên gọi là *enum* thứ mà kiểu dữ liệu có thể ở một trong nhiều trạng thái có thể. Chúng ta gọi từng trạng thái đó là *các biến thể*.

Chương 6 sẽ bao hàm enums chi tiết hơn. Mục đích của những kiểu dữ liệu `Result` là để mã hóa thông tin xử lý lỗi.

Các biến thể của `Result` là `Ok` và `Err`. Biến thể `Ok` chỉ ra rằng toán tử đã thành công và bên trong `Ok` là một giá trị được khởi tạo thành công. Biến thể `Err` có nghĩa rằng toán tử đã thất bại, và `Err` bao gồm những thông tin về việc tại sao hay bằng cách nào mà toán tử lại thất bại.

Giá trị của kiểu dữ liệu `Result` giống như nhiều giá trị và kiểu khác, là đều có những phương thức định nghĩa trên chúng. Một instance của `Result` có [`expect` method][expect]<!-- ignore --> mà bạn có thể gọi. Nếu instance này của `Result` là một giá trị `Err`, `expect` sẽ là nguyên nhân tại sao chương trình crash và hiển thị tin nhắn đó, được bạn bỏ qua như là một tham số của `expect`. Nếu phương thức `read_line` được trả về như `Err`, nó sẽ giống như kết quả của một lỗi đến từ hệ điều hành của bạn. Nếu như instance của `Result` này là một giá trị `Ok`, `expect` sẽ nhận giá trị trả về khi `Ok` đang nắm giữ giá trị và trả về đúng giá trị đó cho bạn để sử dụng. Trong trường hợp này giá trị đó là số của những byte trong đầu vào người dùng.

Nếu bạn không gọi `expect`, chương trình vẫn sẽ biên dịch nhưng bạn sẽ nhận được cảnh báo:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust sẽ cảnh báo bạn vì bạn chưa sử dụng giá trị trả về `Result` từ `read_line`, nó chỉ ra tằng chương trình chưa được xử lý những lỗi có thể xảy ra. 

Cách phù hợp để giảm cảnh báo thực ra là viết phần xử lý lỗi, nhưng trong trường hợp này chúng ta chỉ muốn crash chương trình khi vấn đề xuất hiện, vì vậy chúng ta sử dùng `expect`. Bạn sẽ học được cách xử lý lỗi trong [Chương 9][recover]<!-- ignore -->.

### In ra giá trị với trình giữ chỗ `println!`

Bên cạnh dấu ngoặc nhọn, chỉ còn duy nhất một dòng để bàn về đoạn code đến bây giờ:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Dòng này in ra một chuỗi chứa đầu vào được nhập bởi người dùng. Dấu `{}` là một tập các dấu ngoặc nhọn là một trình giữ chỗ: Hãy liên tưởng `{}` giống như một chiếc càng cua nhỏ giữ một giá trị đúng chỗ. Bạn có thể in nhiều hơn một giá trị nếu dử dụng dấu ngoặc nhọn: dấu ngoặc nhọn đầu tiên của các dấu ngoặc nhọn giữ giá trị đầu tiên được liệt kê dưới định dạng chuỗi, và cứ thế tiếp tục. In ra nhiều giá trị trong một lần gọi `println!` sẽ trông như sau:

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

Dòng code này sẽ in ra: `x = 5 and y = 10`.

### Kiểm thử phần đầu tiên

Hãy cùng nhau kiểm thử phần đầu tiên của trò chơi đoán số. Khởi chạy chương trình sử dụng `cargo run`:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

Tính đến thời điểm này, phần đầu tiên của trò chơi đã hoàn tất: chúng ta đã nhận đầu vào được nhập bởi bàn phím người dùng và in ra nó.

## Khởi tạo ra một số bí mật

Tiếp theo, chúng ta cần tạo ra một số bí mật để người dùng đoán. Số bí mật sẽ thay đổi sau mỗi lần để trò chơi thêm phần thú vị. Chúng ta sẽ sử dụng một số bí mật nằm giữa 1 và 100 để trò chơi không trở nên quá khó. Rust chưa bao gồm một hàm khởi tạo ngẫu nhiên số bí mật. Nhưng đội ngũ Rust đã cung cấp một thư viện [`rand` crate][randcrate] như chúng ta đã nói.

### Sử dụng Crate để lấy thêm chức năng

Hãy nhớ rằng crate là một tập các file source code Rust. Một dự án mà chúng tôi đã từng xây dựng là một *binary crate*, thứ mà có thể thực thi. Crate `rand` là một *thư viện crate*, thứ mà bao gồm đoạn mã được dự định sẽ sử dụng trong chương trình khác và không thể tử thực thi bởi chính nó.

Sự phối hợp của Cargo trong những crate ngoại cảnh là điểm sáng của Cargo. Trước khi chúng ta có thể viết những đoạn code sử dụng `rand`, chúng ta cần chỉnh sửa file *Cargo.toml* để bao gồm `rand` như một dependency. Mở file đó và thêm vào những dòng sau ở bên dưới cùng chỗ `[dependencies]` trong mục header mà Cargo đã tạo cho bạn.

<!-- Khi cập nhật phiên bản của `rand` được sử dụng, đồng thời cần cập nhật phiên bản của `rand` được sử dụns trong những file sau để chúng giống nhau:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Trong file *Cargo.toml*, tất cả đều theo phần đầu, đó là mục tiếp theo cho đến khi mục khác bắt đầu. Trong `[dependencíe]` bạn chỉ ra với Cargo rằng những crates ngoại vi nào phụ thuộc trong dự án của bạn và phiên bản nào của những crate này mà bạn yêu cầu. Trong trường hợp này, chúng ta xác định rằng crate `rand` về mặt ngữ nghĩa được xác định là phiên bản `0.8.3`. Cargo hiểu [Semantic Versioning][semver]<!-- ignore --> (thi thoảng được gọi là *SemVer*), là tiêu chuẩn cho viết phiên bản số bao nhiêu. Số `0.8.3` thực ra là tốc ký cho `^0.8.3`. Điều này có ý nghĩa rằng phiên bản tối thiểu là `0.8.3` nhưng dưới `0.9.0`.

Cargo cho rằng những phiên bản này phải có những APIs công khai, tương thích với phiên bản `0.8.3` và sự chỉ rõ này đảm bảo rằng bạn sẽ nhận bản vá mới nhất được công bố, thứ mà vẫn có thể biên dịch với đoạn code ở trong chương này. Mọi phiên bản `0.9.0` trở lên không được cho phép có cùng API theo như những gì mà ví dụ sau sử dụng.

Bây giờ, thay vì thay đổi bất kỳ phần code nào, hãy xây dựng dự án này, như đã thể hiện trong mục Listing 2-2.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

<span class="caption">Listing 2-2: Đầu ra sau khi chạy `cargo build` sau khi thêm crate ngẫu nhiên như một dependency </span>

Bạn có thể thấy sự khác biệt trong số của phiên bản (nhưng chúng sẽ đều tương thích với đoạn code, cảm ơn SemVer!), Những dòng khác nhau (phụ thuộc vào hệ điều hành) và những dòng này code thể ở những trật tự khác nhau.

Khi chúng ta bao hàm những dependency ngoại vi, Cargo sẽ tìm về phiên bản cũ nhất của tất cả những gì mà dependency cần từ *registry*, đó là một bản sao của dữ liệu từ [Crates.io][cratesio]. Crate.io là nơi mà mọi người trong hệ sinh thái Rust có thể đăng những đoạn code Rust mã nguồn mở để mọi người khác có thể sử dụng. 

Sau khi cập nhật phần đăng ký, Cargo sẽ kiểm tra mục `[dependencies]` và tải về bất kỳ crates nào được liệt kệ nhưng chưa được tải xuống. Trong trường hợp này, mặc dù chúng ta mới liệt kê `rand` như là một dependency, Cargo đồng thời mang những crates khác mà `rand` cần để hoạt động. Sau khi tải xuống các crate, Rust sẽ biên dịch chúng và sau đó biên dịch dự án với những dependency khả dụng.

Nếu bạn ngay lập tức khởi chạy `cargo build` lần nữa mà không thay đổi bất kỳ điều gì, bạn sẽ không nhận được bất kỳ đầu ra ngoài nào từ dòng `Finished`. Cargo biết đó đã sẵn sàng được tải xuống và biên dịch các dependency, và bạn sẽ không cần thay đổi bất kỳ thứ gì trong file *Cargo.toml* của bạn. Cargo đồng thời biết rằng bạn chưa hề thay đổi bất cứ điều gì trong đoạn code của bạn, vì vậy nó không hề biên dịch lại luôn. Với việc không có gì để làm, nó chỉ đơn giản là thoát ra.

Nếu bạn mở file *src/main.rs*, thay đổi không đáng kể và sau đó lưu chúng và build lại từ đầu, bạn sẽ nhìn thấy hai dòng đầu ra như sau:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

Những dòng sau cho thấy rằng Cargo chỉ cập những bản build với sự thay đổi nhỏ nhất trong file *src/main/rs*. Những dependency của bạn không hề thay đổi, vì vậy Cargo biết rằng chúng có thể tái sử dụng những thứ gì đã được tải xuống và biên dịch những thứ đó.

#### Đảm bảo rằng có thể tái sử dụng những bản Build trong file *Cargo.lock*

Cargo có một cơ chế để đảm bảo rằng bạn có thể build lại cùng một sản phẩm từng lần, dù bạn hay người khác build code đó: Cargo sẽ chỉ sử dụng những phiên bản của những dependency mà bạn chỉ định đến khi bạn biểu thị cái khác. Lấy ví dụ, phiên bản tiếp theo của 0.8.4 của `rand` crate được công bố, và phiên bản đó đã bao gồm những lỗi đã được sửa, những nó đồng thời bao gồm sự cấp lại, mà có thể phá hỏng code của bạn. Để xử lý điều này, rust tạo ra file *Cargo.lock*, lần đầu tiên bạn chạy `cargo.build`, giờ đây chúng ta có điều này trong thư mục *guesing_game*

Khi bạn build một project lần đầu, Cargo tìm ra tất cả những phiên bản của các dependency phù hợp với những tiêu chí và rồi ghi chúng vào trong file *Cargo.lock* tồn tại và sử dụng những phiên bản được chỉ định hơn là làm hết những công việc được tìm ra trong những phiên bản trước đó. Điều này cho bạn khả năng tái sử dụng các bản build một cách tự động. Mặt khác, dự án của bạn sẽ được duy trì ở `0.8.3` đến khi bạn chỉ định cập nhật, cảm ơn file *Cargo.lock*. Bởi vì file *Cargo.lock* vô cùng quan trọng cho những bản build có thể tái sử dụng, nó thường xuyên kiếm tra trong trình kiểm soát mã với phần còn lại của đoạn code trong dự án của bạn.

#### Cập nhật Crate để nhận phiên bản mới hơn

Khi bạn muốn cập nhật một crate, Cargo cung cấp câu lệnh `update`, thứ mà bỏ qua file *Cargo.lock* và tìm ra tất cả những phiên bản mới nhất mà phù hợp với chỉ định của bạn trong file *Cargo.toml*. Cargo khi đó sẽ viết những phiên bản này vào trong file *Cargo.lock*. Mặt khác, mặc định, Cargo sẽ chỉ khóa những phiên bản lớn hơn `0.8.3` và nhỏ hơn `0.9.0`. Nếu crate `rand` đã được công bố hai phiên bản `0.8.4` và `0.9.0` thì bạn sẽ thấy những dòng sau đây nếu bạn chạy `cargo update`: 

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Cargo sẽ bỏ qua phiên bản `0.9.0`. Ở thời điểm này, bạn để ý rằng không có thay đổi trong file *Cargo.lock* của bạn mà những phiên của crate `rand` của bạn giờ đây sử dụng `0.8.4`. Để sử dụng phiên bản `0.9.0` của `rand` hoặc bất kỳ phiên bản nào khác trong tập `0.9.x`, bạn cần cập nhật trong file *Cargo.toml* để khóa thay vì làm như sau:

```toml
[dependencies]
rand = "0.9.0"
```

Lần tiếp theo bạn chạy `cargo build`, Cargo sẽ cập nhật bản đăng ký của những crate khả dụng và đánh giá lại yêu cầu `rand` thay vì phiên bản mới mà bạn chỉ định.

Có nhiều điều để nói về [Cargo][doccargo]<!-- ignore --> và [its ecosystem][doccratesio]<!-- ignore --> thứ mà chúng ta sẽ bàn kỹ hơn trong Chương 14, nhưng không phải bây giờ, đó là tất cả những gì mà bạn cần phải biết. Cargo sẽ làm nó trở nên vô cùng đơn giản để tái sử dụng thư viện, vì vậy những lập trình viên Rust có khả năng viết những dự án nhỏ hơn, những dự án được tập hợp bởi một số lượng các package.

### Khởi tạo một số ngẫu nhiên

Hãy bắt đầu sử dụng `rand` để khởi tạo một số để đoán. Bước tiếp theo là cập nhật *src/main.rs* được hiển thị trong Listing 2-3.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Listing 2-3: Thêm phần code để khởi tạo ngẫu nhiên</span>

Đầu tiên, chúng ta thêm dòng `use rand::Rng`. `Rnd` định nghĩa phương thức mà trình khởi tạo số triển khai, và đặc điểm này cần phải trong phạm vi cho chúng ta có thể sử dụng những phương thức này. Chương 10 sẽ bao gồm những đặc điểm này về chi tiết.

Tiếp theo, chúng ta sẽ thêm hai dòng ở giữa. Trong dòng đầu tiên, chúng ta gọi hàm `rand::thread_rng` để cho chúng ta một trình khởi tạo số ngẫu nhiên mà sẽ được chúng ta sử dụng: một là từ local đến thread hiện tại của trình thực thi và được seed bởi hệ điều hành. Sau đó chúng. Sau đó chúng ta gọi phương thức `gen_rand` trên trình khởi tạo số ngẫu nhiên. Phương thức này được định nghĩa bởi `Rng` mà chúng đã đã mang vào trong phạm vi này trước đó với câu lệnh `use rand::Rng`. Phương thức `gen_rand` lấy hai khoảng của biểu thức như là tham số và khởi tạo ra một số trong phạm vi đó. Kiểu của phạm vi biểu thức này chúng ta sử dụng ở đây để nhận mẫu `start..=end` và nó bao gồm giới hạn thấp và cao, vì vậy chúng ta cần chỉ ra `1..=100` để yêu cầu một số giữa 1 và 100.

> Ghi chú: bạn sẽ không thể biết đặc điểm nào được sử dụng và phương thức hay hàm nào được gọi trong crate, vì vậy mỗi crate có một tài liệu cho cách sử dụng. Với tính năng neat khác của Cargo được chạy bởi câu lệnh `cargo doc--open` sẽ build một tài liệu được cung cấp bởi tất cả các dependency locally và mở nó trong trình duyệt cua bạn. Nếu bạn cảm thấy thú vị với những tính năng này trong crate `rand` này, ví dụ, chạy `cargo doc --open` và bấm vào `rand` bên trái trên thành sidebar.

Dòng thứ hai in ra số bí mật. Nó sẽ vô cùng hữu ích khi mà chúng ta phát triển chương trình để có khả năng kiểm thử nó, nhưng chúng ta sẽ xóa nó ra khỏi phiên bản cuối cùng. Sẽ không có nhiều trò chơi nếu chương trình in ra kết quả ngay khi trò chơi bắt đầu.

Thử chạy chương trình vài lần:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

Bạn cần nhận được các số ngẫu nhiên khác nhau, và chúng sẽ đều nằm trong khoảng 1 và 100. Làm tốt lắm!

## So sánh dự đoán với số bí mật

Bây giờ chúng ta đã có đầu vào của người dùng và một số bí mật, chúng ta có thể so sánh chúng. Đó là bước được hiển thị trong Listing 2-4. Ghi chú rằng đoạn code này chưa được biên dịch, như chúng ta sẽ giải thích.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Listing 2-4: Xử lý các khả năng của giá trị trả về khi so sánh hai số</span>

Đầu tiên chúng ta thêm câu lệnh `use`, mang kiểu gọi `std::cmp::Ordering` vào trong phạm vi từ thư viện tiêu chuẩn. Kiểu `Odering` là một kiểu enum khác và có giá trị `Less`, `Greater`, và `Equal`. Đây là ba đầu ra khả dụng khi bạn so sánh hai giá trị.

Sau đó thêm năm dòng vào phần cuối sử dụng kiểu `Ordering`. Phương thức `cmp` so sánh hai giá trị và có thể được gọi trên bất kỳ thứ gì có thể so sánh được. Có cần tham chiếu tới bất kỳ thứ giá ban muốn để so sánh với: đây là cách so sánh `guess` với `secret_number`. Sau đó trả về giá trị của `Ordering` enum mà chúng ta mang vào trong phạm vi này với câu lệnh `use`. Chúng ta sử dụng biểu thức [`match`][match]<!-- ignore --> để khai báo những gì cần làm tiếp theo dựa trên những biến nào của `Ordering` được trả lại từ lời gọi đến hàm `cmp` với những giá trị trong `guess` và `secret_number`.

Biểu thức `match` được tạo ra bởi *arms*. Một cánh tay bao gồm những *pattern* để khớp với, và đoạn code nên được chạy nếu giá trị được cho đến `match` phù hợp với mô hình cánh tay. Rust nhận những giá trị được cho đến `match` và nhìn qua từng mô hình cánh tay trong một lượt. Các mô hình và cấu trúc `match` là những tính năng Rust vô cùng mạnh mẽ cho phép bạn thể hiện một vài những tình huống trong code của bạn có thể gặp và đảm bảo rằng bạn có thể xử lý được hết chúng. Những tính năng này sẽ được bao hàm về chi tiết trong Chương 6 và Chương 18 tương ứng.

Cùng nhau dạo qua một ví dụ với biểu thức `match` mà chúng ta sử dụng ở đây. Nói rằng người dùng đã đoán 50 lần và số được khởi tạo ngẫu nhiên lần này là 38. Khi mà code so sánh 50 và 38, phương thức `cmp` sẽ trả về giá trị `Orderring::Greater` và bắt đầu kiểm tra từng mô hình cánh tay. Nó nhìn đến mô hình cánh tay đầu tiên, `Ordering::Less` và nhìn thấy giá trị `Ordering::Greater` không khớp với `Ordering::Less`, vì vậy nó quả qua đoạn code trong cánh tay đó và chuyển tiếp đến canh tay tiếp theo. Mô hình cánh tay tiếp theo là `Ordering::Greater`, thứ mà khớp với `Ordering::Greater`! Sự liên quan trong đoạn code từ cánh tay đó sẽ thực thi và in ra `Too big!` lên màn hình. Biểu thức khớp sau khi lần khớp thành công đầu tiên, vì vậy nó sẽ không nhìn đến cánh tay cuối trong kịch bản.

Mặc dù, đoạn code trong Listing 2-4 sẽ chưa được biên dịch. Hãy thử điều này

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Cốt lõi của trạng thái lỗi là nó có *kiểu không khớp*. Rust có hệ thống mạnh, tĩnh. Mặc dù thế, nó cũng có sự suy luận. Khi chúng ta viết `let mut guess = String::new()`, Rust có khả năng suy luận ra `guess` có thể là kiểu `String` và không làm chúng ta viết kiểu. Mặt khác, `secret_number` là kiểu số. Và một vài kiểu số của Rust có thể có giá trị từ 1 đến 100: `i32`, một số 32 bit; `u32`, một số không dấu 32 bit; `i64`, một số 64 bit, giống với các số khác. Trừ khi được chỉ định khác, Rust sẽ mặc định sử dụng `i32`, đó là kiểu dành cho số `secret_number` trừ khi bạn thêm thông tin về kiểu khác, điều đó khiển Rust suy luận một kiểu số khác. Lý do cho lỗi này là Rust không thể so sánh một chuỗi với một kiểu số.

Cuối cùng, chúng ta muốn chuyển đổi kiểu chuỗi để chương trình đọc đầu vào thành một số thực, chúng ta có thể so sánh về mặt số học với số bí mật. Chúng ta làm điều đó bằng cách thêm dòng này vào phần thân của hàm `main`.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Dòng đó là:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

Chúng ta tạo ra một biến được đặt tên là `guess`. Nhưng đợi đã, không chương trình đã có biến tên là `guess` rồi hay sao? Đúng vậy, nhưng may thay, Rust cho phép chúng ta *shadow* giá trị trước đó của `guess` với một biến mới. Shadowing cho phép chúng ta tái sử dụng biến `guess` thay vì phải tạo ra biến mới, như là `guess_str` và `guess`. Chúng ta sẽ bao hàm chi tiết vấn đề này trong Chương 3, nhưng bây giờ chỉ cần biết rằng tính năng này thường xuyên được sử dụng khi bạn muốn chuyển đổi một giá trị sang giá trị khác.

Chúng ta gắn biến này vào biểu thức `guess.trim().parse()`. `guess` trong biểu thúc này cho thấy rằng biến `guess` nguyên bản chứa đầu vào như là một chuỗi. Phương thức `trim` lên `String` instance sẽ xóa bỏ bất kỳ khoảng trắng nào ở đầu và cuối chuỗi, chúng ta cần làm vậy để có khả năng so sánh chuỗi với `u32`, thứ mà chỉ bao gồm dữ liệu số. Người dùng sẽ nhập <span class="keystroke">enter</span> để thõa mãn `read_line` và nhập dự đoán của họ, thứ mà được thêm một dòng những ký tự mới thành chuỗi. Ví dụ, nếu người dùng nhập <span class="keystroke">5</span> và nhấn <span class="keystroke">enter</span>, `guess` sẽ giống như này `5\n`. `\n` biểu thị dòng mới (Trên windows, nhấn <span class="keystroke">enter</span> kết quả trong toa xe sẽ trả về và một dòng mới `\r\n`). Phương thức `trim` sẽ xóa bỏ `\n` và `\r\n`, kết quả sẽ chỉ là `5`.

[`parse` method on strings][parse]<!-- ignore --> sẽ chuyển đổi một chuỗi thành kiểu dữ liệu khác. Ở đây, chúng ta sẽ sử dụng nó để chuyển đối nó từ một chuỗi thành số. Chúng ta cần nói với Rust chính xác kiểu dữ liệu mà chúng ta cần, sử dụng `let guess: u32`. Dấu hai chấm (`:`) sau `guess` nói với Rust rằng chúng ta sẽ chú thích biến ở đây. Rust có một vài kiểu số xây dựng trong; `u32` ở đây là một số không dấu, số nguyên 32 bit. Đây là một lựa chọn mặc định tốt dành cho số dương nhỏ. Bạn sẽ học được về kiểu số khác trong Chương 3. Trong điều kiện đó, chú thích `u32` trong chương trình ví dụng này và sự so sánh với `secret_number` có ý nghĩa là Rust sẽ suy luận rằng `secret_number` sẽ là `u32`. Giờ đây sự so sánh sẽ là giữa hai giá trị có cùng kiểu.

Phương thức `parse` sẽ chỉ làm việc trên những ký tự có thể chuyển đổi hợp lý thành số và nó cũng dễ gây ra lỗi. Nếu, ví dụ là, chuỗi chứa `A👍%`, sẽ không thể nào chuyển đôi nó thành một số được. Vì nó sẽ thất bại, phương thức `parse` sẽ trả về kiểu `Result`, giống như phương thức `read_line` làm (đã được thảo luận trước đó trong [“Handling Potential Failure with the`Result` Type”](#handling-potential-failure-with-the-result-type)<!-- ignore-->). Chúng ta sẽ đối xử với `Result` cùng với cách mà chúng ta sử dụng phương thức `expect` lần nữa. Nếu `parse` trả về biến thể `Err` của `Result` vì không thể tạo một số từ một chuỗi, lời gọi `expect` sẽ phá hỏng trò chơi và in tin nhắn mà chúng ta đã đưa cho nó. Nếu `parse` có thể thành công chuyển đổi chuỗi thành số, nó sẽ trả về biến thể `Ok` của `Result`, và `expect` sẽ trả về số mà chúng ta muốn ở giá trị `Ok`.

Bây giờ hãy khởi chạy chương trình!

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

Tốt! Kể cả khi khoảng trắng được thêm vào trước dự đoán, chương trình vẫn tìm ra được dự đoán của người dùng là 76. Chạy chương trình vài lần để kiểu tra hành vi khác nhau với những kiểu đầu vào khác nhau: dự đoán số chính xác, dự đoán số quá cao, dự đoán số quá thấp.

Chúng ta đã có gần hết trò chơi bây giờ, nhưng người chơi chỉ có thể đoán một lần. Hãy thay đổi điều đó bằng cách thêm vào môt vòng lặp!

Từ khóa `loop` tạo ra một vòng lặp vô hạn. Chúng ta sẽ thêm một vòng lặp để cho người dùng có nhiều cơ hội hơn để đoán số:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Như bạn đã thấy, chúng ta đã chuyển tất cả từ đầu vào người dùng trở đi vào trong vòng lặp. Hãy chắc chắn rằng dòng mới được thụt vào trong với 4 khoảng trắng và chạy lại chương trình lần nữa. Chương trình sẽ hỏi dự đoán mãi mãi, đây là một vấn đề mới. Dường như là người chơi không thể thoát.

Người dùng có thể gián đoạn chương trình bất cứ lúc nào, sử dụng tổ hợp phím: <span class="keystroke">ctrl-c</span>. Nhưng có cách khác để thoát ra, như đã đề cập trước đó ở phần thảo luận `parse` trong [“Comparing the Guess to the Secret Number”](#comparing-the-guess-to-the-secret-number)<!--ignore -->: Nếu người chơi nhấn một phím không phải số, chương trình sẽ hỏng. Chúng ta cần tận dụng lợi thế đó để cho phép người chơi thoát, như dưới đây:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Gõ `quit` sẽ thoát trò chơi, nhưng bạn cần chú ý rằng người dùng có thể nhập một ký tự không phải số. Đây là cách không hề tối ưu, ít nhất, chúng ta muốn trò chơi dừng lại khi người chơi dự đoán đúng số.

### Thoát sau khi dự đoán đúng

Hãy lập trình một trò chơi khi người chơi thắng bằng cách thêm câu lệnh `break`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Thêm dòng `break` sau `You win!` làm cho chương trình thoát ra khỏi vòng lặp khi mà người chơi dự đoán đúng. Thoát khỏi vòng lặp đồng nghĩa với việc thoát khỏi chương trình, bởi vì vòng lặp là phần cuối cùng của `main`.

### Xử lý đầu vào không hợp lệ

Để lọc nhiều hơn những hành vi của trò chơi, hơn là chỉ dừng chương trình khi người chơi nhập một ký tự không phải số, hãy làm cho trò chơi bỏ qua ký tự đó để người chơi có thể tiếp tục dự đoán. Chúng là có thể làm được điều đó bằng cách thay đổi dòng sau, nơi mà `guess` được chuyển đổi từ `String` thành `u32`. Được thể hiện trong Listing 2-5;

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Listing 2-5: Bỏ qua một ký tự không phải số và yêu cầu một dự đoán khác thay vì dừng chương trình</span>

Chúng ta chuyển từ lời gọi `expect` thành biểu thức `match` để từ dừng chương trình thành một lỗi có thể xử lý được. Nhớ rằng `parse` sẽ trả về một kiểu `Result` là một enum có biến thể `Ok` và `Err`. Chúng ta đang sử dụng biểu thức `match` ở đây, như cách chúng ta đã làm với kết quả `Ordering` của phương thức `cmp`.

Nếu `parse` cho phép chuyển thành công từ chuỗi thành số, nó sẽ trả về giá `Ok` chứa kết quả số. Giá trị `Ok` đó sẽ khớp với mô hình cánh tay đầu tiên, và biểu thức `match` sẽ chỉ trả về giá trị `num` mà thủ tục `parse` và trả về bên trong giá trị `Ok`. Số này sẽ kết thúc ở nơi ta muốn nó ở trong giá trị `guess` mà chúng ta tạo ra.

Nếu `parse` không có khả năng để chuyển một chuỗi thành số, nó sẽ trả về giá trị `Err` hàm chứa nhiều thông tin hơn về lỗi. Giá trị `Err` không khợp với mẫu `Err(_)` trong cánh tay `match` đầu tiên. Dấu gạch dưới `_` là một giá trị catchall; Trong ví dụ này, chúng ta nói rằng chúng ta muốn khớp tất cả giá trị `Err`, không quan trọng thông tin nào mà họ có. Vì thế chương trình sẽ thực thi đoạn mã trong cánh tay thứ hai, `continue` nói với chương trình đi tới lần lặp tiếp theo của `loop` và tiếp tục hỏi dự đoán khác. Vì thế, vô cùng hiệu quả, chương trình sẽ bỏ qua tất cả lỗi mà `parse` có thể gặp!

Giờ đây mọi tính năng trong chương trình cần hoạt động như kì vọng. Hãy chạy thử:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

Tuyệt vời! Với một chút chỉnh sửa, chúng ta sẽ kết thúc trò chơi đoán số. Gọi lại chương trình vẫn sẽ in ra số bí mật. Nó hoạt động tốt trong lúc kiểm thử, nhưng nó sẽ phá hỏng trò chơi, Hãy cùng xóa `println!` mà in ra số bí mật. Listing 2-6 sẽ hiển thị đoạn code chính thức.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Listing 2-6: Hoàn thiện đoạn mã của trò chơi đoán số</span>

## Tổng kết

Ở thời điểm này, bạn đã thành công xây dựng trò chơi đoán số. Chúc mừng!

Dự án này là cách bắt tay vào để giới thiệu cho bạn những ý tưởng mới trong Rust: các hàm `let`, `match`, cách dùng các crate ngoại vi, và nhiều hơn nữa. Trong vài chương tiếp theo, bạn sẽ học được về những ý tưởng này chi tiết hơn. Chương 3 sẽ bao hàm các ý tưởng mà hầu hết các ngôn ngữ lập trình có, ví dụ như biến, kiểu dữ liệu và các hàm, và cho bạn thấy cách để dùng chúng trong Rust. Trong chương 4, chúng ta sẽ khám phá quyền sở hữu, một tính năng khiến Rust trở nên khác biệt so với các ngôn ngữ khác. Chương 5 chúng ta sẽ thảo luận sẽ các cấu trúc và các cú pháp phương thức. Và chương 6 sẽ giải thích enum hoạt động như nào.

[prelude]: ../std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: ../std/string/struct.String.html
[iostdin]: ../std/io/struct.Stdin.html
[read_line]: ../std/io/struct.Stdin.html#method.read_line
[result]: ../std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: ../std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[parse]: ../std/primitive.str.html#method.parse
