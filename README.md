# mgmt

A tool for managing DE deployments.

## Development

### Install Rust and Cargo on MacOS with zsh

Instructions originally found at https://stackoverflow.com/a/68617314.

Run the following in a terminal.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

When prompted, customize the installation. When asked about modifying the PATH,
select `No`. All of the other questions can be set to their default setting.

Next, open your `.zshrc` file (typically located at `~/.zshrc`) and add:

```
export PATH PATH="$HOME/.cargo/bin:$PATH"
```

Finally, source your `.zshrc` file:\

```
. ~/.zshrc
```

Run `rustc --version` to make sure the rust compiler is installed.

Run `cargo --version` to make sure that cargo is installed.
