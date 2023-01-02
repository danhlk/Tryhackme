> # Intro to Endpoint Security

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 2 - Endpoint Security Fundamentals](#task-2---endpoint-security-fundamentals)
    - [Task 3 - Endpoint Logging and Monitoring](#task-3---endpoint-logging-and-monitoring)
    - [Task 4 - Endpoint Log Analysis](#task-4---endpoint-log-analysis)

<!-- /TOC -->

## Task 2 - Endpoint Security Fundamentals
1. What is the normal parent process of services.exe?<br>
    > wininit.exe > services.exe

    **Answer:** wininit.exe

1. What is the name of the network utility tool introduced in this task?<br>
    **Answer:** TCPView

## Task 3 - Endpoint Logging and Monitoring
* Windows Event Logs

    The Windows Event Logs are not text files that can be viewed using a text editor. However, the raw data can be translated into XML using the Windows API. The events in these log files are stored in a proprietary binary format with a .evt or .evtx extension. The log files with the .evtx file extension typically reside in **C:\Windows\System32\winevt\Logs**.

    There are three main ways of accessing these event logs within a Windows system:<br>
    * Event Viewer (GUI-based application)
    * Wevtutil.exe (command-line tool)
    * Get-WinEvent (PowerShell cmdlet)

* Sysmon
* Osquery
* Wazuh

1. What is the PowerShell cmdlet for viewing Windows Event Logs?<br>
    > Get-WinEvent (PowerShell cmdlet)

    **Answer:** Get-WinEvent

1. Provide the command used to enter OSQuery CLI.<br>
    **Answer:** osqueryi

1. What does EDR mean? Provide the answer in lowercase.<br>
    **Answer:** endpoint detection and response

## Task 4 - Endpoint Log Analysis
1. Provide the flag for the simulated investigation activity.<br>
    The Baseline Document doesn't contain beacon.exe -> it's the abnormal running process.<br>
    **Answer:** THM{3ndp01nt_s3cur1ty!}

