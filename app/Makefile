incdebug:
	kitty -e /home/dingduck/Cell/GitCell/xx32/MRS_Toolchain_Linux_x64_V1.90/OpenOCD/bin/openocd -f .wwch-riscv.cfg &
	riscv64-elf-gdb -x openocd.gdb 

binary:
	rust-objcopy --strip-all ./target/riscv32imac-unknown-none-elf/release/app ./target/riscv32imac-unknown-none-elf/release/app.bin
