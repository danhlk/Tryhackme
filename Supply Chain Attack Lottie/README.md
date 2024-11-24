> # Supply Chain Attack: Lottie

## Summary
- [Summary](#summary)
  - [Task 2 - Lottie Player Supply Chain Attack](#task-2---lottie-player-supply-chain-attack)
  - [Task 3 - How to Exploit](#task-3---how-to-exploit)

### Task 2 - Lottie Player Supply Chain Attack
1. Which of the following Lottie Player's versions were vulnerable? Enter the correct option only.<br>
    > The malicious versions of the Lottie Player package were: 2.05, 2.06, 2.07

    **Answer:** a

1. What is the name of the C2 server used for registration/authentication? Use the Redacted/defanged version.<br>
    > The vulnerable code makes a web socket connection to a domain castleservices01[.]com

    **Answer:** castleservices01[.]com

### Task 3 - How to Exploit
1. Per the above code, what updated package version does the attacker push on npm.thm?<br>
    >  open the package.json file and update the version from 1.0.0 to 1.1.0 to reflect the new (malicious) version

    **Answer:** 1.1.0

1. What is the port number on which the attacker will receive data?<br>
    **Answer:** 9090

1. What is the name of the form validator JS file being fetched from the CDN?<br>
    **Answer:** form-validator.bundle

1. What is the flag value after uploading the package v1.1.0? Visit the flag validator page to get the flag.<br>
    **Answer:** THM{MALICIOUS_PACKAGE_UPLOADED007}
