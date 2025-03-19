> # Snyk Code

## Summary
- [Summary](#summary)
  - [Task 3 - Understanding Code Security Risks](#task-3---understanding-code-security-risks)
  - [Task 4 - Getting Started With Snyk Code](#task-4---getting-started-with-snyk-code)
  - [Task 5 - Diving Deeper Into Vulnerabilities](#task-5---diving-deeper-into-vulnerabilities)
  - [Task 6 - Remediating Vulnerabilities](#task-6---remediating-vulnerabilities)
  - [Task 7 - CI/CD Pipelines Automation \& Continuous Monitoring](#task-7---cicd-pipelines-automation--continuous-monitoring)
  - [Task 8 - Establishing Best Practices](#task-8---establishing-best-practices)

### Task 3 - Understanding Code Security Risks
1. How many dependencies do we have for this new feature?<br>
    ```
    "dependencies": {
      "express": "^4.17.1",
      "body-parser": "^1.19.0",
      "sqlite3": "^5.0.2"
    }
    ```
    **Answer:** 3

### Task 4 - Getting Started With Snyk Code
1. How many vulnerabilities are flagged on the search-feature.js file?<br>
    **Answer:** 4

1. How many high-severity vulnerabilities are flagged on the search-feature.js file?<br>
    **Answer:** 2

1. What are the two medium-severity vulnerabilities flagged on the search-feature.js file? (in alphabetical order)<br>
    **Answer:** Cross-Site Request Forgery, Information Exposure

### Task 5 - Diving Deeper Into Vulnerabilities
1. What is the CWE for Cross-site Scripting?<br>
    **Answer:** CWE-79

1. What is the CWE for SQL injection?<br>
    **Answer:** CWE-89

1. What is the unsanitised user input in the chat-controller.js file?<br>
    **Answer:** searchTerm

### Task 6 - Remediating Vulnerabilities
1. What is the new vulnerability introduced with the XSS fix?<br>
    **Answer:** Allocation of resources without limits or throttling

1. Which Express method is used to fix the XSS vulnerability in the code snippet?<br>
    **Answer:** res.render

1. What is the updated code in the code snippet using to fix the SQL injection?<br>
    **Answer:** parameterised queries

### Task 7 - CI/CD Pipelines Automation & Continuous Monitoring
1. Establishing sensible alert thresholds in continuous monitoring practices involves considering the severity, frequency, and rate of change of vulnerabilities. (y/n)<br>
    **Answer:** y

### Task 8 - Establishing Best Practices
1. Which OWASP framework serves as a guide to strengthen the relationship between development teams and information security teams?<br>
    **Answer:** OWASP Security Champion Playbook
