## Control Flow

Khả năng chạy code phụ thuộc vào một điều kiện là đúng hoặc chạy code lặp đi lặp
lại trong khi điều kiện đúng là các khối xây dựng cơ bản trong hầu hết ngôn ngữ 
lập trình. Các cấu trúc phổ biến nhất cho phép bạn kiểm soát luồng thực thi code
của Rust là biểu thức `if` và vòng lặp (loops).

### Biểu thức `if`

Biểu thức `if` cho phép bạn phân nhánh code phụ thuộc vào các điều kiện. Bạn
đưa ra một điều kiện, sau đó yêu cầu: “Nếu thỏa mãn điều kiện này, hãy chạy code
này. Nếu điều kiện này không thỏa mãn, không chạy code này.”

Khởi tạo một dự án mới có tên *branches* trong thư mực *projects* của bạn để
tìm hiểu về biểu thức `if`. Trong file *src/main.rs*, hãy nhập code như bên dưới:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tất cả biểu thức `if` bắt đầu với từ khóa `if`, theo sau là một điều kiện. Trong
trường hợp này, điều kiện sẽ kiểm tra xem liệu biến `number` có giá trị nhỏ hơn
5 hay không. Ngay sau điều kiện, bên trong cặp dấu ngoặc nhọn, chúng ta viết code
cần thực thi nếu điều kiện đúng. Đoạn code liên kết với điều kiện trong biểu thức
`if` thường được gọi là *nhánh (arms)*, giống như arms trong biểu thức
`match` mà chúng ta đã đề cập trong phần [“So sánh số dự đoán với số bí
mật”][comparing-the-guess-to-the-secret-number]<!--
ignore --> ở Chương 2.

Chúng ta cũng có thể tùy ý bao gồm thêm biểu thức `else`, như chúng ta đã làm
ở đây, để cung cấp cho chương trình một đoạn code thay thế khác để thực thi nếu điều
kiện sai. Nếu bạn không cung cấp biểu thức `else` và điều kiện không thỏa mãn,
chương trình sẽ bỏ qua đoạn code chứa `if` và chuyển sang đoạn code tiếp theo.

Hãy thử chạy code này; bạn sẽ nhận được kết quả như sau:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Hãy thử thay đổi giá trị của biến `number` thành một giá trị làm cho điều kiện
`sai` để xem điều gì xảy ra:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Chạy chương trình một lần nữa và xem kết quả:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Cũng cần lưu ý rằng điều kiện trong code *phải* là `bool`. Nếu điều kiện
không phải là `bool`, chúng ta sẽ gặp lỗi. Ví dụ, bạn hãy thử chạy đoạn code
sau đây:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Lần này điều kiện `if` có giá trị là `3` và Rust đưa ra
lỗi:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Lỗi chỉ ra rằng Rust mong đợi `bool` nhưng lại nhận được số nguyên. Không giống
như các ngôn ngữ khác như Ruby và JavaScript, Rust sẽ không tự động chuyển đổi
các kiểu dữ liệu không phải Boolean sang Boolean. Bạn phải rõ rằng và luôn luôn
cung cấp Boolean cho câu điều kiện `if`. Ví dụ, nếu bạn muốn đoạn code `if` chỉ
chạy khi một số khác `0`, chúng ta có thể thay đổi biểu thức `if` như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Khởi chạy đoạn code này sẽ in ra `number was something other than zero`.

#### Xử lý nhiều điều kiện với `else if`

Bạn có thể so sánh nhiều điều kiện bằng cách kết hợp `if` và `else` trong biểu thức `else if`
Ví dụ như:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Chương trình này có 4 điều kiện có thể thực hiện. Sau khi chạy code, bạn sẽ
thấy kết quả như bên dưới:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Khi thực thi chương trình, Rust sẽ kiểm tra lần lượt mỗi biểu thức `if` và
thực thi đoạn code đầu tiên mà điều kiện thỏa mãn. Lưu ý rằng 6 chia hết cho
2, chúng ta không thấy kết quả đầu ra là đoạn text `number is divisible by 2`
cũng như đoạn text `number is not divisible by 4, 3, or 2` từ `else`.
Bởi vì Rust chỉ thực thi đoạn code cho điều kiện đúng đầu tiên và nó sẽ
bỏ qua phần code còn lại.

Sử dụng quá nhiều biểu thức `else if` có thể làm lộn xộn code của bạn, vì vậy nếu
bạn có nhiều hơn một biểu thức, bạn có thể cần tái cấu trúc lại code của mình.
Chương 6 giới thiệu một cấu trúc phân nhánh mạnh mẽ, `match`, cho các trường hợp này.

#### Sử dụng `if` trong câu lệnh `let`

Bởi vì `if` là một biểu thức, chúng ta có thể sử dụng nó bên phải câu lệnh `let`
để giá kết quả cho biến, như trong Listing 3-2.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption">Listing 3-2: Gán kết quả của biểu thức `if`
cho một biến</span>

Biến `number` sẽ được gán với giá trị dựa vào kết quả của biểu thức `if`.
Hãy chạy đoạn code này để xem điều gì xảy ra:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Hãy nhớ rằng đoạn code đánh giá đến biểu thức cuối cùng và các số cũng bản thân nó  
cũng là biểu thức. Trong trường hợp này, giá trị của toàn bộ biểu thức
`if` phụ thuộc vào đoạn code mà nó thực thi. Điều này có nghĩa là các giá trị
của mỗi nhánh `if` phải có kiểu dữ liệu giống nhau; trong Listing 3-2, kết quả
từ cả nhánh `if` và nhánh `else` đều là số nguyên `i32`. Nếu kiểu dữ liệu không khớp,
bạn sẽ gặp lỗi như trong ví dụ dưới đây:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Khi bạn biên dịch code này, bạn sẽ gặp lỗi. Nhánh `if` và `else` có kiểu giá trị
không tương thích và Rust chỉ ra chính xác vị trí vấn đề trong chương trình:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

Biểu thức trong khối `if` đưa ra một số nguyên và biểu thức trong khối
`else` đưa ra một string. Điều này sẽ không thực hiện được bởi vì các biến
phải có một kiểu duy nhất và Rust cần biết tại thời điểm biên dịch biến
`number` có kiểu gì. Biết được kiểu của `number` cho phép trình biên dịch
xác minh kiểu đó hợp lệ ở mọi chỗ khi chúng ta sử dụng `number`. Rust không thể
làm điều đó nếu kiểu của `number` chỉ được xác định tại thời điểm runtime; trình
biên dịch sẽ phức tạp hơn và sẽ ít đảm bảo hơn về code nếu nó phải theo dõi
nhiều kiểu giả định cho bất kỳ biến nào.

### Lặp lại với các vòng lặp (Loops)

Việc thực thi một đoạn code nhiều lần sẽ rất hữu ích. Để làm điều này,
Rust cung cấp một số *vòng lặp (loop)*, vòng lặp này sẽ chạy code bên trong
thân của vòng lặp từ đầu đến cuối và sau đó ngay lập tức bắt đầu lại từ đầu.
Để trải nghiệm với vòng lặp, hãy khởi tạo một dự án mới có tên *loops*.

Rust có 3 loại vòng lặp: `loop`, `while`, và `for`. Hãy thử từng loại.

#### Lặp lại code với `loop`

Từ khóa `loop` yêu cầu Rust thực thi một đoạn code lặp đi lặp lại mãi mãi
hoặc cho đến khi bạn yêu cầu nó dừng lại.

Ví dụ, thay đổi file *src/main.rs* trong thư mục *loops* như bên dưới:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Khi bạn chạy chương trình này, bạn sẽ thấy kết quả `again!` được in liên tục
cho đến khi chúng ta dừng chương trình một cách thủ công. Hầu hết terminal
hỗ trợ phím tắt <span class="keystroke">ctrl-c</span> để ngắt một chương trình
bị mắc kẹt trong một vòng lặp liên tục. Hãy thử một lần:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->

```console
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

Ký hiệu `^C` thể hiện vị trí bạn nhấn <span class="keystroke">ctrl-c
</span>. Bạn có thể gặp lại từ `again!` này hoặc không được in ra sau `^C`,
phụ thuộc vào vị trí của code trong vòng lặp khi nó nhận được tín hiệu
ngắt.

May mắn thay, Rust cũng cung cấp một cách để thoát khỏi vòng lặp bằng cách sử dụng code.
Bạn có thể đặt từ khóa `break` trong vòng lặp để cho chương trình biết khi nào nên dừng
thực hiện vòng lặp. Chúng ta đã làm điều này trong trò chơi đoán số trong phần
[“Thoát ra sau khi đoán đúng”][quitting-after-a-correct-guess]<!-- ignore
--> ở Chương 2 để thoát khỏi chương trình khi người dùng đoán đúng con số bí mật.

Chúng ta cũng đã dùng `continue` trong trò chơi đoán số, trong vòng lặp `continue` sẽ yêu cầu
chương trình bỏ qua đoạn code còn lại trong lần lặp này của vòng lặp và đi
đến vòng lặp tiếp theo.

Nếu bạn có vòng lặp lồng bên trong vòng lặp, `break` và `continue` áp dụng cho vòng
lặp trong cùng tại điểm đó. Bạn có thể tùy chọn chỉ định *gắn nhãn cho vòng lặp*, chúng ta có thể
sử dụng `break` hoặc `continue` để chỉ định rằng các từ khóa đó sẽ áp dụng cho vòng
lặp nào được gắn nhãn thay vì vòng lặp trong cùng. Dưới đây là một ví dụ về hai
vòng lặp lồng nhau:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Vòng lặp bên ngoài có nhãn `'counting_up` và nó sẽ đếm từ 0 đến 2.
Vòng lặp bên trong không có nhãn đếm ngược từ 10 đến 9. Câu lệnh`break` đầu tiên
không chỉ định nhãn của vòng lặp nên sẽ chỉ thoát khỏi vòng lặp bên trong. Câu lệnh `break
'counting_up;` sẽ thoát khỏi vòng lặp ngoài. Code này sẽ in ra:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Trả về giá trị từ vòng lặp

Một trong các cách sử dụng `vòng lặp` là thử một phép toán mà bạn biết có thể nó
sẽ thất bại, chẳng hạn như kiểm tra xem một luồng có hoàn thành công việc hay không.
Bạn cũng có thể cần chuyển kết quả của phép toán đó ra khỏi vòng lặp đến phần còn lại
của code. Để làm điều này, bạn có thể thêm giá trị bạn muốn trả về sau biểu thức `break`
mà bạn sử dụng để dừng vòng lặp; giá trị đó sẽ được trả về khỏi vòng lặp và do đó bạn
có thể sử dụng giá trị đó như được hiển thị ở đây:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Trước vòng lặp, chúng ta khai báo một biến có tên `counter` và gán cho nó giá trị
`0`. Sau đó chúng ta khai báo một biến có tên `result` để chứa giá trị mà vòng lặp
trả về. Trên mỗi lần lặp của vòng lặp, chúng ta cộng `1` vào biến `counter` và sau
đó kiểm tra xem liệu counter bằng `10` hay không. Khi nó thỏa mãn điều kiện, chúng ta
sử dụng từ khóa `break` với `counter * 2`. Sau vòng lặp, chúng ta sử dụng dấu chấm phẩy
để kết thúc câu lệnh gán giá trị vào biến `result`. Cuối cùng, chúng ta in ra giá trị
của `result`, trong trường hợp này là 20.

#### Vòng lặp có điều kiện với `while`

Một chương trình sẽ thường cần đánh giá một điều kiện bên trong vòng lặp. Trong
khi điều kiện là đúng, vòng lặp sẽ chạy. Khi điều kiện không còn đúng nữa, chương
trình sẽ gọi `break` để dừng vòng lặp. Có thể triển khai hành vi như vậy bằng cách
sử dụng kết hợp `loop`, `if`, `else` và `break`; bạn có thể thử điều đó trong một
chương trình ngay nếu bạn muốn. Tùy nhiên, mẫu này phổ biến đến mức Rust đã xây dựng
một cấu trúc cho nó gọi là vòng lặp `while`. Trong Listing 3-3, chúng ta sử dụng `while`
để lặp lại chương trình ba lần, mỗi lần lặp đếm ngược, in ra một thông báo và thoát

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Listing 3-3: Sử dụng vòng lặp `while` để chạy code
trong khi điều kiện đúng</span>

Cấu trúc này loại bỏ nhiều lồng ghép cần thiết neus bạn sử dụng
`loop`, `if`, `else` và `break` và nó rõ ràng hơn nhiều. Trong khi
một điều kiện đúng, code sẽ hoạt động; nếu không nó sẽ thoát vòng lặp.

#### Vòng lặp cho Collection với `for`

Bạn có thể sử dụng cấu trúc `while` để lặp qua các phần tử của một
collection, chẳng hạn như array. Ví dụ, vòng lặp trong Listing 3-4 in ra
từng phần tử trong array `a`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Listing 3-4: Lặp qua từng phần tử của một collection
sử dụng vòng lặp `while`</span>

Ở đây, code sẽ đếm lên thông qua các phần tử trong array. Nó bắt đầu ở chỉ mục
`0` và sau đó lặp lại cho đến chỉ mục cuối cùng trong array (đó là khi
`index < 5` không còn đúng nữa). Khởi chạy code này, nó sẽ in ra mọi phần tử
trong array:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Tất cả năm giá trị của array đều xuất hiện trong terminal như mong đợi. Mặc dù `index`
sẽ đạt giá trị `5` tại một số thời điểm , vòng lặp dừng thực thi trước khi cố gắng
tìm nạp giá trị thứ sáu từ array.

Tuy nhiên, các tiếp cận này dễ xảy ra lỗi; chúng ta có thể làm chương trình panic nếu
giá trị chỉ mục hoặc điều kiện kiểm tra không đúng. Ví dụ nếu bạn thay đổi định nghĩa
của array `a` chỉ có bốn phần tử nhưng quên cập nhật điều kiện thành
`while index < 4`, code khi đó sẽ panic. Nó cũng chậm bởi vì trình biên dịch thêm
runtime code để thực hiện kiểm tra có điều kiện liệu chỉ mục có nằm trong giới hạn
của array trên mỗi lần lặp qua vòng lặp hay không.

Để thay thế ngắn gọn hơn, bạn có thể sử dụng vòng lặp `for` và thực thi code cho mỗi
item trong collection. Vòng lặp `for` trong giống như code trong Listing 3-5.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Listing 3-5: Lặp qua từng phần tử của collection
sử dụng vòng lặp `for`</span>

Khi chúng ta sử dụng đoạn code này, chúng ta sẽ nhận được kết quả giống hệt như trong
Listing 3-4. Quan trọng hơn, chúng ta có thể tăng tính an toàn của code và loại bỏ
khả năng lỗi từ việc vượt qua phần tử cuối của array hoặc lặp không hết và thiếu
sót một vài item.

Sử dụng vòng lặp `for`, bạn sẽ không cần phải thay đổi code nếu bạn thay đổi số lượng phần
tử của array như với phương pháp mà bạn sử dụng trong Listing 3-4.

Tính an toàn và độ ngắn gọn của vòng lặp `for` khiến chúng trở thành cấu trúc vòng lặp
được sử dụng phổ biến trong Rust. Ngay cả trong các tình huống bạn muốn chạy code một
số lần nhất định, như ví dụ đếm ngược đã sử dụng vòng lặp `while` trong
Listing 3-3, phần lớn Rustaceans sẽ sử dụng vòng lặp `for`. Các để làm điều đó là sử
dụng `phạm vi (Range)`, được cung cấp bởi thư viện chuẩn, tạo ra tất cả các số theo
thứ tự bắt đầu từ một số và kết thúc trước một số khác.

Dưới đây là cách đếm ngược sử dụng vòng lặp `for` và một phương thức khác mà
chúng ta chưa đề cập đến, `rev`, để đảo ngược phạm vi:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Code này đẹp hơn một chút rồi phải không?

## Tóm tắt

Bạn đã làm được! Đây là một chương khá lớn: bạn đã học về biến, kiểu dữ liệu vô
hướng và kết hợp, hàm, comments, biểu thức `if` và vòng lặp!
Để thực hành các khái niệm này, hãy thử xây dựng chương trình để thực hiện những
điều sau:

* Chuyển đổi nhiệt độ giữa Fahrenheit và Celsius.
* Tạo ra số Fibonacci thứ n.
* In lời bài hát mừng Giáng sinh “The Twelve Days of Christmas,”
  tận dụng sự lặp lại trong bài hát.

Khi bạn đã sẵn sàng để tiếp tục, chúng ta sẽ nói về một khái niệm trong Rust *không*
thường tồn tại trong các ngôn ngữ lập trình khác: quyền sở hữu (ownership).

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:
ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
