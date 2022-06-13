# gba2k

A GBA development project.

## System Setup

There's a few steps of system setup that I did which need to be mentioned before
you'll be able to build/use this respository.

### Get mGBA

You should download a copy of the [mgba](https://mgba.io/downloads.html)
emulator. Downloads are provided for the common platforms.

If you're the foolish kind of person like me that does GBA development in a shed
in their backyard on a raspberry pi then you'll need to build mGBA from source
yourself:

```sh
sudo apt-get install cmake libelf-dev qtbase5-dev libsdl2-dev
git clone git@github.com:mgba-emu/mgba.git
cd mgba
mkdir build
cd build
cmake -DCMAKE_INSTALL_PREFIX:PATH=/usr ..
make
sudo make install
```

There's a few GBA emulators besides just mGBA, but I happen to know that mGBA
lets you load and run ELF files directly, which makes just a little nicer for
us. It also has some handy debugging features that will help during development.

### Get your compilation tools

We'll need to use Nightly for our project because we'll have to use `build-std`.
It should be included in a default installation, but we'll need the `rust-stc`
component so that the build of the standard library can happen.

```sh
rustup component add rust-src
```

Also, we'll need the ARM Binutils because the linker that comes with rust
doesn't work as far back as we need. You can download these from the [Official
ARM Developer Site][arm-dev], or you can maybe get them from your package
manager. For debian-like systems the correct package should be under
`binutils-arm-none-eabi`, Arch seems to call it `arm-linux-gnueabi-binutils`.

[arm-dev]: https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads

Lastly, a normal compilation will have `rustc` produce a ELF file. mGBA can load
and run an ELF file, but if you want an actual ROM that could run on hardware
there's a few extra steps, one of which is using a tool called `gbafix`. There's
a C version that you can get with the Dev Kit Pro ARM distribution, but there's
also a Rust version you can just build/install it through `cargo`.

```sh
cargo install gbafix
```
