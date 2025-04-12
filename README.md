# RUSB - RUSB Unveils Safe Booting
An UEFI program coded in Rust to load EFI files.

![Image](https://github.com/user-attachments/assets/935b3d60-d4c5-4907-a0cb-4536e478bf6f)

## What and why ?

Having both a motherboard that doesn't implement custom booting path and a strong desire to experiment with UEFI, I came up with this.

Hence the idea was to create a FAT32 formated EFI partition on a USB key that I will never unplug from my PC.

Then I just put the generated EFI program and ask for the computer to boot on the key in order to always start an EFI file located in another drive at startup.


## Warnings
This code was written for learning purpose and may (SAFELY !) crash on some hardware.

## Run on QEMU

You must first create a copy of your local firmware files (`OVMF_VARS.fd` & `OVMF_CODE.fd`) to the root of the project directory. Mine were located at `/usr/share/edk2/x64`.

Then the `Makefile` will do the rest of the job for you.

## Run on real hardware

You must create in GPT mode a FAT32 formated EFI partition on your USB key (or any other storage device).

Then, while the key is still plugged, **modify** and run the `load.sh` script to setup the program.


```bash
sudo mount /dev/your_efi_partition /mnt
sudo rm -rf /mnt/efi
sudo mkdir /mnt/efi
sudo mkdir /mnt/efi/boot
sudo cp esp/efi/boot/bootx64.efi /mnt/efi/boot/bootx64.efi
sudo cp esp/efi/boot/picture.rgb /mnt/efi/boot/picture.rgb
sudo umount /dev/your_efi_partition
```


