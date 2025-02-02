> # MBR and GPT Analysis

## Summary

### Task 1 - Introduction
1. What are the separate sections on a disk known as?<br>
    > In the Windows OS, these partitions are represented by drive letters such as C, D, E, etc. Other operating systems may use different ways to refer to these partitions. 

    **Answer:** partitions

1. Which type of malware infects the boot process?<br>
    > often known as Bootkits, or tampering with them to make the system un-bootable.

    **Answer:** Bootkit

### Task 2 - Boot Process
![](images/1.png)<br>
1. What is the name of the hardware diagnostic check performed during the boot process?<br>
    > . The BIOS/UEFI then starts a Power-On-Self-Test to ensure all the system’s hardware components are working fine. You

    **Answer:**  Power-On-Self-Test

1. Which firmware supports a GPT partitioning scheme?<br>
    > UEFI uses a GPT partitioning scheme

    **Answer:** UEFI

1. Which device has the operating system to boot the system?<br>
    **Answer:** bootable device

### Task 3 - What if MBR?
1. Which component of the MBR contains the details of all the partitions present on the disk?<br>
    > This table contains the details of all the partitions present on the disk. 

    **Answer:** partition table

1. What is the standard sector size of a disk in bytes?<br>
    > The Master Boot Record (MBR) takes up 512 bytes of space at the very first sector of the disk. 

    **Answer:** 512

1. Which component of the MBR is responsible for finding the bootable partition?<br>
    > This Bootloader code contains the Initial Bootloader. The initial bootloader is the first thing that executes in the MBR. This initial bootloader code has a primary purpose of finding the bootable partition

    **Answer:** Bootloader Code

1. What is the magic number inside the MBR?<br>
    **Answer:** 55 AA

1. What is the maximum number of partitions MBR can support?<br>
    > An MBR disk has a total of 4 partitions, and each partition is represented by 16 bytes in the partition table.

    **Answer:** 4

1. What is the size of the second partition in the MBR file found in C:\Analysis\MBR\? (rounded to the nearest GB)<br>
    Convert the 4 byte to decimal then calculate with 512 to get the size in GB.<br>
    ![](images/2.png)<br>
    **Answer:** 16

### Task 5 - MBR Tampering Case
You must edit signature and LAB to 55AA and 00080000 to FTK can read the file.<br>
1. How many partitions are on the disk?<br>
    There is only one partition on the disk.<br>
    ![](images/3.png)<br>
    **Answer:** 1

1. What is the first byte at the starting LBA of the partition? (represented by two hexadecimal digits)<br>
    Go to offset 1048576.<br>
    **Answer:** EB

1. What is the type of the partition?<br>
    The value of byte is `07`, so the partition's type is NTFS.<br>
    **Answer:** NTFS

1. What is the size of the partition? (rounded to the nearest GB)<br>
    ![](images/4.png)<br>
    **Answer:** 32

1. What is the flag hidden in the Administrator's Documents folder?<br>
    ![](images/5.png)<br>
    **Answer:** THM{Cure_The_MBR}

### Task 6 - What if GPT?
1. How many partitions are supported by the GPT?<br>
    > The GPT also supports up to 128 partitions

    **Answer:** 128

1. What is the partition type GUID of the 2nd partition given in the attached GPT file?<br>
    Follow the instruction to get the partition type GUID.<br>
    **Answer:** E3C9E316-0B5C-4DB8-817D-F92DF00215AE

### Task 8 - UEFI Bootkit Case
1. Which partition has the bootloader in it?<br>
    > In the GPT, the bootloader is comprised of multiple files with .efi extension, and they all are stored in this EFI System Partition (ESP). 

    **Answer:** EFI System Partition

1. What is the malicious string embedded in the bootloader?<br>
    I found a base64 string.<br>
    ![](images/6.png)<br>
    **Answer:** Hello, EFI Bootkit!
