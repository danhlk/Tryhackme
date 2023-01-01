> # Traffic Analysis Essentials

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 2 - Network Security and Network Data](#task-2---network-security-and-network-data)
    - [Task 3 - Traffic Analysis](#task-3---traffic-analysis)

<!-- /TOC -->


## Task 2 - Network Security and Network Data
1. Which Security Control Level covers contain creating security policies?<br>
    > Administrative: Administrative security controls provide consistency in security operations like creating policies, access levels and authentication processes.

    **Answer:** Administrative

1. Which Access Control element works with data metrics to manage data flow?<br>
    > Load Balancing: Controls the resource usage to distribute (based on metrics) tasks over a set of resources and improve overall data processing flow.

    **Answer:** Load Balancing

1. Which technology helps correlate different tool outputs and data sources?<br>
    > Security Orchestration Automation and Response (SOAR): Technology that helps coordinate and automates tasks between various people, tools, and data within a single platform to identify anomalies, threats, and vulnerabilities. It also supports vulnerability management, incident response, and security operations.

    **Answer:** SOAR

## Task 3 - Traffic Analysis
1. Level-1 is simulating the identification and filtering of malicious IP addresses. What is the flag?<br>
    The two suspicious IP address are 10.10.99.99, 10.10.99.62. After add them to filter, the server will not compromised anymore.<br>
    **Answer:** THM{PACKET_MASTER}

1. Level-2 is simulating the identification and filtering of malicious IP and Port addresses. What is the flag?<br>
    The three suspicious ports are 2222 (10.10.99.74), 4444 and 7777. After block them, the server will not be sompromised anymore.<br>
    **Answer:** THM{DETECTION_MASTER}

