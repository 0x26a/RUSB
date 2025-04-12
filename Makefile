all:
	cargo +nightly build --target x86_64-unknown-uefi
	mkdir -p esp/efi/boot
	cp target/x86_64-unknown-uefi/debug/uefi-grub-preloader.efi esp/efi/boot/bootx64.efi
	cp picture.rgb esp/efi/boot/picture.rgb
	qemu-system-x86_64 -enable-kvm \
    	-drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.4m.fd \
    	-drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.4m.fd \
    	-drive format=raw,file=fat:rw:esp
