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

For details about out-of-tree modules, see https://docs.kernel.org/kbuild/modules.html.

## rust-analyzer

Rust for Linux (with https://lore.kernel.org/rust-for-linux/20230121052507.885734-1-varmavinaym@gmail.com/ applied) supports building a `rust-project.json` configuration for [`rust-analyzer`](https://rust-analyzer.github.io/), including for out-of-tree modules:

```sh
make -C .../linux-with-rust-support M=$PWD rust-analyzer
```