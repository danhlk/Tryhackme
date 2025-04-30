> # Breaking Crypto the Simple Way

## Summary
- [Summary](#summary)
  - [Task 2 - Brute-forcing Keys](#task-2---brute-forcing-keys)
  - [Task 3 - Breaking Hashes](#task-3---breaking-hashes)
  - [Task 4 - Exposed Keys](#task-4---exposed-keys)
  - [Task 5 - Bit Flipping Attacks](#task-5---bit-flipping-attacks)

### Task 2 - Brute-forcing Keys
1. What is the flag?<br>
    Use FactorDB to fractor n to q and q.<br>
    ```python
    from sympy import factorint
    from Crypto.Util.number import inverse, long_to_bytes

    # Given values
    n = 43941819371451617899582143885098799360907134939870946637129466519309346255747

    # Factor n
    p = 205237461320000835821812139013267110933
    q = 214102333408513040694153189550512987959
    print("Prime factors:")
    print("p =", p)
    print("q =", q)
    phi_n = (p - 1)*(q - 1)
    e = 65537
    d = inverse(e, phi_n)
    print("Private key (d):", d)

    c = 9002431156311360251224219512084136121048022631163334079215596223698721862766

    plaintext = pow(c, d, n)
    flag = long_to_bytes(plaintext)
    print(flag.decode())
    print("Decrypted Plaintext:", flag)
    ```
    The run the above script.<br>
    ```
    $ python task2.py
    Prime factors:
    p = 205237461320000835821812139013267110933
    q = 214102333408513040694153189550512987959
    Private key (d): 42863673506531127160266519316271436658935017712647978759376543290403486562425
    THM{Psssss_4nd_Qsssssss}
    Decrypted Plaintext: b'THM{Psssss_4nd_Qsssssss}'
    ```
    **Asnwer:** THM{Psssss_4nd_Qsssssss}

### Task 3 - Breaking Hashes
1. What is the secret used to encrypt the message?<br>
    ```
    $ hashcat -m 150 digest.txt rockyou.txt 
    hashcat (v6.2.6) starting

    OpenCL API (OpenCL 3.0 PoCL 5.0+debian  Linux, None+Asserts, RELOC, SPIR, LLVM 16.0.6, SLEEF, DISTRO, POCL_DEBUG) - Platform #1 [The pocl project]
    ==================================================================================================================================================    

    Minimum password length supported by kernel: 0
    Maximum password length supported by kernel: 256

    Hashes: 1 digests; 1 unique digests, 1 unique salts
    Bitmaps: 16 bits, 65536 entries, 0x0000ffff mask, 262144 bytes, 5/13 rotates
    Rules: 1

    Optimizers applied:
    * Zero-Byte
    * Not-Iterated
    * Single-Hash
    * Single-Salt

    ATTENTION! Pure (unoptimized) backend kernels selected.
    Pure kernels can crack longer passwords, but drastically reduce performance.
    If you want to switch to optimized kernels, append -O to your commandline.
    See the above message to find out about the exact limits.

    Watchdog: Hardware monitoring interface not found on your system.
    Watchdog: Temperature abort trigger disabled.

    Host memory required for this attack: 1 MB

    Dictionary cache built:
    * Filename..: rockyou.txt
    * Passwords.: 14344391
    * Bytes.....: 139921497
    * Keyspace..: 14344384
    * Runtime...: 3 secs

    1484c3a5d65a55d70984b4d10b1884bda8876c1d:CanYouGuessMySecret:sunshine
                                                            
    Session..........: hashcat
    Status...........: Cracked
    Hash.Mode........: 150 (HMAC-SHA1 (key = $pass))
    Hash.Target......: 1484c3a5d65a55d70984b4d10b1884bda8876c1d:CanYouGues...Secret
    Time.Started.....: Wed Apr 30 04:16:30 2025 (0 secs)
    Time.Estimated...: Wed Apr 30 04:16:30 2025 (0 secs)
    Kernel.Feature...: Pure Kernel
    Guess.Base.......: File (rockyou.txt)
    Guess.Queue......: 1/1 (100.00%)
    Speed.#1.........:    55912 H/s (1.56ms) @ Accel:1024 Loops:1 Thr:1 Vec:8
    Recovered........: 1/1 (100.00%) Digests (total), 1/1 (100.00%) Digests (new)
    Progress.........: 4096/14344384 (0.03%)
    Rejected.........: 0/4096 (0.00%)
    Restore.Point....: 0/14344384 (0.00%)
    Restore.Sub.#1...: Salt:0 Amplifier:0-1 Iteration:0-1
    Candidate.Engine.: Device Generator
    Candidates.#1....: 123456 -> oooooo

    Started: Wed Apr 30 04:15:46 2025
    Stopped: Wed Apr 30 04:16:31 2025
    ```
    **Answer:** sunshine

### Task 4 - Exposed Keys
1. What is the flag?<br>
    ```
    [SNIP]
    Trying: ankhzljjgu
    Response: Access granted! Here's your flag: THM{3nD_2_3nd_is_n0t_c0mpl1c4ted}
    [+] Found the correct message: ankhzljjgu
    ```
    **Answer:** THM{3nD_2_3nd_is_n0t_c0mpl1c4ted}

### Task 5 - Bit Flipping Attacks
1. What is the flag?<br>
    Login as Guest, the use role's cookie to calculate admin's role.<br>
    **Answer:** THM{flip_n_flip}
