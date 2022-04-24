## Kiểu dữ liệu

Mọi giá trị trong Rust đều có một *kiểu dữ liệu* xác định, dựa vào kiểu dữ liệu
Rust sẽ biết phải làm việc với dữ liệu đó như thế nào. Chúng ta sẽ xem xét
hai tập con của kiểu dữ liệu: vô hướng và kết hợp.

Hãy nhớ rằng Rust là ngôn ngữ *định kiểu tĩnh (statically typed)*, tức là Rust phải biết được kiểu 
dữ liệu của tất cả các biến tại thời điểm biên dịch. Trình biên dịch thông thường
có thể suy luận kiểu dữ liệu mà chúng ta đang dùng dựa trên giá trị và cách chúng ta sử dụng
giá trị đó. Trong các trường hợp có nhiều kiểu, chẳng hạn như khi chúng ta chuyển
đổi `String` sang kiểu số bằng cách sử dụng `parse` trong phần [“So sánh số dự đoán với
số bí mật”][comparing-the-guess-to-the-secret-number]<!-- ignore --> trong Chương 2,
chúng ta phải chú thích rõ kiểu dữ liệu như sau:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

Nếu chúng ta không thêm chú thích kiểu dữ liệu vào, Rust sẽ hiển thị lỗi như bên dưới, trình
biên dịch cần thêm thông tin để biết chúng ta đang sử dụng kiểu dữ liệu nào:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Các loại dữ liệu khác nhau sẽ chú thích khác nhau.

### Kiểu dữ liệu vô hướng

Một kiểu dữ liệu *vô hướng (scalar)* đại diện cho một giá trị duy nhất. Rust có 4 kiểu vô hướng chính:
số nguyên (integers), số thực dấu phẩy động (floating-point numbers), Booleans và ký tự (characters).
Bạn có thể thấy chúng quen thuộc ở các ngôn ngữ lập trình khác. Hãy cùng tìm hiểu cách chúng hoạt
động trong Rust.

#### Kiểu số nguyên (interger)

*Số nguyên* là một số không có phần thập phân. Chúng ta đã sử dụng kiểu số nguyên
trong Chương 2, kiểu `u32`. Việc khai báo kiểu dữ liệu này cho giá trị chỉ ra rằng giá trị
được khai báo phải là một số nguyên không dấu chiếm 32 bit (kiểu số nguyên có dấu bắt đầu bằng `i`,
thay vì `u`). Bảng 3-1 chỉ ra các kiểu số nguyên được xây dựng sẵn trong Rust. Chúng ta
có thể sử dụng bất kì variant nào trong này để khai báo kiểu số nguyên.

<span class="caption">Bảng 3-1: Các kiểu số nguyên trong Rust</span>

| Độ dài  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Mỗi variant có thể có dấu hoặc không dấu và có kích thước rõ ràng.
*Signed* và *unsigned* chỉ ra rằng số đó liệu có thể có giá trị âm 
hay không, một số có dấu (signed) có thể chứa giá trị âm và dương, trong
khi một số không có dấu (unsigned) sẽ chỉ chứa giá trị dương. Giống như
việc viết số trên giấy: khi dấu là quan trọng, một số sẽ được ghi kèm với
dấu cộng hoặc dấu trừ; tuy nhiên, thông thường khi viết số dương thường không
có dấu. Các số có dấu được lưu trữ sử dụng [two’s
complement](https://en.wikipedia.org/wiki/Two%27s_complement)<!-- ignore -->
representation.

Mỗi signed variant có dấu có thể lưu trữ các số từ -(2<sup>n - 1</sup>) đến 2<sup>n -
1</sup> - 1, trong đó *n* là số bits mà variant sử dụng. Do đó,
`i8` có thể chứa các số từ -(2<sup>7</sup>) đến 2<sup>7</sup> - 1, tương ứng từ
-128 đến 127. Unsigned variants có thể chứa các số từ 0 đến 2<sup>n</sup> - 1,
do đó `u8` chứa các số từ 0 đến 2<sup>8</sup> - 1, tương ứng từ 0 đến 255.

Ngoài ra, kiểu `isize` and `usize` phụ thuộc vào cấu hình máy tính mà chương trình bạn
đang chạy, các kiểu này được ký hiệu trong bảng là “arch”:
64 bits nếu bạn đang dùng máy tính cấu hình 64-bit và 32 bits nếu bạn
đang dùng cấu hình 32-bit.

Bạn có thể viết số nguyên ở bất kỳ dạng nào như trong Bảng 3-2. Lưu ý rằng
các ký tự số (number literals) có thể là nhiều kiểu số cho phép một hậu tố để chỉ
định kiểu dữ liệu, chẳng hạn như `57u8`. Number literals cũng có thể sử dụng `_` để phân
tách số cho dễ đọc hơn, ví dụ `1_000` sẽ có cùng giá trị như khi bạn khai
báo `1000`.

<span class="caption">Bảng 3-2: Integer Literals trong Rust</span>

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |

Vì vậy làm cách nào bạn biết nên sử dụng loại số nguyên nào? Nếu bạn không chắc chắn
về điều đó, kiểu dữ liệu mặc định của Rust thường hữu ích: ví dụ, kiểu dữ liệu mặc định của
số nguyên trong Rust là `i32`. Còn `isize` và `usize` bạn sẽ sử dụng khi cần lập chỉ mục một
số loại collection.

> ##### Tràn số nguyên (Integer Overflow)
>
> Giả sử bạn có một biến có kiểu dữ liệu `u8` có thể lưu giá trị từ 0 đến
> 255. Nếu bạn cố gắng thay đổi giá trị của biến vượt ra khỏi phạm vi trên,
> chẳng hạn như 256, *integer overflow* sẽ xảy ra có thể dẫn đến một trong hai
> hành vi. Khi bạn biên dịch ở chế độ debug, Rust sẽ bao gồm các kiểm tra về
> integer overflow có phải là nguyên nhân khiến chương trình của bạn *panic* ở thời
> gian chạy nếu hành vi này xảy ra. Rust sử dụng thuật ngữ panicking khi thoát một
> chương trình bị lỗi, chúng ta sẽ thảo luận panics sâu hơn trong phần
> [“Các lỗi không thể phục hồi với
> `panic!`”][unrecoverable-errors-with-panic]<!-- ignore --> trong Chương
> 9.
>
> Khi bạn biên dịch trong chế độ release với cờ `--release`, Rust sẽ
> *không* bao gồm các kiểm tra về integer overflow. Thay vào đó, nếu
> overflow xảy ra, Rust thực hiện *two’s complement wrapping*. Tóm lại, các giá trị
> lớn hơn giá trị mà kiểu dữ liệu có thể chứa sẽ “wrap around” vào giá trị nhỏ nhất
> mà kiểu dữ liệu có thể lưu giữ. Trong trường hợp `u8`, giá trị 256 trở thành
> 0, giá trị 257 trở thành 1, v..v.. Chương trình sẽ không panic, nhưng biến sẽ
> có một giá trị mà bạn không mong đợi. Đây được coi là một lỗi.
>
> Để xử lý rõ ràng khả năng overflow, bạn có thể sử dụng các phương thức do thư viện
> chuẩn cung cấp cho các kiểu số nguyên thủy:
>
> - Wrap trong tất cả các chế độ bằng các phương thức `wrapping_*`, như `wrapping_add`
> - Trả về giá trị `None` nếu overflow xảy ra bằng phương thức `checked_*`
> - Trả về giá trị và một boolean cho biết liệu overflow có xảy ra hay không bằng
>   phương thức `overflowing_*`
> - Saturate ở các giá trị tối thiểu hoặc tối đa bằng phương thức `saturating_*`

#### Kiểu dấu phẩy động

Rust cũng có hai kiểu dữ liệu nguyên thủy cho *số thực dấu phẩy động*, các số
có dấu phần thập phân. Kiểu dấu phẩy động của Rust là `f32` và `f64`,
tương ứng với kích thước 32 bits và 64 bits. Kiểu mặc định trong Rust là `f64`
bởi vì các CPU hiện đại bây giờ tốc độ giống như `f32` nhưng chính xác hơn.
Tất cả kiểu dấu phẩy động đều có dấu.

Dưới đây là một ví dụ về sử dụng số thực dấu phẩy động:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Số thực dấu phẩy động được biểu diễn theo tiêu chuẩn IEEE-754 standard. Kiểu float
`f32` có độ chính xác đơn và `f64` có độ chính xác gấp đôi.

#### Các phép toán số học

Rust hỗ trợ tất cả các phép toán cơ bản cho tất cả kiểu số:
cộng, trừ, nhân, chia và phần dư. Phép chia số nguyên được làm
tròn đến số nguyên gần nhất. Code bên dưới chỉ bạn cách sử dụng
mỗi phép toán trong câu lệnh `let`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Mỗi biểu thức trong các câu lệnh sử dụng một toán tử toán học và cho ra kết quả là một
giá trị đơn được gán vào một biến. [Phụ lục B][appendix_b]<!-- ignore --> chứa danh sách
bao gồm tất cả các toán tử mà Rust cung cấp.

#### Kiểu Boolean

Như trong hầu hết các ngôn ngữ lập trình khác, kiểu Boolean trong Rust chứa hai giá trị:
`true` và `false`. Booleans có kích thước 1 byte. Sử dụng `bool` để chỉ ra kiểu Boolean
trong Rust. Ví dụ:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Giá trị Boolean được sử dụng chủ yếu trong các câu điều kiện, chẳng hạn như biểu thức `if`.
Chúng ta sẽ đề cập về biểu thức `if` được sử dụng như thế nào trong Rust trong phần [“Control
Flow”][control-flow]<!-- ignore -->.

#### Kiểu ký tự

Kiểu `char` trong Rust là kiểu chữ cái nguyên thủy nhất của ngôn ngữ. Sau đây là
một vài ví dụ về khai báo giá trị `char`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Lưu ý rằng chúng ta chỉ định ký tự `char` bằng dấu nháy đơn, trong khi ký tự chuỗi
sử dụng dấu nháy kép. Kiểu `char` trong Rust có kích thước 4 bytes và đại diện
cho một Giá trị Vô hướng Unicode (Unicode Scalar Value), tức là nó có thể đại diện
cho nhiều thứ hơn ASCII. Tấy cả các chữ cái có dấu; chữ Trung Quốc, Nhật Bản và Hàn Quốc; emoji
và zero-width spaces đều là giá trị `char` hợp lệ trong Rust. Unicode Scalar
Values nằm trong khoảng từ `U+0000` đến `U+D7FF` và `U+E000` đến `U+10FFFF`.
Tuy nhiên, một “ký tự” không thực sự là một khái niệm trong Unicode, vì vậy trực giác của
con người về “ký tự” là gì có thể không trùng khớp với `char` trong
Rust. Chúng ta sẽ thảo luận chi tiết về chủ đề này trong [“Storing UTF-8 Encoded Text with
Strings”][strings]<!-- ignore --> ở Chương 8.

### Kiểu kết hợp (Compound Types)

*Compound types* có thể nhóm nhiều giá trị vào một kiểu. Rust có hai
kiểu kết hợp nguyên thủy là: tuples và arrays.

#### Kiểu Tuple

Tuple là cách thông thường nhóm một số giá trị tương ứng với những kiểu 
dữ liệu khác nhau lại. Tuples có độ dài cố định: một khi được khai báo,
chúng ta không thể tăng hoặc giảm kích thước của chúng.

Chúng ta tạo ra tuple bằng cách viết một danh sách các giá trị được phân
cách nhau bằng dấu phẩy bên trong dấu ngoặc tròn. Mỗi vị trí trong tuple
mang một kiểu dữ liệu và không nhất thiết tất cả các giá trị trong tuple
phải có kiểu dữ liệu giống nhau. Chúng tôi đã thêm chú thích kiểu dữ liệu
như trong ví dụ sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Toàn bộ tuple được gán cho biến `tup` , vì một tuple được coi là một phần tử
kết hợp đơn lẻ. Để lấy các giá trị riêng lẻ ra khỏi tuple, chúng ta có thể
sử dụng pattern matching để destructure giá trị như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Chương trình ban đầu tạo ra một a tuple và gán nó vào biến `tup`. Sau đó
sử dụng pattern với `let` để biến `tup` trở thành ba biến riêng biệt,
`x`, `y` và `z`. Cách làm này được gọi là *destructuring*, bởi vì nó chia một
tuple đơn lẻ thành ba phần riêng biệt. Cuối cùng, chương trình in ra giá trị của
`y` là `6.4`.

Chúng ta cũng có thể truy cập trực tiếp vào một tuple bằng cách sử dụng dấu chấm (`.`)
theo sau đó là vị trí của giá trị mà chúng ta muốn truy cập. Ví dụ:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Chương trình tạo ra một tuple `x`, sau đó tạo các biến mới cho từng phần tử
bằng cách sử dụng chỉ số ứng với chúng. Như với hầu hết các ngôn ngữ lập trình,
vị trí đầu tiên trong tuple là 0.

Tuple không chứa bất kì giá trị nào là một kiểu đặc biệt chỉ chứa một giá trị,
`()`. Kiểu này được gọi là *unit type* và giá trị đó được gọi là
*unit value*. Các biểu thức sẽ trả về unit value nếu chúng không trả về bất kì
giá trị nào khác.

#### Kiểu Mảng (Array)

Một cách khác để có một collection với nhiều giá trị là sử dụng *array*. Không
giống như tuple, mọi phần tử của array phải có kiểu dữ liệu giống nhau. Không giống
như array trong các ngôn ngữ khác, các array trong Rust có độ dài cố định.

Chúng ta tạo array bằng cách viết một danh sách các giá trị được phân cách nhau bằng
dấu phẩy đặt bên trong dấu ngoặc vuông:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Arrays rất hữu ích khi bạn muốn dữ liệu của mình được phân bổ trên stack hơn là
trên heap (chúng ta sẽ thảo luận nhiều hơn về stack và heap trong [Chương
4][stack-and-heap]<!-- ignore -->) hoặc khi bạn muốn đảm bảo rằng bạn luôn luôn
có số lượng phần tử cố định. Tuy nhiên array không linh hoạt như kiểu vector.
Vector là một kiểu collection tương tự như array do thư viện chuẩn cung cấp, vector
*cho phép* bạn tăng hoặc giảm kích thước. Nếu bạn không chắc liệu nên sử dụng array
hay vector thì bạn nên sử dụng vector. Chúng ta sẽ thảo luận chi tiết hơn về vector trong
[Chương 8][vectors]<!-- ignore -->.

Tuy nhiên, array sẽ hữu ích hơn khi bạn đã biết được số lượng phần tử mà bạn cần.
Ví dụ, nếu bạn sử dụng tên của các tháng trong chương trình, bạn nên sử dụng array
hơn là vector bởi vì bạn biết chắc chắn rằng nó sẽ luôn luôn chứa 12 phần tử:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

Bạn viết kiểu array bằng cách sử dụng dấu ngoặc vuông với bên trong là kiểu của phần tử, dấu chấm
phẩy và sau đó là số lượng phần tử trong array, như sau:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Ở đây `i32` là kiểu dữ liệu của tất cả phần tử. Số `5`nằm sau dấu chấm phẩy
cho biết rằng array chứ 5 phần tử.

Bạn cũng có thể tạo ra một array chứa các phần tử có cùng giá trị bằng cách chỉ
định giá trị ban đầu, theo sau là dấu chấm phẩy và kế đó là độ dài của array, tất cả
được đặt bên trong dấu ngoặc vuông như bên dưới:

```rust
let a = [3; 5];
```

Array `a` sẽ chứa `5` phần tử có cùng giá trị ban đầu là
`3`. Tương tự như khi chúng ta viết `let a = [3, 3, 3, 3, 3];` nhưng
ngắn gọn hơn.

##### Truy cập các phần tử của array

Array là một đoạn bộ nhớ đơn đã biết có kích thước cố định có thể được phân
bổ trên stack. Bạn có thể truy cập các phần tử của array bằng cách sử dụng chỉ mục
như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

Trong ví dụ này, biến `first` sẽ nhận giá trị `1`, bởi vì đó
là giá trị tại chỉ mục `[0]` trong array. Biến `second` sẽ
nhận giá trị `2` từ chỉ mục `[1]` trong array.

##### Truy cập phần tử array không hợp lệ

Hãy xem điều gì xảy ra nếu bạn cố gắng truy cập vào một phần tử nằm vượt quá
phần cuối của array. Giả sử bạn chạy code này, tương tự như game đoán số trong
Chương 2, để nhận chỉ mục array từ người dùng:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Code này sẽ biên dịch thành công. Nếu bạn chạy `cargo run` và nhập
0, 1, 2, 3, hoặc 4, chương trình sẽ in ra giá trị tương ứng tại chỉ
mục trong array đó. Thay vào đó nếu bạn nhập một số vượt quá phần cuối
của array, ví dụ như 10, bạn sẽ thấy đầu ra như sau:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Chương trình sẽ dẫn đến lỗi *runtime* tại điểm mà bạn sử dụng giá trị không hợp lệ
trong thao tác lập chỉ mục. Chương trình thoát ra với thông báo lỗi và không
thực thi câu lệnh `println!` cuối cùng. Khi bạn cố gắng truy cập một phần tử sử dụng
lập chỉ mục, Rust sẽ kiểm tra xem chỉ mục mà bạn chỉ định có nhỏ đơn độ dài của array
hay không. Nếu chỉ mục lớn hơn hoặc bằng chiều dài của array, Rust sẽ panic.
Kiểm tra này diễn ra tại thời điểm runtime, nhất là trong trường hợp này, bởi vì
trình biên dịch không thể biết người dùng sau đó sẽ nhập giá trị nào khi họ chạy code.

Đây là một ví dụ về nguyên tắc an toàn bộ nhớ của Rust đang hoạt động. Trong nhiều
ngôn ngữ cấp thấp, loại kiểm tra này không được thực hiện và khi bạn cung cấp một
chỉ mục không chính xác, bộ nhớ không hợp lệ có thể được truy cập. Rust bảo vệ bạn
tránh khỏi lỗi này bằng cách thoát chương trình ngay lập tức, thay vì cho phép truy
cập bộ nhớ và tiếp tục chạy. Chương 9 sẽ thảo luận nhiều hơn về cách xử lý lỗi của Rust.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: ../std/num/struct.Wrapping.html
[appendix_b]: appendix-02-operators.md
