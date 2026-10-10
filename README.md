# Antigravity Unlocker — Windows

Windows x64-версия Antigravity Unlocker с исходным кодом и готовым исполняемым файлом.

## Готовый файл

Запустите `Ag_Unlocker_1.1.exe` от имени администратора. 
SHA-256 файла:

```text
1496725024DA7FC55B213022CD9CA3CECF325DED1C6A422F0631981C0EEAE5CB
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





