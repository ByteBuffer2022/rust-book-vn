## References và Borrowing

Vấn đề với tuple trong Listing 4-5 là chúng ta phải trả lại `String` cho hàm gọi vì vậy chúng ta vẫn có thể sử dụng `String` sau khi gọi `calculate_length`, vì `String` đã được chuyển đến `calculate_length`. Thay vào đó, chúng ta có thể cung cấp một tham chiếu (reference) đến giá trị của `String`.
Một *tham chiếu* (reference) giống như một con trỏ ở chỗ đó là một địa chỉ mà chúng ta có thể đi theo để truy cập vào dữ liệu được lưu trữ tại địa chỉ thuộc sở hữu của một số biến khác. Không giống như một con trỏ, một tham chiếu được đảm bảo trỏ đến một giá trị hợp lệ của một kiểu cụ thể. Đây là cách bạn sẽ xác định và sử dụng một hàm `calculate_length` có tham chiếu đến một đối tượng dưới dạng tham số thay vì dùng ownership để có quyền sở hữu giá trị:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

Đầu tiên, hãy lưu ý rằng tất cả tuple trong khai báo biến và giá trị trả về của hàm đã biến mất. Thứ hai, lưu ý rằng chúng tôi chuyển `&s1` vào `calculate_length` và, theo định nghĩa của nó, chúng tôi lấy `&String` thay vì `String`. Các ký hiệu & này đại diện cho các *tham chiếu* (references), và chúng cho phép bạn tham chiếu đến một số giá trị mà không cần lấy ownership của nó. Hình 4-5 mô tả khái niệm này.

<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />

<span class="caption">Figure 4-5: Một sơ đồ của `&String s` trỏ vào `String s1`</span>

> Lưu ý: Ngược lại với tham chiếu bằng cách sử dụng `&` is *dereferencing*, được thực hiện với toán tử dereference, `*`. Chúng ta sẽ thấy một số cách sử dụng của toán tử dereference trong Chương 8 và thảo luận chi tiết về dereferencing trong Chương 15.

Chúng ta hãy xem xét kỹ hơn lệnh gọi hàm tại đây:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

`&s1` cú pháp cho phép chúng ta tạo một tham chiếu đề cập đến giá trị của `s1` nhưng không sở hữu nó.Bởi vì nó không sở hữu nó, giá trị nó trỏ đến sẽ không bị drop khi tham chiếu ngừng được sử dụng.

Tương tự như vậy, chữ ký hàm (signature of the function) sử dụng uses `&` để chỉ ra rằng kiểu tham số
 `s` là một tham chiếu. Hãy xem một số chú thích để giải thích cho việc này:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

Phạm vi mà biến `s` có giá trị giống như phạm vi của bất kỳ thông số hàm nào, nhưng giá trị được trỏ đến bởi tham chiếu không bị drop khi `s` được dừng sử dụng vì `s` không có ownership. Khi các hàm có tham chiếu dưới dạng tham số thay vì giá trị thực, chúng ta sẽ không cần trả lại giá trị để trả lại ownership vì chúng ta chưa bao giờ có ownership.

Chúng tôi gọi hành động tạo tham chiếu là *borrowing*. Như trong cuộc sống thực, nếu một người sở hữu một thứ gì đó, bạn có thể mượn nó từ họ. Khi bạn làm xong, bạn phải trả lại nó. Bạn không sở hữu nó.

Vậy điều gì sẽ xảy ra nếu chúng ta cố gắng sửa đổi thứ mà chúng ta đang vay mượn (borrowing)? Hãy thử code trong Listing 4-6. Spoiler cảnh báo: nó không hoạt động!

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<span class="caption">Listing 4-6: Attempting to modify a borrowed value</span>

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

Cũng giống như các biến là bất biến theo mặc định, các tham chiếu cũng vậy. Chúng tôi không được phép sửa đổi điều gì đó mà chúng tôi có tham chiếu đến.

### Tham chiếu (References) có thể thay đổi

Chúng ta có thể sửa code từ Listing 4-6 để cho phép chúng ta sửa đổi một giá trị đã mượn chỉ với một vài chỉnh sửa nhỏ, bằng cách sử dụng *tham chiếu có thể thay đổi* (mutable reference):

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

Đầu tiên, chúng ta thay đổi s `s` thành `mut` để có thể thay đổi. TSau đó, chúng tôi tạo một tham chiếu có thể thay đổi với `&muts` nơi chúng ta gọi hàm `change`, và cập nhật chữ ký hàm (function signature) để chấp nhận một tham chiếu có thể thay đổi với `some_string: &mut String`. Điều này làm cho nó rất rõ ràng rằng hàm `change` sẽ thay đổi giá trị mà nó vay.

Tham chiếu có thể thay đổi có một hạn chế lớn: bạn chỉ có thể có một tham chiếu có thể thay đổi cho một phần dữ liệu cụ thể tại một thời điểm. Code này cố gắng tạo hai tham chiếu có thể thay đổi cho `s` sẽ không thành công:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Đây là lỗi:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

Lỗi này cho biết rằng code không hợp lệ vì chúng ta không thể mượn (borrow) `s` có thể thay đổi nhiều lần tại một thời điểm. Đầu tiên là tại s1 `r1` and must và nó kéo dài cho tới khi sử dụng `println!`, nhưng giữa chừng, chúng ta đã cố gắng tạo một tham chiếu có thể thay đổi khác tại `r2` mượn cùng một dữ liệu như `r1`.

Hạn chế ngăn nhiều tham chiếu có thể thay đổi đến cùng một dữ liệu cùng một lúc cho phép tạo ra đột biến nhưng theo cách rất được kiểm soát. Đó là điều mà những Rustaceans mới gặp khó khăn vì hầu hết các ngôn ngữ đều cho phép bạn thay đổi bất cứ khi nào bạn muốn. Lợi ích của việc hạn chế này là Rust có thể ngăn chặn hiện tượng data race tại thời điểm biên dịch. Một *data race* tương tự như một race condition và xảy ra khi ba hành vi này xảy ra:

* Hai hoặc nhiều con trỏ truy cập cùng một dữ liệu cùng một lúc.
* Ít nhất một trong các con trỏ đang được sử dụng để ghi vào dữ liệu.
* Không có cơ chế nào được sử dụng để đồng bộ hóa quyền truy cập vào dữ liệu.

Data races gây ra hành vi không xác định (undefined behavior) và có thể khó chẩn đoán và khắc phục khi bạn đang cố gắng theo dõi chúng trong runtime; Rust ngăn chặn vấn đề này bằng cách từ chối biên dịch code với data races!

Như mọi khi, chúng ta có thể sử dụng dấu ngoặc nhọn để tạo một phạm vi mới, cho phép nhiều tham chiếu có thể thay đổi, chỉ là những tham chiếu không được xảy ra *đồng thời*:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

Rust thực thi một quy tắc tương tự để kết hợp các tham chiếu có thể thay đổi và bất biến. Code này dẫn đến lỗi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

Chà! Chúng ta cũng không thể có một tham chiếu có thể thay đổi (mutable reference) trong khi chúng ta có một tham chiếu bất biến (immutable reference) với cùng một giá trị. Người dùng tham chiếu bất biến không mong đợi giá trị đột ngột thay đổi! Tuy nhiên, việc sử dụng nhiều tham chiếu bất biến thì được cho phép vì không ảnh hưởng đến việc đọc dữ liệu của bất kỳ ai khác.

Lưu ý rằng phạm vi của tham chiếu bắt đầu từ nơi nó tạo ra và tiếp tục cho đến lần cuối cùng tham chiếu đó được sử dụng. Ví dụ: code này vãn sẽ biên dịch vì nơi sử dụng cuối cùng của các tham chiếu bất biến tại `println!`, xảy ra trước khi tham chiếu có thể thay đổi được tạo ra:

```rust,edition2021
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

Phạm vi của các tham chiếu bất biến `r1` và `r2` kết thúc sau `println!` nơi chúng được sử dụng lần cuối, trước tham chiếu có thể thay đổi `r3` được tạo ra. Các phạm vi này không trùng lặp, vì vậy code này được phép sử dụng. Khả năng của trình biên dịch để thông báo rằng một tham chiếu không còn được sử dụng tại một điểm trước khi kết thúc phạm vi được gọi là *Non-Lexical Lifetimes* (viết tắt là NLL), và bạn có thể đọc thêm về nó trong [The Edition Guide][nll].

Mặc dù đôi khi lỗi borrowing có thể khiến bạn bực bội, hãy nhớ rằng đó là trình biên dịch Rust chỉ ra một lỗi tiềm ẩn sớm (tại tại thời điểm biên dịch thay vì tại runtime) và cho bạn thấy chính xác vấn đề nằm ở đâu. Sau đó, bạn không phải theo dõi lý do tại sao dữ liệu của bạn không giống như bạn nghĩ nữa.

### Tham chiếu treo (Dangling References)

Trong các ngôn ngữ có con trỏ, rất dễ tạo sai một *con trỏ treo* (dangling pointer)--một con trỏ tham chiếu đến một vị trí trong bộ nhớ có thể đã được cấp cho người khác--bằng cách giải phóng một số bộ nhớ trong khi vẫn bảo toàn con trỏ tới bộ nhớ đó. Ngược lại, trong Rust, trình biên dịch đảm bảo rằng các tham chiếu sẽ không bao giờ là tham chiếu treo (dangling references): nếu bạn có tham chiếu đến một số dữ liệu, trình biên dịch sẽ đảm bảo rằng dữ liệu sẽ không vượt ra khỏi phạm vi trước khi tham chiếu đến dữ liệu đó.

Hãy thử tạo một tham chiếu treo (dangling references) để xem cách Rust ngăn chặn chúng với lỗi biên dịch:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Here’s the error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

Thông báo lỗi này đề cập đến một tính năng mà chúng tôi chưa đề cập đến: lifetimes. Chúng ta sẽ thảo luận về lifetimes chi tiết trong Chương 10. Tuy nhiên, nếu bạn chưa nắm rõ về lifetimes thì thông báo vẫn có chỉ ra tại sao code này lại đang có vấn đề:

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from
(kiểu trả về của hàm này chứa giá trị được borrow, nhưng không có giá trị nào cho nó được borrow)
```


Chúng ta hãy xem xét kỹ hơn chính xác những gì đang xảy ra ở mỗi giai đoạn trong code:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

Bởi vì `s` được tạo ra bên trong `dangle`, khi code của `dangle` đã hoàn thành, `s` sẽ được phân bổ. Nhưng chúng ta đã cố gắng trả về một tham chiếu đến nó. Điều đó có nghĩa là tham chiếu này sẽ trỏ đến một `String` không còn giá trị. Điêu đó không tôt! Rust sẽ không để chúng ta làm điều này.

Giải pháp ở đây là trả về `String` trực tiếp:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

Điều này hoạt động mà không có bất kỳ vấn đề gì. Ownership đã được chuyển ra ngoài, và không có gì được phân bổ.


### Các quy tắc của References

Hãy tóm tắt lại những gì chúng ta đã thảo luận về  tham chiếu (references):

* Tại một thời điểm, bạn  chỉ có thể có một tham chiếu có thể thay đổi (mutable reference) và có thể có nhiều tham chiếu bất biến (immutable references).
* Tham chiếu phải luôn có giá trị.

Tiếp theo, chúng ta sẽ xem xét một loại tham chiếu khác: slices.

[nll]: https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html
