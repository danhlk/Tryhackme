> # Windows Command Line

## Summary
- [Summary](#summary)
  - [Task 1 - Introduction](#task-1---introduction)
  - [Task 2 - Basic System Information](#task-2---basic-system-information)
  - [Task 3 - Network Troubleshooting](#task-3---network-troubleshooting)
  - [Task 4 - File and Disk Management](#task-4---file-and-disk-management)
  - [Task 5 - Task and Process Management](#task-5---task-and-process-management)
  - [Task 6 - Conclusion](#task-6---conclusion)

### Task 1 - Introduction
1. What is the default command line interpreter in the Windows environment?<br>
    > cmd.exe, the default command-line interpreter in the Windows environment.

    **Answer:** cmd.exe

### Task 2 - Basic System Information
1. What is the OS version of the Windows VM?<br>
    Use `ver`.<br>
    **Answer:** 10.0.20348.2655

1. What is the hostname of the Windows VM?<br>
    Use `hostname` command.<br>
    **Answer:** WINSRV2022-CORE

### Task 3 - Network Troubleshooting
1. Which command can we use to look up the server’s physical address (MAC address)?<br>
    Use `ipconfig /all` to get all information about network on the computer.<br>
    ```
    >ipconfig /all

    Windows IP Configuration

    Host Name . . . . . . . . . . . . : WINSRV2022-CORE
    Primary Dns Suffix  . . . . . . . :
    Node Type . . . . . . . . . . . . : Hybrid
    IP Routing Enabled. . . . . . . . : No
    WINS Proxy Enabled. . . . . . . . : No
    DNS Suffix Search List. . . . . . : eu-west-1.compute.internal
                                        eu-west-1.ec2-utilities.amazonaws.com

    Ethernet adapter Ethernet:

    Connection-specific DNS Suffix  . : eu-west-1.compute.internal
    Description . . . . . . . . . . . : Amazon Elastic Network Adapter
    Physical Address. . . . . . . . . : 02-B9-B0-EA-A3-65
    DHCP Enabled. . . . . . . . . . . : Yes
    Autoconfiguration Enabled . . . . : Yes
    Link-local IPv6 Address . . . . . : fe80::ee3f:d38e:e62a:8be6%5(Preferred)
    IPv4 Address. . . . . . . . . . . : 10.10.6.205(Preferred)
    Subnet Mask . . . . . . . . . . . : 255.255.0.0
    Lease Obtained. . . . . . . . . . : Sunday, October 27, 2024 4:45:12 AM
    Lease Expires . . . . . . . . . . : Sunday, October 27, 2024 6:45:11 AM
    Default Gateway . . . . . . . . . : 10.10.0.1
    DHCP Server . . . . . . . . . . . : 10.10.0.1
    DHCPv6 IAID . . . . . . . . . . . : 84601211
    DHCPv6 Client DUID. . . . . . . . : 00-01-00-01-2D-B9-B7-EF-00-0C-29-FF-E5-C8
    DNS Servers . . . . . . . . . . . : 10.0.0.2
    NetBIOS over Tcpip. . . . . . . . : Enabled
    ```
    **Answer:** ipconfig /all

1. What is the name of the process listening on port 3389?<br>
    Use `netstat -aonb` to view listening port and the process belong to.<br>
    ```shell
    Can not obtain ownership information
    TCP    0.0.0.0:3389           0.0.0.0:0              LISTENING       992
    TermService
    ```
    **Answer:** TermService

1. What is the IP address of your gateway?<br>
    **Answer:** 10.10.0.1

### Task 4 - File and Disk Management
1. What are the file’s contents in C:\Treasure\Hunt?<br>
    Use `type C:\Treasure\Hunt\flag.txt` to read the file's content.<br>
    **Answer:** 

### Task 5 - Task and Process Management
1. What command would you use to find the running processes related to notepad.exe?<br>
    Use `tasklist /FI "imagename eq notepad.exe`.<br>
    **Answer:** tasklist /FI "imagename eq notepad.exe"

1. What command can you use to kill the process with PID 1516?<br>
    **Answer:** taskkill /PID 1516

### Task 6 - Conclusion
1. The command shutdown /s can shut down a system. What is the command you can use to restart a system?<br>
    **Answer:** shutdown /r

1. What command can you use to abort a scheduled system shutdown?<br>
    **Answer:** shutdown /a

