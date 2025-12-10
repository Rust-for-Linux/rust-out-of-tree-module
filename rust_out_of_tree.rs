// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample with character device, module parameters, and synchronization
//!
//! This module demonstrates:
//! - Character device interface via miscdevice
//! - Module parameters for runtime configuration
//! - Mutex-based synchronization for thread safety
//! - Proper error handling and logging

use kernel::{
    file::{File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev,
    prelude::*,
    sync::{Arc, Mutex},
    c_str,
};

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    authors: ["Rust for Linux Contributors"],
    description: "Rust out-of-tree sample with char device",
    license: "GPL",
    params: {
        max_numbers: i32 {
            default: 10,
            permissions: 0o644,
            description: "Maximum number of integers to store",
        },
        initial_value: i32 {
            default: 42,
            permissions: 0o644,
            description: "Initial value to add on module load",
        },
    },
}

/// Shared data structure protected by mutex
struct SharedData {
    numbers: Vec<i32>,
    max_capacity: usize,
}

impl SharedData {
    fn new(max_capacity: usize) -> Result<Self> {
        Ok(Self {
            numbers: Vec::new(),
            max_capacity,
        })
    }

    fn add_number(&mut self, num: i32) -> Result<()> {
        if self.numbers.len() >= self.max_capacity {
            pr_warn!("Vector is at capacity ({}), cannot add more numbers\n", self.max_capacity);
            return Err(ENOSPC);
        }
        self.numbers.try_push(num)?;
        pr_info!("Added number: {}, total count: {}\n", num, self.numbers.len());
        Ok(())
    }

    fn clear(&mut self) {
        self.numbers.clear();
        pr_info!("Cleared all numbers\n");
    }

    fn get_count(&self) -> usize {
        self.numbers.len()
    }
}

/// Main module structure
struct RustOutOfTree {
    _dev: Pin<Box<miscdev::Registration<RustOutOfTree>>>,
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");
        
        let max = *max_numbers.read();
        let initial = *initial_value.read();
        
        pr_info!("Module parameters: max_numbers={}, initial_value={}\n", max, initial);

        // Validate parameters
        if max < 1 {
            pr_err!("Invalid max_numbers parameter: {}\n", max);
            return Err(EINVAL);
        }

        // Create shared data with mutex protection
        let data = Arc::try_new(Mutex::new(SharedData::new(max as usize)?))?;
        
        // Add initial value
        {
            let mut guard = data.lock();
            guard.add_number(initial)?;
            guard.add_number(72)?;  // 'H' in ASCII
            guard.add_number(108)?; // 'l' in ASCII
        }

        // Register character device
        let reg = miscdev::Registration::new_pinned::<RustOutOfTree>(
            c_str!("rust_device"),
            data,
        )?;

        pr_info!("Character device /dev/rust_device created\n");

        Ok(Self { _dev: reg })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}

/// File operations for the character device
#[vtable]
impl Operations for RustOutOfTree {
    type OpenData = Arc<Mutex<SharedData>>;
    type Data = Arc<Mutex<SharedData>>;

    fn open(context: &Arc<Mutex<SharedData>>, _file: &File) -> Result<Self::Data> {
        pr_info!("Device opened\n");
        Ok(context.clone())
    }

    fn read(
        data: &Arc<Mutex<SharedData>>,
        _file: &File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        let guard = data.lock();
        
        // Format numbers as a string
        let mut output = kernel::fmt::KernelStr::try_from_fmt(fmt!("Numbers: {:?}\n", guard.numbers))?;
        let bytes = output.as_bytes();
        
        let len = bytes.len();
        writer.write_slice(bytes)?;
        
        pr_info!("Device read: {} bytes\n", len);
        Ok(len)
    }

    fn write(
        data: &Arc<Mutex<SharedData>>,
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        let mut buffer = [0u8; 32];
        let len = reader.read_slice(&mut buffer)?;
        
        // Parse the input as an integer
        let input = core::str::from_utf8(&buffer[..len])
            .map_err(|_| EINVAL)?
            .trim();
        
        let num: i32 = input.parse().map_err(|_| EINVAL)?;
        
        let mut guard = data.lock();
        guard.add_number(num)?;
        
        pr_info!("Device write: added number {}\n", num);
        Ok(len)
    }

    fn ioctl(
        data: &Arc<Mutex<SharedData>>,
        _file: &File,
        cmd: &mut kernel::ioctl::Command,
    ) -> Result<i32> {
        match cmd.cmd() {
            // IOCTL command 0: Clear all numbers
            0 => {
                let mut guard = data.lock();
                guard.clear();
                Ok(0)
            }
            // IOCTL command 1: Get count
            1 => {
                let guard = data.lock();
                let count = guard.get_count();
                pr_info!("Device ioctl: get count = {}\n", count);
                Ok(count as i32)
            }
            _ => Err(EINVAL),
        }
    }
}
