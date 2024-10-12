> # Hypervisor Internals

## Summary
- [Summary](#summary)
  - [Task 2 - Types of Hypervisors](#task-2---types-of-hypervisors)
  - [Task 3 - Hypervisor Landscape](#task-3---hypervisor-landscape)
  - [Task 4 - Hypervisors in Cyber Security](#task-4---hypervisors-in-cyber-security)
  - [Task 5 - Hypervisor Internals](#task-5---hypervisor-internals)
  - [Task 6 - Guest Additions](#task-6---guest-additions)
  - [Task 7 - Practical](#task-7---practical)

### Task 2 - Types of Hypervisors
1. What type of Hypervisors have direct access to bare metal?<br>
    > These types of Hypervisors have direct access to a system's physical hardware
    
    **Answer:** type 1

1. What type of Hypervisors do not have access to bare metal but run inside and through another Operating System?<br>
    > they run on top of an existing operating system (such as Windows or Linux).

    **Answer:** type 2

### Task 3 - Hypervisor Landscape
1. What is the name of the Hypervisor that can be found as both a type 1 and type 2 Hypervisor?<br>
    > Hyper-V is a full operating system (for example, Hyper-V Server 2019), which is a Type 1 Hypervisor. However, the Hyper-V service can be installed on Windows servers and desktops as a role/feature (such as Pro and Enterprise editions). If installed, the Hyper-V service manages the Windows OS, giving the illusion that it is a type 2 hypervisor

    **Answer:** Hyper-V

1. What is the name of the open-source Hypervisor developed by Oracle?<br>
    > Oracle's VirtualBox is a popular choice among Hypervisors due to its open-source, free, and cross-platform nature. 

    **Answer:** VirtualBox

### Task 4 - Hypervisors in Cyber Security
1. As of the time of writing, what is the maximum amount that Microsoft offers for disclosed Hyper-V vulnerabilities?<br>
    > at the time of writing, Microsoft offers up to $250,000 for Hyper-V vulnerabilities submitted to its bug bounty program. 

    **Answer:** $250,000

1. What category of use do cyber security analysts use Hypervisors to analyse malicious code?<br>
    > Research: Virtual machines also provide security thanks to their sandboxing mechanism; you can run tooling to investigate malicious code without the risk of exposing your physical device. Researchers can take snapshots of a machine in various states that can be reverted back to during experimentation with malicious code. 

    **Answer:** Research

1. What is the name of one of the APT groups that has been identified as targeting ESXi Hypervisors?<br>
    **Answer:** AlphaV

### Task 5 - Hypervisor Internals
1. What is the acronym for a virtual CPU?<br>
    > Hypervisors use virtual CPUs (vCPUs) to virtualise the CPU

    **Answer:** vCPU

1. What is the acronym for a virtual network adapter?<br>
    > This abstraction is achieved by using Virtual Network Interfaces (vNICs).

    **Answer:** vNIC

1. What virtualisation method allows for a Hypervisor to be ran within a virtual machine?<br>
    **Answer:** Nested Virtualisation

### Task 6 - Guest Additions
1. What is the full CVE of the vulnerability that allowed attackers to exploit guest additions to escape the guest environment? Format: CVE-XXXX-XXXX<br>
    >  CVE-2018-2693 was disclosed within the guest additions of VirtualBox. This CVE abused a vulnerability within the guest additions package to escape from the guest to the host.

    **Answer:** CVE-2018-2693

1. What name does the VMware guest additions process show up as on the guest?<br>
    **Answer:** VMware Tools Core Service

### Task 7 - Practical
1. What is the flag from the practical?<br>
    ![](images/1.png)<br>
    **Answer:** THM{LAYERS_UPON_LAYERS}
