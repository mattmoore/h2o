# h2o

Game management utility for installing h2o games.

## Installation

### Arch Linux

`h2o` is available as the `h2ogames` package on the [AUR](https://aur.archlinux.org/packages/h2ogames).

You can install via `yay`:

```shell
yay -S h2ogames
```

To uninstall:

```shell
yay -R h2ogames
```

### Windows & macOS

Support coming soon.

## Usage

`h2o` is the CLI program. It comes with bash and zsh shell completion. Here are some examples:

### List games

```shell
h2o list
```

### Install game

```shell
h2o install fantasy
```

### Uninstall game

```shell
h2o uninstall fantasy
```

### Clear download cache

This won't uninstall any games, but clears the download cache which could get quite large over time.

```shell
h2o clear-downloads
```
