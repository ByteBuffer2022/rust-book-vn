## Advanced Traits

Traits đã được nhắc đến trong chương 10 [“Traits: Defining Shared Behavior”][traits-defining-shared-behavior]<!-- ignore -->, tuy nhiên đó chỉ là những kiến thức cơ bản nhất và traits mà thôi. Trong chương này, ta sẽ đi sâu hơn vào những tính năng nâng cao hơn của traits.

### Sử dụng Associated Types khi định nghĩa Trait

*Associated types* có thể được sử dụng khi định nghĩa Trait mà khi implement nó ta hoàn toàn biết trước được kiểu dữ liệu mà trait đó muốn sử dụng.

Các tính năng nâng cao khác ở chương này đa số đều ít khi được sử dụng, tuy nhiên associated types lại ở khoảng giữa: nó không được sử dụng quá nhiều như những tính năng khác được mô tả ở trong cuốn sách này nhưng lại được sử dụng phổ biến hơn các tính năng nâng cao khác.

Một ví dụ điển hình của việc sử dụng associated type trong trait là `Iterator` của thư viện chuẩn trong Rust. Associated type có tên là `Item` ở trong trường hợp này. Trong phần [“The `Iterator` Trait and the `next` Method”][the-iterator-trait-and-the-next-method]<!-- ignore -->, ta đã đề cập đến phần định nghĩa của `Iterator` trait

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-12/src/lib.rs}}
```

<span class="caption">Listing 19-12: Định nghĩa `Iterator` trait sử dụng associated type `Item`</span>

Kiểu `Item` còn được gọi là placeholder type, `next` method sẽ trả về một kiểu `Option<Self::Item>`. Các struct có implement `Iterator` này đều sẽ có một kiểu dữ liệu duy nhất và cố định là `Item`, `next` method làm nhiệm vụ trả về `Option` chứa Item đó.

Đến đây ta có thể thấy khá nhiều điểm tương đồng giữa associated type và generics type, vậy tại sao phải sử dụng associated types?

Ví dụ sau sẽ cho ta thấy sự khác biệt giữa 2 cách dùng. Ở chương 13, listing 13-21 sử dụng associated type `Item` bằng `u32`:

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-13-21-reproduced/src/lib.rs:ch19}}
```

Và cú pháp dùng generics được mô tả trong listing 19-13?

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-13/src/lib.rs}}
```

<span class="caption">Listing 19-13: Đinh nghĩa `Iterator` trait sử dụng generics</span>

Sự khác biệt ở đây là khi dùng generics, ta phải chú thích kiểu dữ liệu cho mỗi lần implement; hoàn toàn có thể implement `Iterator<String> for Counter` hoặc bất kì kiểu dữ liệu nào khác ngoài `u32`, do đó ta có thể có rất nhiều các phiên bản khác nhau của `Iterator` cho `Counter`. Nói một cách khác, khi một trait sử dụng generics parameter, nó có thể implement rất nhiều lần, thay đổi kiểu dữ liệu cho mỗi lần đó. Khi sử dụng method `next`, ta bắt buộc phải cung cấp kiểu dữ liệu để thể hiện `Iterator` nào được sử dụng.

Với associated types, ta không cần phải chú thích kiểu dữ liệu như vậy bởi trait này không thể implement nhiều lần, `Iterator` chỉ có `Item` với kiểu dữ liệu duy nhất là `u32` mà thôi.

### Tham số Generic Type mặc định và nạp chồng toán tử (Operator Overloading)

Khi sử dụng generic type, ta có thể chỉ định tham số mặc định cho nó. Cú pháp ở đây là `<PlaceholderType=ConcreteType>`.

Một ví dụ tuyệt vời nhất cho trường hợp này là khi dùng đến nạp chồng toán tử (operator overloading). *Operator overloading* dùng để biến tấu hành vi của một toán tử  (như là `+`) trong vài trường hợp cụ thể.

Rust không cho phép bạn tạo mới một toán tử hay nạp chồng một toán tử bất kì. Tuy nhiên, bạn có thể nạp chồng một toán tử nếu toán tử đó nằm trong thư viện `std::ops` bằng cách implement toán tử nằm trong chính thư viện này. Ví dụ trong listing 19-4, ta sẽ nạp chồng toán tử `+` để cộng 2 `Point` với nhau. Để làm được điều này, ta phải implement `Add` trait cho `Point` struct:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-14/src/main.rs}}
```

<span class="caption">Listing 19-14: Implementing `Add` trait để nạp chồng toán tử `+` cho instances `Point`</span>

Method `add` sẽ cộng hoành độ và tung độ tương ứng của 2 `Point`. Trait `Add` lúc này sẽ có một associated type là `Output`.

Generic type mặc định được nằm trong phần định nghĩa `Add` trait, như sau:

```rust
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
```

Ta có thể thấy phần khác biệt ở đây là `Rhs=Self`: cú pháp này được gọi là *default type parameters*. `Rhs` (viết tắt của "right hand side") sẽ định nghĩa kiểu dữ liệu cho biến `rhs` được dùng trong method `add`. Nếu ta không chỉ định kiểu dữ liệu cho `Rhs` khi implement, `Rhs` khi đó sẽ mặc định có kiểu `Self`.

Khi implement `Add` cho `Point`, ta sẽ sử dụng kiểu mặc định cho `Rhs` vì mục đích cuối cùng là cộng 2 `Point` instances. Cùng xem ví dụ mà ở đây ta sẽ implement `Add` trait và không sử dụng kiểu mặc định cho `Rhs` nữa.

Ở đây có 2 structs, `Millimeters` và `Meters`, thể hiện giá trị ở các đơn vị đo khác nhau. Các struct này sẽ bao bên ngoài của kiểu dữ liệu đã tồn tại (`u32`), cách làm này được gọi là *newtype pattern* (xem thêm trong phần [“Using the Newtype Pattern to Implement External Traits on External Types”][newtype]<!-- ignore-->). Ở đây ta sẽ cộng giá trị ở đơn vị millimeters với giá trị ở đơn vị meters với việc bắt buộc phải chuyển đổi đơn vị đo.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-15/src/lib.rs}}
```

<span class="caption">Listing 19-15: Implementing `Add` trait cho `Millimeters` để cộng `Millimeters` với `Meters`</span>

Để làm được điều này, ta sẽ chỉ định `impl Add<Meters>` để set giá trị cho tham số `Rhs` thay vì dùng tham số mặc định `Self`.

Bạn sẽ sử dụng kiểu tham số mặc định với mục đích:

* Để mở rộng một kiểu dữ liệu mà không làm thay đổi source code đã tồn tại.
* Cho phép tuỳ biến một trường hợp cụ thể  nào đó.

Trait `Add` ở trong thư viện chuẩn mà ta vừa sử dụng là một ví dụ cho mục đích thứ 2: ta sẽ phải cộng 2 kiểu khác nhau, tuy nhiên `Add` trait còn cung cấp khả năng tuỳ biến nhiều hơn như vậy. Sử dụng kiểu tham số mặc định (default type parameter) trong phần định nghĩa trait sẽ giúp bạn không bắt buộc phải thêm tham số trong đa số các trường hợp.

Mục đích thứ nhất thì ngược lại với mục đích thứ hai: nếu muốn cộng một kiểu dữ liệu cho một trait có trước, bạn có thể cho nó một tham số mặc định để có thể tăng khả năng tuỳ biến mà không cần phải thay đổi logic của source code đã có sẵn.

### Gọi các method có cùng tên

Rust không ngăn cản việc bạn tạo method có cùng tên với method của trait khác, cũng như cấm việc implement 2 trait có cùng một kiểu. Ta hoàn toàn có thể implement một method có cùng tên với các method của các traits khác.

Khi gọi các methods có cùng tên , bạn sẽ cần chỉ ra đâu là method mà bạn cần. Xem xét Listing 19-16 sau đây, có 2 trait là `Pilot` và `Wizard` đều định nghĩa method `fly`. Sau đó implement cả 2 cho kiểu `Human` cũng đã có sẵn method là `fly`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-16/src/main.rs:here}}
```

<span class="caption">Listing 19-16: Định nghĩa method `fly`</span>

Khi gọi `fly` ở `Human` instance, compiler sẽ mặc định gọi method nào được implement *trực tiếp*, được thể hiện trong Listing 19-17.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-17/src/main.rs:here}}
```

<span class="caption">Listing 19-17: Gọi `fly` ở instance `Human`</span>

Kết quả là dòng chữ `*waving arms furiously*` được in ra, thể hiện rằng Rust đã gọi `fly` trực tiếp từ `Human`.

Nếu muốn gọi methods `fly` từ `Pilot` hoặc `Wizard`, ta cần phải khai báo rõ ràng hơn bằng một cú pháp khác trong Listing 19-18.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-18/src/main.rs:here}}
```

<span class="caption">Listing 19-18: Chỉ định method `fly` nào sẽ được gọi</span>

Sau khi chạy, ta sẽ được kết quả:

```console
{{#include ../listings/ch19-advanced-features/listing-19-18/output.txt}}
```

Ở đây, method `fly` có tham số `self`, vì vậy ta có thể truyền `person` vào và Rust có thể tìm ra trait nào cần sử dụng trong trường hợp này.

Vậy trong trường hợp sử dụng associated functions (không có tham số `self`) thì sao? Rust sẽ không thể biết được bạn cần gọi method của trait nào nếu không sử dụng *fully qualified syntax*. Xét ví dụ dưới đây:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-19/src/main.rs}}
```

<span class="caption">Listing 19-19: Gọi một associated function có cùng tên với các associated function của traits khác</span>

Trong hàm `main`, hàm `Dog::baby_name` được gọi, khi đó ta sẽ có kết quả:

```console
{{#include ../listings/ch19-advanced-features/listing-19-19/output.txt}}
```

Kết quả này không phải cái ta mong muốn. Hàm `baby_name` phải in ra dòng chữ `A baby dog is called a puppy`. Do vậy, kĩ thuật được sử dụng trong Listing 19-18 không áp dụng được trong trường hợp này; nếu thay đổi code như là Listing 19-20 thì sao:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-20/src/main.rs:here}}
```

<span class="caption">Listing 19-20: Gọi hàm `baby_name` từ trait `Animal`</span>

Vì `Animal::baby_name` không có tham số `self`, và có thể có các struct khác cũng sẽ implement `Animal` trait, do đó Rust không thể biết được hàm `Animal::baby_name` sẽ sử dụng implementation nào. Lỗi sẽ như sau:

```console
{{#include ../listings/ch19-advanced-features/listing-19-20/output.txt}}
```

Vậy để pass qua được lỗi này, cùng xem Listing 19-21:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-21/src/main.rs:here}}
```

<span class="caption">Listing 19-21: Sử dụng fully qualified syntax để chỉ định hàm `baby_name` được implement từ `Dog` sẽ được gọi</span>

Ta sẽ cung cấp cho Rust một kiểu chú thích trong cặp ngoặc `<>`, trong đó sẽ chỉ rõ rằng phương thức `baby_name` của `Animal` trait được implement từ `Dog`. Khi đó kết quả sẽ như ta mong muốn:

```console
{{#include ../listings/ch19-advanced-features/listing-19-21/output.txt}}
```

Một cách tổng quát, fully qualified syntax được định nghĩa như sau:

```rust,ignore
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

Sẽ không có `receiver` trong trường hợp đó là một associated functions chứ không phải methods. Bạn có thể sẽ phải sử dụng fully qualified syntax, tuy nhiên hoàn toàn có thể bỏ qua một phải phần nếu Rust có đủ thông tin để tự mình tìm ra được bạn sẽ muốn gọi hàm nào, giống như các ví dụ đã bàn ở trên.

### Using Supertraits to Require One Trait’s Functionality Within Another Trait

Sometimes, you might need one trait to use another trait’s functionality. In
this case, you need to rely on the dependent trait also being implemented.
The trait you rely on is a *supertrait* of the trait you’re implementing.

For example, let’s say we want to make an `OutlinePrint` trait with an
`outline_print` method that will print a value framed in asterisks. That is,
given a `Point` struct that implements `Display` to result in `(x, y)`, when we
call `outline_print` on a `Point` instance that has `1` for `x` and `3` for
`y`, it should print the following:

```text
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of `outline_print`, we want to use the `Display` trait’s
functionality. Therefore, we need to specify that the `OutlinePrint` trait will
work only for types that also implement `Display` and provide the functionality
that `OutlinePrint` needs. We can do that in the trait definition by specifying
`OutlinePrint: Display`. This technique is similar to adding a trait bound to
the trait. Listing 19-22 shows an implementation of the `OutlinePrint` trait.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-22/src/main.rs:here}}
```

<span class="caption">Listing 19-22: Implementing the `OutlinePrint` trait that
requires the functionality from `Display`</span>

Because we’ve specified that `OutlinePrint` requires the `Display` trait, we
can use the `to_string` function that is automatically implemented for any type
that implements `Display`. If we tried to use `to_string` without adding a
colon and specifying the `Display` trait after the trait name, we’d get an
error saying that no method named `to_string` was found for the type `&Self` in
the current scope.

Let’s see what happens when we try to implement `OutlinePrint` on a type that
doesn’t implement `Display`, such as the `Point` struct:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/src/main.rs:here}}
```

We get an error saying that `Display` is required but not implemented:

```console
{{#include ../listings/ch19-advanced-features/no-listing-02-impl-outlineprint-for-point/output.txt}}
```

To fix this, we implement `Display` on `Point` and satisfy the constraint that
`OutlinePrint` requires, like so:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-03-impl-display-for-point/src/main.rs:here}}
```

Then implementing the `OutlinePrint` trait on `Point` will compile
successfully, and we can call `outline_print` on a `Point` instance to display
it within an outline of asterisks.

### Using the Newtype Pattern to Implement External Traits on External Types

In Chapter 10 in the [“Implementing a Trait on a
Type”][implementing-a-trait-on-a-type]<!-- ignore --> section, we mentioned
the orphan rule that states we’re allowed to implement a trait on a type as
long as either the trait or the type are local to our crate. It’s possible to
get around this restriction using the *newtype pattern*, which involves
creating a new type in a tuple struct. (We covered tuple structs in the
[“Using Tuple Structs without Named Fields to Create Different
Types”][tuple-structs]<!-- ignore --> section of Chapter 5.) The tuple struct
will have one field and be a thin wrapper around the type we want to implement
a trait for. Then the wrapper type is local to our crate, and we can implement
the trait on the wrapper. *Newtype* is a term that originates from the Haskell
programming language. There is no runtime performance penalty for using this
pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement `Display` on `Vec<T>`, which the
orphan rule prevents us from doing directly because the `Display` trait and the
`Vec<T>` type are defined outside our crate. We can make a `Wrapper` struct
that holds an instance of `Vec<T>`; then we can implement `Display` on
`Wrapper` and use the `Vec<T>` value, as shown in Listing 19-23.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-23/src/main.rs}}
```

<span class="caption">Listing 19-23: Creating a `Wrapper` type around
`Vec<String>` to implement `Display`</span>

The implementation of `Display` uses `self.0` to access the inner `Vec<T>`,
because `Wrapper` is a tuple struct and `Vec<T>` is the item at index 0 in the
tuple. Then we can use the functionality of the `Display` type on `Wrapper`.

The downside of using this technique is that `Wrapper` is a new type, so it
doesn’t have the methods of the value it’s holding. We would have to implement
all the methods of `Vec<T>` directly on `Wrapper` such that the methods
delegate to `self.0`, which would allow us to treat `Wrapper` exactly like a
`Vec<T>`. If we wanted the new type to have every method the inner type has,
implementing the `Deref` trait (discussed in Chapter 15 in the [“Treating Smart
Pointers Like Regular References with the `Deref`
Trait”][smart-pointer-deref]<!-- ignore --> section) on the `Wrapper` to return
the inner type would be a solution. If we don’t want the `Wrapper` type to have
all the methods of the inner type—for example, to restrict the `Wrapper` type’s
behavior—we would have to implement just the methods we do want manually.

Now you know how the newtype pattern is used in relation to traits; it’s also a
useful pattern even when traits are not involved. Let’s switch focus and look
at some advanced ways to interact with Rust’s type system.

[newtype]: ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
[implementing-a-trait-on-a-type]:
ch10-02-traits.html#implementing-a-trait-on-a-type
[the-iterator-trait-and-the-next-method]:
ch13-02-iterators.html#the-iterator-trait-and-the-next-method
[traits-defining-shared-behavior]:
ch10-02-traits.html#traits-defining-shared-behavior
[smart-pointer-deref]: ch15-02-deref.html#treating-smart-pointers-like-regular-references-with-the-deref-trait
[tuple-structs]: ch05-01-defining-structs.html#using-tuple-structs-without-named-fields-to-create-different-types
