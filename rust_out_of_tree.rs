// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

module! {
    type: RustOutOfTree,
    name: b"rust_out_of_tree",
    author: b"Rust for Linux Contributors",
    description: b"Rust out-of-tree sample",
    license: b"GPL",
}

struct RustOutOfTree {
    message: String,
}

impl kernel::Module for RustOutOfTree {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        Ok(RustOutOfTree {
            message: "on the heap!".try_to_owned()?,
        })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My message is {}\n", self.message);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
