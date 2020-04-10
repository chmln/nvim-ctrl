# nvim-ctrl

I wanted an easy way toggle `background=dark` from a night mode script but figured this is nice and flexible enough for any other use as well.

Examples:

```sh
# Toggle dark mode (set background=dark)
nvim-ctrl 'set background=dark'

# Toggle night mode based on environment variable
nvim-ctrl 'let $NIGHT_MODE="true"'
nvim-ctrl 'source ~/.dotfiles/nvim/init.vim`
```

## Install

1. Download a binary from [releases](https://github.com/chmln/nvim-ctrl/releases)
OR
`cargo install --git https://github.com/chmln/nvim-ctrl`
2. Open up neovim in a few places and run some commands with `nvim-ctrl`

## How It Works

Neovim [automatically](https://github.com/neovim/neovim/blob/master/runtime/doc/api.txt) sets up RPC sockets in /tmp on unix systems, unless its overridden by `--listen`. `nvim-ctrl` finds these sockets and sends them the appropriate commands.
