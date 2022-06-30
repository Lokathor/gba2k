# gba2k

A GBA development project.

## FAQ

* **How does this relate to the `gba` crate?**

Starting with the hard ones I see. Uh, so the story there is that I guess I've
given up on trying develop the `gba` crate further. It's at a point where it has
a non-zero number of users other than myself use it, and I don't want to mess up
what those people are doing, but I also think that the entire organization of
the crate needs a major overhaul.

* **How do I do \[thing\] with this crate?**

You probably can't yet! Currently the crate is very new, and quite limited.
Rather than just adding a million things at once, I'm trying this time to add to
the crate more slowly and carefully. Also, I usually only work on this when I'm
avoiding my other project that I want to get done in 2022. So expect development
to move quite slowly.

## System Setup

There's a few steps of system setup that I did which need to be mentioned before
you'll be able to build/use this repository.

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

## License

The majority of the crate, including all Rust code, is licensed under `Zlib OR
Apache-2.0 OR MIT`. This is compatible with the usual `Apache-2.0 OR MIT`
license that people expect with Rust projects.

Some of the provided assembly code is adapted from the
[agbabi](https://github.com/felixjones/agbabi) project, which *only* uses the
`Zlib` license (not MIT or Apache-2.0). All of the Zlib-only code is kept behind
cargo feature flags, so it can be disabled if that's somehow a problem for you.
Since the Zlib license is even *more* permissive than the MIT license it's
assumed that this isn't a problem, and so these features are on by default.
