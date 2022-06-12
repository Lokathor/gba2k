# gba-2k

Get MGBA. If you're on a linux that doesn't work with the
pre-built binaries then use *something like* this.

```
sudo apt-get install cmake libelf-dev qtbase5-dev libsdl2-dev
git clone git@github.com:mgba-emu/mgba.git
cd mgba
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX:PATH=/usr ..
make
sudo make install
```

And get Rust ready to do its thing.
You'll need a linker for armv4t, so you'll want the ARM binutils.
You can get them from the ARM website, or maybe from your package manager.

```
rustup component add rust-src
sudo apt-get install binutils-arm-none-eabi
```

First, if you're using VS Code with rust-analyzer, we'll need to turn off the "all targets" check.
Because the `test` crate won't be available, the "all targets" check will never work.
You can turn this off globally, or you can turn it off in `.vscode/settings.json` of a specific folder.
```json
{
  "rust-analyzer.checkOnSave.allTargets": false
}
```

Now we're ready to begin our GBA project.

```toml
# Cargo.toml
[package]
name = "gba-2k"
version = "0.1.0"
edition = "2021"
publish = false
```

```toml
# .cargo/config.toml
[build]
target = "thumbv4t-none-eabi"
[unstable]
build-std = ["core"]
[target.thumbv4t-none-eabi]
runner = "mgba-qt"
rustflags = ["-Clink-arg=-Tgba_link_script.ld"]
```

```toml
# rustfmt.toml, obviously very important
edition = "2021"
fn_args_layout = "Compressed"
max_width = 80
tab_spaces = 2
use_field_init_shorthand = true
use_small_heuristics = "Max"
# Unstable
format_code_in_doc_comments = true
imports_granularity = "Crate"
wrap_comments = true
```

TODO: linker script


Now we can begin the setup of a specific project.