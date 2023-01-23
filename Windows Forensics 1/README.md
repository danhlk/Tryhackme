> # Windows Forensics 1

# Summary 
<!-- TOC -->

- [Summary](#summary)
    - [Task 1 - Introduction to Windows Forensics](#task-1---introduction-to-windows-forensics)
    - [Task 2 - Windows Registry and Forensics](#task-2---windows-registry-and-forensics)
    - [Task 3 - Accessing registry hives offline](#task-3---accessing-registry-hives-offline)
    - [Task 6 - System Information and System Accounts](#task-6---system-information-and-system-accounts)
    - [Task 7 - Usage or knowledge of files/folders](#task-7---usage-or-knowledge-of-filesfolders)
    - [Task 8 - Evidence of Execution](#task-8---evidence-of-execution)
    - [Task 9 - External Devices/USB device forensics](#task-9---external-devicesusb-device-forensics)
    - [Task 10 - Hands-on Challenge](#task-10---hands-on-challenge)

<!-- /TOC -->

## Task 1 - Introduction to Windows Forensics
1. What is the most used Desktop Operating System right now?<br>
    > Microsoft Windows is by large the most used Desktop Operating System right now. 

    **Answer:** Microsoft Windows

## Task 2 - Windows Registry and Forensics
1. What is the short form for HKEY_LOCAL_MACHINE?<br>
    **Answer: ** HKLM

## Task 3 - Accessing registry hives offline
Locations where registry are stored on disk.<br>
The majority of these hives are located in the C:\Windows\System32\Config directory and are:<br>
* DEFAULT (mounted on HKEY_USERS\DEFAULT)
* SAM (mounted on HKEY_LOCAL_MACHINE\SAM)
* SECURITY (mounted on HKEY_LOCAL_MACHINE\Security)
* SOFTWARE (mounted on HKEY_LOCAL_MACHINE\Software)
* SYSTEM (mounted on HKEY_LOCAL_MACHINE\System)

For Windows 7 and above, a user’s profile directory is located in C:\Users\\\<username>\ where the hives are:<br>
* NTUSER.DAT (mounted on HKEY_CURRENT_USER when a user logs in)
* USRCLASS.DAT (mounted on HKEY_CURRENT_USER\Software\CLASSES)

The USRCLASS.DAT hive is located in the directory C:\Users\\\<username>\AppData\Local\Microsoft\Windows. <br>

C:\Windows\AppCompat\Programs\Amcache.hve. Windows creates this hive to save information on programs that were recently run on the system.

1. What is the path for the five main registry hives, DEFAULT, SAM, SECURITY, SOFTWARE, and SYSTEM?<br>
    **Answer:** C:\Windows\System32\config

1. What is the path for the AmCache hive?<br>
    **Answer:** C:\Windows\appcompat\Programs\Amcache.hve

## Task 6 - System Information and System Accounts
1. What is the Current Build Number of the machine whose data is being investigated?<br>
    ![](images/1.png)<br>
    **Answer:** 19044

1. Which ControlSet contains the last known good configuration?<br>
    **Answer:** 1

1. What is the Computer Name of the computer?<br>
    ![](images/2.png)<br>
    **Answer:** THM-4N6

1. What is the value of the TimeZoneKeyName?<br>
    ![](images/3.png)<br>
    **Answer:** Pakistan Standard Time

1. What is the DHCP IP address?<br>
    ![](images/4.png)<br>
    **Answer:** 192.168.100.58

1. What is the RID of the Guest User account?<br>
    ![](images/5.png)<br
    **Answer:** 501

## Task 7 - Usage or knowledge of files/folders
1. When was EZtools opened?<br>
    ![](images/6.png)<br>
    **Answer:** 2021-12-01 13:00:34

1. At what time was My Computer last interacted with?<br>
    ![](images/7.png)<br>
    **Answer:** 2021-12-01 13:06:47

1. What is the Absolute Path of the file opened using notepad.exe?<br>
    ![](images/8.png)<br>
    **Answer:** C:\Program Files\Amazon\Ec2ConfigService\Settings

1. When was this file opened?<br>    
    **Answer:** 2021-11-30 10:56:19

## Task 8 - Evidence of Execution
1. How many times was the File Explorer launched?<br>
    ![](images/9.png)<br>
    **Answer:** 26

1. What is another name for ShimCache?<br>
    > It is also called Application Compatibility Cache (AppCompatCache).

    **Answer:** AppCompatCache

1. Which of the artifacts also saves SHA1 hashes of the executed programs?<br>
    > This performs a similar function to ShimCache, and stores additional data related to program executions. This data includes execution path, installation, execution and deletion times, and SHA1 hashes of the executed programs. 

    **Answer:** AmCache

1. Which of the artifacts saves the full path of the executed programs?<br>
    **Answer:** BAM/DAM

## Task 9 - External Devices/USB device forensics
1. What is the serial number of the device from the manufacturer 'Kingston'?<br>
    ![](images/10.png)<br>
    **Answer:** 1C6F654E59A3B0C179D366AE&O

1. What is the name of this device?<br>
    **Answer:** Kingston DataTraveler 2.0 USB Device

1. What is the friendly name of the device from the manufacturer 'Kingston'?<br>
    **Answer:** USB

## Task 10 - Hands-on Challenge
1. How many user created accounts are present on the system? <br>
    Use RegistryExplorer to load `C:\Windows\System32\config\SAM` file, under User you will see three created accounts on system.<br>
    ![](images/11.png)<br>
    **Answer:** 3

1. What is the username of the account that has never been logged in?<br>
    Account thm-user2 doesn't have value on **Last Logon Time**.<br>
    **Answer:** thm-user2

1. What's the password hint for the user THM-4n6?<br>
    Under the **Password hint** filed.<br>
    **Answer:** count

1. When was the file 'Changelog.txt' accessed?<br>
    Load NTUSER.DAT and follow this path `NTUSER.DAT\Software\Microsoft\Windows\CurrentVersion\Explorer\RecentDocs`.<br>
    ![](images/12.png)<br>
    **Answer:** 2021-11-24 18:18:48

1. What is the complete path from where the python 3.8.2 installer was run? <br>
    Load NTUSER.DAT and follow `SOFTWARE\Microsoft\Windows\CurrentVersion\Explorer\UserAssist\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\Count` then search for string `python`.<br>
    ![](images/13.png)<br>
    **Answer:** Z:\setups\python-3.8.2.exe

1. When was the USB device with the friendly name 'USB' last connected?<br>
    Load SYSTEM hive and navigate to `ControlSet001\Enum\USBTOR`.<br>
    ![](images/14.png)<br>
    **Answer:** 2021-11-24 18:40:06
