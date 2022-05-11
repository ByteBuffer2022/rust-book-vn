# Introduction

> Note: This edition of the book is the same as [The Rust Programming
> Language][nsprust] available in print and ebook format from [No Starch
> Press][nsp].

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

Chào mừng bạn đến với *The Rust Programming Language*, một cuốn sách giới thiệu về Rust.
Ngôn ngữ lập trình Rust giúp bạn viết phần mềm nhanh hơn, đáng tin cậy hơn.
High-level ergonomics and low-level control thường mâu thuẫn nhau trong thiết kế ngôn
ngữ lập trình; Rust thách thức xung đột đó. Thông qua việc cân bằng giữa năng lực kỹ thuật
mạnh mẽ và kinh nghiệm tuyệt vời của nhà phát triển, Rust cho bạn tùy chọn để kiểm soát
các chi tiết cấp thấp (chẳng hạn như sử dụng bộ nhớ) mà không gặp phải mọi rắc rối
truyền thống liên quan đến việc kiểm soát như vậy.

## Rust dành cho ai

Rust là lý tưởng cho nhiều người vì nhiều lý do. 
Hãy xem xét một số nhóm quan trọng nhất.

### Nhóm các developer

Rust đang chứng tỏ là một công cụ hiệu quả để cộng tác giữa các nhóm developer lớn
với các cấp độ khác nhau về kiến thức lập trình hệ thống. Code cấp thấp dễ mắc phải
nhiều bugs tinh vi, mà ở hầu hết các ngôn ngữ khác chỉ có thể gặp phải thông qua thử nghiệm
rộng rãi và xem xét code cẩn thận những developer có kinh nghiệm. Trong Rust, trình biên dịch
đóng vai trò người gác cổng bằng cách từ chối biên dịch code với các bugs khó nắm bắt này,
bao gồm các bug đồng thời. Bằng cách làm việc cùng với trình biên dịch, nhóm có thể dành thời
gian tập trung vào logic của chương trình hơn là tìm kiếm các bug.

Rust cũng mang các công cụ dành cho nhà phát triển vào thế giới lập trình hệ thống:

* Cargo, công cụ bao gồm việc xây dựng và quản lý dependency, giúp việc thêm, biên dịch và
 quản lý các dependency trở nên dễ dàng và nhất quán trên toàn bộ hệ sinh thái Rust.
* Rustfmt đảm bảo coding style nhất quán giữa các developer.
* Máy chủ Ngôn ngữ Rust hỗ trợ tích hợp Môi trường Phát triển Tích hợp (Integrated Development
 Environment, IDE) cho việc hoàn thiện code và thông báo lỗi nội tuyến.

Bằng cách sử dụng những công cụ này và các công cụ khác trong hệ sinh thái Rust, các developer
có thể làm việc hiệu quả trong khi viết code cấp hệ thống.

### Sinh viên

Rust dành cho sinh viên, những người mà quan tâm tìm hiểu các khái niệm hệ thống. Sử dụng Rust,
nhiều người đã học về các chủ đề như phát triển hệ điều hành. Cộng đồng rất hoan nghênh và sẵn
lòng trả lời các câu hỏi của các bạn sinh viên. Thông qua những nỗ lực chẳng hạn như quyển sách
này, nhóm Rust muốn làm cho các khái niệm hệ thống trở trên dễ tiếp cận hơn với nhiều người,
đặc biệt nhất là những người mới lập trình.

### Các công ty

Hàng trăm công ty, lớn và nhỏ, sử dụng Rust trong sản xuất cho nhiều nhiệm vụ khác nhau.
Những tác vụ đó bao gồm công cụ dòng lệnh, dịch vụ web, DevOps tooling, thiết bị nhúng,
phân tích và chuyển mã âm thanh và video, tiền điện tử, bioinformatics, công cụ tìm kiếm,
các ứng dụng Internet of Things, machine learning, và thậm chí các phần chính của trình
duyệt Firefox.

### Các developer mã nguồn mở

Rust dành cho những người muốn xây dựng ngôn ngữ lập trình Rust, cộng đồng, công cụ nhà phát triển,
và các thư viện. Chúng tôi muốn bạn đóng góp cho ngôn ngữ Rust.

### Những người coi trọng tốc độ và sự ổn định

Rust dành cho những ai khao khát tốc độ và sự ổn định trong một ngôn ngữ. Với tốc độ, chúng tôi
muốn nói đến tốc độ của chương trình mà bạn có thể tạo ra bằng Rust và tốc độ mà Rust cho phép
bạn viết chúng. Các kiểm tra của trình biên dịch Rust đảm bảo tính ổn định thông qua việc bổ sung
tính năng và tái cấu trúc. Điều này trái ngược với the brittle legacy code trong các ngôn ngữ
không có các kiểm tra này, mà các developer thường e ngại sửa đổi. Bằng cách cố gắng tạo ra các
tính năng cấp cao hơn, trừu tượng không tốn chi phí có thể biên dịch thành code cấp thấp hơn nhanh
như code được viết thủ công, Rust cũng nỗ lực làm cho code an toàn trở thành code nhanh.

Ngôn ngữ Rust cũng hi vọng sẽ hỗ trợ nhiều người dùng khác; những người được đề cập ở đây chỉ là
một số bên liên quan lớn nhất. Nhìn chung, tham vọng lớn nhất của Rust là loại bỏ những đánh đổi
mà các lập trình viên đã chấp nhận trong hàng thập kỷ bằng cách cung cấp sự an toàn *và* năng suất,
tốc độ *và* ergonomics. Hãy thử Rust và xem liệu các lựa chọn của nó có phù hợp với bạn không.

## Cuốn sách này dành cho ai

Cuốn sách này giả định bạn đã viết code bằng một ngôn ngữ lập trình khác nhưng không đưa ra bất
kỳ giả định nào về ngôn ngữ lập trình đó là ngôn ngữ nào. Chúng tôi đã cố gắng làm cho tài liệu
có thể tiếp cận rộng rãi với những người từ nhiều nền tảng lập trình khác nhau. Chúng tôi không
dành nhiều thời gian nói về lập trình *là gì* hoặc cảm nghĩ về nó. Nếu bạn là hoàn toàn mới về
lập trình, bạn sẽ được phục vụ tốt hơn bằng cách đọc một quyển sách giới thiệu cụ thể về lập
trình.

## Làm thế nào để sử dụng cuốn sách này

Nói chung, cuốn sách này giả định bạn đang đọc nó theo thứ tự từ trước đến sau. Các chương sau
xây dựng dựa trên các khái niệm trong các chương trước đó và các chương trước đó có thể không
đi sâu vào chi tiết một chủ đề, chúng ta thường xem lại chủ đều trong chương sau.

Bạn sẽ tìm thấy hai loại chương trong quyển sách này: các chương về khái niệm và các chương
dự án. Trong các chương khái niệm, bạn sẽ tìm hiểu về một khía cạnh của Rust. Trong các chương
dự án, chúng ta sẽ xây dựng các chương trình nhỏ cùng nhau, áp dụng những gì bạn đã được học
cho đến nay. Các chương 2, 12 và 20 là các chương về dự án; phần còn lại là các chương về khái niệm.

Chương 1 giải thích cách cài đặt Rust, cách viết chương trình “Hello, world!” và
cách sử dụng Cargo, công cụ xây dựng và quản lý package của Rust. Chương 2 là phần
giới thiệu thực tế về ngôn ngữ Rust. Ở đây chúng ta đề cập các khái niệm ở cấp độ cao
và các chương sau sẽ cung cấp thêm chi tiết. Nếu bạn muốn bẩn tay ngay lập tức, Chương 2
chính là nơi dành cho điều đó. Lúc đầu, bạn thậm chí sẽ muốn bỏ qua Chương 3, bao gồm các
tính năng Rust tương tự như các ngôn ngữ lập trình khác và đi thẳng tới Chương 4 để 
tìm hiểu về hệ thống quyền sở hữu (ownership) của Rust. Tuy nhiên, nếu bạn là một người
học đặc biệt tỉ mỉ, thích tìm hiểu mọi chi tiết trước khi chuyển qua phần tiếp theo, 
bạn có thể bỏ qua Chương 2 và đi thẳng đến Chương 3, trở lại Chương 2 khi bạn muốn 
làm việc trên một dự án ứng dụng những chi tiết bạn đã được học.

Chương 5 thảo luận về structs và phương thức, và Chương 6 nói về enums, biểu thức `match`,
và cấu trúc control flow `if let`. Bạn sẽ sử dụng structs và enums để tạo ra các kiểu tùy 
chọn trong Rust.

Trong Chương 7, bạn sẽ tìm hiểu về hệ thống module của Rust và về các nguyên tắc bảo mật để
tổ chức code của bạn và Giao diện Lập trình Ứng dụng (Application Programming Interface, API)
công khai của nó. Chương 8 thảo luận về một số cấu trúc dữ liệu collection phổ biến mà thư viện
chuẩn cung cấp, như vectors, strings, và hash maps. Chương 9 khám phá triết lý và kỹ thuật
xử lý error của Rust.

Chương 10 đào sâu vào generics, traits, và lifetimes, cung cấp cho bạn sức mạnh để định nghĩa
code ứng dụng cho nhiều loại dữ liệu. Chương 11 nói về testing, mà ngay cả với các đảm bảo an
toàn của Rust cần thiết để đảm bảo logic chương trình của bạn là chính xác. Trong Chương 12,
chúng ta sẽ xây dựng cách triển khai của riêng mình với một nhóm chức năng từ công cụ dòng lệnh
`grep` tìm kiếm đoạn text trong file. Đối với điều này, chúng ta sẽ sử dụng các khái niệm mà
chúng ta đã thảo luận trong các chương trước.

Chương 13 khám phá về closures và iterators: các tính năng của Rust đến từ ngôn ngữ lập trình
chức năng. Trong Chương 14, chúng ta sẽ xem xét Cargo sâu hơn và nói về các phương pháp hay nhất
để chia sẻ thư viện của bạn với những người khác. Chương 15 thảo luận về smart pointers mà thư
viện chuẩn cung cấp và traits để kích hoạt chức năng của chúng.

Trong Chương 16, chúng ta sẽ đi qua các mô hình lập trình đồng thời khác nhau và nói về cách Rust
giúp bạn lập trình đa luồng một cách dễ dàng. Chương 17 xem xét cách Rust idioms so sánh với các
nguyên tắc lập trình hướng đối tượng mà bạn có thể quen thuộc.

Chương 18 là tài liệu tham khảo về patterns và pattern matching, đây là các cách mạnh mẽ thể hiện
ý tưởng trong suốt chương trình Rust. Chương 19 chứa một loạt các chủ đề nâng cao được quan tâm,
bao gồm Rust không an toàn, macros, và nhiều hơn nữa về lifetimes, traits, types, functions
và closures.

Trong Chương 20, chúng ta sẽ hoàn thành một dự án trong đó chúng ta sẽ triển khai một máy chủ web
đa luồng cấp thấp!

Cuối cùng, một số phụ lục chứa thông tin hữu ích về ngôn ngữ trong một định dạng giống như tham
khảo hơn. Phụ lục A bao gồm các từ khóa của Rust, Phụ lục B bao gồm các toán tử và ký hiệu của Rust,
Phụ lục C bao gồm các derivable traits được cung cấp bởi thư viện chuẩn, Phụ lục D bao gồm một số công
cụ phát triển hữu ích và Phụ lục E giải thích các phiên bản Rust.

Mọi phương pháp đọc cuốn sách này đều đúng: nếu bạn muốn bỏ qua chương nào đó, hãy làm điều đó.
Bạn có thể phải quay lại các chương trước đó nếu bạn gặp bất kỳ sự khó hiểu nào.
Nhưng hãy làm bất cứ điều gì có ích cho bạn.

<span id="ferris"></span>

Một phần quan trọng của quá trình học Rust là học cách đọc thông báo lỗi mà trình biên dịch
hiển thị: chúng sẽ hướng dẫn bạn cách làm việc với code.
Do đó, chúng tôi sẽ cung cấp nhiều ví dụ minh họa không biên dịch cùng với thông báo lỗi mà
trình biên dịch sẽ hiển thị cho bạn trong mỗi tình huống. Biết rằng nếu bạn nhập và chạy một 
ví dụ ngẫu nhiên, nó có thể sẽ không biên dịch! Đảm bảo rằng bạn đọc đoạn text xung quanh để
xem liệu ví dụ bạn đang cố chạy có bị lỗi hay không. Ferris cũng sẽ giúp bạn phân biệt code
không hoạt động:

| Ferris                                                                                                           | Meaning                                          |
|------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain" alt="Ferris with a question mark"/>            | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain" alt="Ferris throwing up their hands"/>                   | This code panics!                                |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain" alt="Ferris with one claw up, shrugging"/> | This code does not produce the desired behavior. |

Trong hầu hết các tình huống, chúng tôi sẽ dẫn bạn đến phiên bản chính xác
của bất kỳ đoạn code nào không biên dịch được.

## Source Code

Bạn có thể tìm thấy các file nguồn mà cuốn sách này được tạo nên trên 
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/main/src
