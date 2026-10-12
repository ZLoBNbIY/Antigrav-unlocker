# Antigravity Unlocker — Windows

Windows x64-версия Antigravity Unlocker с исходным кодом и готовым исполняемым файлом.

## Готовый файл

Запустите `target\release\ag_unlocker.exe` от имени администратора.
SHA-256 актуальной сборки:

```text
B884D93E584934B41595108A000B67157E3E9C10879CDE6448A4E61F1D39A262
```

## Сборка из исходников

Требуется Rust toolchain для `x86_64-pc-windows-msvc` и Visual Studio Build Tools с компонентом MSVC.

```powershell
cargo build --release
```

Результат сборки: `target\release\ag_unlocker.exe`.

Конфигурация `.cargo/config.toml` включает статическую линковку MSVC runtime, поэтому готовому EXE не требуется отдельно установленный Visual C++ Redistributable.

## Важно

- Программа изменяет настройки DNS/NRPT и системные файлы только в рамках выбранных пользователем операций; для этих действий нужны права администратора.
- В репозитории только Windows-исходники и необходимые ресурсы. 





