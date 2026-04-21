Commit 1 Reflection Notes

Di commit pertama ini, kita membuat sebuah server TCP sederhana yang mendengarkan koneksi pada alamat `127.0.0.1:7878` dan menangani setiap koneksi yang masuk dengan memanggil fungsi `handle_connection`. Kita juga menambahkan beberapa import yang diperlukan untuk bekerja dengan TCP dan I/O. Fungsi 'handle_connection' berfungsi untuk mengambil aliran TCP yang masuk dan memprint tiap line dari aliran tersebut menggunakan 'BufReader' dan berhenti ketika mencapai akhir aliran. 

Commit 2 Reflection Notes

Pada commit kedua ini, kita memperluas fungsi 'handle_connection' untuk mengirimkan respons HTTP sederhana kepada klien yang terhubung. Kita membaca file 'hello.html' dan mengirimkan isinya sebagai bagian dari respons HTTP, termasuk status line dan header 'Content-Length'. Dengan perubahan ini, ketika kita mengakses server melalui browser, kita akan melihat halaman HTML yang berisi pesan "Hello!" dan "Hi from Rust, running from bisma’s machine."

![Commit 2 screen capture](/assets/images/commit2.png)

Commit 3 Reflection Notes

Pada commit ketiga ini, kita menambahkan conditional jika page yang diminta tidak ditemukan, maka server akan mengirimkan respons HTTP dengan status "404 NOT FOUND" dan menampilkan halaman '404.html'. Kita juga menambahkan file '404.html' yang berisi pesan "404: File Not Found". Dengan perubahan ini, jika kita mencoba mengakses halaman yang tidak ada di server, kita akan melihat halaman 404 yang sesuai.

![Commit 3 screen capture](/assets/images/commit3.png)

Commit 4 Reflection Notes

Pada commit keempat ini, kita menambahkan simulasi delay pada server dengan menggunakan fungsi 'thread::sleep' untuk menunda respons selama 10 detik, ketika mengakses 127.0.0.1/sleep. Dengan perubahan ini, ketika kita mengakses URL tersebut, kita akan melihat bahwa server membutuhkan waktu 10 detik untuk merespons, yang mensimulasikan kondisi di mana server sedang sibuk atau mengalami beban tinggi.

Commit 5 Reflection Notes

Pada commit kelima ini, kita menambahkan fitur untuk menangani multiple connection secara bersamaan menggunakan threadpool. Kita membuat struct 'ThreadPool' yang mengelola sejumlah thread worker yang siap untuk menangani tugas. Kita juga menambahkan mekanisme untuk mengirimkan tugas ke worker melalui channel menggunakan Arc dan Mutex sebagai penanganan concurrency sehingga ketika request pertama kali dikirimkan, response yang sesuai dengan request tersebut masuk ke fungsi execute di ThreadPool yang kemudian disend ke Worker yang tersedia untuk kemudian ditangani. Di aplikasi ini dibuat 4 worker yang siap untuk menangani tugas secara bersamaan. Dengan perubahan ini, server kita sekarang dapat menangani beberapa koneksim tanpa harus menunggu satu koneksi selesai sebelum menangani koneksi berikutnya. Ini meningkatkan performa server dan memungkinkan untuk melayani lebih banyak klien secara efisien.

Commit Bonus Reflection Notes

Pada commit terakhir ini, kita mengganti setiap kemungkinan yang dapat menimbulkan panic dengan menggunakan Result dan error handling yang sesuai. Kita juga menambahkan beberapa error handling untuk menangani kasus di mana file yang diminta tidak ditemukan atau terjadi kesalahan saat membaca file. Dengan perubahan ini, server kita menjadi lebih robust dan dapat menangani berbagai situasi error dengan lebih baik, memberikan pengalaman pengguna yang lebih baik dan mencegah crash pada server.