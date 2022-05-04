# Enums and Pattern Matching

Trong chương này chúng ta sẽ tìm hiểu về *enumerations*, còn được gọi là *enums*.
Enums cho phép bạn có thể định nghĩa  một kiểu dữ liệu bằng cách liệt kê *các kiểu dữ liệu* của nó. Đầu tiên,
chúng ta sẽ định nghĩa và sử dụng một *enum* để chỉ cách một *enum* có thể mã hoá ý nghĩa cùng với dữ liệu. 
Tiếp theo, chúng ta sẽ khá phá một *enum* đặc biệt hữu ích, được sử dụng rất nhiều trong Rust, 
đó là `Option`, điều này thể hiện một giá trị có thể có giá trị nào đó hoặc không.
Sau đó chúng ta sẽ cách sử dụng `match` với *enum* giúp bạn dễ dàng chạy các đoạn 
code khác nhau cho những giá trị khác nhau của một *enum*. 
Cuối cùng, chúng ta sẽ đề cập đến cách sử dung `if let` để xử lý các *enum* trong code của bạn

Enums là một tính năng được sử dụng trong nhiều ngôn ngữ lập trình, 
nhưng khả năng sử dụng của chúng khác nhau trong mỗi ngôn ngữ. 
Enum của Rust giống *kiểu dữ liệu đại số* trong ngôn ngữ lập trình chức năng, như là F#, OCaml và Haskell
