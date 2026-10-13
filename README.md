# Antigravity Unlocker — Windows

Windows x64-версия Antigravity Unlocker с исходным кодом и готовым исполняемым файлом.

## Готовый файл

Запустите `Ag_Unlocker_1.4.exe` (или `target\release\ag_unlocker.exe`) от имени администратора.
SHA-256 актуальной сборки:

```text
CE888233D9221CD31295A3E61A3204D197761CC071FD2972176A871C03E4ADDE
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





