Commit 1 Reflection Notes

Di commit pertama ini, kita membuat sebuah server TCP sederhana yang mendengarkan koneksi pada alamat 127.0.0.1:7878 dan menangani setiap koneksi yang masuk dengan memanggil fungsi handle_connection. Kita juga menambahkan beberapa import yang diperlukan untuk bekerja dengan TCP dan I/O secara efektif di dalam ekosistem Rust. Fungsi handle_connection bertugas untuk mengambil aliran TCP yang masuk dan mencetak setiap baris dari aliran tersebut ke terminal menggunakan BufReader. Proses pembacaan ini dilakukan secara berulang dan hanya akan berhenti ketika mencapai akhir aliran data dari klien. Untuk tahap awal ini, aplikasi masih menangani koneksi secara serial di mana setiap permintaan diproses satu per satu secara berurutan. Dengan perubahan ini, kita dapat menjalankan server dan memverifikasi bahwa data dari browser berhasil diterima dan diproses melalui log output di terminal.

Commit 2 Reflection Notes

Pada commit kedua ini, kita memperluas fungsi handle_connection untuk mengirimkan respons HTTP sederhana kepada klien yang terhubung kembali. Kita mulai membaca file fisik hello.html dari direktori lokal dan menyiapkan isinya untuk dikirim sebagai badan respons. Respons yang disusun kini mencakup komponen standar HTTP seperti status line dan header Content-Length agar browser dapat memahami format data yang diterima. File hello.html itu sendiri berisi pesan sambutan "Hello!" dan keterangan khusus "Hi from Rust, running from bisma’s machine." Dengan implementasi perubahan ini, ketika kita mengakses alamat server melalui browser, kita tidak lagi hanya melihat layar kosong melainkan sebuah halaman HTML yang terformat dengan baik.

![Commit 2 screen capture](/assets/images/commit2.png)

Commit 3 Reflection Notes

Pada commit ketiga ini, kita menambahkan logika pengkondisian untuk menangani situasi di mana halaman yang diminta oleh klien tidak ditemukan di server. Jika permintaan tidak sesuai dengan rute yang tersedia, server akan secara otomatis mengirimkan respons HTTP dengan status "404 NOT FOUND". Kita juga telah menyediakan file 404.html khusus yang berisi pesan peringatan "404: File Not Found" untuk memberikan informasi yang jelas kepada pengguna. Penambahan ini sangat penting untuk meningkatkan pengalaman pengguna agar mereka tidak bingung saat mengakses URL yang salah. Melalui perubahan ini, struktur server menjadi lebih lengkap karena sudah mampu membedakan antara konten yang tersedia dan konten yang tidak ada.

![Commit 3 screen capture](/assets/images/commit3.png)

Commit 4 Reflection Notes

Pada commit keempat ini, kita menambahkan simulasi delay pada server dengan menggunakan fungsi thread::sleep untuk menunda pengiriman respons selama 10 detik. Simulasi ini sengaja dipicu ketika pengguna mencoba mengakses rute khusus pada alamat 127.0.0.1/sleep. Dengan adanya perubahan ini, kita dapat mengamati secara langsung bagaimana server yang bersifat serial akan mengalami hambatan dalam merespons permintaan lainnya. Kondisi ini mensimulasikan situasi dunia nyata di mana server mungkin sedang melakukan perhitungan berat atau mengalami beban trafik yang tinggi. Hal ini menjadi dasar alasan yang kuat bagi kita untuk segera beralih ke metode penanganan koneksi yang lebih efisien di tahap berikutnya.

Commit 5 Reflection Notes

Pada commit kelima ini, kita menambahkan fitur utama untuk menangani multiple connection secara bersamaan menggunakan mekanisme ThreadPool. Kita membangun struktur ThreadPool yang bertugas mengelola sejumlah thread worker yang selalu siap siaga untuk mengambil dan menangani tugas yang masuk. Untuk memastikan keamanan data, kita menggunakan mekanisme channel yang dipadukan dengan Arc dan Mutex sebagai fondasi utama penanganan concurrency di Rust. Setiap permintaan yang masuk ke fungsi execute akan dikirimkan melalui saluran tersebut dan ditangkap oleh Worker pertama yang tersedia untuk segera diproses. Dalam implementasi kali ini, kita menetapkan 4 worker permanen yang memungkinkan server melayani banyak klien secara paralel tanpa saling menunggu satu sama lain.

Commit Bonus Reflection Notes

Pada commit terakhir ini, kita melakukan optimasi besar dengan mengganti setiap bagian kode yang berisiko menimbulkan panic menggunakan tipe Result. Langkah ini diambil untuk memastikan bahwa server tidak akan langsung berhenti beroperasi ketika menghadapi kesalahan yang tidak terduga di tengah jalan. Kita menambahkan penanganan error yang lebih spesifik, seperti saat file HTML yang diminta hilang atau terjadi kegagalan mendadak pada aliran I/O. Selain itu, fungsi new yang sebelumnya berisiko kini digantikan dengan fungsi build yang mengembalikan Result agar lebih aman secara logika pemrograman. Dengan seluruh perubahan ini, server kita menjadi jauh lebih tangguh, profesional, dan siap menghadapi berbagai situasi error tanpa mengakibatkan crash sistem.