## Định nghĩa và khởi tạo Structs

Structs tương tự như tuples (trình bày tại phần ["The Tuple Type"][tuples]<!-- ignore -->), đó là chúng đều giữ các giá trị có mối liên hệ với nhau. Một điểm chung nữa đó là các thành phần bên trong struct và tuple có thể khác kiểu dữ liệu. Tuy nhiên, sự khác nhau cơ bản ở đây là các phần tử trong struct sẽ được đặt tên tương ứng cho từng trường, làm cho struct sẽ có ý nghĩa rõ ràng hơn so với tuple. Có thể gán tên đồng nghĩa với việc struct sẽ linh hoạt hơn tuple: ví dụ, bạn sẽ không phải dựa vào thứ tự của các phân tử để truy cập giá trị.

Để định nghĩa một struct, chúng ta sử dụng từ khóa `struct` kèm theo tên struct đó. Tên của struct nên đặt một cách gợi nhớ và có ý nghĩa. Sau đó, bên trong cặp ngoặc nhọn, ta sẽ định nghĩa tên và kiểu dữ liệu cho từng phần tử , hay "trường dữ liệu" (được gọi là *field*) trong struct đó. Ví dụ, mục 5-1 thể hiện một struct lưu trữ thông tin về một tài khoản của người dùng.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-01/src/main.rs:here}}
```

<span class="caption">Listing 5-1: Định nghĩa một struct có tên `User`</span>

Để sử dụng struct sau khi đã định nghĩa, chúng ta sẽ tạo ra một *instance* của struct đó bằng cách đặt giá trị cho từng trường. Một instance được tạo ra bắt đầu bằng tên struct, bên trong cặp ngoặc nhọn sẽ chứa các cặp `khóa: giá trị` (`key: value`), trong đó keys đại diện cho tên của từng trường và value thể hiện cho giá trị cần lưu của trưởng đó trong struct. Nói một cách khác, struct như một bản mẫu chung cho kiểu dữ liệu, còn instances sẽ hoàn thiện struct đó bằng cách *điền* các giá trị vào mẫu chung đó. Ví dụ, ta có thể khởi tạo một user cụ thể như sau.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-02/src/main.rs:here}}
```

<span class="caption">Listing 5-2: Tạo một instance "User"</span>

Để lấy giá trị của các trường trong struct, ta sử dụng dấu chấm. Nếu bạn muốn lấy ra địa chỉ email của user, có thể dùng `user1.email`. Nếu instance có thể thay đổi được (mutable), ta có thể thay đổi giá trị của bất kì trường nào mà mình muốn. Listing 5-3 cho ta thấy làm sao để thay đổi `email` trong instance `User`.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-03/src/main.rs:here}}
```

<span class="caption">Listing 5-3: Thay đổi email trong instance "User"</span>

Chú ý rằng instance này phải thay đổi được (mutable); Rust không cho phép chúng ta đánh dấu mutable cho các trường dữ liệu bên trong struct. Ta cũng có thể tạo ra một instance và đặt nó ở cuối của thân hàm (last expression) để ngầm định giá trị trả về cho hàm đó.

Listing 5-4 cho ta thấy một hàm `build_user` trả về  một `User` instance với email và username. Trường `active` có giá trị `true` và trường `sign_in_count` có giá trị 1.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-04/src/main.rs:here}}
```

<span class="caption">Listing 5-4: Hàm `build_user` trả về  `User` instance với email và username</span>

Đặt tên các tham số truyền vào của hàm trùng với tên của các trường trong struct sẽ giúp chương trình dễ đọc hơn, tuy nhiên việc này sẽ khiến ta cảm thấy hơi "tẻ nhạt". Việc này sẽ khiến bạn cảm thấy hơi khó chịu nếu gặp một struct phức tạp. May mắn thay, ta đã có một cách thuận tiện hơn!

<a id="using-the-field-init-shorthand-when-variables-and-fields-have-the-same-name"></a>
### Using the Field Init Shorthand

Nếu tham số truyền vào trùng tên với tên trường dữ liệu, ta có thể sử dụng cú pháp viết tắt (*field init shorthand*) để viết lại như sau.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-05/src/main.rs:here}}
```

<span class="caption">Listing 5-5: Hàm `build_user` sử dụng cách khởi tạo mới bởi vì `email` và `username` ở tham số truyền vào trùng tên với tên trường dữ liệu struct</span>

Ở đây, chúng ta tạo mới một instance của User struct, có trường `email`. Ta cũng muốn gán giá trị của trường `email` bởi giá trị của tham số `email` truyền vào hàm `build_user`. Do chúng đều có cùng tên, nên ta chỉ cần viết `email` thay vì `email:email`.

### Tạo Instances từ Instances khác với Struct Update Syntax

Việc tạo ra một instance từ một instance khác gần giống nhau là một việc làm rất phổ biến trong lập trình. Với Rust, bạn có thể sử dụng *struct update syntax*.

Đầu tiên, Listing 5-6 sẽ cho ta thấy cách tạo User instance một cách thông thường, không sử dụng *update syntax*. Ta sẽ gán giá trị mới cho trường `email`, còn các trường còn lại sẽ giữ nguyên như ở `user1` trong Listing 5-2.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-06/src/main.rs:here}}
```

<span class="caption">Listing 5-6: Tạo mới `User` instance sử dụng lại một vài giá trị của `user1`</span>

Sử dụng struct syntax update, ta có thể có kết quả tương tự nhưng với ít dòng code hơn, như ở Listing 5-7. Cú pháp `..` chú thích rằng các trường còn lại không được khai báo một cách tường minh sẽ có cùng một giá trị với instance cho trước.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-07/src/main.rs:here}}
```

<span class="caption">Listing 5-7: Sử dụng struct update syntax để gán giá trị cho
`email` của `User` instance nhưng sử dụng các giá trị còn lại từ `user1`</span>

Đoạn code ở Listing 5-7 tạo ra instance `user2` khác `email` nhưng lại giống `username`, `active` và `sign_in_count` với `user1`. Cú pháp `..user1` phải được đặt ở cuối để thể hiện rằng các giá trị còn lại sẽ phải bằng với các trường tương ứng của `user1`.

Chú ý rằng struct update syntax sử dụng `=` như một phép gán; điều này có đươc là bởi vì nó đã chuyển quyền sở hữu dữ liệu (move), như chúng ta đã biết trong phần [“Ways Variables and Data Interact: Move”][move]<!-- ignore -->. Ở trong ví dụ này, ta không thể sử dung `user1` sau khi đã tạo `user2` vì `String` trong trường `username` của `user1` đã chuyển quyền sở hữu (move) vào trong `user2`. Do đó, ta chỉ có thể sử dụng `active` và `sign_in_count` từ `user1`. Kiểu dữ liệu của `active` và `sign_in_count` là những kiểu đã implement `Copy` trait, vì vậy các hành vi (behavior) mà ta đã bàn trong phần [“Stack-Only Data: Copy”][copy]<!-- ignore --> có thể dùng để giải thích.

### Sử dụng Tuple Structs

Rust cũng hỗ trợ tạo structs trông giống như tuples, được gọi là *tuple struct*. Tuple structs không có tên của các trường dữ liệu trong struct đó; chúng chỉ có các kiểu dữ liệu. Tuple structs rất hữu dụng khi bạn muốn tạo cho tuple đó một cái tên và khiến cho chúng khác với những tuples còn lại.

Để định nghĩa một tuple struct, hãy bắt đầu với từ khóa `struct` và tên của struct, sau đó đến kiểu dữ liệu trong tuple. Ví dụ, hãy tạo hai tuple structs có tên là `Color` và `Point`:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-01-tuple-structs/src/main.rs}}
```

Chú ý rằng `black` và `origin` có kiểu dữ liệu khác nhau. Mỗi struct bạn định nghĩa đều có kiểu riêng của nó, kể cả khi các trường trong struct đó có chung một kiểu dữ liệu. Ví dụ, một hàm truyền tham số kiểu `Color` không thể nhận tham số truyền vào kiểu `Point`, kể cả khi chúng đều có chung kiểu dữ liệu của từng trường (là `i32`). Mặt khác, instance của  tuple struct có thể sự dụng như một tuple: bạn có thể tách chúng thành nhiều phần bằng cách sử dụng `.` để truy cập đến từng trường.
> ### Ownership of Struct Data
>
> In the User struct definition in Listing 5-1, we used the owned String
> type rather than the &str string slice type. This is a deliberate choice
> because we want each instance of this struct to own all of its data and for
> that data to be valid for as long as the entire struct is valid.
>
> It’s also possible for structs to store references to data owned by something
> else, but to do so requires the use of *lifetimes*, a Rust feature that we’ll
> discuss in Chapter 10. Lifetimes ensure that the data referenced by a struct
> is valid for as long as the struct is. Let’s say you try to store a reference
> in a struct without specifying lifetimes, like the following; this won’t work:
>
> <span class="filename">Filename: src/main.rs</span>
>
> <!-- CAN'T EXTRACT SEE https://github.com/rust-lang/mdBook/issues/1127 -->
>
> 
> ```rust,ignore,does_not_compile
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
>
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> The compiler will complain that it needs lifetime specifiers:
>
> 
> ```console
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
>
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
>
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> In Chapter 10, we’ll discuss how to fix these errors so you can store
> references in structs, but for now, we’ll fix errors like these using owned
> types like String instead of references like &str.

<!-- manual-regeneration
for the error above
after running update-rustc.sh:
pbcopy < listings/ch05-using-structs-to-structure-related-data/no-listing-02-reference-in-struct/output.txt
paste above
add >  before every line -->

[tuples]: ch03-02-data-types.html#the-tuple-type
[move]: ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move
[copy]: ch04-01-what-is-ownership.html#stack-only-data-copy