> # KQL (Kusto): Introduction

## Summary
- [Summary](#summary)
  - [Task 2 - Overview of Microsoft Sentinel](#task-2---overview-of-microsoft-sentinel)
  - [Task 3 - What Is KQL](#task-3---what-is-kql)
  - [Task 4 - KQL Concepts in Microsoft Sentinel](#task-4---kql-concepts-in-microsoft-sentinel)
  - [Task 5 - KQL Statement Structure](#task-5---kql-statement-structure)
  - [Task 6 - KQL Use Cases](#task-6---kql-use-cases)

### Task 2 - Overview of Microsoft Sentinel
1. In addition to being a SIEM solution, what else is Microsoft Sentinel? (use the abbreviation)<br>
    > It is not only a SIEM solution but also a Security Orchestration, Automation, and Response (SOAR) solution, which enhances its capabilities.

    **Answer:** SOAR

1. How does MS Sentinel support other security solutions that are not included in the built-in connectors? <br>
    > REST API integration: Sentinel offers an alternative through REST API for security solutions not covered by built-in connectors. This option may require experts to build custom scripts or applications that interact with the service API to transmit data to Microsoft Sentinel

    **Answer:** REST API integration

### Task 3 - What Is KQL
1. What initial service was KQL created for?<br>
    >  It was initially created for Azure Data Explorer but has been widely adopted across various Microsoft services

    **Answer:** Azure Data Explorer

1. Analyze the example query from the task. How many computers will the query return?<br>
    ```KQL
    Heartbeat
    | summarize AggregatedHeartbeatCount = count() by Computer
    | order by AggregatedHeartbeatCount desc
    | take 10
    ```
    **Answer:** 10

1. What table is the example query retrieving its data from?<br>
    **Answer:** Heartbeat

### Task 4 - KQL Concepts in Microsoft Sentinel
1. What operator can be used to output results in graphical form?<br>
    > render: Renders results as a graphical output	SecurityEvent | render timechart 

    **Answer:** render

1. What operator can be used to filter a specified table based on specified conditions?<br>
    > where: Filters the specified table based on specified conditions

    **Answer:** where

1. What user account name was queried in our second example query above?<br>
    **Answer:** JBOX00$

### Task 5 - KQL Statement Structure
1. What is the name of the table queried in our example query?<br>
    ```
    SecurityEvent
    | where TimeGenerated > ago(1d)
    | summarize EventCount = count() by Computer
    | order by EventCount desc | limit 10 
    ```
    **Answer:** SecurityEvent

1. Analyze the example query from the task. What does the query aggregate per computer?<br>
    **Answer:** EventCount 

### Task 6 - KQL Use Cases
1. What are we searching for in the SecurityEvent table on the first query?<br>
    > To identify failed login attempts, you can search the SecurityEvent table for failed login attempts using the query below.

    **Answer:** EventID

1. Which operator was used on the second query to streamline our search to a range of user accounts from the TargetUserName column? <br>
    **Answer:** Contains

