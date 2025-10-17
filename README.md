# Config Manager (Unix)

Works for configs under `$HOME/.config`. 

The application, whose config directory is `$HOME/.config/<app-name>`, will keeps all the possible potential configs under `$HOME/.config/<app-name>/possible-configs`. The user will provide `app-name` and a `config-name`, which will decide which config under `possible-configs` will be used.

Usage:
`config-manager app-name config-name`

e.g. `config-manager waybar myconfig`
