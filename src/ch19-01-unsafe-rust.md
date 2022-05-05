## Unsafe Rust

Tất cả các đoạn code đã trình bày trước đó đều được Rust bảo vệ và ngăn chặn nếu gặp lỗi ngay từ compile time (Rust's memory safety guarantees). Tuy nhiên, Rust cũng có một tính năng ẩn khác mà không hề được compiler kiểm định và soát lỗi khi biên dịch: đó là *unsafe Rust*. Nó cũng giống như Rust thông thường, tuy nhiên *unsafe Rust* tự do và khó kiểm soát hơn Rust.

Tại sao phải sinh ra unsafe rust! Lý do là Rust compiler đôi khi tỏ ra quá an toàn khi thực hiện biên dịch chương trình. Khi code của bạn rơi vào trường hợp không chắc chắn an toàn hay không, thì mặc định compiler sẽ từ chối và coi đây là một lỗi, mặc dù có thể không phải như vậy. Cơ chế này giúp ta ngăn ngừa các lỗi tiềm ẩn, tuy nhiên nếu lập trình viên cố tình muốn thực hiện các đoạn code này thì sao? Ta sẽ nói với compiler, "Hãy tin ở tôi, tôi hiểu mình đang làm gì". Đây là một sự đánh đổi, code của bạn sẽ được thực thi với một tỉ lệ rủi ro nào đó, tính toán sai có thể dẫn đến các lỗi về bộ nhớ (memory) như truy cập vào con trỏ null (null pointer), rò rỉ bộ nhớ (leaked memory), ...

Một lí do khác khiến cơ chế unsafe được tạo ra là Rust muốn tiếp cận với phần cứng của hệ điều hành giống như những ngôn ngữ lập trình bậc thấp, mà bản chất các ngôn ngữ này đều chạy cơ chế unsafe. Nếu Rust không cũng cấp unsafe, sẽ rất khó để bạn có thể làm được những điều này. Hãy cùng khám phá những điều có thê làm với unsafe Rust.

### Sức mạnh của Unsafe

Để sử dụng unsafe Rust, dùng keyword `unsafe` và tạo một block chứa các unsafe code mà bạn muốn. Có 5 điều mà unsafe Rust có thể làm mà bạn sẽ không thể có được ở Rust thông thường:

* Dereference một raw pointer (các khái niệm Dereference và raw pointer sẽ được giải thích sau)
* Gọi unsafe function hoặc unsafe method
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

### Gọi đến Unsafe Function hoặc Unsafe Method

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

#### Tạo một Safe Abstraction bằng Unsafe Code

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

#### Sử dụng `extern` để gọi tới External Code

Trong một vài trường hợp, ta muốn sử dụng Rust để tương tác với một ngôn ngữ lập trình khác. Trong trường hợp này, Rust cung cấp từ khóa `extern`, giúp ta có thể dễ dàng hơn trong việc sử dụng *Foreign Function Interface (FFI)*. FFI là cách để một ngôn ngữ lập trình có thể định nghĩa các hàm để ngôn ngữ khác có thể gọi tới.

Listing 19-8 giải thích cách thực hiện với hàm `abs` từ thư viện của ngôn ngữ C. Hàm này được định nghĩa ở trong `extern` blocks và được coi như là unsafe code trong Rust. Lý do là vì các ngôn ngữ khác không có cơ chế về bảo vệ và quản lí vùng nhớ như Rust, do đó Rust không thể kiểm soát chúng một cách thông thường được.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-08/src/main.rs}}
```

<span class="caption">Listing 19-8: Khai báo và gọi một `extern` function được định nghĩa bởi ngôn ngữ khác</span>

> #### Gọi Rust Functions từ ngôn ngữ khác
>
> Ta hoàn toàn có thể sử dụng `extern` để tạo một interface cho phép các ngôn ngữ lập trình khác gọi đến hàm của Rust. Thay vì một `extern` block, ta sẽ sử dụng từ khóa `extern` kèm theo ABI (application binary interface) cụ thể ngay phía trước từ khóa `fn`. Annotation `#[no_mangle]` sẽ được sử dụng để chỉ dẫn cho compiler không biến đổi (mangle) tên hàm. *Mangling* xảy ra khi compiler thay đổi tên của hàm phục vụ cho quá trình biên dịch nhưng sẽ khó nhìn hơn cho lập trình viên. Mỗi ngôn ngữ sẽ có cách biến đổi tên của riêng mình, vì vậy ta phải disable cách biến đổi tên của Rust (Rust compiler's name mangling).
>
> Ở ví dụ sau đây, hàm `call_from_c` sẽ được gọi từ code C, sau khi đã được biên dịch và liên kết các thư viện cần thiết.
>
> ```rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
>     println!("Just called a Rust function from C!");
> }
> ```
>
> Trường hợp này không bắt buộc dùng từ khóa `unsafe`.

### Truy cập hoặc thay đổi một Mutable Static Variable

Rust không định nghĩa kiểu biến *global*, lí do là bởi quyền sở hữu (ownership rules). Nếu 2 threads cùng truy cập một biến global, có thể sẽ dẫn đến hiện tượng data race.

Trong Rust, biến global được gọi là biến *static*. Listing 19-9 là một ví dụ về cách sử dụng biến static.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-09/src/main.rs}}
```

<span class="caption">Listing 19-9: Định nghĩa và sử dụng một immutable static variable</span>

Biến static có nhiều điểm tương đồng với hằng số (constants), điều đã được nhắc đến trong phần [“Differences Between Variables and Constants”][differences-between-variables-and-constants]<!-- ignore --> ở chương 3. Biến static chỉ được tham chiếu với lifetime là `static`, nghĩa là Rust có thể biết được lifetime của biến đó ngay từ đầu và ta không có cách nào thay đổi. Truy cập vào một immutable static variable là một hành động an toàn. 

Constants và immutable static variable có nhiều điểm tưởng đồng, nhưng chúng khác nhau ở chỗ giá trị của biến static có địa chỉ cố định. Sử dụng giá trị này, ta sẽ luôn truy cập đến một vùng nhớ duy nhất. Đối với contants, dữ liệu sẽ được sao chép tới một vùng nhớ khác mỗi khi ta truy cập vào hằng số đó.

Một điểm khác biệt nữa giữa contants và static variable là biến static có thể thay đổi được. Tuy nhiên việc truy cập và thay đổi một mutable static variable là một hành động *unsafe*. Listing 19-10 chỉ ra cách khai báo, truy cập và thay đổi một mutable static variable có tên là `COUNTER`.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-10/src/main.rs}}
```

<span class="caption">Listing 19-10: Đọc và ghi một mutable static variable là một hành động unsafe</span>

Sử dụng từ khóa `mut` để khai báo một mutable static variable. Các đoạn code liên quan đến việc đọc à ghi biến `COUNTER` đều phải được đặt trong `unsafe` block. Đoạn code trên sẽ in ra màn hình `COUNTER: 3` như kì vọng bởi đây là chương trình single threaded. Đa luồng với chương trình trên có thể sẽ dẫn tới hiện tượng data races.

Với việc thay đổi dữ liệu với quyền truy cập toàn cục, rất khó để đảm bảo rằng không có data race xảy ra, đó là lý do Rust phải đưa chúng vào trong unsafe. Nếu có thể, hay sử dụng các kĩ thuật về đa luồng và lập trình song song được nhắc đến trong chương 16 để giúp cho chương trình an toàn hơn.

### Implementing một Unsafe Trait

Một trường hợp khác phải dùng `unsafe` là khi implement một unsafe trait. Trait được gọi là unsafe khi ít nhất một method trong nó khiến compiler không thể chắc chắn rằng method đó an toàn. Ta có thể khai báo một `unsafe` trait bằng cách thêm từ khóa `unsafe` trước trait đó đồng thời đánh dấu `unsafe` cho trait khi implement. Ví dụ:

```rust
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-11/src/main.rs}}
```

<span class="caption">Listing 19-11: Định nghĩa và implement một unsafe trait</span>

Ta sẽ giao kèo với compiler rằng sẽ chịu trách nhiệm cho các unsafe method bằng cách sử dụng `unsafe impl`.

### Truy cập vào các trường trong một Union

Một `union` tương tự như một `struct`, nhưng chỉ có một trường dữ liệu được sử dụng trong một instance ở một thời điểm. Truy cập vào các trường trong union là một hành động unsafe. Bạn có thể đọc thêm tại đây [the Rust Reference][reference].

### Khi nào thì sử dụng Unsafe Code

Sử dụng `unsafe` khi muốn có một trong 5 hành động (superpowers) đã nhắc đến ở phía trên. Hãy sử dụng chỉ khi thực sự cần thiết, bởi bạn chứ không phải compiler sẽ là người phải chịu trách nhiệm nếu cho các lỗi phát sinh sau này.

[dangling-references]:
ch04-02-references-and-borrowing.html#dangling-references
[differences-between-variables-and-constants]:
ch03-01-variables-and-mutability.html#constants
[extensible-concurrency-with-the-sync-and-send-traits]:
ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits
[the-slice-type]: ch04-03-slices.html#the-slice-type
[reference]: ../reference/items/unions.html
