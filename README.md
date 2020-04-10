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
2. `mkdir /tmp/nvim_rpc`
3. Run neovim with `--listen=/tmp/nvim_rpc/some_random_socket_name`
4. Open up neovim in a few places and run some commands with `nvim-ctrl`

---

The lazy (read: my) approach: wrap `nvim` because I dont wanna think about sockets with every invocation
```sh
 #!/bin/sh
mkdir -p /tmp/nvim_rpc/
/usr/bin/nvim --listen $(TMPDIR=/tmp/nvim_rpc mktemp -u) "$@"
```
