Commit 1 Reflection Notes

Di commit pertama ini, kita membuat sebuah server TCP sederhana yang mendengarkan koneksi pada alamat `127.0.0.1:7878` dan menangani setiap koneksi yang masuk dengan memanggil fungsi `handle_connection`. Kita juga menambahkan beberapa import yang diperlukan untuk bekerja dengan TCP dan I/O. Fungsi 'handle_connection' berfungsi untuk mengambil aliran TCP yang masuk dan memprint tiap line dari aliran tersebut menggunakan 'BufReader' dan berhenti ketika mencapai akhir aliran. 

Commit 2 Reflection Notes

Pada commit kedua ini, kita memperluas fungsi 'handle_connection' untuk mengirimkan respons HTTP sederhana kepada klien yang terhubung. Kita membaca file 'hello.html' dan mengirimkan isinya sebagai bagian dari respons HTTP, termasuk status line dan header 'Content-Length'. Dengan perubahan ini, ketika kita mengakses server melalui browser, kita akan melihat halaman HTML yang berisi pesan "Hello!" dan "Hi from Rust, running from bisma’s machine."

![Commit 2 screen capture](/assets/images/commit2.png)

Commit 3 Reflection Notes

Pada commit ketiga ini, kita menambahkan conditional jika page yang diminta tidak ditemukan, maka server akan mengirimkan respons HTTP dengan status "404 NOT FOUND" dan menampilkan halaman '404.html'. Kita juga menambahkan file '404.html' yang berisi pesan "404: File Not Found". Dengan perubahan ini, jika kita mencoba mengakses halaman yang tidak ada di server, kita akan melihat halaman 404 yang sesuai.

![Commit 3 screen capture](/assets/images/commit3.png)