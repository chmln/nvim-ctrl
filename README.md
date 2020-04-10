# nvim-ctrl

I wanted an easy way toggle `background=dark` from a night mode script but figured this is nice and flexible enough for any other use as well.

Examples:

```sh
# Toggle dark mode (set background=dark)
nvim-ctrl set background dark

# Toggle night mode based on environment variable
nvim-ctrl run 'let $NIGHT_MODE="true"'
nvim-ctrl run 'source ~/.dotfiles/nvim/init.vim`
```

## Setup

1. Download a binary from [releases](https://github.com/chmln/nvim-ctrl/releases)
OR
`cargo install --git https://github.com/chmln/nvim-ctrl`
2. Put this in `~/.local/bin/nvim`:
```sh
 #!/bin/sh
mkdir -p /tmp/nvim_rpc/
tmp_path=$(env TMPDIR=/tmp/nvim_rpc mktemp -u)
/usr/bin/nvim --listen "$tmp_path" "$@"
```
3. `chmod +x ~/.local/bin/nvim`
4. Open up neovim in a few places and run some commands with `nvim-ctrl`
