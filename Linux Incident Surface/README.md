> # Linux Incident Surface

## Summary
- [Summary](#summary)
  - [Task 2 - Lab Connection](#task-2---lab-connection)
  - [Task 4 - Processes and Network Communication](#task-4---processes-and-network-communication)
  - [Task 5 - Persistence](#task-5---persistence)
  - [Task 6 - Footprints on Disk](#task-6---footprints-on-disk)
  - [Task 7 - Linux Logs](#task-7---linux-logs)

### Task 2 - Lab Connection
1. Connect with the lab. How many files and folders are in the /home/activities/processes directory?<br>
    ```
    $ ls /home/activities/processes/
    netcom  simple.c  suspicious
    ```
    **Answer:** 3

### Task 4 - Processes and Network Communication
1. What is the remote IP to which the process netcom establishes the connection?<br>
    Run `lsof` command to view network connection of `netcom`.<br>
    ```
    $ lsof -i -P -n
    COMMAND    PID   USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
    Xtigervnc 1018 ubuntu    7u  IPv4  30092      0t0  TCP 127.0.0.1:5901 (LISTEN)
    Xtigervnc 1018 ubuntu    8u  IPv6  30093      0t0  TCP [::1]:5901 (LISTEN)
    Xtigervnc 1018 ubuntu   22u  IPv4  37649      0t0  TCP 127.0.0.1:5901->127.0.0.1:53416 (ESTABLISHED)
    Xtigervnc 1018 ubuntu   30u  IPv4  37663      0t0  TCP 127.0.0.1:5901->127.0.0.1:34720 (ESTABLISHED)
    netcom    1894 ubuntu    3u  IPv4  39942      0t0  TCP 10.10.153.143:60578->68.53.23.246:443 (SYN_SENT)
    ```
    **Answer:** 68.53.23.246

1. Update the osquery command. What is the remote port the netcom process is communicating to?<br>
    The answer is in above output.<br>
    **Answer:** 443

### Task 5 - Persistence
1. What is the default path that contains all the installed services in Linux?<br>
    > All services installed and enabled on the Linux host can be found in the /etc/systemd/system directory.

    **Answer:** /etc/systemd/system

1. Which suspicious service was found to be running on the host?<br>
    Run `ls /etc/systemd/system`, you will see a service named `benign.service` which is not the default service on Linux.<br>
    **Answer:** benign.service

1. What process does this service point to?<br>
    Run `systemctl status benign.service` to view information.<br>
    ```shell
    $ systemctl status benign.service    
    ● benign.service
        Loaded: loaded (/etc/systemd/system/benign.service; enabled; vendor preset: enabled)
        Active: failed (Result: exit-code) since Sun 2024-10-06 10:07:57 UTC; 1h 9min ago
        Process: 656 ExecStart=/tmp/benign (code=exited, status=203/EXEC)
    Main PID: 656 (code=exited, status=203/EXEC)

    Oct 06 10:07:57 tryhackme systemd[1]: Started benign.service.
    Oct 06 10:07:57 tryhackme systemd[1]: benign.service: Main process exited, code=exited, status=203/EXEC
    Oct 06 10:07:57 tryhackme systemd[1]: benign.service: Failed with result 'exit-code'.
    Oct 06 10:07:57 tryhackme systemd[1]: benign.service: Scheduled restart job, restart counter is at 5.
    Oct 06 10:07:57 tryhackme systemd[1]: Stopped benign.service.
    Oct 06 10:07:57 tryhackme systemd[1]: benign.service: Start request repeated too quickly.
    Oct 06 10:07:57 tryhackme systemd[1]: benign.service: Failed with result 'exit-code'.
    Oct 06 10:07:57 tryhackme systemd[1]: Failed to start benign.service.
    Oct 06 11:17:48 tryhackme systemd[1]: /etc/systemd/system/benign.service:2: Assignment outside of section. Ignoring.
    Oct 06 11:17:48 tryhackme systemd[1]: /etc/systemd/system/benign.service:3: Assignment outside of section. Ignoring.
    ```
    The service point to `ExecStart=/tmp/benign`.<br>
    **Answer:** benign

1. Before getting this service stopped on 11th Sept, how many log entries were observed in the journalctl against this service?<br>
    ```shell
    $ journalctl -u benign    
    -- Logs begin at Sun 2022-02-27 13:52:14 UTC, end at Sun 2024-10-06 11:21:06 UTC. --
    Sep 11 23:07:06 tryhackme systemd[1]: /etc/systemd/system/benign.service:2: Assignment outside of section. Ignoring.
    Sep 11 23:07:06 tryhackme systemd[1]: /etc/systemd/system/benign.service:3: Assignment outside of section. Ignoring.
    Sep 11 23:07:06 tryhackme systemd[1]: /etc/systemd/system/benign.service:2: Assignment outside of section. Ignoring.
    Sep 11 23:07:06 tryhackme systemd[1]: /etc/systemd/system/benign.service:3: Assignment outside of section. Ignoring.
    Sep 11 23:07:15 tryhackme systemd[1]: Started benign.service.
    Sep 11 23:07:23 tryhackme systemd[1]: /etc/systemd/system/benign.service:2: Assignment outside of section. Ignoring.
    Sep 11 23:07:23 tryhackme systemd[1]: /etc/systemd/system/benign.service:3: Assignment outside of section. Ignoring.
    Sep 11 23:11:38 tryhackme systemd[1]: Stopping benign.service...
    Sep 11 23:11:38 tryhackme systemd[1]: benign.service: Succeeded.
    Sep 11 23:11:38 tryhackme systemd[1]: Stopped benign.service.
    -- Reboot --

    ```
    Before the service stop at `Sep 11 23:11:38`, there are seven entries.<br>
    **Answer:** 7

### Task 6 - Footprints on Disk
1. Create a suspicious Debian package on the disk by following the steps mentioned in the task. How many log entries are observed in the dpkg.log file associated with this installation activity?<br>
    Filter package name in `dpkg.log`.<br>
    ```shell
    # grep "malicious-package" /var/log/dpkg.log
    2024-10-06 13:56:08 install malicious-package:all <none> 1.0
    2024-10-06 13:56:08 status half-installed malicious-package:all 1.0
    2024-10-06 13:56:08 status unpacked malicious-package:all 1.0
    2024-10-06 13:56:08 configure malicious-package:all 1.0 1.0
    2024-10-06 13:56:08 status half-configured malicious-package:all 1.0
    2024-10-06 13:56:08 status installed malicious-package:all 1.0
    ```
    **Answer:** 6

1. What package was installed on the system on the 17th of September, 2024?<br>
    Filter `install` string to find which application was install.<br>
    ```shell
    # grep " install " /var/log/dpkg.log
    2024-09-11 05:18:12 install linux-modules-5.15.0-1068-aws:amd64 <none> 5.15.0-1068.74~20.04.1
    2024-09-11 05:18:19 install linux-image-5.15.0-1068-aws:amd64 <none> 5.15.0-1068.74~20.04.1
    2024-09-11 05:18:20 install linux-aws-5.15-headers-5.15.0-1068:all <none> 5.15.0-1068.74~20.04.1
    2024-09-11 05:18:28 install linux-headers-5.15.0-1068-aws:amd64 <none> 5.15.0-1068.74~20.04.1
    2024-09-11 21:44:47 install osquery:amd64 <none> 5.13.1-1.linux
    2024-09-17 06:31:29 install linux-modules-5.15.0-1069-aws:amd64 <none> 5.15.0-1069.75~20.04.1
    2024-09-17 06:31:32 install linux-image-5.15.0-1069-aws:amd64 <none> 5.15.0-1069.75~20.04.1
    2024-09-17 06:31:32 install linux-aws-5.15-headers-5.15.0-1069:all <none> 5.15.0-1069.75~20.04.1
    2024-09-17 06:31:35 install linux-headers-5.15.0-1069-aws:amd64 <none> 5.15.0-1069.75~20.04.1
    2024-09-17 10:45:50 install c2comm:all <none> 1.0
    2024-10-06 13:56:08 install malicious-package:all <none> 1.0
    ```
    **Answer:** c2comm

### Task 7 - Linux Logs
1. Examine the auth.log files. Which user attempted to connect with SSH on 11th Sept 2024?<br>
    Read file `auth.log.1`.<br>
    ```shell
    # cat auth.log.1 | grep ssh
    Sep 11 04:53:27 tryhackme sshd[875]: Server listening on 0.0.0.0 port 22.
    Sep 11 04:53:27 tryhackme sshd[875]: Server listening on :: port 22.
    Sep 11 22:04:34 tryhackme sshd[53291]: Accepted publickey for ubuntu from 10.11.75.247 port 58633 ssh2: RSA SHA256:ecXxIHdpi9cpIPbjewybKqpDqrM1bw/OlKeuDT6rmzc
    Sep 11 22:04:34 tryhackme sshd[53291]: pam_unix(sshd:session): session opened for user ubuntu by (uid=0)
    Sep 11 22:04:50 tryhackme sshd[53407]: Connection reset by authenticating user ubuntu 10.11.75.247 port 58641 [preauth]
    Sep 11 22:05:04 tryhackme sshd[53409]: Invalid user saqib from 10.11.75.247 port 58647
    Sep 11 22:05:04 tryhackme sshd[53409]: Connection reset by invalid user saqib 10.11.75.247 port 58647 [preauth]
    Sep 11 22:06:51 tryhackme sudo:     root : TTY=pts/5 ; PWD=/tmp ; USER=root ; COMMAND=/usr/sbin/service ssh restart
    Sep 11 22:06:51 tryhackme sshd[875]: Received signal 15; terminating.
    Sep 11 22:06:51 tryhackme sshd[53438]: Server listening on 0.0.0.0 port 22.
    Sep 11 22:06:51 tryhackme sshd[53438]: Server listening on :: port 22.
    Sep 11 22:08:25 tryhackme sshd[53440]: Connection reset by authenticating user ubuntu 10.11.75.247 port 58757 [preauth]
    Sep 11 23:11:38 tryhackme sshd[53291]: pam_unix(sshd:session): session closed for user ubuntu
    Sep 16 10:58:53 tryhackme sshd[862]: Server listening on 0.0.0.0 port 22.
    Sep 16 10:58:53 tryhackme sshd[862]: Server listening on :: port 22.
    ```
    **Answer:** saqib

1. From which IP was this failed SSH connection attempt made?<br>
    **Answer:** 10.11.75.247
