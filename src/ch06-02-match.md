<a id="the-match-control-flow-operator"></a>
## The `match` Control Flow Construct

Rust có một cấu trúc luồng điều khiển cực kỳ mạnh mẽ được gọi là `match` cho 
phép bạn so sánh một giá trị với một loạt các mẫu và sau đó thực thi mã dựa 
trên mẫu nào phù hợp. Các mẫu có thể được tạo thành từ các giá trị chữ, 
tên biến, ký tự đại diện và nhiều thứ khác; Chương 18 bao gồm tất cả các 
loại mẫu khác nhau và những gì chúng làm. Sức mạnh của `match` đxuất phát 
từ sự biểu đạt của các mẫu và thực tế là trình biên dịch xác nhận rằng tất 
cả các trường hợp có thể xảy ra đều được xử lý.

Hãy nghĩ về một biểu thức `match` như là giống như một cỗ máy phân loại tiền xu:
đồng xu trượt xuống một đường ray với các lỗ có kích thước khác nhau dọc theo nó,
và mỗi đồng xu rơi qua lỗ đầu tiên mà nó gặp phải mà nó vừa vào.
Theo cách tương tự, các giá trị đi qua từng mẫu trong một `match`,
và ở mẫu đầu tiên, giá trị "phù hợp",
giá trị rơi vào khối mã liên kết sẽ được sử dụng trong quá trình thực thi
Nói về tiền xu, hãy sử dụng chúng làm ví dụ bằng cách sử dụng `match`!
Chúng ta có thể viết một hàm lấy một đồng xu Hoa Kỳ không xác định và,
theo cách tương tự như máy đếm, xác định đó là đồng xu nào và trả về 
giá trị của nó bằng xu, như được hiển thị ở đây trong Listing 6-3.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<span class="caption">Listing 6-3: Một enum và một biểu thức so khớp có các biến thể của enum là các mẫu của nó
</span>

Hãy chia nhỏ `match` trong hàm `value_in_cents`. Đầu tiên, chúng ta liệt kê  từ khoá `match` 
là một biểu thức, trong trường hợp này là `coin`. Điều này có vẻ rất giống với một biểu thức 
được sử dụng với `if`, nhưng có một sự khác biệt lớn: với `if`,
biểu thức cần trả về giá trị Boolean, nhưng ở đây, nó có thể trả về bất kỳ loại nào. Kiểu của `coin` trong ví dụ này là một enum `Coin`
mà chúng ta đã định nghĩa ở dòng đầu tiên.

Tiếp theo là các nhánh `match`. Một nhánh gồm 2 phần: mẫu và code. Nhánh đầu tiên ở đây có một mẫu là giá trị `Coin::Penny` và sau đó toán tử `=>`
phân tách mẫu và code để chạy. Code trong trường hợp này nhận giá trị là `1`. Mỗi nhánh được ngăn cách với nhánh tiếp theo bằng dấu phẩy.

Khi biểu thức `match` thực thi, nó so sánh giá trị kết quả với mẫu của mỗi nhánh, theo thứ tự. 
Nếu một mẫu khớp với giá trị, code với mẫu đó sẽ được thực thi. 
Nếu mẫu đó không khớp với giá trị, thì việc thực thi sẽ tiếp tục cho nhánh tiếp theo,
giống như trong một máy phân loại tiền xu.
Chúng ta có thể có nhiều nhánh như chúng ta cần: trong Listing 6-3, `match` của chúng ta có 4 nhánh.

Code được liên kết với mỗi nhánh là một biểu thức, và giá trị kết quả của biểu thức trong nhánh phù hợp là giá trị được trả về cho toàn bộ biểu thức `match`.

Chúng ta thường không sử dụng dấu ngoặc nhọn nếu mã nhánh đối sánh ngắn, như là trong  Listing 6-3 
trong đó mỗi nhánh chỉ trả về một giá trị. Nếu bạn muốn chạy nhiều dòng mã trong một nhánh, bạn phải sử dụng dấu ngoặc nhọn. 
Ví dụ, mã sau in “Lucky penny!” mỗi khi phương thức được gọi tới một `Coin::Penny`, nhưng vẫn trả về giá trị cuối cùng của block, `1`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

### Các mẫu ràng buộc với giá trị

Một tính năng hữu ích khác của các nhánh match là chúng có thể liên kết với các phần của giá trị khớp với mẫu. 
Đây là cách chúng tôi có thể trích xuất các giá trị từ các trường hợp của enum.

Như ví dụ dưới, hãy thay đổi một trong các biến thể enum của chúng tôi để giữ dữ liệu bên trong nó.
Từ năm 1999 đến năm 2008, Hoa Kỳ đúc đồng 25 xu với các thiết kế khác nhau cho mỗi bên trong số 50 tiểu bang.
Không có đồng xu nào khác có thiết kế trạng thái, vì vậy chỉ các đồng 25 xu có giá trị bổ sung này. Chúng ta có thể thêm thông tin này vào `enum` 
bằng cách thay đổi trường hợp `Quarter` có chứa 1 giá trị  `UsState` được lưu trữ bên trong nó, mà chúng ta đã làm ở đây trong Listing 6-4.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<span class="caption">Listing 6-4: Một enum `Coin` trong đó trường hợp `Quarter` chứa 1 giá trị `UsState`</span>

Hãy tưởng tượng rằng một người bạn đang cố gắng thu thập tất cả 50 đồng 25 xu của các tiểu bang. Trong khi chúng ta phân loại tiền lẻ của mình theo loại tiền xu, 
chúng tôi cũng sẽ gọi ra tên của tiểu bang được liên kết với mỗi đồng 25 xu vì vậy nếu đó là một trong những người bạn của chúng tôi không có, 
họ có thể thêm nó vào bộ sưu tập của họ.

Trong biểu thức match cho code này, chúng tôi thêm một biến được gọi là `state` vào mẫu phù hợp với các giá trị của trường hợp `Coin::Quarter`. Khi một 
`Coin::Quarter` khớp, biến `state` sẽ liên kết với giá trị của trạng thái của đồng 25 xu đó. Sau đó, chúng ta có thể sử dụng `state` trong code cho nhánh này, giống như bên dưới:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

Nếu chúng ta gọi `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin` sẽ là `Coin::Quarter(UsState::Alaska)`. 
Khi chúng tôi so sánh giá trị đó với từng nhánh, không ai trong số chúng phù hợp cho đến khi chúng tôi đạt được `Coin::Quarter(state)`.Tại thời điểm đó, ràng buộc đối với `state` sẽ là giá trị `UsState::Alaska`. Chúng tôi có thể sử dụng ràng buộc đó trong câu lệnh `println!`, do đó nhận được giá trị trạng thái bên trong của giá trị `Coin` cho `Quarter`.

### Matching với `Option<T>`

Trong phần trước, chúng ta muốn có được bên trong giá trị `T` của `Some`trường hợp khi sử dụng `Option<T>`; 
chúng ta cũng có thể xử lý `Option<T>` sử dung `match` như là chúng ta đã làm với `Coin`! Thay vì so sánh tiền xu, chúng tôi sẽ so sánh các trường hợp của
 `Option<T>`, nhưng cách thức hoạt động của biểu thức `match` vẫn giống nhau.

Giả sử chúng tôi muốn viết một hàm sử dụng `Option<i32>`, nếu có một giá trị bên trong, hãy thêm 1 vào giá trị đó. 
Nếu không có giá trị bên trong, hàm sẽ trả về giá trị `None` và không cố gắng thực hiện bất kỳ hoạt động nào.

Hàm này rất dễ viết, nhờ `match`, và sẽ trông như Listing 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: Một hàm sử dụng biểu thức `match`h trên `Option<i32>`</span>

Chúng ta hãy kiểm tra việc thực hiện đầu tiên của `plus_one` chi tiết hơn. Khi chúng ta gọi
`plus_one(five)`, biến `x` trong thân hàm `plus_one` sẽ có giá trị
 `Some(5)`. Sau đó, chúng tôi so sánh điều đó với từng nhánh `match`.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Giá trị `Some(5)` không khớp với mẫu `None`,vì vậy chúng ta tiếp tục đến nhánh tiếp theo.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

`Some(5)` có khớp với `Some(i)`? Tại sao lại có nó! Chúng ta có cùng một trường hợp. 
 `i` liên kết với giá trị chứa trong `Some`, vì vậy `i` nhận giá trị `5`. 
 Sau đó, đoạn code trong nhánh `match` được thực thi, vì vậy chúng ta cộng thêm 1 vào giá trị của `i` và 
tạo ra một giá trị mới `Some` với tổng là `6`.

Bây giờ chúng ta hãy xem xét lời gọi hàm thứ hai của `plus_one` trong Listing 6-5, trong đó `x` là
`None`. Chúng ta nhập `match` và so sánh giá trị ở nhánh đầu tiên.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

Nó phù hợp! Không có giá trị nào để thêm vào, vì vậy chương trình dừng và trả về giá trị 
`None` ở phía bên phải của `=>`. Bởi vì nhánh đầu tiên khớp với nhau, nên không có nhánh nào khác được so sánh.

Kết hợp `match` và `enum` rất hữu ích trong nhiều trường hợp. Bạn sẽ thấy mẫu này rất nhiều trong mã Rust: 
 `match` chống lại một enum, liên kết một biến với dữ liệu bên trong, và sau đó thực thi mã dựa trên nó. 
Lúc đầu hơi phức tạp, nhưng một khi bạn đã quen với nó, bạn sẽ ước bạn có nó trong tất cả các ngôn ngữ. Nó luôn là một sự yêu thích của người dùng.

### Matches là toàn diện

Có một khía cạnh khác của `match` chúng ta cần thảo luận. Hãy xem xét phiên bản hàm `plus_one` này của chúng ta 
 có một lỗi và sẽ không biên dịch:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Chúng ta không xử lý trường hợp `None`, vì vậy mã này sẽ gây ra lỗi. May mắn thay, đó là một lỗi mà Rust biết cách bắt. 
 Nếu chúng ta cố gắng biên dịch mã này, chúng ta sẽ gặp lỗi sau:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust biết rằng chúng ta không bao gồm trường hợp nào có thể xảy ra và thậm chí biết chúng ta đã quên mẫu nào!
 Matches trong Rust là *toàn diện*: chúng ta phải sử dụng hết mọi khả năng để mã có hiệu lực. 
Đặc biệt là trong trường hợp của `Option<T>`, khi Rust ngăn chúng ta quên xử lý rõ ràng trường hợp `None`, 
nó bảo vệ chúng ta khỏi giả định rằng chúng ta có một giá trị khi chúng ta có thể có null, 
 do đó làm cho sai lầm hàng tỷ đô la đã được thảo luận trước đó là không thể.

### Catch-all Patterns and the `_` Placeholder

Using enums, we can also take special actions for a few particular values, but
for all other values take one default action. Imagine we’re implementing a game
where, if you roll a 3 on a dice roll, your player doesn’t move, but instead
gets a new fancy hat. If you roll a 7, your player loses a fancy hat. For all
other values, your player moves that number of spaces on the game board. Here’s
a `match` that implements that logic, with the result of the dice roll
hardcoded rather than a random value, and all other logic represented by
functions without bodies because actually implementing them is out of scope for
this example:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

For the first two arms, the patterns are the literal values 3 and 7. For the
last arm that covers every other possible value, the pattern is the variable
we’ve chosen to name `other`. The code that runs for the `other` arm uses the
variable by passing it to the `move_player` function.

This code compiles, even though we haven’t listed all the possible values a
`u8` can have, because the last pattern will match all values not specifically
listed. This catch-all pattern meets the requirement that `match` must be
exhaustive. Note that we have to put the catch-all arm last because the
patterns are evaluated in order. Rust will warn us if we add arms after a
catch-all because those later arms would never match!

Rust also has a pattern we can use when we don’t want to use the value in the
catch-all pattern: `_`, which is a special pattern that matches any value and
does not bind to that value. This tells Rust we aren’t going to use the value,
so Rust won’t warn us about an unused variable.

Let’s change the rules of the game to be that if you roll anything other than
a 3 or a 7, you must roll again. We don’t need to use the value in that case,
so we can change our code to use `_` instead of the variable named `other`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

This example also meets the exhaustiveness requirement because we’re explicitly
ignoring all other values in the last arm; we haven’t forgotten anything.

If we change the rules of the game one more time, so that nothing else happens
on your turn if you roll anything other than a 3 or a 7, we can express that
by using the unit value (the empty tuple type we mentioned in [“The Tuple
Type”][tuples]<!-- ignore --> section) as the code that goes with the `_` arm:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Here, we’re telling Rust explicitly that we aren’t going to use any other value
that doesn’t match a pattern in an earlier arm, and we don’t want to run any
code in this case.

There’s more about patterns and matching that we’ll cover in [Chapter
18][ch18-00-patterns]<!-- ignore -->. For now, we’re going to move on to the
`if let` syntax, which can be useful in situations where the `match` expression
is a bit wordy.

[tuples]: ch03-02-data-types.html#the-tuple-type
[ch18-00-patterns]: ch18-00-patterns.html
