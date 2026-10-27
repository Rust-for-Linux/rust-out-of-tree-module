# Rust out-of-tree module

This is a basic template for an out-of-tree Linux kernel module written in Rust.

Please note that:

  - The Rust support is experimental.

  - The kernel that the module is built against needs to be Rust-enabled (`CONFIG_RUST=y`).

  - The kernel tree (`KDIR`) requires the Rust metadata to be available. These are generated during the kernel build, but may not be available for installed/distributed kernels (the scripts that install/distribute kernel headers etc. for the different package systems and Linux distributions are not updated to take into account Rust support yet).

  - All Rust symbols are `EXPORT_SYMBOL_GPL`.

Example:

```sh
$ make KDIR=.../linux-with-rust-support LLVM=1
make -C .../linux-with-rust-support M=$PWD
make[1]: Entering directory '.../linux-with-rust-support'
  RUSTC [M] .../rust-out-of-tree-module/rust_out_of_tree.o
  MODPOST .../rust-out-of-tree-module/Module.symvers
  CC [M]  .../rust-out-of-tree-module/rust_out_of_tree.mod.o
  LD [M]  .../rust-out-of-tree-module/rust_out_of_tree.ko
make[1]: Leaving directory '.../linux-with-rust-support'
```

```txt
[    1.076945] rust_out_of_tree: Rust out-of-tree sample (init)
[    1.084944] rust_out_of_tree: My numbers are [72, 108, 200]
[    1.085944] rust_out_of_tree: Rust out-of-tree sample (exit)
```

For details about the Rust support, see https://rust-for-linux.com.

For details about getting started with kernel development in Rust, see https://docs.kernel.org/rust/.

For details about out-of-tree modules, see https://docs.kernel.org/kbuild/modules.html.

## rust-analyzer

Rust for Linux (with https://lore.kernel.org/rust-for-linux/20230121052507.885734-1-varmavinaym@gmail.com/ applied) supports building a `rust-project.json` configuration for [`rust-analyzer`](https://rust-analyzer.github.io/), including for out-of-tree modules:

```sh
make -C .../linux-with-rust-support M=$PWD rust-analyzer
```

## Exact steps for Ubuntu 26.04, 26.10

The quickest and easiest first step is to build this out of tree module for the distribution kernel
you are already running. To do this, you will need to build this module using the Rust packages
from Ubuntu, because they are what Ubuntu's Rust-for-Linux support is built against.

From a fresh Ubuntu 26.04 or 26.10 install, you will need to install the following packages,
including the correct `linux-lib-rust-VERSION` package:

```sh
sudo apt install build-essential rust-all pahole linux-lib-rust-`uname -r`
```

Which will install something like `linux-lib-rust-7.0.0-38-generic` or
`linux-lib-rust-7.3.0-9-generic` for the `linux-lib-rust-VERSION` package.

And compile and install the kernel module like this:

```sh
make
sudo insmod rust_out_of_tree.ko
```

Unload the module like this:

```sh
sudo rmmod rust_out_of_tree
```

If you already have a Rust install via rustup, you will get an error if you try to compile this
module because the Rust version does not match what the kernel was built with.  In this case, you
can tell rustup to use the system install like this:

```sh
rustup toolchain link system /usr
rustup default system
```

You can watch the output from the `pr_info!()` macro like this:

```sh
sudo dmesg -w
```

Once you're in the groove, it's handy to have `dmesg -w` running in one terminal and then run this
each time you need to test a change:

```sh
make && sudo rmmod rust_out_of_tree && sudo insmod rust_out_of_tree.ko
```
