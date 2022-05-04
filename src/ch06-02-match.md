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

<span class="caption">Listing 6-4: A `Coin` enum in which the `Quarter` variant
also holds a `UsState` value</span>

Let’s imagine that a friend is trying to collect all 50 state quarters. While
we sort our loose change by coin type, we’ll also call out the name of the
state associated with each quarter so if it’s one our friend doesn’t have, they
can add it to their collection.

In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter’s state. Then we can use `state` in the code for that arm, like so:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.

### Matching with `Option<T>`

In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match` as we
did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way that the `match` expression works remains
the same.

Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.

This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>

Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value contained in `Some`, so `i` takes the value `5`. The
code in the match arm is then executed, so we add 1 to the value of `i` and
create a new `Some` value with our total `6` inside.

Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm.

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.

Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.

### Matches Are Exhaustive

There’s one other aspect of `match` we need to discuss. Consider this version
of our `plus_one` function that has a bug and won’t compile:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

Rust knows that we didn’t cover every possible case and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.

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
