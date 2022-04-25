# Understanding Ownership

Ownership (Quyền sở hữu) là tính năng độc đáo nhất của Rust và có ý nghĩa sâu sắc đối với phần còn lại của ngôn ngữ. Nó cho phép Rust đảm bảo an toàn cho bộ nhớ mà không cần bộ thu gom rác (garbage collector), vì vậy điều quan trọng là phải hiểu cách thức hoạt động của Ownership. Chúng ta sẽ nói về ownership cũng như một số tính năng liên quan: borrowing, slices, và cách Rust đưa dữ liệu ra ngoài trong bộ nhớ.
