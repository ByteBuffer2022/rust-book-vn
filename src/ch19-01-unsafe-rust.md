## Unsafe Rust

Tất cả các đoạn code đã trình bày trước đó đều được Rust bảo vệ và ngăn chặn nếu gặp lỗi ngay từ compile time (Rust's memory safety guarantees). Tuy nhiên, Rust cũng có một tính năng ẩn khác mà không hề được compiler kiểm định và soát lỗi khi biên dịch: đó là *unsafe Rust*. Nó cũng giống như Rust thông thường, tuy nhiên *unsafe Rust* tự do và khó kiểm soát hơn Rust.

Tại sao phải sinh ra unsafe rust! Lý do là Rust compiler đôi khi tỏ ra quá an toàn khi thực hiện biên dịch chương trình. Đôi khi code của bạn rơi vào trường hợp không chắc chắn an toàn hay không, thì mặc định compiler sẽ từ chối và coi đây là một lỗi, mặc dù có thể không phải như vậy. Cơ chế này giúp ta ngăn ngừa các lỗi tiềm ẩn, tuy nhiên nếu lập trình viên cố tình muốn thực hiện các đoạn code này thì sao? Ta sẽ nói với compiler, "Hãy tin ở tôi, tôi hiểu mình đang làm gì". Đây là một sự đánh đổi, code của bạn sẽ được thực thi với một tỉ lệ rủi ro nào đó, tính toán sai có thể dẫn đến các lỗi về bộ nhớ (memory) như con trỏ null (null pointer), rò rỉ bộ nhớ (leaked memory), ...

Một lí do khác khiến cơ chế unsafe được tạo ra là Rust muốn tiếp cận với phần cứng của hệ điều hành giống như những ngôn ngữ lập trình bậc thấp, mà bản chất các ngôn ngữ này đều chạy cơ chế unsafe. Nếu Rust không cũng cấp unsafe, sẽ rất khó để bạn có thể làm được những điều này. Hãy cùng khám phá những điều có thê làm với unsafe Rust.

### Sức mạnh của Unsafe

Để sử dụng unsafe Rust, dùng keyword `unsafe` và tạo một block chứa các unsafe code mà bạn muốn. Có 5 điều mà unsafe Rust có thể làm mà bạn sẽ không thể có được ở Rust thông thường:

* Dereference một raw pointer (các khái niệm Dereference và raw pointer sẽ được giải thích sau)
* Gọi unsafe function hoặc method
* Truy cập và chỉnh sửa một mutable static variable
* Implement unsafe trait
* Truy cập vào trường dữ liệu bên trong `union`

Việc sử dụng `unsafe` hoàn toàn không loại bỏ việc sử dụng borrow checker (dùng để quản lí quyền sở hữu của dữ liệu) hay các công cụ quan lí bộ nhớ khác của Rust: nếu bạn sử dụng tham chiếu trong unsafe code, tham chiếu này vẫn sẽ được kiểm tra. Từ khóa `unsafe` chỉ làm cho compiler không quản lí các tác vụ liên quan đến vùng nhớ nằm trong 5 tính năng đã đề cập ở trên.

Hơn nữa, `unsafe` không có nghĩa là đoạn code đó lúc nào cũng gây lỗi bộ nhớ: nó chỉ chuyển quyền quyết định từ compiler cho người lập trình. Lập trình viên chính là người quản lí những rủi ro đó.

Con người thì luôn mắc sai lầm, tuy nhiên việc giới hạn ở 5 tính năng trên sẽ giúp bạn khoanh vùng lỗi một cách dễ dàng hơn (nếu có lỗi xảy ra), `unsafe` block càng nhỏ thì bạn sẽ debug càng dễ dàng.

Để tách bạch phần unsafe code, ta nên bao bên ngoài chúng một safe API để có thể ngăn ngừa việc mất kiểm soát chương trình nếu đoạn unsafe đó xảy ra lỗi.

Bây giờ hãy xem xét lần lượt 5 tính năng trên
### Dereferencing một Raw Pointer

Phần [“Dangling References”][dangling-references]<!-- ignore --> trong chương 4, đề cập đến việc compiler luôn luôn kiểm tra tham chiếu có hợp lệ hay không. Unsafe Rust cung cấp kiểu tham chiếu mới có tên là *raw pointers*, tương tự như tham chiếu trong safe Rust. Raw pointers có thể thay đổi được (mutable) hoặc không (immutable), cú pháp tương ứng ở đây là `*mut T` và `*const T`. Dấu `*` không phải là toán tử dereference mà chỉ đơn giản là cú pháp bắt buộc của raw pointer. Chú ý rằng, *immutable* nghĩa là con trỏ đó sẽ không thể  trực tiếp thay đổi giá trị của biến mà nó trỏ tới.

Sự khác biệt của raw pointers với tham chiếu (references) và smart pointers:

* Cho phép có nhiều mutable pointers cùng trỏ vào một vùng nhớ
* Không kiểm tra tham chiếu có hợp lệ hay không
* Cho phép sử dụng con trỏ null
* Không tự động giải phóng vùng nhớ

Với việc bỏ qua các rules về tham chiếu, con trỏ mà safe Rust cung cấp, ta có thể nâng cao được hiệu năng hoạt động của chương trình cũng như việc tương tác với phần cứng máy tính.

Listing 19-1 cho ta thấy cách tạo một immutable và mutable raw pointer.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-01/src/main.rs:here}}
```

<span class="caption">Listing 19-1: Cách tạo một immutable và mutable raw pointer.</span>

Raw pointer khá giống với con trỏ trong C/C++, sẽ rất đơn giản cho những ai đã có nền tảng về lập trình hệ thống, lập trình nhúng... sử dụng C/C++. Ở đây ta không cần sử dụng từ khóa unsafe khi khởi tạo raw pointers vì đây là một hành động chưa gây hại cho hệ thống, unsafe chỉ cần thiết khi ta truy cập vào giá trị mà con trỏ đó trỏ đến (dereference).

Tiếp theo, ta sẽ tạo một raw pointer mà không biết được nó có hợp lệ hay không. Listing 19-2 là một ví dụ: tạo một raw pointer để trỏ đến một địa chỉ ô nhớ bất kì trong memory. Compiler sẽ không biết được tại địa chỉ này có dữ liệu hay không, vì vậy ta có thể sẽ gặp lỗi segmentation fault (khá giống trong C/C++). Mặc dù không nên viết code như vậy, nó vẫn được chấp nhận trong unsafe Rust.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-02/src/main.rs:here}}
```

<span class="caption">Listing 19-2: Tạo raw pointer để trỏ đến vùng nhớ bất kì</span>

Nhớ rằng ta có thể tạo một raw pointer trong safe code, nhưng không thể truy cập vào giá trị mà nó trỏ đến (dereference). Ở Listing 19-3 là một ví dụ sử dụng toán tử dereference `*` trong unsafe code.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-03/src/main.rs:here}}
```

<span class="caption">Listing 19-3: Truy cập vào giá trị mà raw pointer trỏ đến (dereference) trong unsafe block</span>

Khởi tạo một con trỏ sẽ không gây hại cho hệ thống; nó chỉ nguy hiểm khi ta cố gắng truy cập vào vùng nhớ không hợp lệ mà nó trỏ đến

Chú ý rằng ở Listing 19-1 và 19-3, ta sử dụng immutable và mutable raw pointers để trỏ đến cùng một vùng nhớ của biến `num`. Nếu sử dụng immutable và mutable reference thay vì raw pointer, khi compile sẽ xảy ra lỗi vì liên quan đến quyền sở hữu trong Rust (Rust's ownership). Tuy nhiên với raw pointer, ta hoàn toàn có thể làm được điều này, chỉ có điều việc này có thể sẽ tiềm tàng lỗi liên quan đến data race. Hãy cần trọng khi sử dụng!

Với những nguy hiểm tiềm tàng như vậy, tại sao raw pointer vẫn được sinh ra? Câu trả lời sẽ có trong phần tiếp theo, [“Calling an Unsafe Function or Method.”](#calling-an-unsafe-function-or-method)<!-- ignore -->. 

### Calling an Unsafe Function or Method

Tạo một unsafe function hay unsafe method cũng giống như tạo function hay method thông thường, chỉ khác ở từ khóa unsafe ở phía trước.

Đây là một ví dụ về việc tạo unsafe function có tên `dangerous`

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

Phải gọi hàm `dangerous` này trong một unsafe block riêng biệt. Nếu không khi compile chương trình sẽ báo lỗi.

```console
{{#include ../listings/ch19-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

Phần thân của unsafe function hoạt động giống như `unsafe` blocks, vì vậy ta không cần phải dùng từ khóa `unsafe` cho thân hàm nữa.

#### Creating a Safe Abstraction over Unsafe Code

Hàm có chưa một đoạn unsafe code không đồng nghĩa với việc cả hàm đó là unsafe. Trong thực tế, bọc unsafe code bởi một safe function là một việc làm rất phổ biến. Xét ví dụ sau, safe method `split_at_mut` sẽ bao bên ngoài của unsafe code. Chức năng của hàm này là chia một mutable slice thành hai phần và trả về 2 slices đó.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-04/src/main.rs:here}}
```

<span class="caption">Listing 19-4: Sử dụng safe function `split_at_mut`</span>

Nếu bạn chỉ viết hàm này ở safe code, chương trình sẽ báo lỗi vào không thể biên dịch (listing 19-5). Để đơn giản, ta sẽ dùng function thay vì method và dùng slice kiểu `i32` thay cho generic type `T`.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-05/src/main.rs:here}}
```

<span class="caption">Listing 19-5: Viết hàm `split_at_mut` sử dụng safe Rust</span>

Hàm này đầu tiên sẽ lấy được tổng số phần tử của slice. Sau đó sẽ kiểm tra xem phần tử có index truyền vào có thuộc slice đó không qua việc so sánh với length. Nếu không chương trình sẽ panic.

Sau đó hàm sẽ return 2 mutable slices ở trong một tuple: slice thứ nhất sẽ bắt đầu từ phần tử 0 đến phần tử `mid` của slice gốc và slice thứ 2 sẽ là phần còn lại.

Nếu compile chương trình ở Listing 19-5, ta sẽ gặp lỗi như sau:

```console
{{#include ../listings/ch19-advanced-features/listing-19-05/output.txt}}
```

Rust's borrow checker (dùng để kiểm tra quyền sở hữu của các biến) không thể biết được ta đang mượn 2 slice tách biệt; nó chỉ biết rằng ta đang mượn từ cùng một slice gốc. Do đó để tránh rủi ro, Rust sẽ coi đây là một lỗi và không cho chương trình được biên dịch, lúc này ta phải cần đến unsafe code.

Listing 19-6 cho ta thấy cách sử dụng `unsafe` block, raw pointer, unsafe function để viết hàm `split_at_mut`.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-06/src/main.rs:here}}
```

<span class="caption">Listing 19-6: Sử dụng unsafe code để viết hàm `split_at_mut`</span>

Slice thực chất là con trỏ trỏ tới một vùng nhớ có kích thước xác định (xem thêm [“The Slice Type”][the-slice-type]<!-- ignore -->). Dùng method `len` để lấy ra kích thước của slice và method `as_mut_ptr` để tạo ra raw pointer của slice đó.

Sau đó là phần unsafe code, hàm `slice::from_raw_parts_mut` sẽ tạo ra một slice mới dựa trên raw pointer truyền vào và kích thước mong muốn. Method `add` với tham số `mid` có nhiệm vụ đưa con trỏ trỏ tới vị trí `mid` của slice gốc.

Hàm `slice::from_raw_parts_mut` là một unsafe function bởi nó sử dụng raw pointer và không biết được con trỏ đó có hợp lệ hay không. Method `add` cũng vậy, vì nó hoàn toàn không biết index được truyền vào có nằm trong slice hay không. Do đó, ta phải đưa những đoạn code này vào trong unsafe block.

Chú ý rằng ta không cần phải đánh dấu hàm `split_at_mut` là unsafe bởi nó chỉ return các con trỏ hoàn toàn hợp lệ. Nhớ rằng việc tạo con trỏ không hề nguy hiểm, nó chỉ nguy hiểm khi truy cập đến giá trị của con trỏ đó mà thôi.

Ngược lại, việc dùng hàm `slice:from_raw_parts_mut` ở Listing 19-7 có thể xảy ra lỗi khi chạy chương trình. Đoạn code này sẽ truy cập vào một vùng nhớ bất kì và tạo một slice có 10000 phần tử.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-07/src/main.rs:here}}
```

<span class="caption">Listing 19-7: Tạo slice từ một vùng nhớ bất kì</span>

Ta không chắc rằng mình có quyền sở hữu vùng nhớ đó, nên không thể chắc chắn rằng vùng nhớ đó chỉ chứa các giá trị `i32`. Cố gắng sử dụng các giá trị đó làm một hành động không được phép (undefined behavior).

#### Using `extern` Functions to Call External Code

Sometimes, your Rust code might need to interact with code written in another
language. For this, Rust has a keyword, `extern`, that facilitates the creation
and use of a *Foreign Function Interface (FFI)*. An FFI is a way for a
programming language to define functions and enable a different (foreign)
programming language to call those functions.

Listing 19-8 demonstrates how to set up an integration with the `abs` function
from the C standard library. Functions declared within `extern` blocks are
always unsafe to call from Rust code. The reason is that other languages don’t
enforce Rust’s rules and guarantees, and Rust can’t check them, so
responsibility falls on the programmer to ensure safety.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-08/src/main.rs}}
```

<span class="caption">Listing 19-8: Declaring and calling an `extern` function
defined in another language</span>

Within the `extern "C"` block, we list the names and signatures of external
functions from another language we want to call. The `"C"` part defines which
*application binary interface (ABI)* the external function uses: the ABI
defines how to call the function at the assembly level. The `"C"` ABI is the
most common and follows the C programming language’s ABI.

> #### Calling Rust Functions from Other Languages
>
> We can also use `extern` to create an interface that allows other languages
> to call Rust functions. Instead of an `extern` block, we add the `extern`
> keyword and specify the ABI to use just before the `fn` keyword. We also need
> to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle
> the name of this function. *Mangling* is when a compiler changes the name
> we’ve given a function to a different name that contains more information for
> other parts of the compilation process to consume but is less human readable.
> Every programming language compiler mangles names slightly differently, so
> for a Rust function to be nameable by other languages, we must disable the
> Rust compiler’s name mangling.
>
> In the following example, we make the `call_from_c` function accessible from
> C code, after it’s compiled to a shared library and linked from C:
>
> ```rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> This usage of `extern` does not require `unsafe`.

### Accessing or Modifying a Mutable Static Variable

Until now, we’ve not talked about *global variables*, which Rust does support
but can be problematic with Rust’s ownership rules. If two threads are
accessing the same mutable global variable, it can cause a data race.

In Rust, global variables are called *static* variables. Listing 19-9 shows an
example declaration and use of a static variable with a string slice as a
value.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-09/src/main.rs}}
```

<span class="caption">Listing 19-9: Defining and using an immutable static
variable</span>

Static variables are similar to constants, which we discussed in the
[“Differences Between Variables and
Constants”][differences-between-variables-and-constants]<!-- ignore -->
section in Chapter 3. The names of static variables are in
`SCREAMING_SNAKE_CASE` by convention. Static variables can only store
references with the `'static` lifetime, which means the Rust compiler can
figure out the lifetime and we aren’t required to annotate it explicitly.
Accessing an immutable static variable is safe.

Constants and immutable static variables might seem similar, but a subtle
difference is that values in a static variable have a fixed address in memory.
Using the value will always access the same data. Constants, on the other hand,
are allowed to duplicate their data whenever they’re used.

Another difference between constants and static variables is that static
variables can be mutable. Accessing and modifying mutable static variables is
*unsafe*. Listing 19-10 shows how to declare, access, and modify a mutable
static variable named `COUNTER`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-10/src/main.rs}}
```

<span class="caption">Listing 19-10: Reading from or writing to a mutable
static variable is unsafe</span>

As with regular variables, we specify mutability using the `mut` keyword. Any
code that reads or writes from `COUNTER` must be within an `unsafe` block. This
code compiles and prints `COUNTER: 3` as we would expect because it’s single
threaded. Having multiple threads access `COUNTER` would likely result in data
races.

With mutable data that is globally accessible, it’s difficult to ensure there
are no data races, which is why Rust considers mutable static variables to be
unsafe. Where possible, it’s preferable to use the concurrency techniques and
thread-safe smart pointers we discussed in Chapter 16 so the compiler checks
that data accessed from different threads is done safely.

### Implementing an Unsafe Trait

Another use case for `unsafe` is implementing an unsafe trait. A trait is
unsafe when at least one of its methods has some invariant that the compiler
can’t verify. We can declare that a trait is `unsafe` by adding the `unsafe`
keyword before `trait` and marking the implementation of the trait as `unsafe`
too, as shown in Listing 19-11.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-11/src/main.rs}}
```

<span class="caption">Listing 19-11: Defining and implementing an unsafe
trait</span>

By using `unsafe impl`, we’re promising that we’ll uphold the invariants that
the compiler can’t verify.

As an example, recall the `Sync` and `Send` marker traits we discussed in the
[“Extensible Concurrency with the `Sync` and `Send`
Traits”][extensible-concurrency-with-the-sync-and-send-traits]<!-- ignore -->
section in Chapter 16: the compiler implements these traits automatically if
our types are composed entirely of `Send` and `Sync` types. If we implement a
type that contains a type that is not `Send` or `Sync`, such as raw pointers,
and we want to mark that type as `Send` or `Sync`, we must use `unsafe`. Rust
can’t verify that our type upholds the guarantees that it can be safely sent
across threads or accessed from multiple threads; therefore, we need to do
those checks manually and indicate as such with `unsafe`.

### Accessing Fields of a Union

The final action that works only with `unsafe` is accessing fields of a
*union*. A `union` is similar to a `struct`, but only one declared field is
used in a particular instance at one time. Unions are primarily used to
interface with unions in C code. Accessing union fields is unsafe because Rust
can’t guarantee the type of the data currently being stored in the union
instance. You can learn more about unions in [the Rust Reference][reference].

### When to Use Unsafe Code

Using `unsafe` to take one of the five actions (superpowers) just discussed
isn’t wrong or even frowned upon. But it is trickier to get `unsafe` code
correct because the compiler can’t help uphold memory safety. When you have a
reason to use `unsafe` code, you can do so, and having the explicit `unsafe`
annotation makes it easier to track down the source of problems when they occur.

[dangling-references]:
ch04-02-references-and-borrowing.html#dangling-references
[differences-between-variables-and-constants]:
ch03-01-variables-and-mutability.html#constants
[extensible-concurrency-with-the-sync-and-send-traits]:
ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits
[the-slice-type]: ch04-03-slices.html#the-slice-type
[reference]: ../reference/items/unions.html
