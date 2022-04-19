## Một vài ví dụ khi sử dụng Structs

Để hiểu khi nào ta nên dùng structs, hãy cùng đến với một ví dụ về chương trình tình toán diện tích hình chữ nhật. Ta sẽ bắt đầu với việc sử dụng các biến đơn, sau đó sẽ thay thế bằng struct để so sánh.

Đầu tiền, tạo một binary project với Cargo đặt tên là *rectangles*, có đầu vào là chiều dài và chiều rộng của một hình chữ nhật cụ thể và sau đó tính toán ra diện tích của hình chữ nhật đó. Listing 5-8 cho ta thấy một đoạn code mẫu cho chương trình trên.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

<span class="caption">Listing 5-8: Tính diện tích của hình chữ nhật với chiều dài và chiều rộng cho trước</span>

Chạy chương trình với `cargo run`:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Ta có thể viết code rõ ràng và dễ đọc hơn.

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

Hàm `area` tính toán diện tích của một hình chữ nhật, nhưng ta lại sử dụng tới 2 tham số, như vậy chương trình sẽ không rõ ràng và ta không thấy được mối quan hệ giữa các tham số. Để cải thiện điều này, ta có thể nhóm 2 tham số này lại với nhau. Ở phần [“The Tuple Type”][the-tuple-type]<!-- ignore --> của chương 3, việc sử dụng tuple sẽ giúp ích trong trường hợp này.
### Chỉnh sửa code với Tuples

Listing 5-9 là một cách làm khác sử dụng tuples.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

<span class="caption">Listing 5-9: Nhóm chiều dài và chiều rộng của hình chữ nhật vào trong tuple</span>

Chương trình này cũng có điểm tốt và chưa tốt. Điểm tốt ở chỗ các tham số đã được nhóm lại và trông có cấu trúc hơn, hàm lúc này chỉ cần nhận một tham số. Tuy nhiên, điểm trừ là tuple sẽ không rõ ràng tên các tham số, ta không biết đâu là chiều dài và đâu là chiều rộng.

Việc lẫn lộn giữa chiều dài và chiều rộng sẽ không phải vấn đề trong trường hợp tính diện tích, nhưng giả sử yêu cầu bài toán là vẽ hình chữ nhật, lúc này sẽ có vấn đề xảy ra! Bạn có thể nhớ trong đầu rằng phần tử `0` là chiều dài và phần tử `1` là chiều rộng. Tuy nhiên sẽ gây khó khăn cho người khác nếu họ sử dụng code của bạn.

### Chỉnh sửa code dùng struct: cho chương trình thêm rõ nghĩa hơn

Ta sử dụng structs để gán nhãn cho dữ liệu, làm chúng dễ đọc hơn. Sử dụng struct cho bài toán trên như sau.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

<span class="caption">Listing 5-10: Định nghĩa một `Rectangle` struct</span>

Ta sẽ tạo ra một struct có tên `Rectangle`. Bên trong đó sẽ định nghĩa 2 trường `width` và `height` đều có kiểu `u32`. Trong hàm `main`, một instance sẽ được tạo ra với chiều dài bằng 50 và chiều rộng bằng 30.

Hàm `area` bây giờ chỉ có một tham số duy nhất có tên `rectangle`. Như đã đề cập trong chương 4, ta nên mượn (borrow) struct hơn là lấy quyền sở hữu của nó, `main` lúc này sẽ giữ lại quyền sở hữu của `rect1`.

Hàm `area` truy cập vào `widht` và `height` của `Rectangle` instance. 
### Thêm các chức năng hữu dụng khác với Derived Traits

Sẽ rất tuyệt với nếu ta có thể in ra màn hình cả một struct trong khi debug. Listing 5-11 sử dụng [`println!` macro][println]<!-- ignore --> mà chúng ta thường dùng trong các chương trước. Tuy nhiên, sẽ xảy ra lỗi.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

<span class="caption">Listing 5-11: In `Rectangle` instance ra console</span>

Khi biên dịch đoạn code này, lỗi sẽ xảy ra với message:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

Macro `println!` có thể dùng cho rất nhiều định dạng, mặc định, cặp ngoặc nhọn `{}` sẽ chỉ dẫn cho `println!` sử dụng một loại định dạng có tên là `Display`. Các kiểu dữ liệu nguyên thủy (primitive types) đều implement `Display`. Nhưng với structs, mọi thứ sẽ khác, do có rất nhiều cách để thể hiện một struct: có dùng dấu phẩy hay không, có muốn in dấu ngoặc nhọn hay không, ... Vì vậy mặc định structs sẽ không implement `Display`.

Nếu bạn để ý lỗi in ra màn hình, bạn có thể tìm được giải pháp:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

Hãy thử cách này! Macro `println!` sẽ sử dụng `{:?}`. Cách này giúp cho macro `println!` sử dụng kiểu định dạng có tên là `Debug`. `Debug` cho phép ta in một struct ra console, rất thuận tiền trong quá trình debug code.

Biên dịch chương trình với cách mới. Ta vẫn sẽ gặp lỗi:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

Nhưng một lần nữa, compiler lại cho ta một gợi ý:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

Ta phải khai báo tường minh một outer attribute `#[derive(Debug)]` ngay trên phần định nghĩa struct, như Listing 5-12.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

<span class="caption">Listing 5-12: Thêm attribute để sử dụng `Debug` trait và in ra màn hình `Rectangle` instance</span>

Chạy chương trình, ta sẽ nhận được kết quả:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Tuyệt! Output này chưa dễ nhìn lắm, tuy nhiên nó đã đáp ứng được việc thể hiện ra tất cả các trường dữ liệu. Ngoài ra, có thể sử dụng `{:#?}` để in ra dễ nhìn hơn.

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

Another way to print out a value using the `Debug` format is to use the [`dbg!`
macro][dbg]<!-- ignore -->, which takes ownership of an expression, prints the
file and line number of where that `dbg!` macro call occurs in your code along
with the resulting value of that expression, and returns ownership of the value.

> Note: Calling the `dbg!` macro prints to the standard error console stream
> (`stderr`), as opposed to `println!` which prints to the standard output
> console stream (`stdout`). We’ll talk more about `stderr` and `stdout` in the
> “[“Writing Error Messages to Standard Error Instead of Standard
> Output” section in Chapter 12][err]<!-- ignore -->.

Here’s an example where we’re interested in the value that gets assigned to the
`width` field, as well as the value of the whole struct in `rect1`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

We can put `dbg!` around the expression `30 * scale` and, because `dbg!`
returns ownership of the expression’s value, the `width` field will get the
same value as if we didn’t have the `dbg!` call there. We don’t want `dbg!` to
take ownership of `rect1`, so we use a reference to `rect1` in the next call.
Here’s what the output of this example looks like:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

We can see the first bit of output came from *src/main.rs* line 10, where we’re
debugging the expression `30 * scale`, and its resulting value is 60 (the
`Debug` formatting implemented for integers is to print only their value). The
`dbg!` call on line 14 of *src/main.rs* outputs the value of `&rect1`, which is
the `Rectangle` struct. This output uses the pretty `Debug` formatting of the
`Rectangle` type. The `dbg!` macro can be really helpful when you’re trying to
figure out what your code is doing!

In addition to the `Debug` trait, Rust has provided a number of traits for us
to use with the `derive` attribute that can add useful behavior to our custom
types. Those traits and their behaviors are listed in [Appendix C][app-c]<!--
ignore -->. We’ll cover how to implement these traits with custom behavior as
well as how to create your own traits in Chapter 10. There are also many
attributes other than `derive`; for more information, see [the “Attributes”
section of the Rust Reference][attributes].

Our `area` function is very specific: it only computes the area of rectangles.
It would be helpful to tie this behavior more closely to our `Rectangle`
struct, because it won’t work with any other type. Let’s look at how we can
continue to refactor this code by turning the `area` function into an `area`
*method* defined on our `Rectangle` type.

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: ../std/macro.println.html
[dbg]: ../std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: ../reference/attributes.html
