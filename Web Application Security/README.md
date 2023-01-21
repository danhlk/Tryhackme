> # Web Application Security

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 1 - Introduction](#task-1---introduction)
    - [Task 2 - Web Application Security Risks](#task-2---web-application-security-risks)
    - [Task 3 - Practical Example of Web Application Security](#task-3---practical-example-of-web-application-security)

<!-- /TOC -->

## Task 1 - Introduction
* The following are some examples of web applications:

    * Webmail such as Tutanota, Protonmail, Outlook, and Gmail
    * Online office suites such as Microsoft Office 365 (Word, Excel, and PowerPoint), Google Drive (Docs, Sheets, and Slides), and Zoho Office (Writer, Sheet, and Show)
    * Online shopping such as Amazon.com, AliExpress, and Etsy
1. What do you need to access a web application?
    **Answer:** browser

## Task 2 - Web Application Security Risks
1. You discovered that the login page allows an unlimited number of login attempts without trying to slow down the user or lock the account. What is the category of this security risk?<br>
    **Answer:** Identification and Authentication Failure
    
1. You noticed that the username and password are sent in cleartext without encryption. What is the category of this security risk?<br>
    **Answer:** Cryptographic Failures

## Task 3 - Practical Example of Web Application Security
1. Check the other users to discover which user account was used to make the malicious changes and revert them. After reverting the changes, what is the flag that you have received?<br>
    Click on tab *Your Activity*, change user_id to 9, we'll see the information of Database Administrator, revert all of them and retrieve the flag.<br>
    **Answer:** THM{IDOR_EXPLORED}
