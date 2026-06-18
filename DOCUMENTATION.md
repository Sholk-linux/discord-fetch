# Configuration Documentation / Документация по конфигурации

---

## [ENG] English Documentation

This document describes all configuration options available in `discord-fetch`.

###  File Locations
The configuration file is named `config.toml`. It is automatically generated on the first run at the following paths:
- **Linux / Unix**: `~/.config/discord-fetch/config.toml`
- **Windows**: `C:\Users\<Username>\AppData\Roaming\discord-fetch\config.toml`

###  Parameter Reference

#### 1. `app_id`
- **Type**: `Integer`
- **Required**: Yes
- **Description**: Your Discord Developer Application Client ID.

#### 2. `large_image`
- **Type**: `Optional`
- **Default**: `"logo"`
- **Description**: The asset key of the image uploaded to **Rich Presence -> Art Assets** in your Discord Developer Portal.

#### 3. `large_text`
- **Type**: `Optional`
- **Default**: `"OS logo"`
- **Description**: The tooltip text that appears when a user hovers their mouse over the large asset icon.

#### 4. `separator`
- **Type**: `Optional`
- **Default**: `"•"`
- **Description**: The character or string used to separate system values in the main status line (e.g., `•`, `-`, `&`, `|`).

###  Manual Overrides

- `platform_override`: Overrides the platform (displayed in `Details` field, e.g `Linux` or `Windows`).
- `os_override`: Overrides the operating system/distribution name (e.g., "Arch Linux", "Fedora", "Windows 11").
- `desktop_override`: Overrides the Desktop Environment or Window Manager name (e.g., "Niri", "GNOME", "KDE", "i3").
- `shell_override`: Overrides the active shell name (e.g., "zsh", "fish", "bash", "powershell").


---

## [RU] Документация на русском

Этот документ описывает все параметры конфигурации, доступные в `discord-fetch`.

###  Расположение файла
Файл конфигурации называется `config.toml`. Он автоматически создается при первом запуске по следующим путям:
- **Linux / Unix**: `~/.config/discord-fetch/config.toml`
- **Windows**: `C:\Users\<Имя_Пользователя>\AppData\Roaming\discord-fetch\config.toml`

###  Описание параметров

#### 1. `app_id`
- **Тип**: `Целое число`
- **Обязательный**: Да
- **Описание**: ID вашего приложения на Discord Developer Portal.

#### 2. `large_image`
- **Тип**: `Необязательный`
- **По умолчанию**: `"logo"`
- **Описание**: Ключ ассета картинки, загруженной в раздел **Rich Presence -> Art Assets** на Discord Developer Portal.

#### 3. `large_text`
- **Тип**: `Необязательный`
- **По умолчанию**: `"OS logo"`
- **Описание**: Текст всплывающей подсказки, которая появляется при наведении курсора мыши на большую иконку.

#### 4. `separator`
- **Тип**: `Необязательный`
- **По умолчанию**: `"•"`
- **Описание**: Символ или строка, используемая для разделения системных значений в основной строке статуса (например, `•`, `-`, `&`, `|`).

###  Ручные оверрайды (Ручное переопределение)

- `platform_override`: Переопределяет платформу (отображается в поле `Details`, например `Linux` или `Windows`).
- `os_override`: Переопределяет имя операционной системы/дистрибутива (например, "Arch Linux", "Fedora", "Windows 11").
- `desktop_override`: Переопределяет имя графического окружения или оконного менеджера (например, "Niri", "GNOME", "KDE", "i3").
- `shell_override`: Переопределяет имя активного шелла (например, "zsh", "fish", "bash", "powershell").
