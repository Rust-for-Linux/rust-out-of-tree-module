// SPDX-License-Identifier: GPL-2.0

//! Rust out-of-tree sample

use kernel::prelude::*;

module! {
    type: RustOutOfTree,
    name: "rust_out_of_tree",
    authors: ["Rust for Linux Contributors"],
    description: "Rust out-of-tree sample",
    license: "GPL",
}

// Sample values to demonstrate kernel vector usage
const SAMPLE_VALUES: [i32; 3] = [72, 108, 200];

struct RustOutOfTree {
    numbers: KVec<i32>,
}

impl kernel::Module for RustOutOfTree {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust out-of-tree sample (init)\n");

        let mut numbers = KVec::new();
        for &value in &SAMPLE_VALUES {
            numbers.push(value, GFP_KERNEL)?;
        }

        Ok(RustOutOfTree { numbers })
    }
}

impl Drop for RustOutOfTree {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("Rust out-of-tree sample (exit)\n");
    }
}
