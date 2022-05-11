## Đường dẫn tham chiếu đến một mục trong cây module

Để chỉ cho Rust nơi tìm một mục trong cây module, chúng tôi sử dụng một đường dẫn 
giống như cách chúng ta sử dụng một đường dẫn khi điều hướng một hệ thống tệp.
Nếu chúng ta muốn gọi một hàm, chúng ta cần biết đường dẫn của nó.

Một đường dẫn có thể có 2 dạng:

* Một *đường dẫn tuyệt đối* bắt đầu từ một crate bằng cách sử dụng tên một crate (đối với mã từ external crate)
  hoặc theo nghĩa đen `crate` (cho code từ crate hiện tại).
* Một *đường dẫn tương đối* bắt đầu từ module hiện tại và sử dụng `self`, `super`, hoặc một mã định danh trong module hiện tại.


Cả hai đường dẫn tuyệt đối và tương đối đều được theo sau bởi một hoặc nhiều định danh được phân tách bằng dấu hai chấm kép(`::`).

Hãy quay lại ví dụ trong Listing 7-1. Làm thế nào để chúng tôi gọi hàm
`add_to_waitlist`? Điều này cũng giống như yêu cầu, đừng dẫn của function là gì
`add_to_waitlist` ? Listing 7-3 chứa Listing 7-1 với một số 
module và hàm bị loại bỏ. Chúng tôi sẽ chỉ ra hai cách để gọi hàm
`add_to_waitlist` từ một hàm mới `eat_at_restaurant` được định nghĩa trong
crate root. Hàm `eat_at_restaurant` là một phần của public API trong thư viện crate của chúng ta
, vì vậy chúng ta đánh dấu nó với từ khóa `pub`. Trong phần [“Exposing Paths with
the `pub` Keyword”][pub]<!-- ignore -->, Chúng ta sẽ đi vào chi tiết hơn
về `pub`.Lưu ý rằng ví dụ này sẽ chưa biên dịch; chúng tôi sẽ giải thích một chút lý do tại sao.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

<span class="caption">Listing 7-3: Gọi hàm `add_to_waitlist`sử dụng các đường dẫn tuyệt đối và tương đối</span>

Lần đầu tiên chúng ta gọi hàm `add_to_waitlist` trong `eat_at_restaurant`,
chúng tôi sử dụng một đường dẫn tuyệt đối. Hàm `add_to_waitlist`được xác định trong cùng một crate như `eat_at_restaurant`,
có nghĩa là chúng ta có thể sử dụng từ khóa `crate` để bắt đầu một đường dẫn tuyệt đối.

Sau `crate`, chúng tôi bao gồm từng module kế tiếp nhau cho đến khi chúng ta thực hiện theo cách
`add_to_waitlist`.Bạn có thể tưởng tượng một hệ thống tệp có cấu trúc giống nhau, 
và chúng tôi sẽ chỉ định đường dẫn `/front_of_house/hosting/add_to_waitlist` để chạy chương trình
`add_to_waitlist` ; sử dụng tên `crate` để bắt đầu từ crate root
giống như sử dụng `/` để bắt đầu từ gốc hệ thống tệp trong shell của bạn.

Lần thứ hai chúng ta gọi `add_to_waitlist` trong `eat_at_restaurant`, chúng ta sử dụng đường dẫn tương đối.
Đường dẫn bắt đầu với `front_of_house`, tên của module được xác định ở cùng cấp của cây module như `eat_at_restaurant`.
Ở đây hệ thống tệp tương đương sẽ sử dụng đường dẫn `front_of_house/hosting/add_to_waitlist`.
Bắt đầu bằng một cái tên có nghĩa là đường dẫn đó là tương đối.

Việc chọn sử dụng đường dẫn tương đối hay đường dẫn tuyệt đối là quyết định bạn sẽ thực hiện dựa trên dự án của mình.
Quyết định sẽ phụ thuộc vào việc bạn có nhiều khả năng di chuyển các mục code được xác định riêng biệt hoặc cùng với code sử dụng các mục hay không.
Ví dụ, nếu chúng ta di chuyển module `front_of_house`  và hàm`eat_at_restaurant` vào một module được đặt tên `customer_experience`, 
chúng tôi cần cập nhật đường dẫn tuyệt đối đến`add_to_waitlist`, nhưng đường dẫn tương đối sẽ vẫn hợp lệ. Tuy nhiên, nếu chúng ta di chuyển
hàm `eat_at_restaurant` riêng biệt thành một module có tên `dining`,đường dẫn tuyệt đối dẫn đến `add_to_waitlist` cuộc gọi sẽ giữ nguyên,
nhưng đường dẫn tương đối sẽ cần được cập nhật. Sở thích của chúng tôi là chỉ định đường dẫn tuyệt đối vì có nhiều khả năng chúng tôi muốn 
di chuyển các định nghĩa code và lệnh gọi các mục độc lập với nhau.

Hãy thử biên dịch Listing 7-3 và tìm hiểu lý do tại sao nó vẫn chưa được biên dịch! Lỗi chúng tôi gặp phải được hiển thị trong Listing 7-4.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

<span class="caption">Listing 7-4: Lỗi trình biên dịch khi xây dựng code trong
Listing 7-3</span>

Các thông báo lỗi cho biết module `hosting` là private. Nói cách khác, chúng tôi có
các đường dẫn chính xác cho module `hosting`  và hàm `add_to_waitlist`,nhưng Rust sẽ không cho phép 
chúng tôi sử dụng chúng vì nó không có quyền truy cập vào các phần private.

Các module không chỉ hữu ích cho việc tổ chức code của bạn. Chúng cũng định nghĩa Rust’s
*privacy boundary*: bao gồm chi tiết triển khai code bên ngoài không được phép biết, gọi hoặc phụ thuộc vào.
Vì vậy, nếu bạn muốn đặt một mục như function hoặc structs ở chế độ private, bạn đặt nó trong một module.

Cách thức hoạt động của quyền riêng tư trong Rust là tất cả các mục (functions, methods, structs,
enums, modules, and constants) là private theo mặc định. Các mục ở module cha
không thể sử dụng các mục private bên trong các module con,nhưng các mục trong module con có thể sử dụng các mục trong module tổ tiên của chúng.
Lý do là các module con bao bọc và ẩn triển khai chi tiết của chúng, nhưng các module con có thể thấy ngữ cảnh mà chúng được xác định.
Để tiếp tục với phép ẩn dụ về nhà hàng, hãy nghĩ về các quy tắc bảo mật giống như văn phòng phía sau của một nhà hàng: những gì diễn ra 
trong đó là riêng tư đối với khách hàng của nhà hàng, nhưng những người quản lý văn phòng có thể thấy và làm mọi thứ trong nhà hàng mà họ điều hành.

Rust đã chọn để hệ thống module hoạt động theo cách này để ẩn các chi tiết triển khai bên trong là mặc định. 
Bằng cách đó, bạn biết những phần nào của code bên trong mà bạn có thể thay đổi mà không cần phá vỡ mã bên ngoài. 
Nhưng bạn có thể hiển thị các phần bên trong của code module con cho các module tổ tiên bên ngoài bằng cách sử dụng từ khóa `pub` để đặt một mục ở chế độ public.

### Hiển thị các Đường dẫn với Từ khoá `pub`

Hãy quay lại lỗi trong Listing 7-4 Hãy quay lại lỗi trong module `hosting` 
là private.Chúng ta muốn function `eat_at_restaurant` trong module cha để có quyền truy cập vào function`add_to_waitlist` trong module con, vì vậy chúng ta đánh dấu module
`hosting` với từ khóa `pub`, như được hiển thị trong Listing 7-5.

<span class="filename">Filename: src/lib.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs}}
```

<span class="caption">Listing 7-5: Khai báo module `hosting` như `pub` sử dụng nó từ `eat_at_restaurant`</span>

Thật không may, mã trong Listing 7-5 vẫn dẫn đến lỗi, như được hiển thị trong
Listing 7-6.

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

<span class="caption">Listing 7-6:Lỗi trình biên dịch khi xây dựng mã trong
Listing 7-5</span>

Chuyện gì đã xảy ra? Thêm từ khóa `pub` vào trước `mod hosting` đặt 
module ở chế độ public. Với sự thay đổi này, nếu chúng ta có thể truy cập `front_of_house`, 
chúng ta có thể truy cập `hosting`. Nhưng *nội dung* của `hosting` vẫn còn private; đặt module ở chế độ public không làm public nội dung của module.
Từ khóa `pub` trên một module chỉ cho phép code trong các module tổ tiên của nó tham chiếu đến nó.

Các lỗi trong Listing 7-6 nói rằng function `add_to_waitlist` là private.
Các quy tắc bảo mật áp dụng cho structs, enums, functions, and methods cũng như các module.

Hãy cũng làm cho function `add_to_waitlist` public bằng cách thêm từ khóa `pub` trước định nghĩa của nó, như trong Listing 7-7.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs}}
```

<span class="caption">Listing 7-7: Thêm từ khóa `pub` vào `mod hosting`
và `fn add_to_waitlist` cho phép chúng tôi gọi hàm từ
`eat_at_restaurant`</span>

Bây giờ code sẽ biên dịch!Hãy xem xét đường dẫn tuyệt đối và đường dẫn tương đối và kiểm tra kỹ lý do tại sao lại thêm từ khóa `pub` 
cho phép chúng tôi sử dụng những đường dẫn này trong `add_to_waitlist` đối với các quy tắc bảo mật.

Trong đường dẫn tuyệt đối, chúng ta bắt đầu với `crate`, root của cây crate’s module
. Sau đó module `front_of_house`được định nghĩa trong crate root. module
`front_of_house` không là public, nhưng bởi vì function `eat_at_restaurant`
được định nghĩa trong cùng một module như `front_of_house` (đó là,
`eat_at_restaurant` và `front_of_house` là chị em), chúng ta có thể tham khảo
`front_of_house` từ `eat_at_restaurant`.Tiếp theo là module `hosting` được đánh dấu
với `pub`. Chúng ta có thể truy cập module cha của `hosting`, vì vậy chúng ta có thể truy cập
`hosting`.Cuối cùng function `add_to_waitlist` được đánh dấu với `pub` và chúng tôi có thể truy cập module cha của nó, vì vậy lệnh gọi hàm này hoạt động!

Trong đường dẫn tương đối, logic giống như đường dẫn tuyệt đối ngoại trừ bước đầu tiên: thay vì bắt đầu từ crate root, 
đường dẫn bắt đầu từ `front_of_house`. Module `front_of_house` được định nghĩa trong cùng một module
như `eat_at_restaurant`, vì vậy đường dẫn tương đối bắt đầu từ module trong đó
`eat_at_restaurant` là việc định nghĩa. Sau đó,bởi vì `hosting` và
`add_to_waitlist` được đánh dấu với `pub`, phần còn lại của đường dẫn hoạt động và lệnh gọi hàm này hợp lệ!

Nếu bạn định chia sẻ library crate của mình để các dự án khác có thể sử dụng code của bạn,
Public API của bạn là hợp đồng của bạn với người dùng trong crate về cách họ tương tác với mã của bạn.
Có nhiều cân nhắc xung quanh việc quản lý các thay đổi đối với public API của bạn để giúp mọi người phụ thuộc vào crate của bạn dễ dàng hơn.
Những cân nhắc này nằm ngoài phạm vi của cuốn sách này; nếu bạn quan tâm đến chủ đề này, hãy xem [The Rust API Guidelines][api-guidelines].

> #### Các phương pháp hay nhất cho các package có một Binary và a Library
>
> Chúng tôi đã đề cập đến một package có thể chứa cả hai *src/main.rs* binary crate root 
> giống như một *src/lib.rs* library crate root, và cả crates sẽ có
> tên package theo mặc định. hông thường, các package có mẫu này sẽ chỉ có
> đủ code trong binary crate để bắt đầu một tệp thực thi gọi code bằng
> library crate. Điều này cho phép các project khác được hưởng lợi nhiều nhất
> chức năng mà package cung cấp, vì code của library crate có thể
> được chia sẻ.
>
> Cây module nên được định nghĩa trong *src/lib.rs*. Sau đó, bất kỳ mục public nào có thể
> được sử dụng trong binary crate bằng cách bắt đầu các đường dẫn với tên của package.
> Binary crate trở thành người dùng của library crate giống như một
> crate bên ngoài sẽ sử dụng library crate: nó chỉ có thể sử dụng public API.
> Điều này giúp bạn thiết kế một API tốt; bạn không chỉ là tác giả, bạn còn là
> client!
>
> In [Chapter 12][ch12]<!-- ignore -->,chúng ta sẽ chứng minh tổ chức này
> thực hành với một command-line program sẽ chứa cả một binary crate
> và một library crate.

### Bắt đầu Đường dẫn tương đối với `super`

Chúng tôi cũng có thể xây dựng các đường dẫn tương đối bắt đầu trong module cha bằng cách sử dụng
`super` ở đầu đường dẫn. Điều này giống như bắt đầu một đường dẫn hệ thống tệp với cú pháp `..`. Tại sao chúng tôi muốn làm điều này?

Hãy xem xét đoạn mã trong Listing 7-8 mô hình hóa tình huống trong đó đầu bếp sửa một đơn hàng không chính xác
và đích thân mang món đó ra cho khách hàng.Chức năng `fix_incorrect_order` được định nghĩa trong module `back_of_house` gọi hàm `deliver_order` được định nghĩa trong module cha bằng cách chỉ định đường dẫn đến
`deliver_order` bắt đầu với `super`:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

<span class="caption">Listing 7-8: Calling a function using a relative path
starting with `super`</span>

Hàm `fix_incorrect_order` nằm trong module` back_of_house`, 
vì vậy chúng ta có thể sử dụng `super` để chuyển đến module cha của` back_of_house`, 
trong trường hợp này là `crate` root. Từ đó, chúng tôi tìm kiếm `delivery_order` và tìm thấy nó.
Thành công! Chúng tôi nghĩ rằng module `back_of_house` và hàm` delivery_order` có thể giữ nguyên mối quan hệ với nhau 
và được di chuyển cùng nhau nếu chúng tôi quyết định tổ chức lại cây module của crate. 
Do đó, chúng tôi đã sử dụng `super` nên chúng tôi sẽ có ít nơi để cập nhật mã hơn trong tương lai nếu mã này được chuyển sang một module khác.

### Làm cho Structs và Enums Public

Chúng ta cũng có thể sử dụng `pub` để chỉ định các struct và enums là public, nhưng có một số chi tiết bổ sung.
Nếu chúng ta sử dụng `pub` trước định nghĩa struct, chúng ta đặt struct ở chế độ public,
nhưng các trường của struct sẽ vẫn ở chế độ riêng tư. Chúng tôi có thể public từng trường hoặc không tùy từng trường hợp cụ thể. 
Trong Listing 7-9, chúng tôi đã xác định cấu trúc `back_of_house :: Breakfast` public với trường` toast` 
public nhưng là trường `season_fruit` riêng. Điều này mô tả trường hợp trong một nhà hàng, 
nơi khách hàng có thể chọn loại bánh mì đi kèm với bữa ăn, nhưng đầu bếp quyết định loại trái cây nào đi kèm với
bữa ăn dựa trên những gì đang có trong mùa và trong kho. Trái cây có sẵn thay đổi nhanh chóng, 
vì vậy khách hàng không thể chọn trái cây hoặc thậm chí không thể xem họ sẽ lấy trái cây nào.
                    
<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

<span class="caption">Listing 7-9:Một struct với một số trường public và một số trường riêng tư</span>

Vì trường `toast` trong struct ` back_of_house :: Breakfast` là public nên trong `eat_at_restaurant`, 
chúng ta có thể viết và đọc trường` toast` bằng ký hiệu *::* . Xin lưu ý rằng chúng ta không thể sử 
dụng trường `season_fruit` trong` eat_at_restaurant` bởi vì `season_fruit` là private. 
Hãy thử bỏ ghi chú dòng sửa đổi giá trị trường `season_fruit` để xem bạn gặp lỗi gì!

Ngoài ra, hãy lưu ý rằng vì `back_of_house :: Breakfast` có một trường private, 
nên struct cần cung cấp một hàm liên kết public để tạo một phiên bản của` Breakfast` 
(chúng tôi đã đặt tên nó là `summer` ở đây). Nếu `Breakfast` không có chức năng như vậy,
chúng tôi không thể tạo phiên bản của` Breakfast` trong `eat_at_restaurant`
vì chúng tôi không thể đặt giá trị của trường private theo `seasonal_fruit ` trong `eat_at_restaurant`.

Ngược lại, nếu chúng ta public enum, thì tất cả các biến thể của nó sẽ được public. 
Chúng tôi chỉ cần `pub` trước từ khóa` enum`, như được hiển thị trong Listing 7-10.

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

<span class="caption">Listing 7-10:Việc chỉ định một enum là public làm cho tất cả các biến thể của nó ở chế độ public</span>

Vì chúng tôi đã đặt enum `Appetizer` ở chế độ public nên chúng tôi có thể sử dụng các biến thể` Soup` và `Salad` trong` eat_at_restaurant`. 
Enums không hữu ích lắm trừ khi các biến thể của chúng được public; sẽ rất khó chịu khi phải chú thích tất cả các biến thể enum bằng `pub` trong mọi trường hợp,
vì vậy mặc định cho các biến thể enum là public. Các struct thường hữu ích mà không cần trường của chúng là public,
vì vậy, các trường của struct tuân theo quy tắc chung là mọi thứ là private theo mặc định trừ khi được chú thích bằng `pub`.

Còn một trường hợp nữa liên quan đến `pub` mà chúng tôi chưa đề cập đến và đó là tính năng hệ thống public cuối cùng của chúng tôi: từ khóa` use`. 
Trước tiên, chúng tôi sẽ đề cập đến vấn đề `use`, sau đó chúng tôi sẽ chỉ ra cách kết hợp giữa` pub` và `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html
