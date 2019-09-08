#!/bin/bash -eu

export CROSS_COMPILE=powerpc64-linux-gnu-
cargo build

obj="target/powerpc64-unknown-linux-gnu/debug/rustboot"

powerpc64-linux-gnu-objcopy -O binary $obj dump

# find the entry point
#entry="$(powerpc64-linux-gnu-readelf -h $obj | grep 'Entry point' | sed 's@ \+@ @g' | cut -d' ' -f 5)"
entry="0x$(powerpc64-linux-gnu-objdump $obj -t | grep entry_point|cut -d' ' -f 1)"
load_base="0x$(powerpc64-linux-gnu-objdump $obj -h | grep LOAD -B1 | head -1 | sed 's@ \+@ @g' | cut -d' ' -f 5)"

echo "load offset: $load_base"
echo "entry point: $entry"

#export MAMBO_BOOT_LOAD="0x$(printf %x $((0x30000000 + $load_base)))"
export MAMBO_BOOT_LOAD=0x30000000
export MAMBO_BOOT_PC=0x30000000
#export MAMBO_BOOT_PC="0x$(printf %x $(($MAMBO_BOOT_LOAD + $entry - $load_base)))"

echo "actual load: $MAMBO_BOOT_LOAD"
echo "actual entry: $MAMBO_BOOT_PC"

p8.sh
