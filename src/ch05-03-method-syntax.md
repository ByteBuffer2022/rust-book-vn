## Method

*Method* (phương thức) khá tương đồng với *functions* (hàm) ở điểm: đều sử dụng từ khóa `fn` kèm theo tên, đều có tham số truyền vào và giá trị trả về, đều chứa các đoạn code để thực thi khi được gọi trong chương trình. Điểm khác biệt với functions ở chỗ, methods được định nghĩa ở bên trong struct (có thể là enum hoặc trait, ta sẽ đi sâu hơn trong chương 6 và 17); tham số đầu tiên của method luôn là `self`, đại diện cho instance của struct đó.

### Định nghĩa Methods

Biến đổi hàm `area` thành phương thức `area` như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

<span class="caption">Listing 5-13: Định nghĩa một phương thức `area` thuộc struct `Rectangle`</span>

Để định nghĩa các phương thức của `Rectangle`, ta sẽ sử dụng từ khóa `impl` (implementation) kèm theo một block của `Rectangle`. Mọi thứ ở trong `impl` block sẽ thuộc về `Rectangle`. Tiếp theo, đưa hàm `area` vào bên trong `impl` block và thêm `self` vào làm tham số đầu tiên của hàm đó. Trong hàm `main`, sử dụng *method syntax* để gọi phương thức `area` của `Rectangle` instance. Cú pháp ở đây được sử dụng bằng cách thêm dấu chấm sau khi gọi instance.

Phần khai báo của `area`, ta sử dụng `&self` thay vì `rectangle: &Rectangle`, `&self` là viết tắt của `self: &Self`. Ở trong `impl` block, `Self` đại diện cho kiểu dữ liệu mà nó được implement. Method phải có tham số `self` ở đầu tiên, vì vậy Rust giúp ta viết thuận tiện hơn với việc chỉ cần `&self` thay vì `self: &Self`. Để ý rằng vẫn cần có `&` ở trước `self` để báo hiệu việc mượn instance. Method có thể chiếm quyền sở hữu của `self`, vì vậy ta cần cân nhắc khi sử dụng.

Sử dụng `&self` bởi cũng giống như `&Rectangle`: ta không muốn chiếm quyền sở hữu của biến, chỉ cần mượn để đọc. Nếu muốn thay đổi giá trị của instance, hãy sử dụng `&mut self` làm tham số đầu tiên của method. Method chiếm quyền sở hữu của instance bằng cách dùng `self` rất ít khi được dùng; kĩ thuật này chỉ được sử dụng khi ta không muốn người gọi phương thức có thể sử dụng tiếp instance này sau khi gọi mà thôi.

Mục đích chính của việc sử dụng method thay vì function đó là: có thể nhóm mọi thứ liên quan đến instance của một kiểu dữ liệu vào với nhau, giúp cho việc quản lí code cũng như bảo trì code sau này trở nên dễ dàng hơn.

Chú ý rằng ta có thể đặt tên method trùng với tên trường của struct. Ví dụ, method `width` của `Rectangle`:
<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-06-method-field-interaction/src/main.rs:here}}
```

Ở đây, phương thức `width` sẽ trả về `true` nếu giá trị của `width` lớn hơn 0, và `false` nếu trong trường hợp còn lại. Trong hàm `main`, nếu sử dụng `rect1.width()`, Rust sẽ hiểu đây là method `width`, nếu không có cặp ngoặc tròn `()`, Rust sẽ hiểu đây là trường `width`.

Thông thường, ta sẽ đặt tên method trùng với tên trường nếu chỉ muốn trả về giá trị của chính trường đó. Những method dạng này được gọi là *getters*, Rust không implement chúng một cách tự động như những ngôn ngữ khác. Getters rất hữu dụng vì bạn có thể điều khiển được quyền truy cập biến, trường trong struct; cho phép trường đó có thể được thay đổi hay không, hay chỉ có quyền đọc mà thôi. Chủ đề về quyền truy cập (public và private) sẽ được nói đến trong chương 7.

> ### Toán tử `->`
>
> Trong C và C++, có hai toán tử được dùng để gọi method: dùng `.` nếu bạn đang dùng trực tiếp object để gọi method, dùng `->` nếu gọi method dùng con trỏ trỏ đến object đó.
>
> Rust không hỗ trợ toán tử `->`; thay vào đó, Rust cung cấp tính năng được gọi là *automatic referencing and dereferencing*.
>
> Đây là cách hoạt động: mỗi khi bạn gọi một method như `object.something()`, Rust sẽ tự động thêm `&`, `&mut` hay `*` vào object. Xem xét ví dụ sau:
>
> <!-- CAN'T EXTRACT SEE BUG https://github.com/rust-lang/mdBook/issues/1127 -->
> ```rust
> # #[derive(Debug,Copy,Clone)]
> # struct Point {
> #     x: f64,
> #     y: f64,
> # }
> #
> # impl Point {
> #    fn distance(&self, other: &Point) -> f64 {
> #        let x_squared = f64::powi(other.x - self.x, 2);
> #        let y_squared = f64::powi(other.y - self.y, 2);
> #
> #        f64::sqrt(x_squared + y_squared)
> #    }
> # }
> # let p1 = Point { x: 0.0, y: 0.0 };
> # let p2 = Point { x: 5.0, y: 6.5 };
> p1.distance(&p2);
> (&p1).distance(&p2);
> ```
> 
> Cách gọi method `p1.distance` phía trên sẽ dễ nhìn hơn cách phía dưới. Rust sẽ tự động thêm tham chiếu cho p1, ngoài ra Rust có thể kiểm tra được khi nào method chỉ được đọc (`&self`), có thể thay đổi đươc (`&mut self`) hay consuming (`self`). Đây là một cơ chế khá hay mà Rust hỗ trợ cho chúng ta trong quá trình lập trình.

### Methods nhiều tham số

Lần này ta sẽ thử implement thêm method thứ 2 của `Rectangle` struct. Method này có nhiệm vụ kiểm tra xem hình chữ nhật truyền vào có đặt vừa vào bên trong hình chữ nhật cho trước hay không. Method này sẽ có tên là `can_hold` với kiểu trả về là `bool`:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-14/src/main.rs}}
```

<span class="caption">Listing 5-14: Sử dụng `can_hold` method</span>

Output sẽ trông giống như bên dưới, vì cả 2 chiều của `rect2` đều nhỏ hơn `rect1` còn `rect3` thì lớn hơn `rect1`:

```text
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

Method mới này cũng sẽ được đặt trong `impl Rectangle` block. Đặt tên cho method là `can_hold`, có tham số truyền vào là một `Rectangle` khác. Như đoạn code dưới đây:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

<span class="caption">Listing 5-15: Implementing `can_hold` method của struct `Rectangle`</span>

### Associated Functions

Tất cả các hàm được định nghĩa bên trong `impl` đều được gọi là *associated functions* vì chúng đều có liên quan đến struct được `impl`. Ta có thể định nghĩa associated functions mà không có `self` làm tham số truyền vào (do đó không gọi là methods) vì chúng không cần instance của struct đó để hoạt động. Ví dụ như hàm `from` trong `String::from` là một associated function.

Những associated functions mà không phải là method thường được dùng để khởi tạo instance cho struct.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Để gọi một associated function, sử dụng dấu `::` với tên của struct; `let sq = Rectangle::square(3);` là một ví dụ. Cú pháp `::` được sử dụng cho cả associated functions và namespaces để khởi tạo modules. Ta sẽ bàn thêm trong chương 7.

### Multiple `impl` Blocks

Mỗi struct có thể có nhiều `impl` blocks. Ví dụ , Listing 5-15 cũng tương đương với Listing 5-16, đều sở hữu các method như nhau.

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

<span class="caption">Listing 5-16: Viết lại Listing 5-15 sử dụng multiple `impl` blocks</span>

Không có mục đích cụ thể nào cho việc tạo nhiều `impl` blocks cả, chỉ đơn giản là cú pháp này được chấp nhận trong Rust. Tuy nhiên đôi khi nó cũng khá hữu dụng, xem thêm chương 10 phần generic types và traits.

## Tổng kết

Structs giúp bạn có thể tạo ra một kiểu dữ liệu mà mình mong muốn. Sử dụng structs, ta có thể nhóm các hàm, biến có mối liên hệ với nhau và làm cho code rõ ràng, dễ hiểu hơn. Khi `impl` blocks, có 2 loại hàm là associated functions và method (một dạng đặc biệt của asscociated functions), giúp bạn thể hiện các hành vi (behavior) của instance đó.

Structs không phải là cách duy nhất để tạo một kiểu dữ liệu mới: hãy xem thêm kiểu enum trong Rust.