# Сообщение коммита

## Русский

feat: добавлено сохранение веб-страниц со всеми ресурсами (CSS и изображения)

Реализован функционал полного сохранения веб-страниц с встраиванием всех внешних ресурсов:

**Основные изменения:**
- Добавлена функция `embedResources()` для обработки и встраивания CSS и изображений в HTML
- CSS файлы встраиваются как inline стили прямо в HTML (подход SingleFile)
- Изображения конвертируются в base64 data URLs и встраиваются в HTML
- Ресурсы дополнительно сохраняются в структурированные папки (css/, images/) для локального хранения
- Добавлена Rust команда `save_resource` для сохранения ресурсов в подпапки
- Улучшена функция `save_page_local` для поддержки путей с подпапками
- Добавлено детальное логирование процесса обработки ресурсов
- Улучшен поиск CSS файлов и изображений (поддержка различных форматов атрибутов)

**Технические детали:**
- Все ресурсы встраиваются в HTML, делая страницу полностью автономной
- Страницы сохраняются в структуре: `page_TIMESTAMP_hostname/index.html` с подпапками для ресурсов
- Обработка относительных URL в CSS с автоматическим преобразованием в абсолютные
- Поддержка различных форматов изображений (PNG, JPEG, GIF, WebP, SVG, ICO)

**Файлы изменены:**
- `src/components/PageViewer.svelte` - основная логика обработки и встраивания ресурсов
- `src-tauri/src/main.rs` - добавлена команда `save_resource` и улучшена `save_page_local`

---

## English

feat: implement full web page saving with embedded resources (CSS and images)

Implemented functionality for complete web page saving with embedding of all external resources:

**Main changes:**
- Added `embedResources()` function to process and embed CSS and images into HTML
- CSS files are embedded as inline styles directly in HTML (SingleFile approach)
- Images are converted to base64 data URLs and embedded in HTML
- Resources are additionally saved to structured folders (css/, images/) for local storage
- Added Rust command `save_resource` to save resources to subfolders
- Enhanced `save_page_local` function to support paths with subfolders
- Added detailed logging for resource processing
- Improved CSS files and images search (support for various attribute formats)

**Technical details:**
- All resources are embedded in HTML, making the page fully self-contained
- Pages are saved in structure: `page_TIMESTAMP_hostname/index.html` with subfolders for resources
- Processing of relative URLs in CSS with automatic conversion to absolute URLs
- Support for various image formats (PNG, JPEG, GIF, WebP, SVG, ICO)

**Files modified:**
- `src/components/PageViewer.svelte` - main logic for processing and embedding resources
- `src-tauri/src/main.rs` - added `save_resource` command and enhanced `save_page_local`
