> # macOS Forensics: The Basics

## Summary
- [Summary](#summary)
  - [Task 2 - A Brief History of macOS](#task-2---a-brief-history-of-macos)
  - [Task 3 - HFS+ File System](#task-3---hfs-file-system)
  - [Task 4 - APFS File System](#task-4---apfs-file-system)
  - [Task 5 - macOS Directory Structure and Domains](#task-5---macos-directory-structure-and-domains)
  - [Task 6 - macOS File Formats](#task-6---macos-file-formats)
  - [Task 7 - Challenges in Data Acquisition](#task-7---challenges-in-data-acquisition)
  - [Task 8Mounting APFS Disk Image](#task-8mounting-apfs-disk-image)

### Task 2 - A Brief History of macOS
1. In which year was the APFS file system introduced by Apple?<br>
    > This file system was the mainstay of macOS until it was replaced by APFS in 2017. 

    **Answer:** 2017

1. Which file system is common amongst all Apple devices, including iOS, watchOS, and tvOS?<br>
    **Answer:** APFS

### Task 3 - HFS+ File System
1. What is the latest date that the HFS+ file system supports? Format DD/MM/YYYY<br>
    > The HFS+ file system did not support dates beyond 6 February 2040.

    **Answer:** 06/02/2040

1. In the HFS+ file system, which file contains a list of all the files and directories present in the file system?<br>
    > A catalog file contains a catalogue of all the files and directories stored in the volume in a B-tree format. 

    **Answer:** catalog file

### Task 4 - APFS File System
1. In macOS, which command can be used to list all available APFS volumes?<br>
    > To see what different kinds of APFS volumes are available in the macOS machine, the diskutil apfs list command can be used.

    **Answer:** diskutil apfs list

### Task 5 - macOS Directory Structure and Domains
1. Files in which domain can't be modified even by using sudo privileges?<br>
    > The System domain contains software developed and managed by Apple. It maps to the /System directory and contains critical OS applications and configurations. Apple does not allow users to modify or remove files in this domain, even with root privileges.

    **Answer:** System

1. Which domain is the most forensically important domain from a user activity point of view?<br>
    >  the User domain contains juicy information from a forensics standpoint

    **Anser:** User

### Task 6 - macOS File Formats
1. Which of the file types discussed above are similar to device drivers in Windows?<br>
    > the .kext files are kernel extension files. Kernel extensions work similarly to drivers in Windows, providing access to the OS kernel to third-party apps

    **Answer:** .kext files

### Task 7 - Challenges in Data Acquisition
1. Which command can be used to disable SIP?<br>
    > SIP can be disabled by booting into recovery, opening the terminal, and using the command csrutil disable.

    **Answer:** csrutil disable

### Task 8Mounting APFS Disk Image
1. In the disk image provided in the attached VM, what is the UUID of the volume containing user data?<br>
    ```
    Volume 4 84E5F2BD-503F-4E3A-8105-EEBEBC1925B4
    ---------------------------------------------
    Role:               Data
    Name:               Data (Case-insensitive)
    Capacity Consumed:  2114285568 Bytes
    FileVault:          No
    Snapshots:
    ```
    **Answer:** 84E5F2BD-503F-4E3A-8105-EEBEBC1925B4

1. What is the name of the user who used this Mac device?<br>
    `ls mac/root/Users`

    **Answer:** thm

1. There is a text file in the User directory of the above-mentioned user. What is the name of this file?<br>
    ```
    root@tryhackme:/home/ubuntu# ls mac/root/Users/thm
    Desktop  Documents  Downloads  Library  Movies  Music  Pictures  Public  creds.txt
    ```
    **Answer:** creds.txt

1. It looks like the file contains credentials for different users and their passwords. What is the password of the user we identified above?<br>
    ```
    root@tryhackme:/home/ubuntu# cat mac/root/Users/thm/creds.txt 
    thm:12345
    ubuntu:TryHackMe!
    ```
    **Answer:** 12345
