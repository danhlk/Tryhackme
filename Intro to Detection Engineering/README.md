> # Intro to Detection Engineering

# Summary
<!-- TOC -->

- [Summary](#summary)
    - [Task 2 - What is Detection Engineering?](#task-2---what-is-detection-engineering)
    - [Task 4 - Detection Engineering Frameworks 1](#task-4---detection-engineering-frameworks-1)
    - [Task 6 - Detection Detective](#task-6---detection-detective)

<!-- /TOC -->

## Task 2 - What is Detection Engineering?
1. Which detection type focuses on misalignments within the current infrastructure?<br>
    > Configuration Detection Under this detection, we use current knowledge of the known environment and infrastructure to identify misalignments.
    **Answer:** Configuration

1. Which detection approach involves building an asset or activity baseline profile for detection?<br>
    > Modelling: The approach involves building an asset or activity profile that includes baseline events, time and data threshold.
    **Answer:** Modelling

1. Which type of detection integrates with defensive playbooks?<br>
    > Threat Behaviour Detection: Analysts will look at an adversary’s Tactics, Techniques and Procedures (TTPs) to conduct an attack, regardless of any specific indicators. This makes detection more scalable beyond indicators.
    **Answer:** Threat Behaviour

## Task 4 - Detection Engineering Frameworks 1
1. Which framework looks at how to make it difficult for an adversary to change their approach when detected?<br>
    > Pyramid of Pain: This is a well-known framework in the industry and is mainly used to showcase the pain for the adversary; if the defenders detect their TTPs, then how difficult and/or costly it would be for the adversary to change their TTPs.
    **Answer:** Pyramid of Pain

1. What is the improved Cyber Kill Chain framework called?<br>
    > The Unified Kill Chain was developed to complement the Cyber Kill Chain by combining it with other frameworks, such as the MITRE ATT&CK framework.
    **Answer:** Unified Kill Chain

1. How many phases are in the improved kill chain?<br>
    ![](https://tryhackme-images.s3.amazonaws.com/user-uploads/5fc2847e1bbebc03aa89fbf2/room-content/f78d42cc461eaebffd806666646f6cbb.png)<br>
    **Answer:** 18

## Task 6 - Detection Detective
1. What is the flag?<br>
    Q1: Account Manipulation<br>
    Q2: Collect AD log for group change.<br>
    Q3: Modified group<br>
    **Answer:** THM{Sup3r-D3t3ct1v3}
