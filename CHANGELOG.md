# 📝 Changelog

Semua pembaruan penting pada repositori **Pakem Starter Skeleton** (`pakem-rs/pakem`) didokumentasikan di dalam berkas ini.

Format pencatatan mengacu pada [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) dan mengikuti standar [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.5] - 2026-11-27

### 🎨 Rebranding Welcome & Upgrade Ekosistem 1.0.5
- **Upgrade Core Framework**: Menggunakan `pakem = "1.0.5"` berbasis **Rust 2024 Edition** dan **Axum 0.8** stack.
- **Logo & Identitas Visual Pekalongan**:
  - Memasang logo SVG resmi **Pakem Canting** pada navbar [layouts/app.blade.rs](resources/views/layouts/app.blade.rs).
  - Memperbarui hero copywriting pada [welcome.blade.rs](resources/views/welcome.blade.rs) dengan tema ketelitian canting dan batik Pekalongan (*"Crafted with Canting Precision. Driven by the Raw Speed of Rust."*).
  - Menyelaraskan tampilan versi `Pakem 1.0.5`, `Rust 2024 Edition`, dan `Axum 0.8` pada dashboard welcome serta controller.

---

## [1.0.1] - 2026-11-22

### 🎨 Rebranding Bersejarah: Pakem Starter Application Skeleton
- **Transformasi Identitas Resmi (`pakem-rs/pakem`)**:
  - Transisi penuh dari nama rintisan *Lumina* menjadi **Pakem Framework Application Skeleton**.
  - Sinkronisasi dengan rilis crate resmi `pakem v1.0.1` di crates.io.
  - Dokumentasi resmi terhubung ke domain: **[https://pakem.my.id](https://pakem.my.id)**.
  - Repositori resmi: **[pakem-rs/pakem](https://github.com/pakem-rs/pakem)**.
- **Pembaruan Konfigurasi & Tooling**:
  - Pemasangan logo resmi Pakem (SVG & PNG) di `resources/images/`, `public/`, dan `storage/app/public/assets/`.
  - Pembaruan konfigurasi container: `Dockerfile` multi-stage build ber-cache dan `docker-compose.yml` (`pakem-app`, `pakem-net`).
  - Pembaruan konfigurasi production process manager: `supervisor/pakem.conf` (`pakem-server` dan `canting queue:work`).
  - Pembaruan template `.env.example` dengan prefix dan secret default Pakem.
  - History REPL Tinker dialihkan ke `.pakem_tinker_history`.
- **Dual-Mode Welcome Controller & Routing**:
  - `WelcomeController`: Mendukung respons HTML Blade Dashboard untuk peramban web dan JSON respons untuk panggilan API/cURL.
  - Routing terisolasi: `src/routes/web.rs` untuk rute web dan `src/routes/api.rs` untuk RESTful API endpoints.
- **Dukungan Penuh Canting CLI**:
  - Proyek ini adalah target resmi perintah `canting new <nama_proyek>`.
  - Siap digunakan untuk `canting tinker`, `canting make:controller`, `canting make:model`, dan `canting migrate`.

---

## [0.4.0] - 2026-05-18

### ⚡ Starter Skeleton Initial Architecture
- Inisialisasi struktur MVC ergonomis ala Laravel: `app/controllers`, `app/models`, `routes`, `resources/views`.
- Integrasi Tokio async runtime, Axum engine, dan SQLx database pooling (SQLite/PostgreSQL/MySQL).
- Integrasi Blade templating engine dan Tailwind CSS responsive UI.
