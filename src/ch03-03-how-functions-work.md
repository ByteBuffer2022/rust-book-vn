## Hàm

Hàm phổ biến trong Rust. Bạn đã thấy một trong những hàm quan trọng nhất trong ngôn
ngữ này: đó là hàm `main`. Bạn cũng thấy từ khóa `fn` giúp cho phép bạn khai báo
hàm mới.

Rust sử dụng *quy tắc con rắn (snake case)* làm quy ước cách đặt tên hàm và biến,
trong đó tất cả các chữ cái phải viết thường và phân cách các từ bằng dấu gạch dưới.
Bên dưới là một chương trình ví dụ về cách định nghĩa hàm:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Chúng ta định nghĩa hàm trong Rust bằng cách gõ từ khóa `fn` theo sau là
tên của hàm và một cặp dấu ngoặc đơn. Cặp dấu ngoặc nhọn cho trình biên dịch
biết vị trí bắt đầu và kết thúc của thân hàm.

Chúng ta có thể gọi bất kỳ hàm nào đã được định nghĩa bằng cách gõ tên hàm theo sau
là cặp dấu ngoặc tròn. Do hàm `another_function` đã được định nghĩa trong chương trình,
bạn có thể gọi hàm này từ bên trong hàm `main`. Lưu ý rằng chúng ta đã định nghĩa hàm `another_function`
*sau* hàm `main` trong code; chúng ta cũng có thể định nghĩa nó trước hàm `main`.
Rust không quan tâm bạn định nghĩa hàm ở đâu, Rust chỉ quan tâm hàm đã được định
hay chưa.

Hãy bắt đầu một dự án nhị phân có tên *functions* để khám phá thêm về
hàm. Lấy ví dụ hàm `another_function` cho vào *src/main.rs* và khởi chạy chương trình.
Bạn sẽ thấy kết quả như bên dưới:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Các dòng thực thi theo thứ tự xuất hiện trong hàm `main`.
Đầu tiên, thông điệp “Hello, world!” được in ra, sau đó gọi hàm `another_function`
và in thông điệp của nó ra.

### Tham số

Chúng ta có thể định nghĩa các hàm có các *tham số* - đây là những biến đặc biệt
là một phần của chữ ký hàm. Khi một hàm có tham số đi kèm, bạn có thể cung cấp
cho nó các giá trị tham số cụ thể. Về mặt kỹ thuật, các giá trị cụ thể này được
gọi là *đối số (arguments)*, nhưng trong giao tiếp thông thường, mọi người có xu hướng
sử dụng từ *tham số (parameter)* và *đối số (argument)* thay thế cho nhau cho
các biến trong định nghĩa hàm hoặc giá trị cụ thể được chuyển vào khi bạn gọi
hàm.

Trong phiên bản này của hàm `another_function`, chúng ta thêm một tham số vào:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Thử khởi chạy chương trình; bạn sẽ nhận được kết quả sau:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

Khi khai báo hàm `another_function`, chúng ta có một tham số tên `x`. Kiểu của
`x` được chỉ định là `i32`. Khi chúng ta chuyển giá trị `5` vào hàm `another_function`,
`println!` macro sẽ đặt `5` vào vị trí của cặp dấu ngoặc nhọn để định dạng thành
string.

Trong chữ ký hàm, bạn *buộc phải* khai báo kiểu của mọi tham số. Đây là một quyết
định có chủ ý trong thiết kế của Rust: việc yêu cầu chú thích kiểu dữ liệu trong định nghĩa hàm
có nghĩa là trình biên dịch sẽ không bao giờ yêu cầu bạn khai báo kiểu dữ liệu ở bất kỳ
nơi khác trong code để tìm ra kiểu dữ liệu mà bạn muốn nói đến.

Khi bạn định nghĩa hàm với nhiều tham số, phân cách việc khai báo tham số bằng dấu phẩy
như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Ví dụ này tạo ra một hàm có tên `print_labeled_measurement` với hai
tham số. Tham số thứ nhất có tên `value` và có kiểu `i32`. Tham số thứ hai có tên
`unit_label` và có kiểu `char`. Sau đó hàm in ra đoạn văn bản chứa cả
`value` và `unit_label`.

Hãy thử khởi chạy code. Thay thế ví dụ vừa rồi vào file
*src/main.rs* của dự án *functions* và khởi chạy nó với `cargo
run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Bởi vì chúng ta đã gọi hàm với giá trị của `value` là `5` và `unit_label` là `'h'`,
đầu ra của chương trình sẽ chứa các giá trị đó.

### Câu lệnh và Biểu thức

Thân hàm được tạo thành từ một loạt các câu lệnh tùy ý kết thúc bằng
một biểu thức. Cho đến bây giờ, các hàm mà chúng ta đã đề cập chưa có hàm nào kết
bằng một biểu thức, nhưng bạn đã thấy biểu thức như một phần của câu lệnh. Bởi vì
Rust là một ngôn ngữ dựa trên biểu thức, bạn cần hiểu về sự phân biệt quan trọng này.
Những ngôn ngữ khác không có sự phân biệt giống như vậy, vì vậy hãy xem xét câu lệnh
và biểu thức là gì và sự khác biệt của chúng ảnh hưởng như thế nào đến nội dung của
hàm.

*Câu lệnh (Statements)* là các hướng dẫn thực hiện một số hành động và không trả về 
giá trị. *Biểu thức (Expressions)* tính toán để đưa ra một giá trị kết quả. Hãy xem xét một vài ví dụ.

Chúng ta đã thực sự sử dụng câu lệnh và biểu thức. Tạo một biến và gán một giá trị
cho nó với từ khóa `let` là một câu lệnh. Trong Listing 3-1,
`let y = 6;` là một câu lệnh.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption">Listing 3-1: A `main` function declaration containing one statement</span>

Việc định nghĩa một hàm cũng là một câu lệnh; toàn bộ ví dụ trước là một câu lệnh.

Câu lệnh không trả về giá trị. Do đó, bạn không thể gán một câu lệnh `let` statement
vào một biến khác, bạn sẽ gặp lỗi nếu cố gắng làm như đoạn code bên dưới:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Khi bạn chạy chương trình, bạn sẽ bắt gặp lỗi như thế này:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

Câu lệnh `let y = 6` không trả về một giá trị, do đó không có bất kỳ giá trị nào
được gán cho `x`. Điều này khác biệt với những gì xảy ra trong các ngôn ngữ khác,
như C và Ruby, trong đó phép gán trả về giá trị của phép gán đó. Trong những
ngôn ngữ này, bạn có thể viết `x = y = 6`, cả `x` và `y` đều sẽ có giá trị là
`6`; nhưng Rust thì không hiểu như vậy.

Biểu thức tính toán đưa ra một giá trị và tạo nên gần như toàn bộ phần còn lại của code mà
bạn viết bằng Rust. Xét về một phép toán số học như `5 + 6`, đó là một
biểu thức sẽ đưa ra kết quả là `11`. Biểu thức có thể là một phần của câu
lệnh: trong Listing 3-1, giá trị `6` trong câu lệnh `let y = 6;` là một
biểu thức đưa ra giá trị là `6`. Việc gọi một hàm cũng là một biểu
thức. Gọi một macro là một biểu thức. Một khối phạm vi mới được tạo ra bằng
dấu ngoặc nhọn là một biểu thức, ví dụ:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Biểu thức này:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

trong trường hợp này là một khối sẽ đưa ra giá trị là `4`. Giá trị đó được gán vào `y`
như một phần của câu lệnh `let`. Lưu ý rằng dòng `x + 1` không có dấu chấm phẩy ở
cuối câu, không giống như hầu hết các dòng code bạn đã thấy cho đến nay. Các biểu thức
không kết thúc bằng dấu chấm phẩy. Nếu bạn thêm dấu chấm phẩy vào cuối biểu thức, nó sẽ
trở thành một câu lệnh và câu lệnh thì không trả về giá trị.
Hãy nhớ điều này khi bạn tìm hiểu về các hàm trả về giá trị và biểu thức ở phần kế tiếp.

### Hàm trả về giá trị

Hàm có thể trả về các giá trị cho code khi chúng được gọi. Chúng ta không đặt tên
giá trị trả về, nhưng chúng ta buộc phải khai báo kiểu dữ liệu của chúng sau dấu mũi
tên (`->`). Trong Rust, giá trị trả về của hàm đồng nghĩa với giá trị của biểu
thức cuối cùng trong phần thân hàm. Bạn có thể trả về sớm giá trị của hàm bằng việc
sử dụng từ khóa `return` và chỉ định giá trị trả về, nhưng trong hầu hết các hàm
sẽ ngầm định trả về biểu thức cuối cùng. Dưới đây là một ví dụ về một hàm
trả về giá trị:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

Không có gọi hàm, macros, hay thậm chí câu lệnh `let` trong hàm `five`
 — chỉ có duy nhất số `5`. Đó là một hàm hợp lệ trong Rust.
Lưu ý rằng phải biết được kiểu dữ liệu trả về của hàm như `-> i32`. Hãy
khởi chạy code này; đầu ra sẽ như thế này:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

Số `5` trong hàm `five` là giá trị trả về của hàm, đó là lý do tại sao kiểu
dữ liệu trả về là `i32`. Hãy xem xét điều này chi tiết hơn. Có 2 bits quan trọng:
đầu tiên, dòng lệnh `let x = five();` cho biết rằng bạn đang sử dụng giá trị trả về
của hàm để khởi tạo ra một biến. Bởi vì hàm `five` trả về giá trị `5`,
dòng lệnh đó tương đương với dòng lệnh bên dưới:

```rust
let x = 5;
```

Thứ hai, hàm `five` không có tham số và hàm xác định kiểu dữ liệu của giá trị 
trả về, nhưng trong phần thân hàm chỉ có số `5` không có dấu chấm phẩy
bởi vì nó là một biểu thức chứa giá trị mà chúng ta muốn trả về.

Hãy xem xét một ví dụ khác:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Code này sẽ in ra `The value of x is: 6`. Nhưng nếu chúng ta đặt dấu chấm phẩy
vào cuối dòng chứa `x + 1`, nó sẽ từ biểu thức chuyển thành câu lệnh,
chúng ta sẽ gặp lỗi ngay.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Biên dịch code này sẽ gây ra lỗi như sau:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Thông báo lỗi chính, “mismatched types,” cho thấy vấn đề cốt lõi trong
code. Định nghĩa hàm `plus_one` nói rằng nó sẽ trả về một giá trị
`i32`, nhưng câu lệnh không trả về giá trị được biểu thị bằng `()`,
unit type. Do đó, không có gì để trả về, điều này mẫu thuẫn với định nghĩa
của hàm và dẫn đến lỗi. Trong đầu ra này, Rust đưa ra một thông báo để có thể
giúp khắc phục vấn đề này: Rust đề xuất loại bỏ dấu chấm phẩy để khắc phục
lỗi.
