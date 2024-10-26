> # SSDLC

## Summary
- [Summary](#summary)
  - [Task 2 - What is SSDLC?](#task-2---what-is-ssdlc)
  - [Task 3 - Implementing SSDLC](#task-3---implementing-ssdlc)
  - [Task 4 - Risk Assessment](#task-4---risk-assessment)
  - [Task 5 - Threat Modelling](#task-5---threat-modelling)
  - [Task 6 - Secure Coding](#task-6---secure-coding)
  - [Task 7 - Security Assessments](#task-7---security-assessments)
  - [Task 8 - SSDLC Methodologies](#task-8---ssdlc-methodologies)
  - [Task 9 - Secure Space Lifecycle](#task-9---secure-space-lifecycle)

### Task 2 - What is SSDLC?
1. How much more does it cost to identify vulnerabilities during the testing phase?<br>
    ![](https://tryhackme-images.s3.amazonaws.com/user-uploads/5c549500924ec576f953d9fc/room-content/1688d77f6f6401862a668820ab96061e.png)<br>
    **Answer:** 15

### Task 3 - Implementing SSDLC
1. What should you understand before implementing Secure SDLC processes?<br>
    **AnsweR:** Security Posture

1. During which stages should you perform a Risk Assessment?<br>
    > Risk Assessment - during the early stages of SDLC, it is essential to identify security considerations that promote a security by design approach when functional requirements are gathered in the planning and requirements stages.

    **Answer:** Planning and Requirements

1. What should be carried out during the design phase?<br>
    ![](https://tryhackme-images.s3.amazonaws.com/user-uploads/61a7523c029d1c004fac97b3/room-content/3d1e3379a1e3ccc46f3c7471095cbfae.png)<br>
    **Answer:** Threat Modelling

### Task 4 - Risk Assessment
1. What is a formula to assign a Qualitative Risk level?<br>
    >  a typical formula to evaluate qualitative risk is: Risk = Severity x Likelihood
    **Answer:** Severity x Likelihood

1. Which type of Risk Assessment assigns numerical values to determine risk?<br>
    > The Quantitative Risk Assessment is used to measure risk with numerical values.

    **Answer:** Quantitative Risk Assessment  

### Task 5 - Threat Modelling
1. What threat modelling methodology assigns a rating system based on risk probability?<br>
    > It's a model that ranks threats by assigning identified threats according to their severity and priority. 

    **Answer:**  DREAD

1. What threat modelling methodology is built upon the CIA triad?<br>
    > STRIDE is built upon the CIA triad principle (Confidentiality, Integrity & Availability). 

    **Answer:** STRIDE

1. What threat modelling methodology helps align technical requirements with business objectives?<br>
    > PASTA's focus is to align technical requirements with business objectives. 

    **Answer:** PASTA

### Task 6 - Secure Coding
1.  Is it recommended to use SAST analysis at the beginning of the SDLC? (y/n)<br>
    > SAST can even help detect vulnerabilities in your application before the code is merged or integrated into the software if added as part of the SDLC development phase.

    **Answer:** Y

1. Which type of code analysis uses the black-box method?<br>
    > DAST means Dynamic Application Security Testing, a black-box testing method that finds vulnerabilities at runtime.

    **Answer:** DAST

1. Which type of code analysis uses the white-box method?<br>
    > SAST means Static Application Security Testing, a white box testing method that directly analyses the source code.

    **Answer:** SAST

### Task 7 - Security Assessments
1. Which form of assessment is more budget-friendly and takes less time?<br>
    > Vulnerability Assessment: Better for Budget, they are cheaper than Pentests

    **Answer:** Vulnerability Assessment

1. Which type of assessment identifies vulnerabilities and attempts to exploit them?<br>
    **Answer:** Penetration Testing

1. When do you typically carry out Vulnerability Assessments or Pentests?<br>
    >  in the Operations and Maintenance phase, once the version has included all the working components and updates. There are two types of assessments

    **Answer:** Operations and Maintenance

### Task 8 - SSDLC Methodologies
1. What methodology follows a set of mandatory procedures embedded in the SDLC?<br>
    > A development team must complete the mandatory security activities to comply with the Microsoft SDL process. 

    **Answer:** Microsoft SDL

1. What Maturity Model helps you measure tailored risks facing your organisation?<br>
    > The Software Assurance Maturity Model (SAMM) is an open framework to help organisations formulate and implement a software security strategy tailored to the organisation's specific risks.

    **Answer:** SAMM

1. What maturity model acts as a measuring stick to determine your security posture?<br>
    > BSIMM can be described as a "measuring stick" to understand your security posture by providing a comparison of other companies' security states. 

    **Answer:** BSIMM

### Task 9 - Secure Space Lifecycle
1. What is the flag?<br>
    Complete the task to get the flag.<br>
    **Answer:** THM{D0-A-Barr3l-R011}
