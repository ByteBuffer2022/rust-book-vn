## Đưa Đường dẫn vào Scope với từ khoá `use`

Có vẻ như các đường dẫn mà chúng tôi đã viết để gọi các hàm cho đến nay là dài 
và lặp đi lặp lại một cách bất tiện. Ví dụ: trong Listing 7-7, 
cho dù chúng ta chọn đường dẫn tuyệt đối hay tương đối đến hàm `add_to_waitlist`,
mỗi khi chúng ta muốn gọi` add_to_waitlist`, chúng ta cũng phải chỉ định `front_of_house` và` hosting`.
May mắn thay, có một cách để đơn giản hóa quy trình này. Chúng ta có thể tạo một lối tắt đến một đường dẫn với từ khóa `use` một lần,
sau đó sử dụng tên ngắn hơn ở mọi nơi khác trong scope.

Trong Listing 7-11, chúng tôi đưa module `crate :: front_of_house :: hosting` vào scope của hàm` eat_at_restaurant`
vì vậy chúng tôi chỉ phải chỉ định `hosting :: add_to_waitlist` để gọi hàm` add_to_waitlist` trong `eat_at_restaurant`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

<span class="caption">Listing 7-11: Đưa một module vào scope với
`use`</span>

Thêm `use` và một đường dẫn trong một scope tương tự như tạo một liên kết tượng trưng trong hệ thống tệp. 
Bằng cách thêm `use crate :: front_of_house :: hosting` trong crate root,` hosting` bây giờ là một tên hợp lệ trong scope đó, 
giống như module `hosting` đã được xác định trong crate root. 
Các đường dẫn được đưa vào scope với `use` cũng kiểm tra quyền riêng tư, giống như bất kỳ đường dẫn nào khác.

Lưu ý rằng `use` chỉ tạo lối tắt cho scope cụ thể mà` use` xảy ra. Listing 7-12 di chuyển hàm `eat_at_restaurant` 
vào một module con mới có tên là` customer`, sau đó là một scope khác với câu lệnh `use` và thân hàm sẽ không biên dịch:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

<span class="caption">Listing 7-12: Câu lệnh `use` chỉ áp dụng trong scope mà nó nằm trong</span>

Lỗi trình biên dịch cho thấy rằng phím tắt không còn áp dụng trong module `customer`:

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-12/output.txt}}
```

Lưu ý rằng cũng có một cảnh báo rằng `use` không còn được sử dụng trong scope của nó nữa!
Để khắc phục sự cố này, hãy di chuyển luôn cả `use` trong module ` customer` hoặc tham chiếu lối tắt 
trong module cha với `super :: hosting` trong module con` customer`.

### Tạo các đường dẫn `use` theo kiểu Idiomatic

Trong Listing 7-11, bạn có thể thắc mắc tại sao chúng tôi chỉ định `use crate :: front_of_house :: hosting` 
và sau đó gọi là` hosting :: add_to_waitlist` trong `eat_at_restaurant` thay vì chỉ định đường dẫn` use` đến tận hàm `add_to_waitlist` 
để đạt được kết quả tương tự, như trong Listing 7-13.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

<span class="caption">Listing 7-13: Đưa hàm `add_to_waitlist` vào scope với` use`, đó là unidiomatic</span>

Mặc dù cả Listing 7-11 và 7-13 đều hoàn thành cùng một nhiệm vụ, nhưng Listing 7-11 là cách theo khuôn mẫu để đưa một hàm vào scope với `use`. 
Đưa module cha của hàm vào scope với `use` có nghĩa là chúng ta phải chỉ định module cha khi gọi hàm. 
Việc chỉ định module cha khi gọi hàm làm rõ rằng hàm không được xác định cục bộ trong khi vẫn giảm thiểu sự lặp lại của đường dẫn đầy đủ.
Mã trong Listing 7-13 không rõ ràng về nơi mà `add_to_waitlist` được định nghĩa.

Mặt khác, khi nhập các structs, enums, và các item khác với `use`,
it’s khuôn mẫu để chỉ định đường dẫn đầy đủ. Listing 7-14 cho thấy một cách theo khuôn mẫu
để đưa cấu trúc `HashMap` của thư viện tiêu chuẩn vào scope của binary crate.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

<span class="caption">Listing 7-14: Đưa `HashMap` vào scope một cách theo khuôn mẫu/span>

Không có lý do chính đáng nào đằng sau mẫu này: đó chỉ là quy ước đã xuất hiện và mọi người đã quen với việc đọc và viết mã Rust theo cách này.

Ngoại lệ cho khuôn mẫu này là nếu chúng ta đưa hai mục có cùng tên vào scope với câu lệnh `use`, 
bởi vì Rust không cho phép điều đó. Listing 7-15 cho thấy cách đưa hai kiểu `Result`
vào scope có cùng tên nhưng khác module cha và cách tham chiếu đến chúng.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

<span class="caption">Listing 7-15: Đưa hai kiểu có cùng tên vào cùng một scope yêu cầu sử dụng module cha của chúng</span>

Như bạn có thể thấy, việc sử dụng các module cha sẽ phân biệt hai kiểu `Result`. 
Nếu thay vào đó, chúng tôi chỉ định `use std :: fmt :: Result` và` use std :: io :: Result`, chúng tôi sẽ có hai loại `Result` 
trong cùng một scope và Rust sẽ không biết chúng tôi muốn nói đến loại nào khi chúng tôi đã sử dụng `Result`

### Cung cấp Tên Mới với Từ khoá `as`

Có một giải pháp khác cho vấn đề đưa hai kiểu tên giống nhau vào cùng một 
scope với `use`: sau đường dẫn, chúng ta có thể chỉ định` as` 
và một tên cục bộ mới hoặc bí danh cho kiểu đó. Listing 7-16 chỉ ra một cách khác để viết mã trong 
Listing 7-15 bằng cách đổi tên một trong hai kiểu `Result` bằng cách sử dụng` as`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

<span class="caption">Listing 7-16: Đổi tên một loại khi nó được đưa vào scope với từ khóa `as`</span>

Trong câu lệnh `use` thứ hai, chúng tôi đã chọn tên mới` IoResult` cho kiểu `std :: io :: Result`, 
sẽ không xung đột với` Result` từ `std :: fmt` mà chúng tôi có cũng được đưa vào scope. 
Listing 7-15 và Listing 7-16 được coi là theo khuôn mẫu, vì vậy sự lựa chọn là tùy thuộc vào bạn!

### Re-exporting tên với `pub use`

Khi chúng tôi đưa một tên vào scope với từ khóa `use`, tên có sẵn trong scope mới là private.
Để cho phép code gọi code của chúng tôi tham chiếu đến tên đó như thể nó đã được xác định trong scope của code đó,
chúng tôi có thể kết hợp `pub` và` use`. Kỹ thuật này được gọi là `re-exporting` vì chúng tôi đang đưa một item  
vào scope nhưng cũng làm cho item đó khả dụng để những người khác đưa vào scope của họ.

Listing 7-17 hiển thị code trong Listing 7-11 với `use` trong module root được thay đổi thành` pub use`.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

<span class="caption">Listing 7-17: Tạo một tên khả dụng cho bất kỳ code nào để sử dụng từ một scope mới với `pub use`</span>

Trước thay đổi này, external code sẽ phải gọi hàm `add_to_waitlist` bằng cách
sử dụng đường dẫn` restaurant :: front_of_house :: hosting :: add_to_waitlist () `.
Bây giờ, `pub use` này đã re-exported lại module `hosting` từ module root, external code
hiện có thể sử dụng đường dẫn` restaurant :: hosting :: add_to_waitlist () `để thay thế.

Re-exporting hữu ích khi cấu trúc bên trong của code của bạn khác với cách 
các lập trình viên gọi code của bạn sẽ nghĩ về domain. 
Ví dụ, trong phép ẩn dụ về nhà hàng này, những người điều hành nhà hàng nghĩ về “front of house” và “back of house”. 
Nhưng khách hàng đến thăm một nhà hàng có thể sẽ không nghĩ về các bộ phận của nhà hàng theo những thuật ngữ đó.
Với `pub use`, chúng ta có thể viết code của mình với một cấu trúc nhưng hiển thị một cấu trúc khác. 
Làm như vậy làm cho thư viện của chúng tôi được tổ chức tốt cho các lập trình viên làm việc trên thư viện 
và các lập trình viên gọi thư viện. Chúng ta sẽ xem xét một ví dụ khác về `pub use` và cách nó ảnh hưởng đến 
 crate’s documentation của bạn trong phần [“Exporting a
Convenient Public API with `pub use`”][ch14-pub-use]<!-- ignore --> của Chapter 14.

### Sử dụng External Packages

Trong Chương 2, chúng tôi đã lập trình một guessing game project sử dụng một external
package gọi là `rand` để lấy số ngẫu nhiên. Để sử dụng `rand` trong project của bạn,
chúng tôi đã thêm dùng này vào *Cargo.toml*:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch02-00-guessing-game-tutorial.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Thêm `rand` như là một dependency trong *Cargo.toml* yêu cầu Cargo
tải xuống gói `rand` và bất kỳ  dependencies từ [crates.io](https://crates.io/) và cung cấp `rand` cho project của chúng tôi.

Sau đó, để đưa các định nghĩa `rand` vào scope của package của chúng tôi, 
chúng tôi đã thêm một dòng `use` bắt đầu bằng tên của crate, `rand`, 
và liệt kê các mục chúng tôi muốn đưa vào scope.Nhớ lại rằng trong phần [“Generating a Random
Number”][rand]<!-- ignore --> trong Chương 2,chúng tôi đã đưa đặc điểm `Rng` vào scope và gọi hàm` rand :: thread_rng`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Các thành viên của cộng đồng Rust đã cung cấp nhiều package tại
[crates.io](https://crates.io/), và pull bất kỳ trong số chúng vào package của bạn bao gồm các bước tương tự: liệt kê chúng trong 
file package’s *Cargo.toml*  và sử dụng `use` đưa các item từ crate của chúng vào scope.

Note that the standard library (`std`) is also a crate that’s external to our
package. Because the standard library is shipped with the Rust language, we
don’t need to change *Cargo.toml* to include `std`. But we do need to refer to
it with `use` to bring items from there into our package’s scope. For example,
with `HashMap` we would use this line:

```rust
use std::collections::HashMap;
```

Đây là một đường dẫn tuyệt đối bắt đầu bằng `std`, tên của library crate tiêu chuẩn.

### Sử dụng các đường dẫn lồng nhau để clear up danh sách `use` lớn

Nếu chúng tôi đang sử dụng nhiều item được xác định trong cùng một crate hoặc cùng một module,
thì việc liệt kê từng item trên một dòng riêng có thể chiếm nhiều không gian theo chiều dọc trong file của chúng tôi. 
Ví dụ: hai câu lệnh `use` mà chúng ta có trong Guessing Game trong Listing 2-4 đưa các item từ` std` vào scope:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Thay vào đó, chúng ta có thể sử dụng các đường dẫn lồng nhau để đưa các item giống nhau vào scope trong một dòng. 
Chúng tôi thực hiện điều này bằng cách chỉ định phần chung của đường dẫn, theo sau là hai dấu hai chấm,
sau đó là dấu ngoặc nhọn xung quanh danh sách các phần khác nhau của đường dẫn, như được hiển thị trong Listing 7-18.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

<span class="caption">Listing 7-18: Chỉ định một đường dẫn lồng nhau để đưa nhiều item có cùng tiền tố vào scope</span>

Trong các chương trình lớn hơn, đưa nhiều item vào scope từ cùng một crate hoặc module bằng cách sử dụng các 
đường dẫn lồng nhau có thể làm giảm số lượng các câu lệnh `use` riêng biệt cần thiết đi rất nhiều!

Chúng ta có thể sử dụng một đường dẫn lồng nhau ở bất kỳ mức nào trong một đường dẫn,
điều này rất hữu ích khi kết hợp hai câu lệnh `use` dùng chung một đường dẫn con.
Ví dụ: Listing 7-19 hiển thị hai câu lệnh `use`: một câu đưa` std :: io` vào scope và một câu đưa `std :: io :: Write` vào scope.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-19/src/lib.rs}}
```

<span class="caption">Listing 7-19: Two `use` statements where one is a subpath
of the other</span>

Phần chung của hai đường dẫn này là `std :: io` và đó là đường dẫn đầu tiên hoàn chỉnh. 
Để hợp nhất hai đường dẫn này thành một câu lệnh `use`, chúng ta có thể sử dụng` self`
trong đường dẫn lồng nhau, như được hiển thị trong Listing 7-20.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

<span class="caption">Listing 7-20: Kết hợp các đường dẫn trong Listing 7-19 thành một câu lệnh `use`</span>

Dòng này đưa `std :: io` và` std :: io :: Write` vào scope.

### Toán tử Glob

Nếu chúng ta muốn đưa *tất cả* các item public được xác định trong một đường dẫn vào scope, chúng ta có thể chỉ định đường dẫn đó theo sau là `*`, toán tử Glob:

```rust
use std::collections::*;
```

Câu lệnh `use` này đưa tất cả các item public được định nghĩa trong` std :: collection` vào scope hiện tại.
Hãy cẩn thận khi sử dụng toán tử cầu! Glob có thể khiến việc phân biệt những tên nào trong scope
và nơi tên được sử dụng trong chương trình của bạn được xác định là khó hơn.

Toán tử glob thường được sử dụng khi kiểm tra để đưa 
mọi thứ đang được kiểm tra vào module `tests`; chúng ta sẽ nói về điều đó trong phần [“How to Write
Tests”][writing-tests]<!-- ignore --> trong chương 11.Toán tử glob
đôi khi cũng được sử dụng như một phần của mẫu dạo đầu: xem [the standard
library documentation](../std/prelude/index.html#other-preludes)<!-- ignore -->
để biết thêm thông tin về mẫu đó.

[ch14-pub-use]: ch14-02-publishing-to-crates-io.html#exporting-a-convenient-public-api-with-pub-use
[rand]: ch02-00-guessing-game-tutorial.html#generating-a-random-number
[writing-tests]: ch11-01-writing-tests.html#how-to-write-tests
