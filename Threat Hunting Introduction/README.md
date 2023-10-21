> # Threat Hunting: Introduction

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 2 - Core Concept](#task-2---core-concept)
    - [Task 3 - Threat Hunting Mindset](#task-3---threat-hunting-mindset)
    - [Task 4 - Threat Hunting Process](#task-4---threat-hunting-process)
    - [Task 5 - Practical Application](#task-5---practical-application)
    - [Task 6 - Goals](#task-6---goals)

<!-- /TOC -->

## Task 2 - Core Concept
1. What do you call the approach to finding cyber security threats where there's an active effort done to look for signs of malicious activity?<br>
    > Threat hunting is an approach to finding cyber security threats where there’s an active effort done to look for signs of malicious activity.

    **Answer:** Threat hunting

1. In this task, what are we contrasting threat hunting with?<br>
    > Threat Hunting in Contrast with Incident Response

    **Answer:** Incident Response

1. Incident response is innately reactive. What is done first thing when an initial notification or alert is received? (It is _______.)<br>
    > The action, or “response”, is triggered by an initial notification or alert. This initial notification is first triaged, 

    **Answer:** triaged

1. Threat hunting is innately proactive. What is it guided by?<br>
    > There is no actual ‘trigger’ that would mobilise a hunt, except for the pursuit of building the strength of the organisation’s security posture, guided by Threat Intelligence.

    **Answer:** Threat Intelligence

1. Threat Hunting and Incident Response are two different approaches that aim to ensure one specific goal is met. It is to strengthen the organisation's what?<br>
    > This classic example shows the beauty of the synergy between two seemingly different approaches to ensuring one specific goal is met - strengthening the organisation’s security posture.

    **Answer:** security posture

## Task 3 - Threat Hunting Mindset
1. What is the most obvious and straightforward example of a Unique Threat Intelligence?<br>
    > Indicators of Compromise (IOCs) specifically documented on previous intrusions would be the most obvious and straightforward example for this one. 

    **Answer:** Indicators of Compromise

## Task 4 - Threat Hunting Process
1. Malwares are constantly being used in the toolkits of threat actors. What is the live malware repository that we touched upon above?<br>
    > One example of a live malware repository is theZoo, where you may play with live malware and gain insights into how they will work within specific environmental conditions.

    **Answer:** theZoo

1. What example of Threat Intelligence blends well with environmental noise?<br>
    > Attack residues are a great starting point as well, especially if you think that an attack has happened already.

    **Answer:** Attack Residues

1. What should the organisation be extra vigilant in monitoring for announcements of?<br>
    > The organisation should be extra vigilant for announcements of zero-day vulnerabilities that may be affecting these assets. 

    **Answer:** Zero-day Vulnerabilities

1. Characterisation of the subject of the hunt into specific and actionable identifiers is imperative for the hunt's success How is it done most effectively?<br>
    > Upon identifying the subject of the hunt, it’s imperative that we ensure that we characterise them into specific and actionable identifiers by which we will immediately recognize. This is done most effectively via Attack Signatures and IOCs.

    **Answer:** Attack Signatures and IOCs

## Task 5 - Practical Application
1. Which tactic has the most techniques highlighted?<br>
    After merger three layers together, the `Discovery` has the most techniques hightlighted.<br>
    **Answer:** discovery

1. Which tactic has the most techniques highlighted?<br>
    All threats are use `Exploitation of Remote Services` to lateral movement.<br>
    **Answer:** Exploitation of Remote Services

1. What technique does WannaCry and Conficker have in common?<br>
    **Answer:** Inhibit System Recovery

1. What’s the score of techniques that Stuxnet and Conficker have in common?<br>
    **Answer:** 6

## Task 6 - Goals
1. What is the primary goal of Threat Hunting?<br>
    > The primary goal of Threat Hunting is to minimise a threat actor's dwell time.

    **Answer:** minimise a threat actor's dwell time

1. Upon profiling threats through our Threat Hunting efforts, what should these profiles be translated to?<br>
    > once we’ve profiled threats that were previously undetectable by current detection methods, an effort should immediately be poured into translating these profiles into detection mechanisms.

    **Answer:** detection mechanisms

