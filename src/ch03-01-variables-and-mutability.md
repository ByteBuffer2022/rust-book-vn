## Biến (Variable) và Tính biến đổi (Mutability)

Như đã đề cập trong phần [“Sử dụng biến để chứa các
giá trị”][storing-values-with-variables]<!-- ignore -->, các biến mặc định là không thể
thay đổi được (bất biến - immutable). Đây là một trong số nhiều khuyến khích mà Rust cung cấp
để bạn viết code theo cách tận dụng sự an toàn và đồng thời dễ dàng mà Rust đưa ra. Tuy nhiên,
bạn vẫn có tùy chọn để khiến cho biến có thể thay đổi (mutable). Hãy cùng tìm hiểu cách thức
và lý do tại sao Rust khuyến khích bạn ưu tiên tính bất biến (immutability) và tại sao đôi khi
bạn có thể không muốn thế.

Khi một biến là immutable, bạn không thể thay đổi giá trị đã được gán vào biến đó.
Để mô tả điều này, hãy tạo một dự án có tên *variables* trong thư mục *projects*
bằng cách sử dụng `cargo new variables`.

Sau đó, trong thư mục *variables*, mở file *src/main.rs* và thay code của nó
bằng đoạn code bên dưới. Code này sẽ chưa được biên dịch, chúng sẽ kiểm tra
lỗi về tính bất biến trước.

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Lưu lại và chạy chương trình bằng `cargo run`. Bạn sẽ nhận một thông báo lỗi
như được hiển thị trong phần kết quả này:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Ví dụ này chỉ ra cách mà trình biên dịch giúp bạn tìm ra lỗi trong chương trình.
Lỗi trình biên dịch có thể gây khó chịu, nhưng thực sự chúng chỉ có nghĩa rằng
chương trình của bạn chưa thực hiện một cách an toàn những gì bạn muốn; chúng *không*
có nghĩa là bạn không phải là một lập trình viên giỏi! Nhiều Rustacean có kinh nghiệm vẫn
gặp những lỗi này.

Thông báo lỗi chỉ ra nguyên nhân lỗi là `` cannot
assign twice to immutable variable `x` ``, bởi vì bạn đã cố gắng gán một giá trị
thứ hai vào biến immutable `x`.

Điều quan trọng là chúng ta gặp lỗi compile-time khi chúng ta cố gắng thay đổi giá
trị immutable bởi vì chính tình huống này có thể dẫn tới bugs. Nếu một phần code
hoạt động trên giả định rằng một giá trị sẽ không bao giờ bị thay đổi và một phần
khác có thể thay đổi giá trị đó thì phần code đầu tiền sẽ không thể thực hiện được
như những gì nó được thiết kế. Nguyên nhân của loại bug này có thể khó lần ra trong
thực tế, đặc biệt là khi đoạn code thứ hai chỉ *đôi khi* thay đổi giá trị. Trình biên
dịch Rust đảm bảo rằng khi bạn chỉ đinh một giá trị sẽ không thay đổi, nó sẽ thực sự
không đổi, vì vậy bạn không cần phải tự mình theo dõi nó. Do đó, code của bạn sẽ dễ
dàng lập luận hơn.

Nhưng mutability có thể rất hữu ích và có thể khiến bạn viết code thuận tiện hơn.
Các biến chỉ immutable theo mặc định; như bạn đã làm trong Chương 2, bạn có thể
làm cho chúng mutable khi thêm `mut` vào trước tên biến. Việc thêm `mut` cũng
truyền tải ý định đến những người đọc code trong tương lai bằng cách chỉ ra rằng
các phần khác của code sẽ thay đổi giá trị của biến này.

Ví dụ, hãy thay đổi file *src/main.rs* thành như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Khi chúng ta chạy chương trình, chúng ta sẽ nhận được như vầy:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

Chúng ta được phép thay đổi giá trị gán vào `x` từ `5` thành `6` khi chúng ta dùng `mut`.
Có nhiều sự đánh đổi cần cân nhắc ngoài việc ngăn ngừa bugs. Ví dụ, trong trường hợp bạn
đang sử dụng một cấu trúc dữ liệu lớn, việc thay đổi instance tại chỗ có thể sẽ nhanh hơn
so với việc sao chép và trả về các instance mới được phân bổ. Với cấu trúc dữ liệu nhỏ hơn,
việc tạo instace mới và viết theo phong cách lập trình nhiều chức năng hơn có thể dễ dàng hơn 
để suy nghĩ, do đó hiệu suất thấp hơn có thể là một hình phạt đáng giá để tăng đạt được sự rõ
ràng đó.

### Hằng số (Constants)

Giống như các biến immutable, *constants* là các giá trị được gắn với một cái tên và
không được phép thay đổi, nhưng có một vài khác biệt giữa hằng số và biến.

Đầu tiên, bạn không được phép sử dụng `mut` với hằng số. Hằng số không chỉ
immutable theo mặc định - chúng luôn luôn immutable. Bạn khai báo hằng số sử dụng từ khóa
`const` thay vì từ khóa `let`, và kiểu dữ liệu của giá trị *buộc phải*
được chú thích. Chúng ta sắp đề cập đến kiểu dữ liệu và chú thích kiểu dữ liệu trong
phần tiếp theo, [“Các kiểu dữ liệu”][data-types]<!-- ignore --> vì vậy đừng lo lắng
về chi tiết lúc này. Chỉ biết rằng bạn phải luôn luôn chú thích kiểu dữ liệu.

Hằng số có thể được khai báo trong bất kì phạm vi nào, bao gồm cả phạm vi toàn cục (global scope),
điều này làm hằng số trở nên hữu ích cho các phần của code cần sử dụng giá trị của chúng.

Sự khác biệt cuối cùng là các hằng số chỉ có thể khai báo ở dạng biểu thức hằng, chứ không phải
kết quả của một giá trị được tính toán lúc runtime.

Dưới đây là một ví dụ về khai báo hằng:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Tên của hằng số là `THREE_HOURS_IN_SECONDS` và giá trị của nó được thiết lập là
kết quả của 60 (số giây trong một phút) nhân 60 (số phút trong một giờ)
nhân 3 (số giờ chúng ta muốn đếm trong chương trình). Quy ước đặt tên hằng số
của Rust là đặt tên với chữ in hoa và phân cách giữa các từ bằng dấu gạch dưới.
Trình biên dịch có thể tính toán hạn chế một số phép toán ở thời điểm biên dịch,
điều này cho phép chúng ta chọn viết ra giá trị này theo một cách dễ hiểu và dễ
xác minh hơn, hơn là để giá trị 10,800 cho hằng số này.
Hãy xem [Rust Reference’s section on constant evaluation][const-eval] để biết thêm
thông tin về những phép toán có thể được dùng khi khai báo hằng số.

Hằng số hợp lệ trong toàn bộ thời gian chương trình chạy, trong phạm vi
mà chúng được khai báo. Thuộc tính này làm cho hằng số trở nên hữu ích trong cho
các giá trị trong miền ứng dụng của bạn mà nhiều phần của chương trình có thể cần chúng,
chẳng hạn như số điểm tối đa mà bất cứ người chơi nào trong game có thể kiếm được
hoặc tốc độ ánh sáng.

Đặt tên cho các giá trị được mã hóa cứng được sử dụng trong suốt chương trình của bạn
dưới dạng hằng số rất hữu ích trong việc truyền đạt ý nghĩa của giá trị đó đến với những
người bảo trì code trong tương lai. Nó cũng hữu ích khi chỉ có một vị trí trong code của bạn
mà bạn sẽ cần thay đổi nếu giá trị được mã hóa cứng đó cần được cập nhật trong tương lai.

### Phủ bóng (Shadowing)

Như bạn đã thấy trong bài hướng dẫn game đoán số trong [Chương 
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->, bạn có thể khai báo
một biến mới với cùng tên gọi như biến trước đó. Rustaceans nói rằng biến đầu
tiên bị *phủ bóng (shadowed)* bởi biến thứ hai, điều này có nghĩa là giá trị của
biến thứ hai là giá trị mà chương trình thấy khi biến đó được sử dụng. Chúng ta
có thể phủ bóng một biến bằng cách sử dụng tên của biến đó và sử dụng lại từ khóa 
`let` như sau:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Đầu tiên, chương trình gán `x` với giá trị `5`. Sau đó phủ bóng `x` bằng cách 
lặp lại `let x =`, lấy giá trị ban đầu và cộng `1` do đó, giá trị của 
`x` khi đó là `6`. Sau đó, trong phạm vi bên trong, câu lệnh `let` thứ ba cũng 
phủ bóng `x`, nhân giá trị trước đó với `2` để `x` được giá trị là `12`.
Khi phạm vi kết thúc, việc phủ bóng ở phạm vi bên trong kết thúc và `x` trả về giá trị `6`.
Khi chúng ta chạy chương trình, đầu ra sẽ như sau:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Phủ bóng khác với đánh dấu một biến với `mut`, bởi vì chúng ta sẽ gặp lỗi
compile-time nếu chúng ta vô tình cố gắng gán lại cho biến này mà không 
sử dụng từ khóa `let`. Bằng cách sử dụng `let`, chúng ta có thể  thực hiện
một số phép biến đổi trên một giá trị nhưng biến đó không thể thay đổi được
sau khi các phép biến đổi đó hoàn tất.

Sự khác biệt khác giữa `mut` and shadowing là do chúng ta đang tạo ra một biến
mới một cách hiệu quả khi chúng ta sử dụng lại từ khóa `let`, chúng ta có thể
thay đổi kiểu dữ liệu của giá trị nhưng sử dụng lại tên đó. Ví dụ, giả sử
chương trình của chúng ta yêu cầu người dùng hiển thị bao nhiêu khoảng cách mà
họ muốn giữa một số đoạn text bằng cách nhập các ký tự khoảng trắng và sau đó
chúng ta muốn lưu trữ đầu vào đó dưới dạng số:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

Biến `spaces` đầu tiên có kiểu string và biến `spaces` thứ hai có kiểu dữ liệu
số. Do đo shadowing giúp chúng ta không cần phải đặt các tên khác nhau như
`spaces_str` và `spaces_num`; thay vào đó, chúng ta có thể sử dụng lại cái
tên đơn giản hơn `spaces`. Tuy nhiên, nếu chúng ta cố gắng sử dụng `mut` cho
việc này, chúng ta sẽ gặp lỗi compile-time như bên dưới:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Lỗi cho biết chúng ta không được phép thay đổi kiểu dữ liệu của một biến:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Bây giờ chúng ta đã khám phá cách các biến hoạt động, hãy cùng xem xét
thêm các kiểu dữ liệu mà Rust có thể có.

[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: ../reference/const_eval.html
