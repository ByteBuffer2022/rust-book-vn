## Kiểu Slice

*Slices* cho phép bạn tham chiếu một chuỗi các phần tử liền nhau trong một tập
hợp thay vì toàn bộ tập hợp. Một slice là một loại tham chiếu, vì vậy nó không
có ownership.

Có một vấn đề nhỏ trong lập trình: viết một hàm nhận một chuỗi (String) và trả về từ
đầu tiên mà nó tìm thấy trong chuỗi đó. Nếu hàm không tìm thấy khoảng trắng
trong chuỗi, thì toàn bộ chuỗi phải là một từ, do đó, toàn bộ chuỗi phải được trả
về.

Hãy cùng tìm hiểu về cách chúng ta viết chữ ký của hàm (signature of function) mà không
cần sử dụng các slice, để hiểu vấn đề mà các slice sẽ giải quyết:

```rust,ignore
fn first_word(s: &String) -> ?
```

Hàm `first_word` có một `&String` là một tham số. Chúng ta không muốn sử dụng ownership, vì vậy điều này không sao cả. Nhưng chúng ta nên return lại cái gì? Chúng ta thực sự không có cách nào để nói về *một phần* của một string. Tuy nhiên, chúng ta có thể trả về index ở cuối từ, nếu nó là một khoảng trắng. Hẫy thử điều đó trong Listing 4-7.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<span class="caption">Listing 4-7: Hàm `first_word` trả về một giá trị index dạng byte vào tham số `String`</span>

Bởi vì chúng ta cần đi qua phần tử của `String` theo từng phần tử và kiểm tra xem một giá trị có phải là khoảng trắng hay không, chúng ta sẽ chuyển đổi `String` thành một mảng byte bằng method `as_bytes`:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

Tiếp theo, chúng ta tạo một trình lặp trên mảng byte bằng method `iter`:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

Chúng ta sẽ thảo luận chi tiết hơn về trình lặp trong [Chương 13][ch13]<!-- ignore -->. Bây giờ, hãy biết rằng `iter` là một method that trả về từng phần tử trong một tập hợp và `enumerate` sẽ bao bọc kết quả của `iter` và trả về từng phần tử dưới dạng một phần của một tuple. Phần tử đầu tiên của tuple được
trả về từ `enumerate` là index,và phần tử thứ hai là một tham chiếu đến phần tử. Điều này thuận tiện hơn một chút so với việc tự tính toán index.

Bởi vì method `enumerate` một tuple,chúng ta có thể sử dụng các patterns để cấu trúc bộ tuple đó. Chúng ta sẽ thảo luận thêm về patterns trong [Chương 6][ch6]<!-- ignore -->. Trong vòng lặp `for`, chúng ta xác định một pattern có `i` là index trong tuple và `&item` cho single byte trong tuple. Bởi vì chúng ta nhận được một tham chiếu đến phần tử từ `.iter().enumerate()` nên chúng ta sử dụng `&` trong pattern.

Trong vòng lặp `for`, chúng ta tìm kiếm byte đại diện cho khoảng trắng bằng cách sử dụng cú pháp ký tự byte. Nếu chúng ta tìm thấy một khoảng trắng, chúng ta sẽ trả về vị trí của nó. Nếu không, chúng ta trả về độ dài của chuỗi bằng cách sử dụng `s.len()`:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

Bây giờ chúng ta có một cách để tìm ra index của cuối từ đầu tiên trong chuỗi, nhưng có một vấn đề. Chúng ta đang trả lại một `usize` của riêng nó, nhưng nó chỉ là một con số có ý nghĩa trong ngữ cảnh của `&String`. Nói cách khác, bởi vì nó là một giá trị riêng biệt với `String`, không có gì đảm bảo rằng nó sẽ vẫn có giá trị trong tương lai. Xem xét chương trình trong Listing 4-8 sử dụng hàm `first_word` từ Listing 4-7.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

<span class="caption">Listing 4-8: Lưu trữ kết quả trả về khi gọi hàm `first_word` function và sau đó thay đổi nội dung của `String`</span>

Chương trình này biên dịch mà không có bất kỳ lỗi nào và cũng như vậy nếu chúng ta sử dụng `word` sau khi gọi `s.clear()`. Vì `word` hoàn toàn không kết nối với trạng thái của `s`, `word` vẫn chứa giá trị `5`. Nếu chúng ta sử dụng giá trị  `5` với biến `s` để trích xuất từ ​​đầu tiên, nhưng đây sẽ là một lỗi vì nội dung của `s` đã thay đổi kể từ khi chúng ta lưu `5` trong `word`.

Việc phải lo lắng về việc index trong `word` không đồng bộ với dữ liệu trong `s` thật tẻ nhạt và dễ xảy ra lỗi! Việc quản lý các chỉ số này thậm chí còn khó khăn hơn nếu chúng ta viết một hàm `second_word`. Chữ ký hàm của nó sẽ phải trông như thế này:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

Bây giờ chúng ta đang theo dõi index nơi bắt đầu và nơi kết thúc, và chúng ta có nhiều giá trị hơn được tính toán từ dữ liệu ở một trạng thái cụ thể nhưng hoàn toàn không bị ràng buộc với trạng thái đó. Chúng tôi có tới ba biến không liên quan đang trôi nổi cần được đồng bộ hóa. 

May mắn thay, Rust có một giải pháp cho vấn đề này: string slices.

### String Slices

Một *string slice* là một tham chiếu đến một phần của một `String`, và nó trông như thế này:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

Thay vì tham chiếu đến toàn bộ `String`, `hello` là một tham chiếu đến một phần của `String`, được xác định trong `[0..5]` bit. Chúng ta tạo các lát cắt (slices) bằng cách sử dụng một phạm vi trong dấu ngoặc bằng cách chỉ định `[starting_index..ending_index]`, `starting_index` là vị trí đầu tiên trong slice và `ending_index` nhiều hơn một so với vị trí cuối cùng trong slice. Bên trong, cấu trúc dữ liệu slice lưu trữ vị trí bắt đầu và độ dài của slice, tương ứng với `ending_index` trừ đi `starting_index`. Vì vậy, trong trường hợp của `let world = &s[6..11];`, `world` là một slice chứa một con trỏ đến byte tại index 6 của `s` với giá trị độ dài là 5.

Figure 4-6 cho thấy điều này trong một sơ đồ.

<img alt="world containing a pointer to the byte at index 6 of String s and a length 5" src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 4-6: String slice tham chiếu đến một phần của một `String`</span>

Với `..` là cú pháp phạm vi của Rust, nếu bạn muốn bắt đầu từ index 0, bạn có thể viết theo 2 cách sau, chúng tương đương nhau:  

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

Tương tự, nếu slice của bạn bao gồm byte cuối cùng của `String`, bạn có thể bỏ *len* ở sau cùng. 2 cách viết dưới đây tương đương nhau:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

Bạn cũng có thể bỏ cả hai giá trị để lấy một slice của toàn bộ chuỗi:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> Lưu ý: chỉ số phạm vi của String slice phải nằm trong ranh giới ký tự UTF-8 hợp lệ. Nếu bạn cố gắng tạo một string slice ở giữa một ký tự nhiều byte, chương trình của bạn sẽ thoát ra với một lỗi. Với mục đích giới thiệu string slices, chúng tôi chỉ giả định ASCII trong phần này; một cuộc thảo luận kỹ lưỡng hơn về việc xử lý UTF-8 có trong phần [“Storing UTF-8 Encoded Text with Strings”][strings] ở chương 8.

Với tất cả thông tin này, chúng ta hãy viết lại `first_word` để return một slice. Kiểu biểu thị“string slice” được viết là `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

Chúng ta nhận được index cho cuối từ theo cách giống như chúng tôi đã làm trong Listing 4-7, bằng cách tìm kiếm sự xuất hiện đầu tiên của một khoảng trắng. Khi chúng tôi tìm thấy một khoảng trắng, chúng ta trả về một string slice sử dụng nơi bắt đầu của string và index của khoảng trắng làm chỉ số bắt đầu và kết thúc.

Bây giờ khi chúng ta gọi `first_word`, chúng ta nhận lại một giá trị duy nhất được liên kết với dữ liệu cơ bản. Giá trị được tạo thành từ tham chiếu đến điểm bắt đầu của slice và số phần tử trong slice.

Trả lại một  slice cũng sẽ hoạt động cho một hàm `second_word`:

```rust,ignore
fn second_word(s: &String) -> &str {
```

Giờ đây, chúng tôi có một API đơn giản và khó bị xáo trộn hơn nhiều, vì trình biên dịch sẽ đảm bảo các tham chiếu vào `String` vẫn hợp lệ. Hãy nhớ lỗi trong chương trình trong Listing 4-8, khi chúng ta nhận được index của cuối từ đầu tiên nhưng sau đó xóa chuỗi nên index của chúng ta không hợp lệ? Code đó không chính xác về mặt logic nhưng không hiển thị bất kỳ lỗi nào ngay lập tức. Các vấn đề sẽ xuất hiện sau đó nếu chúng tôi tiếp tục cố gắng sử dụng index của từ đầu tiên với một chuỗi trống. Slices khiến lỗi này không thể xảy ra và cho chúng ta biết rằng chúng ta gặp sự cố với code của mình sớm hơn nhiều. Sử dụng phiên bản slice của `first_word` wsẽ gây ra lỗi biên dịch:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

Here’s the compiler error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

Hãy nhớ lại các quy tắc borrowing rằng nếu chúng ta có một tham chiếu bất biến đến một cái gì đó, chúng ta cũng không thể lấy một tham chiếu có thể thay đổi được. Vì `clear` cần phải cắt bớt `String`, nó cần nhận được một tham chiếu có thể thay đổi. `println!` sau lệnh gọi `clear`  sử dụng tham chiếu trong `word`, vì vậy tham chiếu bất biến phải vẫn hoạt động tại thời điểm đó. Rust không cho phép tham chiếu có thể thay đổi trong `clear` và tham chiếu bất biến trong `word` tồn tại cùng một lúc, và biên dịch không thành công. Rust không chỉ làm cho API của chúng ta dễ sử dụng hơn mà còn loại bỏ toàn bộ lớp lỗi tại thời điểm biên dịch!

#### String Literals là Slices

Nhớ lại rằng chúng ta đã nói về string literals .được lưu trữ bên trong tệp nhị phân. Bây giờ chúng ta biết về slices, chúng ta có thể hiểu đúng về string literals:

```rust
let s = "Hello, world!";
```

Kiểu của `s` ở đây là `&str`: nó là một slice chỉ đến điểm cụ thể  của tệp nhị phân. Đây cũng là lý do tại sao string literals là bất biến; `&str` là một tham chiếu bất biến.

#### String Slices như là các tham số (String Slices as Parameters)

Biết rằng bạn có thể lấy slices trong tập hợp các chữ và giá trị `String` dẫn chúng ta đến một cải tiến nữa trên `first_word`, và đó là signature của nó:

```rust,ignore
fn first_word(s: &String) -> &str {
```

Một Rustacean có kinh nghiệm hơn sẽ viết signature trong Listing 4-9 vì nó cho phép chúng ta sử dụng cùng một hàm trên cả giá trị `&String` và giá trị `&str`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<span class="caption">Listing 4-9: Cải tiến hàm `first_word` bằng cách sử dụng một string slice cho kiểu của tham số `s`</span>

Nếu chúng ta có một string slice, chúng ta có thể  truyền vào trực tiếp. Nếu chúng ta có một `String`, chúng ta có thể truyền một slice của `String` hoặc một tham chiếu đến `String`. Sự linh hoạt này tận dụng lợi thế của *deref coercions*, một tính năng chúng ta sẽ đề cập trong phần [“Implicit Deref Coercions with Functions and Methods”][deref-coercions]<!--ignore--> của Chương 15. Việc xác định một hàm để lấy một string slice thay vì tham chiếu đến một `String` làm cho API của chúng ta trở nên tổng quát và hữu ích hơn mà không làm mất bất kỳ chức năng nào:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

### Kiểu Slices khác (Other Slices)

String slices, như bạn có thể tưởng tượng, dành riêng cho chuỗi. Nhưng cũng có một kiểu slice chung chung hơn. Hãy xem xét mảng này:

```rust
let a = [1, 2, 3, 4, 5];
```

Cũng giống như chúng ta có thể muốn tham chiếu đến một phần của chuỗi, chúng ta có thể muốn tham chiếu đến một phần của mảng. Chúng tôi sẽ làm như thế này:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

Slice này có kiểu `&[i32]`. Nó hoạt động theo cách tương tự như string slices, bằng cách lưu trữ một tham chiếu đến phần tử đầu tiên và độ dài. Bạn sẽ sử dụng loại slice này cho tất cả các loại tập hợp khác. Chúng ta sẽ thảo luận chi tiết về các tập hợp này khi chúng ta nói về vectơ trong Chương 8.

## Summary

Các khái niệm về ownership, borrowing, và slices đảm bảo an toàn cho bộ nhớ trong các chương trình Rust tại thời điểm biên dịch. Ngôn ngữ Rust cho phép bạn kiểm soát việc sử dụng bộ nhớ của mình giống như các ngôn ngữ lập trình hệ thống khác, nhưng có tự động xóa dữ liệu đó khi owner vượt ra khỏi scope có nghĩa là bạn không phải viết và gỡ lỗi thêm code để có được quyền kiểm soát này.

Ownership aảnh hưởng đến cách hoạt động của nhiều phần khác của Rust, vì vậy, chúng ta sẽ nói thêm về những khái niệm này trong suốt phần còn lại của cuốn sách. Hãy chuyển sang Chương 5 và xem xét việc nhóm các phần dữ liệu lại với nhau trong `struct`.

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
