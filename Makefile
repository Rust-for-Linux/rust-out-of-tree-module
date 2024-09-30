# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/`uname -r`/build
export KBUILD_EXTMOD = $(CURDIR)
include $(KDIR)/Makefile
