# Config Manager (Unix)

Works for configs under `$HOME/.config`. 

The application, whose config directory is `$HOME/.config/<app>`, will keeps all the possible potential configs under `$HOME/.config/<app>/possible-configs`.

## Usage:
`cargo run <subcommand> <app> <config>`

e.g. `cargo run -- apply waybar myconfig`

Alternatively, you can use the executable at `target/release/config-manager`.

## Subcommands
### apply
The user will provide `app` and a `config`, which will decide which config under `possible-configs` will be used.

### add
The user will provide `app` and a `config`, which will create a new directory at `$HOME/.config/<app>/possible-configs/<config>`.

### del
The user will provide `app` and a `config`, which will remove the directory at `$HOME/.config/<app>/possible-configs/<config>` if it exists.

