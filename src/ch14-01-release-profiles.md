## Tuỳ chỉnh Builds với cấu hình Release

Trong Rust, *cấu hình release* là các cấu hình được xác định trước và có thể tùy chỉnh 
với những config khác nhau cho phép lập trình viên có nhiều quyền kiểm soát hơn đối với 
các tùy chọn khác nhau để biên dịch mã. Mỗi cấu hình được config độc lập với những cấu hình khác.

Cargo có 2 cấu hình chính: cấu hình `dev` Cargo sử dụng khi bạn chạy lệnh `cargo
build` và cấu hình `release` Cargo sử dụng khi bạn chạy lệnh `cargo build
--release`. Cấu hình `dev` được định nghĩa các giá trị mặc định tốt cho quá trình phát triển,
và cấu hình `release` tốt cho các bản phát hành.

Tên các cấu hình này có thể quen thuộc từ output khi bạn build:

<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-->

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

`dev` and `release` được hiển thị trong đầu ra bản dựng này chỉ ra rằng trình 
biên dịch đang sử dụng các cấu hình khác nhau.

Cargo có cài đặt mặc định cho từng cấu hình nó áp dụng khi không có bất kì phần `[profile.*]` 
trong file *Cargo.toml* của dự án. Bằng cách thêm phần `[profile.*]`
cho bất kỳ cấu hình nào bạn muốn tùy chỉnh, bạn có thể ghi đè bất kỳ tập hợp con nào của cấu hình mặc định. 
Ví dụ, đây là cấu giá trị mặc định cài đặt `opt-level` cho cấu hình `dev` và `release`:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Cài đặt `opt-level` kiểm soát số lượng tối ưu hóa Rust sẽ áp dụng cho mã của bạn,
với phạm vi từ 0 đến 3. Áp dụng nhiều tối ưu hóa hơn sẽ kéo dài thời gian biên dịch,
vì vậy nếu bạn đang trong quá trình phát triển và biên dịch mã của mình thường xuyên,
bạn sẽ muốn biên dịch nhanh hơn ngay cả khi mã kết quả chạy chậm hơn. Đó là lý do giá trị mặc định 
`opt-level` cho `dev` là `0`. Khi bạn chuẩn bị phát hành code của mình,
tốt nhất là dành nhiều thời gian hơn để biên dịch. Bạn chỉ phải biên dịch code 1 lần, 
nhưng bạn sẽ chạy code đã compile nhiều làn, vì vậy chế độ phát hành đánh đổi nhiều thời gian 
biên dịch hơn để cho đoạn code chạy nhanh hơn.
Đó là lý do giá trị mặc định `opt-level` cho cấu hình `release` là `3`.

Bạn có thể ghi đè bất kì cài đặt mặc định nào bằng cách thêm một giá trị khác 
vào trong file *Cargo.toml*. Ví dụ, nếu chúng ta muốn sử dụng tối ưu level 1 
trong cấu hình phát triển, chúng ta có thể thêm 2 dòng vào trong file *Cargo.toml*
của dự án:

<span class="filename">Filename: Cargo.toml</span>

```toml
[profile.dev]
opt-level = 1
```

Đoạn code này ghi đè giá trị mặc định là `0`. Bây giờ khi chúng ta run `cargo build`,
Cargo sẽ sử dụng cấu hình mặc định `dev` thêm tuỳ chỉnh của chúng ta với `opt-level`. 
Bởi vì chúng ta cài đặt `opt-level` là `1`, Cargo sẽ chấp nhận tối ưu hơn mặc định,
nhưng không nhiều như trong một bản phát hành.

Để có danh sách đầy đủ các tùy chọn cấu hình và mặc định cho từng cấu hình, xem
[Cargo’s documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).
