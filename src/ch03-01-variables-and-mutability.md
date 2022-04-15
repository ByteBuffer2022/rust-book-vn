## Biến (variables) và Tính biến đổi (Mutability)

Như đã đề cập trong phần [“Lưu trữ giá trị bằng các
biến”][storing-values-with-variables]<!-- ignore -->, theo mặc định
các biến có tính chất bất biến/không thể thay đổi (immutable). Đây là một trong số nhiều khuyến khích
mà Rust cung cấp cho bạn để viết code theo cách tận dụng sự an toàn và đồng thời dễ dàng mà
Rust cung cấp. Tuy nhiên, bạn vẫn có tùy chọn để làm cho các biến của bạn có thể thay đổi (mutable).
Hãy cùng khám phá cách thức và lý do tại sao Rust khuyến khích bạn ưu tiên tính bất biến và tại sao
đôi khi bạn có thể muốn chọn không tham gia.

Khi một biến không thể thay đổi, khi một giá trị được liên kết với một tên, bạn không thể thay đổi
giá trị đó. Để minh họa điều này, hãy tạo ra một dự án mới có tên *variables*
trong thư mục *projects* của bạn bằng cách sử dụng `cargo new variables`.

Sau đó, trong thư mục mới *variables*, mở *src/main.rs* và thay thế đoạn code của nó bằng đoạn code
bên dưới. Code này sẽ chưa được biên dịch, đầu tiên chúng ta sẽ kiểm tra lỗi không thể thay đổi
(immutability error).

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Lưu lại và chạy chương trình sử dụng `cargo run`. Bạn sẽ nhận được một thông báo lỗi,
như được hiển thị trong đầu ra này:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Ví dụ này cho thấy cách trình biên dịch giúp bạn tìm ra lỗi trong chương trình.
Lỗi trình biên dịch có thể gây khó chịu, nhưng thực sự chúng chỉ có ý nghĩa là
chương trình của bạn chưa thực hiện một cách an toàn những gì bạn muốn; chúng *không* có nghĩa là
bạn không phải là một lập trình viên giỏi! Các Rustacean có kinh nghiệm vẫn gặp lỗi trình biên dịch.

Thông báo lỗi chỉ ra nguyên nhân lỗi là `` cannot
assign twice to immutable variable `x` ``, bởi vì bạn đã cố gắng gán một giá trị thứ hai
vào biến `x` không thể thay đổi.

Điều quan trọng là chúng ta gặp lỗi compile-time khi chúng ta cố gắng thay đổi một giá trị
được chỉ định là bất biến (immutable) bởi vì chính tình huống này có thể dẫn đến các bug.
Nếu một phần code của chúng ta hoạt động dựa trên giả định rằng một giá trị sẽ không bao giờ
thay đổi và một phần khác của đoạn code thay đổi giá trị đó, thì có thể phần đầu tiên của đoạn
code sẽ không thực hiện những gì nó được thiết kế để làm. Nguyên nhân của loại bug này có thể khó
lần ra sau thực tế, đặc biệt là khi đoạn code thứ hai chỉ *đôi lúc* thay đổi.
Trình biên dịch Rust đảm bảo rằng khi bạn ghi một giá trị sẽ không thay đổi,
nó sẽ thực sự không thay đổi, vì vậy bạn không cần phải tự theo dõi nó. Do đó,
code của bạn dễ dàng lập luận hơn.

Nhưng khả năng thay đổi (mutability) có thể rất hữu ích và có thể giúp viết code thuận tiện hơn.
Các biến chỉ immutable theo mặc định; như bạn đã làm trong Chương 2, bạn có thể làm cho chúng
trở nên mutable bằng cách thêm `mut` vào trước tên biến. Việc thêm `mut` cũng
truyền tải ý định đến những người đọc code trong tương lai bằng cách chỉ ra rằng các phần khác của 
code sẽ thay đổi giá trị của biến này.

Ví dụ, hãy thay đổi *src/main.rs* thành như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Khi chúng ta chạy chương trình bây giờ, chúng ta nhận được điều này:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Chúng ta được phép thay đổi giá trị gán của `x` từ `5` thành `6` khi sử dụng `mut`.
Có nhiều sự đánh đổi cần cân nhắc ngoài việc ngăn ngừa bug. Ví dụ, trong trường hợp
bạn đang sử dụng cấu trúc dữ liệu lớn, việc thay đổi một instance tại chỗ có thể
nhanh hơn sao chép và trả lại các instance mới được phân bổ. Với cấu trức dữ liệu
nhỏ hơn, việc tạo mới các instance và viết theo phong cách lập trình chức năng
có thể dễ dàng hơn để suy nghĩ kỹ hơn, vì vậy hiệu suất thấp hơn có thể là
một hình phạt đáng giá để đạt được sự rõ ràng đó.

### Hằng số (constants)

Giống như các biến immutable, *constants* là các giá trị được ràng buộc với một tên và
không được phép thay đổi, nhưng có một vài khác biệt giữa hằng số và biến.

First, you aren’t allowed to use `mut` with constants. Constants aren’t just
immutable by default—they’re always immutable. You declare constants using the
`const` keyword instead of the `let` keyword, and the type of the value *must*
be annotated. We’re about to cover types and type annotations in the next
section, [“Data Types,”][data-types]<!-- ignore --> so don’t worry about the
details right now. Just know that you must always annotate the type.

Constants can be declared in any scope, including the global scope, which makes
them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression,
not the result of a value that could only be computed at runtime.

Here’s an example of a constant declaration:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

The constant’s name is `THREE_HOURS_IN_SECONDS` and its value is set to the
result of multiplying 60 (the number of seconds in a minute) by 60 (the number
of minutes in an hour) by 3 (the number of hours we want to count in this
program). Rust’s naming convention for constants is to use all uppercase with
underscores between words. The compiler is able to evaluate a limited set of
operations at compile time, which lets us choose to write out this value in a
way that’s easier to understand and verify, rather than setting this constant
to the value 10,800. See the [Rust Reference’s section on constant
evaluation][const-eval] for more information on what operations can be used
when declaring constants.

Constants are valid for the entire time a program runs, within the scope they
were declared in. This property makes constants useful for values in your
application domain that multiple parts of the program might need to know about,
such as the maximum number of points any player of a game is allowed to earn or
the speed of light.

Naming hardcoded values used throughout your program as constants is useful in
conveying the meaning of that value to future maintainers of the code. It also
helps to have only one place in your code you would need to change if the
hardcoded value needed to be updated in the future.

### Shadowing

As you saw in the guessing game tutorial in [Chapter
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, you can declare a
new variable with the same name as a previous variable. Rustaceans say that the
first variable is *shadowed* by the second, which means that the second
variable’s value is what the program sees when the variable is used. We can
shadow a variable by using the same variable’s name and repeating the use of
the `let` keyword as follows:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

This program first binds `x` to a value of `5`. Then it shadows `x` by
repeating `let x =`, taking the original value and adding `1` so the value of
`x` is then `6`. Then, within an inner scope, the third `let` statement also
shadows `x`, multiplying the previous value by `2` to give `x` a value of `12`.
When that scope is over, the inner shadowing ends and `x` returns to being `6`.
When we run this program, it will output the following:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Shadowing is different from marking a variable as `mut`, because we’ll get a
compile-time error if we accidentally try to reassign to this variable without
using the `let` keyword. By using `let`, we can perform a few transformations
on a value but have the variable be immutable after those transformations have
been completed.

The other difference between `mut` and shadowing is that because we’re
effectively creating a new variable when we use the `let` keyword again, we can
change the type of the value but reuse the same name. For example, say our
program asks a user to show how many spaces they want between some text by
inputting space characters, and then we want to store that input as a number:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

The first `spaces` variable is a string type and the second `spaces` variable
is a number type. Shadowing thus spares us from having to come up with
different names, such as `spaces_str` and `spaces_num`; instead, we can reuse
the simpler `spaces` name. However, if we try to use `mut` for this, as shown
here, we’ll get a compile-time error:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

The error says we’re not allowed to mutate a variable’s type:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Now that we’ve explored how variables work, let’s look at more data types they
can have.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
