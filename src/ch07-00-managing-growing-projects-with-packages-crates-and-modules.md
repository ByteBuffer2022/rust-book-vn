# Quản lý các dự án đang phát triển Packages, Crates, and Modules

Khi bạn viết các chương trình lớn, tổ chức code của bạn sẽ rất quan trọng vì 
theo dõi toàn bộ chương trình của bạn trong đầu sẽ trở nên không thể.
Bằng cách nhóm chức năng liên quan và phân tách code với các tính năng riêng biệt, 
bạn sẽ làm rõ nơi tìm code thực hiện một tính năng cụ thể và nơi để thay đổi cách thức hoạt động của một tính năng.

Các chương trình chúng tôi đã viết cho đến nay đã có trong một mô-đun trong một file.
Là một dự án phát triển, bạn có thể tổ chức code bằng cách chia nó thành nhiều mô-đun và sau đó nhiều file. 
Một package có thể chứa nhiều binary crates và tùy chọn một library crate. 
Như một package phát triển, bạn có thể chia nhỏ các phần vào các crates,những crates này trở thành external dependencies.
Chương này bao gồm tất cả các kỹ thuật trên. Đối với các dự án rất lớn của một tập hợp các package có liên quan đến nhau, 
Cargo cung cấp workspaces,chúng tôi sẽ đề cập trong phần [“Cargo Workspaces”][workspaces]<!-- ignore -->trong Chương 14.
 

Ngoài chức năng nhóm , triển khai tính đóng gói chi tiết
cho phép bạn sử dụng lại code ở mức cao: khi bạn triển khai một hành động, các 
đoạn code khác sẽ gọi code đó thông qua public interface mà không cần biết cách 
triển khai hoạt động.Các mà bạn viết code xác định phần nào public để đoạn code khác sử dụng 
và phần nào private để bạn có thể tuỳ chỉnh khi cần. Đây là một cách khác để hạn chế số lượng 
tổ chức code mà bạn phải nhớ

Một khái niệm liên quan nữa là scope: ngữ cảnh mà code được viết lồng nhau trong đó
một tập hợp các tên được định nghĩa là “in scope.” Khi đọc ,viết và biên dịch code, 
các lập trình viên và trình biên dịch cần biết tại một thời điểm cụ thể đề cập tới
một biến, function,struct,enum, module,constant hoặc các item khác và các item đó có nghĩa là gì.
Bạn có thể tạo ra phạm vi và thay đổi cái tên nào trong hoặc ngoài phạm vi.. Bạn không thể có hai item 
có cùng tên trong cùng phạm vi;tools có sẵn để giải quyết xung đột tên.

Rust có một số tính năng cho phép bạn quản lý tổ chức code của mình, 
bao gồm cả chi tiết nào được công khai, chi tiết nào là private
và tên nào trong mỗi phạm vi trong các chương trình của bạn. 
Những tính năng này, đôi khi được gọi chung là *module system*, bao gồm:

* **Packages:** Một tính năng Cargo cho phép bạn build, test, and chia sẻ crates
* **Crates:** Một cây của modules tạo ta một lib hoặc thực thi
* **Modules** and **use:** để bạn kiểm soát tổ chức (dự án), phạm vi và quyền riêng tư của các đường dẫn
* **Paths:** một cách để đặt tên một item, chẳng hạn như một struct,function hoặc module

Trong chương này ,chúng tôi sẽ đề cập tới tất cả các tính năng trên,bàn luận về cách chúng tương tác 
và giải thích cách sử dụng chúng để quản lí scope. Cuối cùng,bạn nên có một sự hiểu biết vững chắc về 
module system và có thể làm việc với scope như một chuyên gia

[workspaces]: ch14-03-cargo-workspaces.html
