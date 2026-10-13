# Antigravity Unlocker — Windows

Windows x64-версия Antigravity Unlocker с исходным кодом и готовым исполняемым файлом.

## Готовый файл

Запустите `target\release\ag_unlocker.exe` от имени администратора.
SHA-256 актуальной сборки:

```text
2FD0A321C11F7F7342BD4EF2C121F6A045B257DD96646E5D12E80230DA58DA43
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





