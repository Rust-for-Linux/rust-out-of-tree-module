# Rust out-of-tree module

This is an enhanced template for an out-of-tree Linux kernel module written in Rust, demonstrating:

- **Character device interface** via `/dev/rust_device`
- **Module parameters** for runtime configuration
- **Mutex-based synchronization** for thread-safe operations
- **File operations** (read, write, ioctl)

## Requirements

- Rust-enabled kernel (`CONFIG_RUST=y`)
- Linux kernel 6.19+ for module parameters support
- Kernel tree (`KDIR`) with Rust metadata available
- LLVM toolchain

Please note that:

  - The Rust support is experimental.

  - The kernel that the module is built against needs to be Rust-enabled (`CONFIG_RUST=y`).

  - The kernel tree (`KDIR`) requires the Rust metadata to be available. These are generated during the kernel build, but may not be available for installed/distributed kernels (the scripts that install/distribute kernel headers etc. for the different package systems and Linux distributions are not updated to take into account Rust support yet).

  - All Rust symbols are `EXPORT_SYMBOL_GPL`.

## Building

Build the module:

```sh
make KDIR=.../linux-with-rust-support LLVM=1
```

Example output:

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

## Module Parameters

The module accepts the following parameters:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_numbers` | i32 | 10 | Maximum number of integers to store |
| `initial_value` | i32 | 42 | Initial value to add on module load |

**Example: Load module with custom parameters**

```sh
sudo insmod rust_out_of_tree.ko max_numbers=20 initial_value=100
```

**View module parameters at runtime:**

```sh
cat /sys/module/rust_out_of_tree/parameters/max_numbers
cat /sys/module/rust_out_of_tree/parameters/initial_value
```

## Character Device Usage

The module creates `/dev/rust_device` that can be used from userspace.

### Reading from the device

```sh
# Read all stored numbers
cat /dev/rust_device
```

Output:
```
Numbers: [42, 72, 108]
```

### Writing to the device

```sh
# Add a new number
echo "123" | sudo tee /dev/rust_device

# Verify it was added
cat /dev/rust_device
```

Output:
```
Numbers: [42, 72, 108, 123]
```

### Using IOCTL commands

The device supports two IOCTL commands:
- **Command 0**: Clear all numbers
- **Command 1**: Get count of numbers

**Example C program:**

```c
#include <fcntl.h>
#include <sys/ioctl.h>
#include <stdio.h>
#include <unistd.h>

int main() {
    int fd = open("/dev/rust_device", O_RDWR);
    if (fd < 0) {
        perror("open");
        return 1;
    }
    
    // Get count (IOCTL command 1)
    int count = ioctl(fd, 1, 0);
    printf("Number count: %d\n", count);
    
    // Clear all (IOCTL command 0)
    ioctl(fd, 0, 0);
    printf("Cleared all numbers\n");
    
    close(fd);
    return 0;
}
```

Compile and run:
```sh
gcc -o test_device test_device.c
sudo ./test_device
```

## Kernel Logs

View module activity in kernel logs:

```sh
sudo dmesg | grep rust_out_of_tree
```

Example output:
```txt
[  123.456789] rust_out_of_tree: Rust out-of-tree sample (init)
[  123.456790] rust_out_of_tree: Module parameters: max_numbers=10, initial_value=42
[  123.456791] rust_out_of_tree: Added number: 42, total count: 1
[  123.456792] rust_out_of_tree: Added number: 72, total count: 2
[  123.456793] rust_out_of_tree: Added number: 108, total count: 3
[  123.456794] rust_out_of_tree: Character device /dev/rust_device created
[  456.789012] rust_out_of_tree: Device opened
[  456.789013] rust_out_of_tree: Device read: 24 bytes
[  789.012345] rust_out_of_tree: Device write: added number 123
[  999.999999] rust_out_of_tree: Rust out-of-tree sample (exit)
```

## Features

### Thread Safety

All operations on the shared data structure are protected by a mutex (`kernel::sync::Mutex`), ensuring thread-safe access from multiple processes.

### Error Handling

- Input validation for module parameters
- Capacity checks before adding numbers
- Proper error codes (EINVAL, ENOSPC, etc.)
- Informative kernel log messages

### File Operations

Implements the following operations:
- `open()` - Opens the device file
- `read()` - Returns stored numbers as formatted string
- `write()` - Parses and adds new integers
- `ioctl()` - Special commands (clear, get count)

## Additional Resources

For details about the Rust support, see https://rust-for-linux.com.

For details about getting started with kernel development in Rust, see https://docs.kernel.org/rust/.

For details about out-of-tree modules, see https://docs.kernel.org/kbuild/modules.html.

## rust-analyzer

Rust for Linux (with https://lore.kernel.org/rust-for-linux/20230121052507.885734-1-varmavinaym@gmail.com/ applied) supports building a `rust-project.json` configuration for [`rust-analyzer`](https://rust-analyzer.github.io/), including for out-of-tree modules:

```sh
make -C .../linux-with-rust-support M=$PWD rust-analyzer
```

## Testing

1. **Build the module** (as shown above)
2. **Load the module:**
   ```sh
   sudo insmod rust_out_of_tree.ko
   ```
3. **Check device creation:**
   ```sh
   ls -l /dev/rust_device
   ```
4. **Test read/write operations** (as shown in examples above)
5. **Unload the module:**
   ```sh
   sudo rmmod rust_out_of_tree
   ```