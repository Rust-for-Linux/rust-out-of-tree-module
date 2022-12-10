# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

rust-analyzer:
	$(MAKE) -C $(KDIR) rust-analyzer
	$(Q) ./scripts/generate_rust_analyzer.py $(KDIR) `ls *.rs | head -n 1` > rust-project.json

clean:
	$(MAKE) -C $(KDIR) M=$$PWD clean
