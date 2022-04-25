## Unsafe Rust

Tất cả các đoạn code đã trình bày trước đó đều được Rust bảo vệ và ngăn chặn nếu gặp lỗi ngay từ compile time (Rust's memory safety guarantees). Tuy nhiên, Rust cũng có một tính năng ẩn khác mà không hề được compiler kiểm định và soát lỗi khi biên dịch: đó là *unsafe Rust*. Nó cũng giống như Rust thông thường, tuy nhiên *unsafe Rust* tự do và khó kiểm soát hơn Rust.

Tại sao phải sinh ra unsafe rust! Lý do là Rust compiler đôi khi tỏ ra quá an toàn khi thực hiện biên dịch chương trình. Đôi khi code của bạn rơi vào trường hợp không chắc chắn an toàn hay không, thì mặc định compiler sẽ từ chối và coi đây là một lỗi, mặc dù có thể không phải như vậy. Cơ chế này giúp ta ngăn ngừa các lỗi tiềm ẩn, tuy nhiên nếu lập trình viên cố tình muốn thực hiện các đoạn code này thì sao? Ta sẽ nói với compiler, "Hãy tin ở tôi, tôi hiểu mình đang làm gì". Đây là một sự đánh đổi, code của bạn sẽ được thực thi với một tỉ lệ rủi ro nào đó, tính toán sai có thể dẫn đến các lỗi về bộ nhớ (memory) như con trỏ null (null pointer), rò rỉ bộ nhớ (leaked memory), ...

Một lí do khác khiến cơ chế unsafe được tạo ra là Rust muốn tiếp cận với phần cứng của hệ điều hành giống như những ngôn ngữ lập trình bậc thấp, mà bản chất các ngôn ngữ này đều chạy cơ chế unsafe. Nếu Rust không cũng cấp unsafe, sẽ rất khó để bạn có thể làm được những điều này. Hãy cùng khám phá những điều có thê làm với unsafe Rust.

### Sức mạnh của Unsafe

To switch to unsafe Rust, use the `unsafe` keyword and then start a new block
that holds the unsafe code. You can take five actions in unsafe Rust, called
*unsafe superpowers*, that you can’t in safe Rust. Those superpowers include
the ability to:

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

Bây giờ hãy đi lần lượt 5 tính năng trên
### Dereferencing một Raw Pointer

In Chapter 4, in the [“Dangling References”][dangling-references]<!-- ignore
--> section, we mentioned that the compiler ensures references are always
valid. Unsafe Rust has two new types called *raw pointers* that are similar to
references. As with references, raw pointers can be immutable or mutable and
are written as `*const T` and `*mut T`, respectively. The asterisk isn’t the
dereference operator; it’s part of the type name. In the context of raw
pointers, *immutable* means that the pointer can’t be directly assigned to
after being dereferenced.

Phần [“Dangling References”][dangling-references]<!-- ignore --> trong chương 4, đề cập đến việc compiler luôn luôn kiểm tra tham chiếu có hợp lệ hay không. Unsafe Rust cung cấp kiểu tham chiếu mới có tên là *raw pointers*, tương tự như tham chiếu trong safe Rust. Raw pointers có thể thay đổi được (mutable) hoặc không (immutable), cú pháp tương ứng ở đây là `*mut T` và `*const T`. Dấu `*` ở đây không phải là toán tử dereference mà chỉ đơn giản là cú pháp bắt buộc của raw pointer. Chú ý rằng, *immutable* ở đây nghĩa là con trỏ không thể được truy cập sau khi đã dereferenced.

Different from references and smart pointers, raw pointers:

* Are allowed to ignore the borrowing rules by having both immutable and
  mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup

By opting out of having Rust enforce these guarantees, you can give up
guaranteed safety in exchange for greater performance or the ability to
interface with another language or hardware where Rust’s guarantees don’t apply.

Listing 19-1 shows how to create an immutable and a mutable raw pointer from
references.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-01/src/main.rs:here}}
```

<span class="caption">Listing 19-1: Creating raw pointers from references</span>

Notice that we don’t include the `unsafe` keyword in this code. We can create
raw pointers in safe code; we just can’t dereference raw pointers outside an
unsafe block, as you’ll see in a bit.

We’ve created raw pointers by using `as` to cast an immutable and a mutable
reference into their corresponding raw pointer types. Because we created them
directly from references guaranteed to be valid, we know these particular raw
pointers are valid, but we can’t make that assumption about just any raw
pointer.

Next, we’ll create a raw pointer whose validity we can’t be so certain of.
Listing 19-2 shows how to create a raw pointer to an arbitrary location in
memory. Trying to use arbitrary memory is undefined: there might be data at
that address or there might not, the compiler might optimize the code so there
is no memory access, or the program might error with a segmentation fault.
Usually, there is no good reason to write code like this, but it is possible.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-02/src/main.rs:here}}
```

<span class="caption">Listing 19-2: Creating a raw pointer to an arbitrary
memory address</span>

Recall that we can create raw pointers in safe code, but we can’t *dereference*
raw pointers and read the data being pointed to. In Listing 19-3, we use the
dereference operator `*` on a raw pointer that requires an `unsafe` block.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-03/src/main.rs:here}}
```

<span class="caption">Listing 19-3: Dereferencing raw pointers within an
`unsafe` block</span>

Creating a pointer does no harm; it’s only when we try to access the value that
it points at that we might end up dealing with an invalid value.

Note also that in Listing 19-1 and 19-3, we created `*const i32` and `*mut i32`
raw pointers that both pointed to the same memory location, where `num` is
stored. If we instead tried to create an immutable and a mutable reference to
`num`, the code would not have compiled because Rust’s ownership rules don’t
allow a mutable reference at the same time as any immutable references. With
raw pointers, we can create a mutable pointer and an immutable pointer to the
same location and change data through the mutable pointer, potentially creating
a data race. Be careful!

With all of these dangers, why would you ever use raw pointers? One major use
case is when interfacing with C code, as you’ll see in the next section,
[“Calling an Unsafe Function or
Method.”](#calling-an-unsafe-function-or-method)<!-- ignore --> Another case is
when building up safe abstractions that the borrow checker doesn’t understand.
We’ll introduce unsafe functions and then look at an example of a safe
abstraction that uses unsafe code.

### Calling an Unsafe Function or Method

The second type of operation that requires an unsafe block is calls to unsafe
functions. Unsafe functions and methods look exactly like regular functions and
methods, but they have an extra `unsafe` before the rest of the definition. The
`unsafe` keyword in this context indicates the function has requirements we
need to uphold when we call this function, because Rust can’t guarantee we’ve
met these requirements. By calling an unsafe function within an `unsafe` block,
we’re saying that we’ve read this function’s documentation and take
responsibility for upholding the function’s contracts.

Here is an unsafe function named `dangerous` that doesn’t do anything in its
body:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

We must call the `dangerous` function within a separate `unsafe` block. If we
try to call `dangerous` without the `unsafe` block, we’ll get an error:

```console
{{#include ../listings/ch19-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

By inserting the `unsafe` block around our call to `dangerous`, we’re asserting
to Rust that we’ve read the function’s documentation, we understand how to use
it properly, and we’ve verified that we’re fulfilling the contract of the
function.

Bodies of unsafe functions are effectively `unsafe` blocks, so to perform other
unsafe operations within an unsafe function, we don’t need to add another
`unsafe` block.

#### Creating a Safe Abstraction over Unsafe Code

Just because a function contains unsafe code doesn’t mean we need to mark the
entire function as unsafe. In fact, wrapping unsafe code in a safe function is
a common abstraction. As an example, let’s study a function from the standard
library, `split_at_mut`, that requires some unsafe code and explore how we
might implement it. This safe method is defined on mutable slices: it takes one
slice and makes it two by splitting the slice at the index given as an
argument. Listing 19-4 shows how to use `split_at_mut`.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-04/src/main.rs:here}}
```

<span class="caption">Listing 19-4: Using the safe `split_at_mut`
function</span>

We can’t implement this function using only safe Rust. An attempt might look
something like Listing 19-5, which won’t compile. For simplicity, we’ll
implement `split_at_mut` as a function rather than a method and only for slices
of `i32` values rather than for a generic type `T`.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-05/src/main.rs:here}}
```

<span class="caption">Listing 19-5: An attempted implementation of
`split_at_mut` using only safe Rust</span>

This function first gets the total length of the slice. Then it asserts that
the index given as a parameter is within the slice by checking whether it’s
less than or equal to the length. The assertion means that if we pass an index
that is greater than the length to split the slice at, the function will panic
before it attempts to use that index.

Then we return two mutable slices in a tuple: one from the start of the
original slice to the `mid` index and another from `mid` to the end of the
slice.

When we try to compile the code in Listing 19-5, we’ll get an error.

```console
{{#include ../listings/ch19-advanced-features/listing-19-05/output.txt}}
```

Rust’s borrow checker can’t understand that we’re borrowing different parts of
the slice; it only knows that we’re borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two
slices aren’t overlapping, but Rust isn’t smart enough to know this. When we
know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.

Listing 19-6 shows how to use an `unsafe` block, a raw pointer, and some calls
to unsafe functions to make the implementation of `split_at_mut` work.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-06/src/main.rs:here}}
```

<span class="caption">Listing 19-6: Using unsafe code in the implementation of
the `split_at_mut` function</span>

Recall from [“The Slice Type”][the-slice-type]<!-- ignore --> section in
Chapter 4 that slices are a pointer to some data and the length of the slice.
We use the `len` method to get the length of a slice and the `as_mut_ptr`
method to access the raw pointer of a slice. In this case, because we have a
mutable slice to `i32` values, `as_mut_ptr` returns a raw pointer with the type
`*mut i32`, which we’ve stored in the variable `ptr`.

We keep the assertion that the `mid` index is within the slice. Then we get to
the unsafe code: the `slice::from_raw_parts_mut` function takes a raw pointer
and a length, and it creates a slice. We use this function to create a slice
that starts from `ptr` and is `mid` items long. Then we call the `add`
method on `ptr` with `mid` as an argument to get a raw pointer that starts at
`mid`, and we create a slice using that pointer and the remaining number of
items after `mid` as the length.

The function `slice::from_raw_parts_mut` is unsafe because it takes a raw
pointer and must trust that this pointer is valid. The `add` method on raw
pointers is also unsafe, because it must trust that the offset location is also
a valid pointer. Therefore, we had to put an `unsafe` block around our calls to
`slice::from_raw_parts_mut` and `add` so we could call them. By looking at
the code and by adding the assertion that `mid` must be less than or equal to
`len`, we can tell that all the raw pointers used within the `unsafe` block
will be valid pointers to data within the slice. This is an acceptable and
appropriate use of `unsafe`.

Note that we don’t need to mark the resulting `split_at_mut` function as
`unsafe`, and we can call this function from safe Rust. We’ve created a safe
abstraction to the unsafe code with an implementation of the function that uses
`unsafe` code in a safe way, because it creates only valid pointers from the
data this function has access to.

In contrast, the use of `slice::from_raw_parts_mut` in Listing 19-7 would
likely crash when the slice is used. This code takes an arbitrary memory
location and creates a slice 10,000 items long.

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-07/src/main.rs:here}}
```

<span class="caption">Listing 19-7: Creating a slice from an arbitrary memory
location</span>

We don’t own the memory at this arbitrary location, and there is no guarantee
that the slice this code creates contains valid `i32` values. Attempting to use
`values` as though it’s a valid slice results in undefined behavior.

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
