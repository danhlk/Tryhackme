> # MAL: Researching - Tryhackme

# Summary
* [Task 3 - Checksums 101](#task-3---checksums-101)
* [Task 4 - Online Sandboxing](#task-4---online-sandboxing)
* [Task 5 - Practical: Calculating &amp; Reporting Checksums](#task-5---practical-calculating--reporting-checksums)
* [Task 6 - VirusTotal](#task-6---virustotal)
* [Task 7 - Future Reading (References)](#task-7---future-reading-references)

## Task 3 - Checksums 101
1. Name the term for an individual piece of binary<br>
    > Ultimately, the markup of data on a computer system is binary, merely ones and zeros where each value is a "bit".

    **Answer:** bit

1. What are checksums also known as?<br>
    > These checksums are also commonly referred to as "hashes".

    **Answer:** hashes

1. Name the algorithm that is next in the series after SHA-256<br>
    > In the real-world, some popular algorithms are SHA1, SHA-256 and in some cases, SHA-512, 

    **Answer:** SHA-512

1. According to this task, how long will you need to hash 6 million files before a MD5 hash collision occurs?<br>
    > the hashing algorithm MD5 which is a famous recent example will need 6 billion files to be hashed per second - for 100 years on average

    **Answer:** 100 years

1. Who developed the MD5 algorithm?<br>
    Search on Google.<br>
    **Answer:** Ronald Rivest

## Task 4 - Online Sandboxing
1. Name the key term for the type of malware that Emotet is classified as<br>
    ![](images/1.png)<br>
    **Answer:** trojan

1. Research time! What type of emails does Emotet use as its payload?<br>
    From the image above.<br>
    **Answer:** spam email

1. Begin analysing the report, what is the timestamp of when the analysis was made?<br>
    Based on analysis date field.<br>
    **Answer:** 9/16/2019, 13:54:48

1. Name the file that is detected as a "Network Trojan"<br>
    Search string `network trojan`, you will see the answer.<br>
    **Answer:** easywindow.exe

1. What is the PID of the first HTTP GET request?<br>
    In network activity.<br>
    ![](images/3.png)<br>
    **Answer:** 2748

1. What is the only DNS request that is made after the sample is executed?<br>
    In DNS requests panel.<br>
    ![](images/4.png)<br>
    **Answer:** blockchainjoblist.com

## Task 5 - Practical: Calculating & Reporting Checksums
1. Using the HashTab tool, what is the MD5 checksum for "LoginForm.exe"?<br>
    ![](images/5.png)<br>
    **Answer:** FF395A6D528DC5724BCDE9C844A0EE89
1. Using Get-FileHash in Powershell, retrieve the SHA256 of "TryHackMe.exe"
    ![](images/6.png)<br>
    **Answer:** 6F870C80361062E8631282D31A16872835F7962222457730BC55676A61AD1EE0

1. What would be the syntax to retrieve the SHA256 checksum of "TryHackMe.exe" using CertUtil in Powershell?<br>
    ![](images/7.png)<br>
    **Answer:** CertUtil -hashfile .\TryHackMe.exe SHA256

## Task 6 - VirusTotal
1. Navigate to the "Details" tab, what is the other filename and extension reported as present?<br>
    In Names panel.<br>
    ![](images/8.png)<br>
    **Answer:** HxD.exe

1. In the same "Details" tab, what is the reported compilation timestamp?<br>
    In Portable Executable Info.<br>
    ![](images/9.png)<br>
    **Answer:** 2020-02-28 11:16:36

1. What is the THM{} formatted flag on the report?<br>
    In Comminity tab, you will see a comment with the THM flag.<br>
    ![](images/10.png)<br>
    **Answer:** THM{TryHackMe_Malware_Series_Research_Flag}

## Task 7 - Future Reading (References)
<p align="center">

Cryptography and Checksums:<br>
[A Meaningful MD5 Hash Collision Attack](https://scholarworks.sjsu.edu/cgi/viewcontent.cgi?referer=https://www.google.com/&httpsredir=1&article=1020&context=etd_projects) - (Narayana D. Kashyap., 2008)<br>
[Cryptography & Network Security](https://dl.acm.org/doi/book/10.5555/1209579) - (Behrouz A. Forozuan., 2007)<br>
[The first collision for full SHA-1](https://shattered.io/static/shattered.pdf) - (Stevens et al., 2017) / (Shattered.io)<br>

Blog (Selfless Promo)

[So you want to analyse malware?](https://blog.cmnatic.co.uk/posts/so-you-want-to-analyse-malware/<br>)

Sandboxing Engines:

[any.run](https://any.run/)

[hybrid-analysis](https://hybrid-analysis.com/)
</p>
