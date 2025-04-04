setenv bootargs console=ttyS1,115200 console=tty0 root=/dev/mmcblk1p2 rw rootwait smsc95xx.macaddr="${usbethaddr}"
load mmc 0:1 ${kernel_addr_r} /Image
load mmc 0:1 ${ramdisk_addr_r} /initramfs-linux.uimg
load mmc 0:1 ${fdt_addr_r} /dtbs/broadcom/bcm2711-rpi-4-b.dtb
booti ${kernel_addr_r} ${ramdisk_addr_r}:${filesize} ${fdt_addr_r}
