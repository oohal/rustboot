#!/bin/bash -eu

cd ~/rust/rustboot/
export CROSS_COMPILE=powerpc64-linux-gnu-
cargo build $*

obj="target/powerpc64-unknown-linux-gnu/debug/rustboot"

powerpc64-linux-gnu-objcopy -O binary $obj dump

~/code/qemu-cedric/ppc64-softmmu/qemu-system-ppc64 \
	-m 4G \
	-machine powernv \
	-nographic \
	-bios ./dump
