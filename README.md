# TurkCode IDE

TurkCode, Türk geliştiriciler için özel tasarlanmış, açık kaynaklı, modüler ve yapay zeka destekli entegre geliştirme ortamı (IDE).

## Özellikler

- **Modern Mimari**: Tauri 2.0 ile Rust backend + Web frontend
- **Performans Odaklı**: Düşük RAM kullanımı, küçük uygulama boyutu, hızlı açılış süresi
- **Monaco Editor**: VS Code'un editör motorunu kullanan gelişmiş düzenleme deneyimi
- **LSP Desteği**: Tüm popüler programlama dilleri için dil sunucusu protokol desteği
- **Tree-sitter**: Hızlı ve doğru sözdizimi vurgulama ve kod navigasyonu
- **Yapay Zeka**: Claude 4 API entegrasyonu ile Türkçe doğal dil desteği
- **Git Entegrasyonu**: Kolay versiyon kontrol yönetimi
- **Türkçe Arayüz**: Türkçe dil desteği ile yerel geliştirme deneyimi

## Teknoloji Stack

- **Backend**: Rust 1.79+, Tokio, SQLite, Tree-sitter
- **Frontend**: React 18.3+, TypeScript 5.5+, Tailwind CSS 4.0
- **AI**: Claude 4 Opus API, Türkçe doğal dil işleme
- **VCS**: libgit2 entegrasyonu

## Geliştirme Ortamının Kurulumu

### Önkoşullar

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/installation) (opsiyonel)

### Kurulum

```bash
# Projeyi klonla
git clone https://github.com/turkcode/turkcode-ide.git
cd turkcode-ide

# Bağımlılıkları yükle
npm install

# Geliştirme modunda çalıştır
npm run tauri dev
```

## Derleme

```bash
# Üretim sürümünü oluştur
npm run tauri build
```

## Mimari

TurkCode IDE, Tauri çerçevesi üzerine kurulmuştur ve aşağıdaki ana bileşenlerden oluşur:

1. **Core Layer**: Rust ile yazılmış, dosya sistemi operasyonları, LSP entegrasyonu ve veritabanı yönetimini sağlayan temel katman.
2. **UI Layer**: React ve TypeScript ile yazılmış, Monaco Editor'ü kullanan kullanıcı arayüzü katmanı.
3. **Plugin System**: Extensibility için Tauri plugin mimarisi üzerine kurulu modüler eklenti sistemi.
4. **AI Assistant**: Claude 4 API ile entegre edilmiş, kod tamamlama ve refactoring önerileri sunan AI asistan.

## Katkıda Bulunma

TurkCode IDE açık kaynak bir projedir ve katkılarınızı bekliyoruz! Katkıda bulunmak için lütfen:

1. Bu repoyu fork edin
2. Yeni bir branch oluşturun (`git checkout -b feature/amazing-feature`)
3. Değişikliklerinizi commit edin (`git commit -m 'feat: add amazing feature'`)
4. Branch'inizi push edin (`git push origin feature/amazing-feature`)
5. Pull request açın

## Lisans

MIT

## İletişim

- Website: [turkcode.dev](https://turkcode.dev) (yakında)
- Twitter: [@turkcode_ide](https://twitter.com/turkcode_ide) (yakında)
- Email: info@turkcode.dev (yakında)
