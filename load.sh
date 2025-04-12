sudo mount /dev/your_efi_partition /mnt
sudo rm -rf /mnt/efi
sudo mkdir /mnt/efi
sudo mkdir /mnt/efi/boot
sudo cp esp/efi/boot/bootx64.efi /mnt/efi/boot/bootx64.efi
sudo cp esp/efi/boot/picture.rgb /mnt/efi/boot/picture.rgb
sudo umount /dev/your_efi_partition
