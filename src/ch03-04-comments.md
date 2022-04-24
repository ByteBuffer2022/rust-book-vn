## Comments

Tất cả lập trình viên đều cố gắng làm cho code dễ hiểu, nhưng đôi khi cần
những lời giải thích thêm. Trong các trường hợp này, lập trình viên sẽ để
lại *comments* trong code, trình biên dịch sẽ bỏ qua các comment này nhưng
chúng lại có thể có ích cho những ai đọc code.

Đây là một comment đơn giản:

```rust
// hello, world
```

Trong Rust, comment bắt đầu với hai dấu gạch chéo và kéo dài cho đến cuối
dòng. Đối với các comment vượt quá một dòng, bạn sẽ cần viết thêm `//` ở
mỗi dòng như thế này:

```rust
// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.
```

Comments cũng có thể được đặt ở cuối dòng chứa mã code:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Nhưng bạn sẽ thường xuyên thấy chúng ở định dạng comment ở trên một dòng
riêng biệt với code cần chú thích:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust cũng có một loại comment khác, documentation comments, chúng ta sẽ thảo luận
về nó trong phần “Publishing a Crate to Crates.io” ở Chương 14.
