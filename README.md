# Tutorial 3: WebChat using yew

## Experiment 3.1: Original Code of WebChat (Yew)
**Input Username**
![Input Username](input_username.png)

**ChatRoom**
![Chat Room](chatroom.png)

Pada eksperimen ini, saya mengkloning dan menguji coba kode *client* WebChat asli yang dibangun dengan ekosistem WebAssembly (WASM) menggunakan bahasa Rust dan framework Yew. Saya menggunakan *source code* dari repositori `YewChat` khusus pada *branch* `websockets-part2`, kemudian menyesuaikan konfigurasi *build tool*-nya agar aplikasi dapat dikompilasi dengan lancar menggunakan *environment* versi terbaru.

Saat aplikasi *client* dijalankan, alurnya adalah sebagai berikut:
1. Pengguna akan diarahkan ke halaman *login* untuk mendaftarkan *username*.
2. Setelah pendaftaran berhasil, *router* aplikasi akan memindahkan tampilan ke ruang obrolan (*chat room*) dan secara otomatis menginisiasi koneksi WebSocket ke *server* lokal di `ws://127.0.0.1:8080`.
3. Di dalam ruang obrolan, setiap pesan yang diketik akan disalurkan melalui **MPSC Channel** ke *server*, yang kemudian mem-*broadcast* pesan tersebut.
4. *Client* menggunakan komponen **Event Bus** untuk menangkap pesan *broadcast* dari *server* dan langsung me-*render* pembaruan UI secara *real-time* tanpa perlu me-*refresh* browser.

Proyek tutorial aslinya dibuat menggunakan versi *library* (seperti `wasm-bindgen`) dan modul *bundler* (`webpack`) yang saat ini sudah cukup usang. Hal ini menyebabkan ketidakcocokan (*incompatibility*) dengan *compiler* Rust modern, sehingga proses *build* ke WASM mengalami kegagalan. Oleh karena itu, saya perlu memperbarui *dependency* tersebut ke versi yang kompatibel agar aplikasi dapat berjalan sebagaimana mestinya tanpa harus mengubah struktur atau logika kode utama dari aplikasi *chat* itu sendiri.

## Experiment 3.2: Add some creativities to the webclient

**Execution Result:**
**Halaman Login: (input username)**
![Halaman Login:](login.png)
**Chat Room:**
![Chat Room](chat.png)

**Question: What creative changes did you implement in the web client?**
Pada eksperimen ini, saya merombak antarmuka pengguna (UI) dari aplikasi WebChat menjadi tema **"Retro 8-bit Arcade"**. Karena aplikasi menggunakan Tailwind CSS melalui Yew, kustomisasi dilakukan secara langsung dengan memodifikasi *utility classes* di dalam macro `html!`.

Rincian kreativitas yang saya tambahkan meliputi:
1. **Redesign Halaman Login (`login.rs`):** Mengubah estetika menjadi gaya *game retro* dengan *background* `indigo-900`. Kotak login didesain menyerupai mesin *arcade* dengan *solid shadow* bergaya 8-bit, font *monospace*, efek teks *gradient*, dan instruksi yang diubah menjadi "INSERT COIN (Enter Player Name)" lengkap dengan animasi `pulse` berkedip.
2. **Kustomisasi Ruang Obrolan (`chat.rs`):** Mengganti bahasa bawaan agar sesuai dengan tema permainan. Judul "Users" diubah menjadi "🏆 Players", status *default* diubah menjadi "Ready Player 1", dan bagian atas obrolan diubah menjadi "🕹️ Multiplayer Chat".
3. **Pembaruan Dynamic Avatar (Pixel Art):** Saya mempertahankan fitur avatar acak (*randomize avatar*) yang dihasilkan berdasarkan *username* pengguna, namun mengubah *endpoint* API ke DiceBear versi 7 dengan *style* `pixel-art`. Hasilnya, setiap *username* yang dimasukkan saat proses *login* akan menghasilkan karakter avatar 8-bit yang unik untuk masing-masing *user*, sangat mendukung gaya visual secara keseluruhan.