> # Bulletproof Penguin

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 2 - Redis Server No Password](#task-2---redis-server-no-password)
    - [Task 3 - Report Default Community Names of the SNMP Agent](#task-3---report-default-community-names-of-the-snmp-agent)
    - [Task 4 - Nginx running as root](#task-4---nginx-running-as-root)
    - [Task 5 - Cleartext Protocols](#task-5---cleartext-protocols)
    - [Task 6 - Weak SSH Crypto](#task-6---weak-ssh-crypto)
    - [Task 7 - Anonymous FTP Login Reporting](#task-7---anonymous-ftp-login-reporting)
    - [Task 8 - Weak Passwords](#task-8---weak-passwords)
    - [Task 9 - Review Sudo Permissions](#task-9---review-sudo-permissions)
    - [Task 10 - Exposed Database Ports](#task-10---exposed-database-ports)

<!-- /TOC -->

## Task 2 - Redis Server No Password
1. What is the redis_nopass flag?<br>
    > Set a password using the "requirepass" directive in "redis.conf".

    The `redis.con` is in `/etc/redis`, uncomment this line to set password to redis.<br>
    ```
    $ sudo cat /etc/redis/redis.conf | grep requirepass -n
    288:# If the master is password protected (using the "requirepass" configuration
    507:# requirepass foobared
    ```
    Use `sed` command to edit `redis.conf`.
    ```
    $ sudo sed -i "s/# requirepass foobared/requirepass foobared/" /etc/redis/redis.conf
    $ sudo cat /etc/redis/redis.conf | grep requirepass -n
    288:# If the master is password protected (using the "requirepass" configuration
    507:requirepass foobared
    ```
    Then run `get-flags` to get `redis_nopass` flag.<br>
    **Answer:** THM{ae4e5bb7aac2c2252363ca466f10ffd0}

## Task 3 - Report Default Community Names of the SNMP Agent
1. What is the snmp_public flag?<br>
    Change the `rocommunity public` in `/etc/snmp/snmpd.conf` to another name.<br>
    **Answer:** THM{aa397a808d527fd71f023c78d3c04591}

## Task 4 - Nginx running as root
1. What is the nginx_asroot flag?<br>
    Change the `user` field in `/etc/nginx/nginx.conf` to `www-data`, then restart nginx service with `sudo service nginx restart`, you can run `sudo ps aux | grep nginx` to see the user running nginx's service.<br>\
    ```
    # ps aux| grep nginx
    root        1506  0.0  0.1  51216  1496 ?        Ss   04:25   0:00 nginx: master process /usr/sbin/nginx -g daemon on; master_process on;
    www-data    1507  0.0  0.5  51780  5204 ?        S    04:25   0:00 nginx: worker process
    www-data    1508  0.0  0.5  51780  5204 ?        S    04:25   0:00 nginx: worker process
    root        1515  0.0  0.0   8168   660 pts/0    S+   04:25   0:00 grep --color=auto nginx
    ```
    **Answer:** THM{bebb02b22bb56b2f79ba706975714ee2}

## Task 5 - Cleartext Protocols
1. What other cleartext service is running on port 69/udp?<br>
    Use `lsof` to get the service name listening on port 69/udp.<br>
    ```
    # lsof -i :69
    COMMAND PID USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
    inetd   481 root    8u  IPv4  20045      0t0  UDP *:tftp
    ```
    **Answer:** tftp

1. What is the cleartext_services flag?<br>
    Both `telnet` and `tftp` is running under `inetd` service, use `systemctl stop inetd` to take down all services.<br>
    Re-run `lsof -i`, you will not see `telnet` and `tftp` anymore.<br>
    **Answer:** THM{33704d74ec53c8cf50daf817bea836a1}"

## Task 6 - Weak SSH Crypto
1. What is the ssh_weak_macs flag?<br>
    Remove `hmac-md5-96` in `/etc/ssh/sshd_config`.<br>
    ```
    # grep "hmac-md5-96" /etc/ssh/sshd_config -n
    113:macs umac-64-etm@openssh.com,umac-128-etm@openssh.com,hmac-sha2-256-etm@openssh.com,hmac-sha2-512-etm@openssh.com,  hmac-sha1-etm@openssh.com,umac-64@openssh.com,umac-128@openssh.com,hmac-md5-96,hmac-sha2-256,hmac-sha2-512,hmac-sha1
    ```
    **Answer:** THM{e3d6b82f291b64f95213583dcd89b659}

1. What is the ssh_weak_kex flag?<br>
    Use `grep` to find which file contains 
    ```
    # grep "diffie-hellman-group1-sha1" /etc/ssh/* -n
    grep: /etc/ssh/ssh_config.d: Is a directory
    /etc/ssh/sshd_config:114:kexalgorithms curve25519-sha256,curve25519-sha256@libssh.org,ecdh-sha2-nistp256,ecdh-sha2-nistp384,ecdh-sha2-nistp521,diffie-hellman-group-exchange-sha256,diffie-hellman-group16-sha512,diffie-hellman-group18-sha512,diffie-hellman-group14-sha256,diffie-hellman-group1-sha1
    ```
    Remove `diffie-hellman-group1-sha1`, the reload config of sshd service with `systemctl reload sshd`.<br>
    **Answer:** THM{d9baf598ee934d79346f425a81bd693a}

1. What is the ssh_weak_ciphers flag?<br>
    Remove three string in `sshd_config`.<br>
    **Answer:** THM{9ff9c182cad601291d45951c01d0b2c7}

## Task 7 - Anonymous FTP Login Reporting
1. What is the anon_ftp flag?<br>
    Change `anonymous_enable=YES` to `anonymous_enable=NO` in `/etc/vsftpd.conf`.<br>
    **Answer:** THM{f20b5ff5a3d4c779e99c3a93d1f68c6d}

## Task 8 - Weak Passwords
1. What is the change_pass flag?<br>
    Use `passwd <username>` to change password for specify user.<br>
    ```
    # passwd mary
    New password: 
    Retype new password: 
    passwd: password updated successfully
    root@ip-10-10-147-108:/home/thm# passwd munra
    New password: 
    Retype new password:
    ```
    **Answer:** THM{be74a521c3982298d2e9b0e347a3807d}

1. What is the unused_accounts flag?<br>
    Use `userdel <username>` to remove the unused account.<br>
    **Answer:** THM{1b354db0e71f75057abe69de26a637ab}

## Task 9 - Review Sudo Permissions
1. What is the sudoers_munra flag?<br>
    Remove `munra` account in `/et/sudoers` file.<br>
    **Answer:** THM{1e9ee13fb42fea2a9eb2730c51448241}

1. What is the sudoers_mary flag?<br>
    Add line `mary ALL=(root)NOPASSWD:/usr/bin/ss` to `/etc/sudoers`.<br>
    **Answer:** THM{a0bcb9b72fd26d0ad55cdcdcd21698f1}

## Task 10 - Exposed Database Ports
1. What is the mysql_port_public flag?<br>
    Port 3306 is listenning on `*:3306`, 
    ```
    # lsof -i :3306
    COMMAND PID  USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
    mysqld  790 mysql   23u  IPv4  25966      0t0  TCP *:mysql (LISTEN)
    ```
    Change from `bind-address        = 0.0.0.0` to `bind-address        = 127.0.0.1` in `/etc/mysql/mysql.conf.d/mysqld.cnf` file, then restart `mysql` service.<br>
    ```
    # lsof -i :3306
    COMMAND  PID  USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
    mysqld  3559 mysql   23u  IPv4 166608      0t0  TCP localhost:mysql (LISTEN)
    ```
    **Answer:** THM{526e33142b54e13bb47b17056823ab60}

1. What is the redis_port_public flag?<br>
    On this server, redis also listening on 0.0.0.0, change `bind 0.0.0.0` to `bind 127.0.0.1` in `/etc/redis/redis.conf` file.<br>
    ```
    # lsof -i :6379
    COMMAND   PID  USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
    redis-ser 774 redis    6u  IPv4  24345      0t0  TCP *:6379 (LISTEN)
    
    # grep "bind" /etc/redis/redis.conf -n
    48:# By default, if no "bind" configuration directive is specified, Redis listens
    51:# the "bind" configuration directive, followed by one or more IP addresses.
    55:# bind 192.168.1.100 10.0.0.1
    56:# bind 127.0.0.1 ::1
    59:# internet, binding to all the interfaces is dangerous and will expose the
    61:# following bind directive, that will force Redis to listen only into
    69:bind 0.0.0.0
    76:# 1) The server is not binding explicitly to a set of addresses using the
    77:#    "bind" directive.
    87:# are explicitly listed using the "bind" directive.
    ```
    Then restart `redis` service.<br>
    ```
    # systemctl restart redis
    # lsof -i :6379
    COMMAND    PID  USER   FD   TYPE DEVICE SIZE/OFF NODE NAME
    redis-ser 3637 redis    6u  IPv4 168178      0t0  TCP localhost:6379 (LISTEN)
    ```
    **Answer:** THM{20a809866dbcf94109189c5bafabc5c2}

