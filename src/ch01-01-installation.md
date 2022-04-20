## Cài đặt

Đầu tiên chúng ta phải cài Rust. Chúng ta sẽ cài Rust thông qua `rustup` - Command line tool 
quản lý phiên bản Rust và các tool liên quan.

> Note: Nếu bạn không muốn dùng `rustup`, hãy truy cập
> [Other Rust Installation Methods page][otherinstall] để có xem các lựa chọn khác.

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html

Các bước sau cài phiên bản ổn định mới nhất của Rust compiler. Phiên bản ổn định của Rust đảm bảo các ví
dụ trong sách có thể biên dịch với phiên bản Rust mới hơn. Tuy nhiên ouput có thể khác 1 chút giữa
các phiên bản, bởi vì Rust thường xuyên cải thiện các message lỗi và cảnh báo.

### Cài đặt `rustup` trên Linux và macOS

Nếu bạn đang sử dụng Linux hay macOs, mở terminal và nhập lệnh sau

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Câu lệnh trên sẽ tải 1 đoạn script và bắt đầu cài đặt `rustup`, nó cài đặt phiên bản ổn định và mới 
nhất của Rust. Nếu cài đặt thành công, terminal sẽ hiển thị:

```text
Rust is installed now. Great!
```

Bạn cũng sẽ cần một linker - một chương trình Rust dùng để ghi complied outputs vào 1 file.
Nếu bạn gặp linker errors, bạn nên cài C compiler (đã bao gồm linker). Bên cạnh đó, một số
Rust packages được viết bằng ngôn ngữ C và cần C compiler để biên dịch

Trên macOS, bạn có thể cài C compiler bằng lệnh sau:

```console
$ xcode-select --install
```

Người dùng linux nên cài GCC hoặc Clang

### Cài đặt `rustup` trên Windows

Trên Windows, truy cập [https://www.rust-lang.org/tools/install][install] và làm theo 
hướng dẫn để cài Rust. Bạn cũng sẽ cần C++ build tools cho Visual Studio 2013 hoặc mới hơn. 
Cách dễ nhất là cài [Build Tools for Visual Studio 2019][visualstudio]. Hãy đảm bảm “C++ build tools”
được chọn.

[install]: https://www.rust-lang.org/tools/install
[visualstudio]: https://visualstudio.microsoft.com/visual-cpp-build-tools/

The rest of this book uses commands that work in both *cmd.exe* and PowerShell.
If there are specific differences, we’ll explain which to use.

### Cập nhật và gỡ cài đặt

Sau khi cài Rust bằng `rustup`, bạn có thể cập nhật phiên bản mới nhất bằng lệnh:

```console
$ rustup update
```

Để gỡ cài đặt Rust và `rustup`, chạy dòng lệnh:

```console
$ rustup self uninstall
```

### Xử lý sự cố

Để kiểm tra bạn đã cài đặt Rust đúng chưa:

```console
$ rustc --version
```

Bạn sẽ thấy phiên bản, commit hash, và ngày commit cho phiên bản bạn đã cài đặt

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Nếu bạn thấy thông tin trên, bạn đã cài đặt Rust thành công! Nếu không thấy trên Window, 
hãy kiểm tra Rust đã được thêm trong `%PATH%` ở system variable hay chưa.
Nếu gặp lỗi hãy tham gia nhóm facebook [Cộng đồng Rust Việt Nam][facebook]. Mọi người sẽ nhiệt tình giúp đỡ

[facebook]: https://www.facebook.com/groups/546307380433651


