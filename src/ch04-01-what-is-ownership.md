## What Is Ownership?

*Ownership* là một tập hợp các quy tắc chi phối cách một chương trình Rust quản lý bộ nhớ.
Tất cả các chương trình phải quản lý cách chúng sử dụng bộ nhớ của máy tính khi chạy.
Một số ngôn ngữ có garbage collection (bộ thu thập rác) liên tục tìm kiếm bộ nhớ không còn được sử dụng khi chương trình chạy; trong các ngôn ngữ khác, lập trình viên phải cấp phát và giải phóng bộ nhớ một cách rõ ràng. Rust sử dụng cách tiếp cận thứ ba: bộ nhớ được quản lý thông qua một hệ thống sở hữu (system of ownership) với một tập hợp các quy tắc mà trình biên dịch kiểm tra khi biên dịch. Nếu bất kỳ quy tắc nào bị vi phạm, chương trình sẽ không biên dịch. Không có tính năng nào của ownership sẽ làm chậm chương trình của bạn khi nó đang chạy.

Vì ownership là một khái niệm mới đối với nhiều lập trình viên, nên nó cần một chút thời gian để làm quen. Tin tốt là bạn càng có nhiều kinh nghiệm hơn với Rust và các quy tắc của hệ thống sở hữu (ownership system) hơn, bạn càng thấy dễ dàng hơn khi phát triển code an toàn và hiệu quả một cách tự nhiên. Hãy kiên trì! 

Khi bạn hiểu ownership, bạn sẽ có một nền tảng vững chắc để hiểu các tính năng làm cho Rust trở nên độc đáo. Trong chương này, bạn sẽ học ownership bằng cách làm việc thông qua một số ví dụ tập trung vào cấu trúc dữ liệu rất phổ biến: chuỗi (strings).

> ### Stack và Heap
>
> Nhiều ngôn ngữ lập trình không yêu cầu bạn phải nghĩ về stack và heap thường xuyên.
> Nhưng trong một ngôn ngữ lập trình hệ thống như Rust, việc một giá trị
> nằm trên stack hoặc sẽ ảnh hưởng đến cách ngôn ngữ hoạt động và tại sao
> bạn phải đưa ra những quyết định nhất định. Các phần của ownership sẽ được mô tả
> liên quan đến stack và heap ở phần sau của chương này, vì vậy đây là một 
> lời giải thích ngắn gọn trong quá trình chuẩn bị.
>
> Cả stack và heap đều là những phần bộ nhớ có sẵn cho code của bạn để sử dụng
> trong runtime, nhưng chúng được cấu trúc theo những cách khác nhau. Stack lưu trữ
> lưu trữ các giá trị theo thứ tự mà nó nhận được và xóa các giá trị theo thứ tự 
> ngược lại. Điều này được gọi là *last in, first out* (Vào sau, ra trước). Hãy nghĩ
> về một chồng đĩa: Khi bạn thêm nhiều đĩa, bạn đặt chúng lên trên đầu và khi bạn 
> cần một chiếc đĩa, bạn lấy một cái trên đầu ra. Thêm hoặc xóa các đĩa ở giữa
> hoặc dưới cùng cũng sẽ không hoạt động! Thêm dữ liệu được gọi là *pushing
> onto the stack*, và xóa dữ liệu được gọi là *popping off the stack*. Tất 
> cả dữ liệu được lưu trữ trên stack phải có kích thước cố định (fixed size), đã biết.
> Thay vào đó dữ liệu có kích thước không xác định (unknown size) tại thời điểm biên 
> dịch hoặc kích thước có thể thay đổi phải được lưu trữ trên heap.
>
> Heap ít được tổ chức hơn: khi bạn đặt dữ liệu trên heap, bạn gửi yêu cầu một
> khoảng trống nhất định trong bộ nhớ. Bộ cấp phát bộ nhớ tìm thấy một chỗ trống  
> trên heap đủ lớn, đánh dấu nó là đang được sử dụng, và trả về một *con trỏ*, đó
> là địa chỉ cuả vị trí đó. Quá trình này được gọi là *allocating on the heap*
> (cấp phát trên heap) và đôi khi được viết tắt là *allocating* (việc đẩy các giá trị 
> vào stack không được coi là cấp phát). Vì con trỏ tới heap có kích thước cố định
> (fixed size) và đã biết, bạn có thể lưu trữ con trỏ trên stack, nhưng khi bạn muốn
> dữ liệu thực sự, bạn phải đi theo con trỏ. Hãy nghĩ đến việc tìm chỗ ngồi tại một
> nhà hàng. Khi bạn tới đó, bạn sẽ cần nói số người trong nhóm của bạn, và 
> nhần viên sẽ tìm một bàn trống đủ chỗ cho mọi người và dẫn bạn tới đó. Nếu
> ai đó trong nhóm của bạn đến muộn, họ có thể hỏi chỗ ngồi của bạn ở đâu để tìm.
>
> Đẩy dữ liệu vào stack nhanh hơn là cấp phát trên heap vì bộ cấp phát không 
> bao giờ phải tìm kiếm một nơi để lưu dữ liệu mới; vị trí đó luôn ở trên cùng 
> của stack. Tương tự, việc phân bổ không gian trên heap đòi hỏi nhiều công việc 
> hơn, bởi vì bộ cấp phát trước tiên phải tìm một không gian đủ lớn để chứa dữ liệu
> sau đó thực hiện ghi sổ  (bookkeeping) để chuẩn bị cho đợt cấp phát tiếp theo.
>
> Truy cập dữ liệu trong heap chậm hơn so với truy cập dữ liệu trên stack vì
> bạn phải đi theo một con trỏ để đến đó. Các bộ xử lý hiện đại nhanh hơn nếu chúng 
> ít nhảy qua lại bộ nhớ hơn. Tiếp tục tương tự, hãy xem xét một máy chủ tại một 
> nhà hàng nhận các orders từ nhiều bàn. Cách hiệu quả nhất là nhận tất cả 
> các orders tại một bàn trước khi chuyển sang bàn tiếp theo. Nhận một
> order từ bàn A, sau đó một order từ bàn B, sau đó lại một từ bàn A lần nữa, 
> và sau đó lại một từ B sẽ là một quá trình chậm hơn nhiều. Cũng vì lẽ ấy, 
> một bộ xử lý có thể thực hiện công việc của nó tốt hơn nếu nó làm việc với các 
> dữ liệu gần nhau (như trên stack) thay vì các dữ liệu xa nhau (như trên heap).
> Việc phân bổ một lượng lớn không gian trên heap cũng có thể mất thời gian.
>
> Khi code của bạn gọi một hàm, các giá trị đã được truyền vào hàm
> (có thể bao gồm cả con trỏ đến dữ liệu trên heap) và các biến cục bộ của hàm  
> được đẩy lên stack. Khi hàm kết thúc, những giá trị đó bị lấy ra khỏi stack.
>
> Theo dõi những phần code nào đang sử dụng dữ liệu nào trên heap,
> giảm thiểu số lượng dữ liệu trùng lặp trên heap, và dọn dẹp dữ liệu không sử dụng
> trên heap do đó bạn không cạn kiệt khoảng trống là tất cả những vấn đề mà ownership
> giải quyết. Khi bạn hiểu về ownership, bạn không cần phải suy nghĩ về stack và 
> heap thường xuyên, nhưng biết được mục đích chính của ownership là để quản lý 
> dữ liệu heap có thể giúp giải thích được tại sao nó hoạt động như vậy.

### Các quy tắc của Ownership

Trước tiên, hãy xem các quy tắc ownership. Hãy ghi nhớ những quy tắc này khi chúng ta xem thông qua các ví dụ minh họa chúng:

* Mỗi giá trị trong Rust có một biến gọi là owner của nó.
* Chỉ có thể có một owner tại một thời điểm.
* Khi owner ra khỏi phạm vi (scope) của nó, giá trị sẽ bị bỏ đi.

### Phạm vi biến

Bây giờ chúng ta đã qua cú pháp Rust cơ bản, chúng ta sẽ không viết tất cả các dòng
 `fn main() {` trong ví dụ nữa, vì vậy khi bạn theo dõi các ví dụ hãy chắc chắn
đã tự đặt các ví dụ bên trong hàm `main` một cách thủ công. Vì vậy, các ví dụ của
chúng ta trông sẽ ngắn gọn hơn một chút, cho phép chúng ta tập trung vào những chi 
tiết hơn là các đoạn mã soạn sẵn.

Trong ví dụ đầu tiên, chúng ta sẽ nói về *phạm vi* (scope) của một số biến. Scope
là phạm vi trong một chương trình mà một item có giá trị. Giả sử ta có biến sau:

```rust
let s = "hello";
```

Biến `s` đề cập tới một chuỗi kí tự (string literal), nơi mà giá trị của chuỗi được 
*gán cứng* (hardcoded) vào một văn bản trong chương trình. Biến có giá trị tại thời điểm mà nó 
được khai báo cho đến khi kết thúc *scope* hiện tại. Trong Listing 4-1 bên dưới có 
các comment chỉ ra nơi biến `s` hợp lệ 

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-01/src/main.rs:here}}
```

<span class="caption">Listing 4-1: Một biến và phạm vi mà nó hợp lệ</span>

Nói cách khác, có hai điểm quan trọng về thời gian ở đây:

* Khi `s` *đi vào scope*, nó có giá trị.
* Nó vẫn có giá trị cho tới khi nó *đi ra khỏi scope*.

Tại thời điểm này, mối quan hệ giữa phạm vi và thời điểm các biến có giá trị tương tự như trong các ngôn ngữ lập trình khác. Bây giờ dựa trên hiểu biết này bây giờ chúng tôi sẽ giới thiệu về kiểu `String`.

### Kiểu `String`

Để minh họa các quy tắc về  ownership,chúng ta cần một kiểu dữ liệu phức tạp hơn những kiểu mà chúng ta đã đề cập trong phần [“Data Types”][data-types]<!-- ignore --> ở chương 3. Các loại được đề cập trước đây đều có kích thước đã biết, có thể được lưu trữ trên stack và bị đẩy ra khỏi stack khi phạm vi của chúng kết thúc, và có thể được sao chép nhanh chóng và đơn để tạo ra một cái mới trong trường hợp độc lập nếu một phần khác của code cần sử dụng cùng một giá trị trong một phạm vi khác.Nhưng chúng ta muốn xem xét dữ liệu được lưu trữ trên heap và khám phá cách Rust biết khi nào cần dọn dẹp dữ liệu đó, và kiểu `String` là một ví dụ tuyệt vời.

Chúng ta sẽ tập trung vào các phần của `String` liên quan đến ownership. Các khía cạnh này cũng áp dụng cho các kiểu dữ liệu phức tạp khác, cho dù chúng được cung cấp bởi thư viện chuẩn hay do bạn tạo. Chúng ta sẽ thảo luận về `String` sâu hơn ở [Chapter 8][ch8]<!-- ignore -->.

Chúng ta đã nhìn thấy những chuỗi kí tự (string literals) có giá trị được *gán cứng* (hardcoded) trong chương trình. Các ký tự kiểu chuỗi rất tiện lợi, nhưng chúng không phù hợp với mọi tình huống mà chúng ta có thể muốn sử dụng văn bản. Một lý do là chúng không thay đổi. Một điều khác là không phải mọi giá trị chuỗi đều có thể được biết khi chúng ta viết mã của mình: ví dụ: nếu chúng ta muốn lấy dữ liệu đầu vào của người dùng và lưu trữ nó thì sao? Trong tình huống này, Rust có một kiểu chuỗi thứ hai, `String`. Kiểu dữ liệu này được phân bổ trên heap, như thế nó có thể lưu trữ một khối lượng văn bản không biết trước ở thời điểm biên dịch. Bạn có thể tạo một `String` từ một string literal bằng cách sử dụng hàm `from`, như sau:

```rust
let s = String::from("hello");
```

Dấu hai chấm `::` là một toán tử cho phép chúng ta gọi hàm (namespace) `from` với kiểu `String` thay vì sử dụng một số loại tên như `string_from`. Chúng ta sẽ thảo luận về cú pháp này nhiều hơn trong phần [“Method Syntax”][method-syntax]<!-- ignore --> chương 5 and when we talkvà khi chúng ta nói về namespacing với module ở phần [“Paths for Referring to an Item in the Module Tree”][paths-module-tree]<!-- ignore --> trong chương 7.

Kiểu chuỗi này cũng có thể biến đổi giá trị (mutated):

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-01-can-mutate-string/src/main.rs:here}}
```

Vậy, sự khác biệt ở đây là gì? Tại sao  `String` có thể biến đổi trong khi chuỗi kí tự (string literals)
thì không? Sự khác biệt là cách hai loại này tương tác với bộ nhớ.

### Memory and Allocation (Bộ nhớ và cấp phát)

Trong trường hợp một chuỗi kí tự (string literal), chúng ta biết nội dung tại thời điểm biên dịch, vì vậy văn bản được gán cứng (hardcoded) trực tiếp vào tệp thực thi cuối cùng. Đây là lý do tại sao các ký tự chuỗi (string literals) nhanh và hiệu quả. Nhưng những thuộc tính này chỉ đến từ tính bất biến của chuỗi ký tự. Thật không may, chúng ta không thể đặt một blob memory vào hệ nhị phân cho mỗi đoạn văn bản
có kích thước không xác định tại thời điểm biên dịch và kích thước của chúng có thể thay đổi trong khi chạy chương trình.

Với kiểu `String`, để hỗ trợ một đoạn văn bản có thể thay đổi, có thể phát triển, chúng ta cần phân bổ một lượng bộ nhớ trên heap, không xác định tại thời điểm biên dịch, để giữ nội dung. Điều này có nghĩa là:

* Bộ nhớ phải được yêu cầu từ bộ cấp phát bộ nhớ trong thời gian chạy (runtime).
* Chúng ta cần một cách để trả lại bộ nhớ này cho bộ cấp phát khi chúng ta hoàn thành `String`.

Phần đầu tiên do chúng ta thực hiện: khi chúng ta gọi `String::from`, việc triển khai của nó yêu cầu bộ nhớ mà nó cần. Điều này khá phổ biến trong các ngôn ngữ lập trình.

Tuy nhiên, phần thứ hai thì khác. Trong các ngôn ngữ có *garbage collector(GC)*, theo dõi và dọn dẹp bộ nhớ không còn được sử dụng nữa và chúng ta không cần phải suy nghĩ về điều đó. Trong hầu hết các ngôn ngữ không có GC, chúng ta có trách nhiệm xác định khi nào bộ nhớ không còn được sử dụng và dùng code để trả lại bộ nhớ một cách rõ ràng, giống như chúng ta đã làm để yêu cầu nó. Trong lịch sử để thực hiện điều này một cách chính xác là một vấn đề khó khăn của lập trình. Nếu chúng ta quên, chúng ta sẽ lãng phí bộ nhớ. Nếu chúng ta làm điều đó quá sớm, chúng ta sẽ có một biến không hợp lệ. Nếu chúng tôi làm điều đó hai lần, đó cũng là một lỗi. Chúng ta cần ghép chính xác một `allocate` với một `free`.

Rust đi theo một con đường khác: bộ nhớ sẽ tự động được trả về khi biến sở hữu nó vượt ra khỏi phạm vi (scope). Dưới đây là một phiên bản của ví dụ về scope từ Listing 4-1 sử dụng một `String` thay vì một chuỗi kí tự (string literal):

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-02-string-scope/src/main.rs:here}}
```

Có một điểm tự nhiên mà chúng ta có thể trả lại vùng nhớ `String` của chúng ta cho bộ cấp phát: khi `s` đi ra khỏi scope. Khi một biến vượt ra ngoài scope, Rust gọi một hàm đặc biệt cho chúng ta. Hàm này được gọi là [`drop`][drop]<!-- ignore -->,  và nó là nơi mà tác giả của `String` có thể đặt code để trả lại bộ nhớ. Rust gọi `drop` tự động tại nơi dấu đóng ngoặc nhọn.

> Note: Lưu ý: Trong C ++, kiểu phân bổ tài nguyên này ở cuối vòng đời của một item đôi khi được gọi là *Resource Acquisition Is Initialization (RAII)*.
> Hàm `drop`  trong Rust sẽ quen thuộc hơn với bạn nếu bạn từng dùng mô hình RAII.

Mô hình này có tác động sâu sắc đến cách viết code của Rust. Nó có vẻ đơn giản ngay bây giờ, nhưng hành vi của code có thể không mong muốn trong các tình huống phức tạp hơn khi chúng ta muốn có nhiều biến sử dụng dữ liệu chúng ta đã phân bổ trên heap. Bây giờ chúng ta hãy khám phá một số tình huống đó.

#### Cách các biến và dữ liệu tương tác: Move

Nhiều biến có thể tương tác với cùng một dữ liệu theo những cách khác nhau trong Rust. Hãy xem một ví dụ sử dụng một số nguyên trong Listing 4-2.

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-02/src/main.rs:here}}
```

<span class="caption">Listing 4-2: Gán giá trị nguyên của biến `x` vào `y`</span>

Chúng ta có thể đoán được đoạn code này đang thể hiện gì: “gán giá trị `5` vào `x`; sau đó tạo một bản sao của giá trị của  `x` và gán nó bằng `y`.” Bây giờ chúng ta có hai biến, `x` và `y`, cả 2 đều bằng `5`. Đây thực sự là những gì đang xảy ra, bởi vì số nguyên là các giá trị đơn giản có giá trị cố định, đã biết, và hai giá trị `5` này được đẩy vào stack.

Giờ hãy cùng nhìn vào phiên bản `String`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-03-string-move/src/main.rs:here}}
```

Điều này trông rất giống nhau, vì vậy chúng tôi có thể giả định rằng cách nó hoạt động sẽ giống nhau: nghĩa là, dòng thứ hai sẽ tạo một bản sao của giá trị trong `s1` và gán nó cho `s2`. Nhưng đây không phải là điều sẽ diễn ra.

Hãy xem Hình 4-1 để xem điều gì đang xảy ra với `String` trong thực tế. `String` được tạo thành từ ba phần, hiển thị ở bên trái: một con trỏ tới bộ nhớ chứa nội dung của string, một độ dài, và một dung lượng (capacity). Nhóm dữ liệu này được lưu trữ trên stack. Ở bên phải là bộ nhớ trên heap chứa nội dung.

<img alt="String in memory" src="img/trpl04-01.svg" class="center" style="width: 50%;" />

<span class="caption">Hình 4-1: Biểu diễn trong bộ nhớ của một `String` giữ giá trị `"hello"` gán cho `s1`</span>

Độ dài là lượng bộ nhớ, tính bằng byte, mà nội dung của `String` đang sử dụng. Dung lượng (capacity) là tổng dung lượng bộ nhớ, tính bằng byte mà `String` đã nhận được từ bộ cấp phát. Có sự khác biệt giữa độ dài và dung lượng (capacity), nhưng không phải trong bối cảnh này, vì vậy hiện tại, bạn có thể bỏ qua dung lượng (capacity).

Khi chúng ta gán `s1` cho `s2`, dữ liệu `String` được sao chép, có nghĩa là chúng ta sao chép con trỏ, độ dài và dung lượng trên stack. Chúng ta không sao chép dữ liệu trên heap mà con trỏ chỉ tới. Nói cách khác, dữ liệu được biểu diễn trong bộ nhớ như Hình 4-2.

<img alt="s1 and s2 pointing to the same value" src="img/trpl04-02.svg" class="center" style="width: 50%;" />

<span class="caption">Hình 4-2: Biểu diễn trong bộ nhớ của biến `s2` chứa bản sao của con trỏ, độ dài và dung lượng của `s1`</span>

Biểu diễn trên Hình 4-2 *không* giống như Hình 4-3, đó là bộ nhớ sẽ trông như thế nào nếu thay vào đó Rust cũng sao chép dữ liệu heap. Nếu Rust đã làm điều này, `s2 = s1` có thể rất tốn kém về hiệu suất thời gian chạy nếu dữ liệu trên heap lớn.

<img alt="s1 and s2 to two places" src="img/trpl04-03.svg" class="center" style="width: 50%;" />

<span class="caption">Hình 4-3: Một khả năng khác cho những gì `s2 = s1` có thể làm nếu Rust cũng sao chép dữ liệu heap</span>

Trước đó, chúng tôi đã nói rằng khi một biến vượt ra ngoài scope, Rust tự động gọi hàm `drop` và dọn dẹp bộ nhớ heap cho biến đó. Nhưng Hình hiển thị cả hai con trỏ dữ liệu trỏ đến cùng một vị trí. Đây là một vấn đề: khi `s2` và `s1` vượt ra ngoài scope, cả hai sẽ cố gắng giải phóng cùng một bộ nhớ. Điều này được gọi là lỗi *double free* và là một trong những lỗi an toàn bộ nhớ mà chúng tôi đã đề cập trước đây. Giải phóng bộ nhớ hai lần có thể dẫn đến hỏng bộ nhớ, có thể dẫn đến lỗ hổng bảo mật.

Để đảm bảo an toàn cho bộ nhớ, sau dòng `let s2 = s1`, Rust coi `s1` không còn giá trị nữa. Do đó, Rust không cần giải phóng bất cứ thứ gì khi khi `s1` đi ra khỏi scope. Kiểm tra những gì sẽ xảy ra khi bạn cố gắng sử dụng `s1` sau khi `s2` được tạo ra; nó sẽ không hoạt động:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/src/main.rs:here}}
```

Bạn sẽ gặp lỗi như thế này vì Rust ngăn bạn sử dụng tham chiếu không hợp lệ:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-04-cant-use-after-move/output.txt}}
```

Nếu bạn đã nghe về thuật ngữ *sao chép cạn* (shallow copy) và *sao chép sâu* (deep copy) trong khi làm việc với các ngôn ngữ khác, khái niệm sao chép con trỏ, độ dài và dung lượng mà không sao chép dữ liệu có thể nghe giống như tạo một sao chép cạn. Nhưng vì Rust cũng làm mất hiệu lực của biến đầu tiên, thay vì gọi nó là một sao chép cạn, nó được gọi là một *move* (di chuyển). Trong ví dụ này, chúng ta sẽ nói rằng `s1` đã được *moved* vào `s2`. Những gì thực sự xảy ra được thể hiện trong Hình 4-4.

<img alt="s1 moved to s2" src="img/trpl04-04.svg" class="center" style="width: 50%;" />

<span class="caption">Hình 4-4: Biểu diễn trong bộ nhớ sau khi `s1` đã bị vô hiệu </span>

Điều đó giải quyết vấn đề của chúng tôi! Chỉ với `s2` còn giá trị và khi nó vượt ra ngoài scope, chỉ nó sẽ giải phóng bộ nhớ và chúng ta đã hoàn tất.

Ngoài ra, có một lựa chọn thiết kế được ngụ ý bởi điều này: Rust sẽ không bao giờ tự động tạo các bản sao "sâu" (deep copy) dữ liệu của bạn. Do đó, bất kỳ *tự động* sao chép nào cũng có thể được coi là không tốn kém về hiệu suất runtime.

#### Cách tương tác giữa các biến và dữ liệu: Clone

Nếu chúng tôi muốn sao chép sâu dữ liệu heap của `String`, không chỉ là dữ liệu stack, chúng ta có thể sử dụng một phương thức (method) phổ biến được gọi là `clone`. Chúng ta sẽ thảo luận về cú pháp của method trong Chương 5, nhưng vì các method là một tính năng phổ biến trong nhiều ngôn ngữ lập trình, nên có thể bạn đã từng thấy chúng trước đây.

Đây là một ví dụ của method `clone`:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-05-clone/src/main.rs:here}}
```

Đoạn code hoạt động tốt và rõ ràng tạo ra hành vi được hiển thị trong Hình 4-3, nơi dữ liệu heap *được* sao chép.

Khi bạn thây một lệnh gọi `clone`, bạn biết rằng đoạn code nào đó đang được thực thi và code đó có thể khá tốn tài nguyên. Đó là một chỉ báo trực quan cho thấy điều gì đó khác thường đang diễn ra.

#### Dữ liệu chỉ trên Stack (Stack-Only Data): Copy

Có một vấn đề khác mà chúng ta chưa nói đến. Đoạn code này sử dụng số nguyên – một phần trong số đó đã được hiển thị trong Listing 4-2 – hoạt động và hợp lệ:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-06-copy/src/main.rs:here}}
```

Nhưng mã này có vẻ mâu thuẫn với những gì chúng ta vừa học được: chúng ta không gọi lệnh `clone`, nhưng `x` vẫn còn hiệu lực và chưa được move vào `y`.

Lý do là các loại như số nguyên có kích thước đã biết tại thời điểm biên dịch được lưu trữ hoàn toàn trên stack, vì vậy các bản sao của các giá trị thực tế được tạo ra nhanh chóng. Điều đó có nghĩa là không có lý do gì chúng tôi muốn ngăn chặn `x` không còn hợp lệ sau khi chúng tôi tạo biến `y`. INói cách khác, không có sự khác biệt giữa sao chép sâu và sao chép cạn ở đây, vì vậy việc gọi `clone` sẽ không làm gì khác so với cách sao chép cạn thông thường và chúng ta có thể bỏ nó đi.

Rust có một chú thích (annotation) đặc biệt gọi là `Copy` trait chúng ta có thể đặt nó trên những kiểu được lưu trữ trên stack như integer (chúng ta sẽ nói thêm về trait trong Chương 10). Nếu một kiểu thực hiện `Copy` trait, một biến vẫn hợp lệ sau khi gán cho một biến khác. Rust không cho phép chúng ta chú thích một kiểu với `Copy` nếu kiểu của nó, hoặc bất kì phần nào của nó, đã thực thi `Drop` trait. Nếu kiểu cần một cái gì đó đặc biệt để xảy ra khi giá trị vượt ra ngoài scope và chúng ta thêm `Copy` annotation vào kiểu đó, chúng ta sẽ gặp lỗi biên dịch. Để tìm hiểu về cách thêm `Copy` annotation vào kiểu của bạn để triển khai trait, hãy xem [“Derivable Traits”][derivable-traits]<!-- ignore -->
trong Phụ lục C.

Vậy, những loại nào triển khai `Copy` trait? Bạn có thể kiểm tra tài liệu về loại đã cho để chắc chắn, nhưng theo quy tắc chung, bất kỳ nhóm giá trị vô hướng đơn giản nào cũng có thể triển khai `Copy`, và không có kiểu nào yêu cầu allocation hoặc là một dạng của resource có thể triển khai `Copy`. Dưới đây là một số kiểu có thể triển khai `Copy`:

* Tất cả các kiểu số nguyên, chẳng hạn như `u32`.
* Kiểu Boolean, `bool`, với giá trị `true` và `false`.
* Tất cả các kiểu dấu phẩy động, chẳng hạn như `f64`.
* Kiểu ký tự, `char`.
* Tuples, nếu chúng chỉ chứa những kiểu có thể triển khai `Copy`. Ví dụ, `(i32, i32)` có thể triển khai `Copy`, nhưng `(i32, String)` thì không.

### Ownership và Hàm

Ý nghĩa cho việc truyền một giá trị tới một hàm tương tự việc gán một giá trị vào một biến. Truyền một biến cho một hàm sẽ di chuyển hoặc sao chép, giống như phép gán. Listing 4-3 có một ví dụ với một số chú thích hiển thị nơi các biến đi vào và ra khỏi scope.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-03/src/main.rs}}
```

<span class="caption">Listing 4-3: Hàm với ownership và scope được chú thích</span>

Nếu chúng ta cố gắng sử dụng `s` sau khi gọi `takes_ownership`, Rust sẽ báo lỗi biên dịch. Những kiểm tra tĩnh này bảo vệ chúng ta khỏi những sai lầm. Thử thêm code vào `main` sử dụng `s` và `x` để xem bạn có thể sử dụng chúng ở đâu và các quy tắc ownership ngăn bạn làm điều đó ở đâu.

### Trả về giá trị và Scope

Giá trị trả về cũng có thể chuyển giao ownership. Listing 4-4 hiển thị một ví dụ về một hàm trả về một số giá trị, với các chú thích tương tự như trong Listing 4-3.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-04/src/main.rs}}
```

<span class="caption">Listing 4-4: Chuyển ownership của giá trị trả về</span>

Ownership của một biến luôn tuân theo cùng một mẫu: việc gán một giá trị cho một biến khác sẽ di chuyển nó. Khi một biến bao gồm dữ liệu trên heap vượt ra khỏi scope, giá trị sẽ được xóa bởi `drop` trừ khi ownership của dữ liệu đã được chuyển sang một biến khác.

Việc lấy ownership và sau đó trả về ownership với mọi hàm có một chút tẻ nhạt. Điều gì sẽ xảy ra nếu chúng ta muốn cho một hàm sử dụng một giá trị nhưng không có ownership? Điều khá khó chịu là bất kỳ thứ gì chúng ta truyền đi cũng cần phải được truyền lại nếu chúng ta muốn sử dụng lại nó, thêm cả bất kỳ dữ liệu nào đến từ phần thân của hàm mà chúng ta có thể muốn trả về.

Rust cho phép chúng ta trả về nhiều giá trị bằng cách sử dụng một bộ tuple, như được hiển thị trong Listing 4-5.

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-05/src/main.rs}}
```

<span class="caption">Listing 4-5: Returning ownership of parameters</span>

Nhưng đây là quá nhiều thao tác và rất nhiều công việc đối với một khái niệm nên phổ biến. May mắn cho chúng ta, Rust có một tính năng để sử dụng một giá trị mà không cần chuyển ownership, được gọi là *references*.

[data-types]: ch03-02-data-types.html#data-types
[ch8]: ch08-02-strings.html
[derivable-traits]: appendix-03-derivable-traits.html
[method-syntax]: ch05-03-method-syntax.html#method-syntax
[paths-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
[drop]: ../std/ops/trait.Drop.html#tymethod.drop
