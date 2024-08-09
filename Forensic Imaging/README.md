> # Forensic Imaging

# Summary
- [Summary](#summary)
  - [Task 2 - Preparation](#task-2---preparation)
  - [Task 4 - Integrity Checking](#task-4---integrity-checking)
  - [Task 5 - Other Types of Imaging](#task-5---other-types-of-imaging)
  - [Task 6 - Practical Exercise](#task-6---practical-exercise)

## Task 2 - Preparation
1. What command can be used to list all block devices in Linux OS?<br>
    > We can still list block devices using the lsblk command with the -a option to list all devices, as shown below.

    **Answer:** lsblk

1. Which bash command displays all commands executed in a session?<br>
    **Asnwer:** history

## Task 4 - Integrity Checking
1. What is the MD5 hash of the image "exercise.img" located in /home/analyst/?<br>
    Use `md5sum` to calculate the MD5 hash.<br>
    **Answer:** 1f1da616156f73083521478c334841bb

##  Task 5 - Other Types of Imaging
1. Mount the image "exercise.img" located in the analyst home directory folder. What is the content of the file "flag.txt" located within exercise.img?<br>
    Follow the instructions, mount the image to `/mnt/example1` then cat the `flag.txt`.<br>
    **Answer:** THM{mounttt-mounttt-me}

##  Task 6 - Practical Exercise
1. Create an image of the attached 1gb loop device. What is the MD5 hash of the image?<br>
    Use `lsblk -a` to list all block device on the machine.<br>
    ```shell
    $ lsblk -a
    NAME    MAJ:MIN RM  SIZE RO TYPE MOUNTPOINT
    loop0     7:0    0 97.8M  1 loop /snap/core/10185
    loop1     7:1    0 55.3M  1 loop /snap/core18/1885
    loop2     7:2    0 28.1M  1 loop /snap/amazon-ssm-agent/2012
    loop3     7:3    0 70.6M  1 loop /snap/lxd/16922
    loop4     7:4    0    1G  0 loop 
    loop5     7:5    0        0 loop 
    loop6     7:6    0        0 loop 
    loop7     7:7    0        0 loop 
    md0       9:0    0        0 md   
    xvda    202:0    0   40G  0 disk 
    └─xvda1 202:1    0   40G  0 part /
    xvdh    202:112  0    1G  0 disk
    ```
    As we can see, the 1GB loop is `/dev/loop4`, so we create an image to this device with `dc3dd`.<br>
    ```shell
    $ sudo dc3dd if=/dev/loop4 of=example1.img log=imaging_loop.txt

    dc3dd 7.2.646 started at 2024-08-09 15:35:07 +0000
    compiled options:
    command line: dc3dd if=/dev/loop4 of=example1.img log=imaging_loop.txt
    device size: 2097152 sectors (probed),    1,073,741,824 bytes
    sector size: 512 bytes (probed)
    1073741824 bytes ( 1 G ) copied ( 100% ),   14 s, 73 M/s                    

    input results for device `/dev/loop4':
    2097152 sectors in
    0 bad sectors replaced by zeros

    output results for file `example1.img':
    2097152 sectors out

    dc3dd completed at 2024-08-09 15:35:21 +0000
    ```
    Calculate the MD5 hash of the output file.<br>
    **Answer:** 1fab86e499934dda789c9c4aaf27101d

1. Mount the image from the 1 GB loop device. What is the content of the file "flag.txt"?<br>
    Create a folder store the mount data `sudo mkdir -p /mnt/example1`. Then mount the iamge to that folder with command `sudo mount -o loop example1.img /mnt/example1`.<br>
    The `flag.txt` is in the `/mnt/example1/flag.txt`.<br>
    **Answer:** THM{well-done-imaginggggggg}

