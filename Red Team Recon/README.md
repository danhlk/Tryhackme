> # Red Team Recon - Tryhackme

# Summary
* [Task 3 - Built-in Tools](#task-3---built-in-tools)
* [Task 4 - Advanced Searching](#task-4---advanced-searching)
* [Task 5 - Specialized Search Engines](#task-5---specialized-search-engines)
* [Task 6 - Recon-ng](#task-6---recon-ng)
* [Task 7 - Maltego](#task-7---maltego)

## Task 3 - Built-in Tools
1. When was thmredteam.com created (registered)? (YYYY-MM-DD)<br>
    ![](images/1.png)<br>
    **Answer:** 2021-09-24

1. To how many IPv4 addresses does clinic.thmredteam.com resolve?<br>
    ![](images/2.png)<br>
    **Answer:** 2

1. To how many IPv6 addresses does clinic.thmredteam.com resolve?<br>
    From image above.<br>
    **Answer:** 2

## Task 4 - Advanced Searching
1. How would you search using Google for xls indexed for http://clinic.thmredteam.com?<br>
    Use `filetypr` and `site` attribute.<br>
    **Answer:** filetype:xls site:clinic.thmredteam.com

1. How would you search using Google for files with the word passwords for http://clinic.thmredteam.com?<br>
    **Answer:** passwords site:clinic.thmredteam.com

## Task 5 - Specialized Search Engines
[ViewDNS.info](ViewDNS.info)<br>
[https://threatintelligenceplatform.com/](https://threatintelligenceplatform.com/)<br>
[https://search.censys.io](https://search.censys.io/)<br>
[https://www.shodan.io/](https://www.shodan.io/)<br>

1. What is the shodan command to get your Internet-facing IP address?<br>
    Print `shodan -h`, you will see flag `myip` will `Print your external IP address`.<br>
    **Answer:** shodan myip

## Task 6 - Recon-ng
1. How do you start recon-ng with the workspace clinicredteam?<br>
    **Answer:** recon-ng -w clinicredteam

1. How many modules with the name virustotal exist?<br>
    Search on marketplace, you will see two modules with the name `virustotal`.<br>
    ![](images/3.png)<br>
    **Answer:** 2

1. There is a single module under hosts-domains. What is its name?<rb>
    Search for `hosts-domains`, you will only one module appear.<br>
    ![](images/4.png)<br>
    **Answer:** migrate_hosts

1. censys_email_address is a module that “retrieves email addresses from the TLS certificates for a company.” Who is the author?<br>
    Display info of this module.<br>
    ![](images/5.png)<br>
    **Answer:** Censys Team

## Task 7 - Maltego
1. What is the name of the transform that queries NIST’s National Vulnerability Database?<br>
    Search on Maltego Tranform Hub.<br>
    ![](images/6.png)<br>
    **Answer:** NIST NVD

1. What is the name of the project that offers a transform based on ATT&CK?<br>
    ![](images/7.png)<br>
    **Answer:** MISP Project

