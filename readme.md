# Dot

Manage your dotfiles with ease.

## Setup

1. Create a directory for your dotfiles, e.g. `~/Documents/dotfiles`.
2. Create a `dot.toml` file in your dotfiles directory to configure Dot.
3. Save all your configuration files with `dot save --all`.
4. You can now apply your configuration files back to your home directory using `dot apply --all`

## Installation

You need to have Cargo installed to install Dot. Install Cargo via `rustup`.

```sh
git clone https://github.com/octowaddle/dot && cd dot && cargo install --path .
```

## Configuration

This is an example `dot.toml` configuration:

```toml
[[set]]
name = "bash"
group = "shells"
files = [
  ".bashrc"
]

[[set]]
name = "fish"
group = "shells"
files = [
  ".config/fish/config.fish"
]

[[set]]
name = "bspwm"
group = "windowmanagers"
files = [
  ".config/bspwm/bspwmrc",
  ".config/sxhkd/sxhkdrc"
]
```

## Filtering

You can filter the configuration files you want to apply like so:

```sh
dot apply --group shells --group browsers --name bspwm --name helix
```

The same settings also work with `dot save`.
