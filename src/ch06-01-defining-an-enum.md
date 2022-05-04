## Định nghĩa một Enum

*Enums* là định nghĩa một kiểu dữ liệu tuỳ chỉnh khác với *structs*. 
Hãy xem xét một tình huống chúng ta phải diễn đạt trong code và xem tại sao enums lại 
hữu ích và phù hợp hơn struct trong trường hợp này.
Giả sử chúng ta làm việc với các địa chỉ IP. Hiện tại, 2 tiêu chuẩn chính được sử
dụng cho địa chỉ IP là: Ipv4 và Ipv6. Bởi vì đây là khả năng duy nhất cho 1 địa chỉ IP mà 
chương trình của chúng ta sẽ bắt gặp, chúng ta có thể sử dụng
*enumerate* để liệt kê hết các biến có thể có, which is where enumeration gets its name.

Bất kì địa chỉ IP nào cũng có thể là Ipv4 hoặc Ipv6, nhưng không phải cả 2 tại 1 thời điểm.
Thuộc tính trên làm cho cấu trúc của *enum* thích hợp để sử dụng cho trường hợp này,
bởi vì một *enum* chỉ có thể lấy 1 giá trị trong các loại của nó.
Cả Ipv4 và Ipv6 về cơ bản vẫn là một địa chỉ IP, vì vậy nên chúng được coi là 1 loại giá trị khi đoạn code
áp dụng cho bất kì loại địa chỉ IP nào.

Chúng ta có thể thực hành khái niệm trên trong code bằng cách khai báo 1 *enum" `IpAddrKind` và
liệt kê các loại địa chỉ IP có thể có: `V4` và `V6`. Bên dưới là các loại của *enum*

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

`IpAddrKind` là một kiểu dữ liệu tuỳ chỉnh mà ta có thể sử dụng ở những nơi khác nhau trong code của mình.

### Enum Values

Chúng ta có thể tạo ra từng biến cho 2 loại `IpAddrKind` giống như thế này:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Lưu ý rằng các loại của *enum* có thể đặt tên dưới định dang của nó, và chúng ta
sử dụng 2 dấu 2 chấm (::) để phân tách. Điều này rất có ích vì cả 2 giá trị
`IpAddrKind::V4` và `IpAddrKind::V6` đều có kiểu giá trị: `IpAddrKind`. Chúng ta có thể định một hàm xử dụng 
bất kì `IpAddrKind` nào:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Và chúng ta có thể gọi hàm này với 1 trong 2 loại *enum*

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

Sử dụng *enum* thậm chí còn nhiều điểm mạnh hơn. 
Chúng tôi suy nghĩ thêm về địa IP, hiện tại chúng tôi không có cách nào để lưu trữ dữ 
liệu thực tế của địa chỉ IP;
chúng tôi chỉ biết chúng là kiểu dữ liệu nào. 
Bạn mới học về cấu struct ở chương 5, bạn có thể giải quyết  vấn đề này với truct như trong 
Listing 6-1.

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

<span class="caption">Listing 6-1: Lưu trữ dữ liệu `IpAddrKind` bằng cách sử dụng *struct*
an IP address using a `struct`</span>

Ở đây, chúng tôi dã định nghĩa một struct `IpAddr` có 2 trường: trường `kind` có kiểu dữ liệu là 
`IpAddrKind` (*enum* chúng ta đã định nghĩa trước đó) và một trường là `address` có kiểu dữ liệu `String`. 
Chúng ta có 2 trường hợp của struct này. Đầu tiên là `home`,
và nó có giá trị `IpAddrKind::V4` như là `kind` với địa chỉ của dữ liệu là `127.0.0.1`. 
Trường hợp thứ 2 là `loopback`. Nó là một trường hợp khác của `IpAddrKind` 
giá trị `kind` là `V6`, và có địa chỉ `::1` liên kết với nó. 
Chúng ta sử dụng 1 struct để nhóm giá trị `kind` và `address`
với nhau, vì vậy hiện giờ biến thể được thể liên kết với nhau.

Tuy nhiên, đại diện cho cùng 1 khái niệm dùng một *enum* là ngắn gọn hơn: hơn là 1 *enum* bên trong 1 struct
, chúng ta có thể đưa dữ liệu trực tiếp vào từng trường hợp của *enum*. 
Định nghĩa mới này của *enum* `IpAddr` cho biết rằng cả 2 giá trị `V4` and `V6`
sẽ có các giá trị `String`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Chúng ta gán giá trị trực tiếp vào cho mỗi trường hợp *enum*, 
vì vậy nên không cần thêm cấu trúc.
Tại đây, bạn cũng có thể dễ dàng xem một chi tiết khác về cách hoạt động của *enum*:
tên của mỗi trường hợp *enum* mà chúng ta định nghĩa đã trở thành một hàm tạo của *enum*. 
Đó là, `IpAddr::V4()` là một hàm sử dụng 1 `String` là tham số và trả về một instance của kiểu `IpAddr`. 
Chúng tự động định nghĩa hàm khởi tạo này như là kết quả của định nghĩa một *enum*.

Có một lợi thế khác khi sử dụng enum thay vì cấu trúc: mỗi trường hợp trong *enum* có thể có
các loại dữ liệu khác nhau. Địa chỉ IP của `V4` sẽ luôn có bốn thành phần số sẽ có giá trị từ 0 đến 255.
Nếu chúng tôi muốn lưu trữ địa chỉ `V4` dưới dạng bốn giá trị u8 nhưng 
vẫn diễn đạt được địa chỉ `V6` là một giá trị `String`, điều này không thể với struct.
Enums xử lý trường hợp này một cách dễ dàng:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Chúng tôi đã chỉ ra một số cách khác nhau để xác định cấu trúc dữ liệu để lưu trữ địa chỉ IP `V4` và `V6`.
Tuy nhiên, hóa ra, muốn lưu trữ địa chỉ IP và mã hóa loại nào chúng rất phổ biến nên Rust 
[định nghĩa một thư viện chuẩn mà chúng ta có thể sử dụng!][IpAddr]<!-- ignore -->
Hãy xem cách thư viện chuẩn định nghĩa `IpAddr`: nó có *enum* chính xác 
và các trường hợp mà chúng ta định nghĩa và sử dụng,
nhưng nó nhúng dữ liệu địa chỉ bên trong các trường hợp *enum* dưới dạng hai cấu trúc khác nhau,
được định nghĩa khác nhau cho từng trường hợp:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Đoạn mã này minh họa rằng bạn có thể đặt bất kỳ loại dữ liệu nào bên trong một trường hợp của *enum*:
strings, numeric, hay struct... ví dụ. Thậm chí có thể chứa cả một *enum* khác!
Ngoài ra, các loại thư viện chuẩn thường không phức tạp hơn nhiều so với những gì mà bạn có thể nghĩ ra.

Lưu ý rằng mặc dù thư viện chuẩn có chứa định nghĩa cho `IpAddr`,
chúng ta vẫn có thể tạo và sử dụng định nghĩa riêng của chúng ta mà không bị xung đột
bởi vì chúng ta không đưa thư viện chuẩn vào trong phạm vi sử dụng của chúng ta.
Chúng ta sẽ nói nhiều hơn về việc đưa các loại vào trong phạm vi sử dụng ở Chương 7.

Hã xem một ví dụ khác về *enum* trong Listing 6-2: cái này có nhiều loại dữ liệu được định nghĩa trong 
các trường hợp *enum*:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

<span class="caption">Listing 6-2: Một *enum* `Message` mà mỗi trường hợp của nó khác nhau về số lượng
và kiểu dữ liệu</span>

*Enum* này có 4 trường hợp với 4 loại khác nhau:

* `Quit` không có chứa dữ liệu nào cả.
* `Move` giống như một struct.
* `Write` bao gồm 1 `String`.
* `ChangeColor` chứa 3 giá trị `i32`.

Định nghĩa 1 *enum* với các trường hợp như trong Listing 6-2 tương tự như việc xác định các loại định nghĩa
struct khác nhau, trừ trường hợp *enum* không sử dụng từ khoá `struct` 
và tất cả các trường hợp trong nhóm lại với nhau bên dưới kiểu `Message`.
Các struct có thể chứa cùng một dữ liệu mà các trường hợp của *enum* trước đó giữ:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Nhưng nếu chúng ta sử dụng các struct khác nhau, mỗi struct có kiểu riêng của chúng,
chúng ta không thể dễ dàng định nghĩa một hàm để nhận bất kì một thông báo nào như 
chúng ta có thể làm với *enum* `Message` được định nghĩa ở Listing 6-2, 
một kiểu duy nhất.

Có một điểm tương đồng nữa giữa *enum* và *struct*:: chúng đều định nghĩa các phương thức sử dụng `impl`. 
Đây là một phương thức `call`, chúng ta có thể định nghĩa trong *enum* `Message`:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Phần thân của phương thức sẽ sử dụng `self` để lấy giá trị mà chúng ta đã gọi phương thức trên.
Trong ví dụ này, chúng tôi đã tạo một biến m có giá trị là `Message::Write(String::from("hello"))`, 
và đó là những gì `self` sẽ ở trong phần thân hàm `call` khi `m.call()` chạy.

Chúng ta hãy xem xét một enum khác trong thư viện chuẩn rất phổ biến và hữu ích: `Option`.

### `Option` Enum và  ưu điểm của nó so với giá trị Null

Trong phần này chúng ta sẽ khám phá 1 trường hợp điển hình của *Enum* là `Option`,
là một enum khác được định nghĩa bởi thư viện tiêu chuẩn. 
`Option` là 1 loại mã hóa rất phổ biến trong đó giá trị có thể là một cái gì đó hoặc nó có thể không là gì cả. 
Ví dụ, nếu bạn yêu cầu phần tử đầu tiên của một danh sách chứa những mục, bạn sẽ nhận một giá trị.
Nếu bạn yêu cầu phần tử đầu tiên của một danh sách rỗng, bạn không nhận được gì.
Diễn đạt khái niệm này theo kiểu hệ thống có nghĩa là trình biên dịch có thể kiểm tra xem bạn đã xử lý tất cả các trường hợp cần xử lý chưa;
chức năng này có thể ngăn chặn các lỗi rất phổ biến trong các ngôn ngữ lập trình khác.

Thiết kế ngôn ngữ lập trình thường được xem xét về các tính năng bạn bao gồm những tính năng nào,
nhưng các tính năng bạn loại trừ cũng quan trọng. Rust không có giá trị rỗng mà nhiều ngôn ngữ khác có. 
*Null* là một giá trị có nghĩa là không có giá trị nào ở đó.
Trong các ngôn ngữ có null, các biến luôn có thể ở một trong hai trạng thái: null hoặc not-null.

Trong bài thuyết trình năm 2009 của anh ấy “Null References: The Billion Dollar Mistake,” Tony Hoare, người phát minh ra null, có điều này để nói:

> Tôi gọi đó là sai lầm hàng tỷ đô la của mình. At that time, Vào thời điểm đó, tôi đang thiết kế hệ thống đầu tiên
> cho các tham chiếu bằng ngôn ngữ hướng đối tượng. Mục tiêu của tôi là đảm bảo rằng tất cả việc sử dụng
> các tham chiếu phải an toàn tuyệt đối, với việc kiểm tra được thực hiện tự động bởi trình biên dịch.
> Nhưng tôi không thể cưỡng lại sự cám dỗ để đưa vào một tham chiếu rỗng,
> đơn giản vì nó rất dễ thực hiện. Điều này đã dẫn đến vô số lỗi, lỗ hổng bảo mật và sự cố hệ thống
> mà có lẽ đã gây ra đau đớn và thiệt hại hàng tỷ đô la trong bốn mươi năm qua.

Vấn đề với giá trị null là nếu bạn cố gắng sử dụng giá trị null làm giá trị không null,
bạn sẽ gặp một lỗi nào đó. Bởi vì thuộc tính null hoặc not-null này là phổ biến,
rất dễ mắc phải loại lỗi này.

Tuy nhiên, khái niệm null đang cố gắng diễn đạt vẫn hữu ích: 
null là một giá trị hiện không hợp lệ hoặc vắng mặt vì lý do nào đó.

Vấn đề không thực sự nằm ở khái niệm mà là ở cách triển khai cụ thể.
Như vậy, Rust không có null, nhưng nó có một *enum* có thể mã hóa khái niệm giá trị hiện diện hoặc vắng mặt. 
Enum này là `Option<T>`, và nó là [defined by the standard library][option]<!-- ignore --> như sau:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` enum hữu ích đến mức chứa phần báo trước; bạn không cần phải đưa nó vào phạm vi một cách rõ ràng.
Các biến thể của nó cũng được bao gồm trong phần mở đầu: bạn có thể sử dụng `Some` và `None` trực tiếp 
mà không có tiền tố `Option::`. `Option<T>` enum vẫn là 1 enum bình thường, và `Some(T)` và
`None` cũng là các biến thể trong kiểu `Option<T>`.

Cú pháp `<T>` là một tính năng của Rust mà chúng tôi chưa nói đến. Nó là 1 kiểu generic,
và chúng tôi sẽ trình bày chi tiết hơn trong Chương 10.
Hiện tại, tất cả những gì bạn cần biết là `<T>` nghĩa là `Some` là biến thể 
`Option` enum có thể chứa bất kì loại dữ liệu nào, và mỗi loại dữ liệu cụ thể được sử dụng thay cho `T`
làm cho loại `Option<T>` trở thành một loại khác.
Dưới đây là một số ví dụ về việc sử dụng giá trị `Option` để lưu các loại số và loại chuỗi:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-06-option-examples/src/main.rs:here}}
```

Kiểu `some_number` là `Option<i32>`. Kiểu `some_string` là
`Option<&str>`, đó là một loại kiểu khác . Rust có thể suy ra các loại này bởi vì chúng tôi đã chỉ định một giá trị bên trong biến thể `Some`.
Đối với `absent_number`, Rust yêu cầu chúng tôi chú thích loại `Option`: trình biên dịch không thể suy ra loại mà tương ứng
`Some` đang lưu trữ là giá trị `None`. Ở đây, chúng tôi nói với Rust rằng ý của chúng tôi là để `absent_number` có kiểu `Option<i32>`.

khi `Some` có một giá trị nào đó, chúng tôi biết rằng một giá trị hiện diện và giá trị được giữ trong
`Some`. Khi chúng tôi cố 1 giá trị `None`, theo một nghĩa nào đó, nó có nghĩa giống như là null:
chúng tôi không có giá trị hợp lệ. Vậy tại sao có `Option<T>` lại tốt hơn là null?

Nói ngắn gọn, bởi vì `Option<T>` và `T` (`T` có thể là bất kì một kiểu nào) là những loại khác nhau
, trình biên dịch sẽ không cho phép chúng tôi sử dụng một giá trị `Option<T>` nếu như nó
không phải một giá trị hợp lệ. Ví dụ: mã này sẽ không biên dịch vì nó đang cố gắng sử dụng 
phép cộng một `i8` với một `Option<i8>`:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/src/main.rs:here}}
```

Nếu chúng tôi chạy mã này, chúng tôi nhận được thông báo lỗi như sau:

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-07-cant-use-option-directly/output.txt}}
```

Mãnh liệt! Thực tế, thông báo lỗi này có nghĩa là Rust không hiểu làm cách nào 
cộng một `i8` với một `Option<i8>`, bởi vì chúng là những loại khác nhau. 
Khi chúng ta có một giá trị kiểu `i8` trong Rust, trình biên dịch sẽ chắc chắn 
chúng ta có một giá trị hợp lệ. Chúng ta có thể tiến hành một cách tự tin mà không cần phải kiểm tra null trước khi sử dụng giá trị đó
. Chỉ khi chúng ta có một `Option<i8>` (hoặc bất kỳ loại giá trị nào mà chúng ta đang làm việc) 
chúng ta phải lo lắng về việc có thể không có giá trị, và trình biên dịch sẽ đảm bảo rằng chúng tôi xử lý trường hợp đó trước khi sử dụng giá trị đó.

Nói cách khác, bạn phải chuyển một kiểu `Option<T>` thành một kiểu `T` trước khi bạn thực hiện bất kì
thao tác nào với nó. Nói chung, điều này giúp khắc phục một trong những vấn đề phổ biến nhất với null: 
giả sử rằng một cái gì đó không phải là null khi nó thực sự là như vậy.

Loại bỏ rủi ro giả định không chính xác giá trị không phải là null giúp bạn tin hơn vào mã của mình.
Để có một giá trị có thể là null, bạn phải chọn một cách rõ ràng bằng cách đặt loại giá trị đó là `Option<T>`.
Sau đó, khi bạn sử dụng giá trị đó, bạn được yêu cầu xử lý rõ ràng trường hợp khi giá trị là null. 
Ở mọi nơi mà một giá trị có một loại không phải là `Option<T>`, bạn có thể an toàn giả định rằng giá trị không phải là null. 
Đây là một quyết định thiết kế có chủ ý của Rust để hạn chế lỗi phổ biến của null và tăng độ an toàn của mã Rust.

Vì vậy, làm thế nào để bạn có được giá trị `T` từ một kiểu `Some` khi bạn có một giá trị thuộc loại 
`Option<T>` để bạn có thể sử dụng giá trị đó? `Option<T>` enum có một số lượng lớn các phương pháp hữu ích 
 trong nhiều tình huống khác nhau; bạn có thể kiểm tra chúng tại [its documentation][docs]<!-- ignore -->. 
 Làm quen với các phương pháp trên `Option<T>` sẽ cực kỳ hữu ích trong hành trình của bạn với Rust.

Nói chung, để sử dụng một giá trị `Option<T>`, bạn muốn có mã sẽ xử lý từng biến thể. 
Bạn muốn một số mã sẽ chỉ chạy khi bạn có một giá trị `Some(T)`, 
và mã này được phép sử dụng bên trong `T`. Bạn muốn một số mã khác chạy nếu bạn có
một giá trị `None`, và mã đó không có sẵn giá trị `T`. Biểu thức `match` là một cấu trúc luồng điều khiển thực hiện điều này khi được sử dụng với enums: 
nó sẽ chạy mã khác nhau tùy thuộc vào biến thể của enum mà nó có, và mã đó có thể sử dụng dữ liệu bên trong giá trị phù hợp.

[IpAddr]: ../std/net/enum.IpAddr.html
[option]: ../std/option/enum.Option.html
[docs]: ../std/option/enum.Option.html
