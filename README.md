# Antigravity Unlocker — Windows

Windows x64-версия Antigravity Unlocker с исходным кодом и готовым исполняемым файлом.

## Готовый файл

Запустите `Ag_Unlocker_1.5.exe` (или `target\release\ag_unlocker.exe`) от имени администратора.
SHA-256 актуальной сборки:

```text
B170F7989F6704E038A9DD1545CB45B958A3EC8D7AA6C6DAB866D31609D490C0
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





