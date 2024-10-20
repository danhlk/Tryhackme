> # Secure Network Architecture

## Summary
- [Summary](#summary)
  - [Task 2 - Network Segmentation](#task-2---network-segmentation)
  - [Task 3 - Common Secure Network Architecture](#task-3---common-secure-network-architecture)
  - [Task 4 - Network Security Policies and Controls](#task-4---network-security-policies-and-controls)
  - [Task 5 - Zone-Pair Policies and Filtering](#task-5---zone-pair-policies-and-filtering)
  - [Task 6 - Validating Network Traffic](#task-6---validating-network-traffic)
  - [Task 7 - Addressing Common Attacks](#task-7---addressing-common-attacks)

### Task 2 - Network Segmentation
1. How many trunks are present in this configuration?<br>
    There are four bridge in configuration.<br>
    **Answer:** 4

1. What is the VLAN tag ID for interface eth12?>br
    > Port eth12 tag: 30 Interface eth12
    **Answer:** 30

### Task 3 - Common Secure Network Architecture
1. From the above table, what zone would a user connecting to a public web server be in?<br>
    **Answer:** External

1. From the above table, what zone would a public web server be in?<br>
    > DMZ (demilitarized zone) Separates untrusted networks or devices from internal resources. BYOD, remote users/guests, public servers

    **Answer:** DMZ

1. From the above table, what zone would a core domain controller be placed in?<br>
    > Restricted Any high-risk servers or databases. Domain controllers, client information

    **Answer:** Restricted

### Task 4 - Network Security Policies and Controls
1. According to the corresponding ACL policy, will the first packet result in a drop or accept? <br>
    The source and destination IP is also in 10.10.212.0/24, so the first packet will be accepted.<br>
    **Answer:** accept

1. According to the corresponding ACL policy, will the second packet result in a drop or accept?<br>
    The destination IP is 10.10.212.209, so the second packet will be dropped.<br>
    **Answer:** drop

### Task 5 - Zone-Pair Policies and Filtering
1. What is the flag found after filling in all blanks on the static site?<br>
    Complete the task to get the flag.<br>
    **Answer:** THM{M05tly_53cure}

### Task 6 - Validating Network Traffic
1. Does SSL inspection require a man-in-the-middle proxy? (Y/N)<br>
    > SSL/TLS inspection uses an SSL proxy to intercept protocols

    **Answer:** Y

1. What platform processes data sent from an SSL proxy?<br>
    > the proxy will decrypt the traffic and send it to be processed by a UTM (Unified Threat Management) platform

    **Answer:** Unified Threat Management

### Task 7 - Addressing Common Attacks
1. Where does DHCP snooping store leased IP addresses from untrusted hosts?<br>
    > The switch will store untrusted hosts with leased IP addresses in a DHCP Binding Database.

    **Answer:** DHCP Binding Database

1. Will a switch drop or accept a DHCPRELEASE packet?<br>
    > A DHCPRELEASE or DHCPDECLINE packet is received on an untrusted interface that does not match an interface that the source address already has registered.

    **Answer:** drop

1. Does dynamic ARP inspection use the DHCP binding database? (Y/N)<br>
    > ARP inspection uses the DHCP binding database filled from DHCP snooping as its list of binding IP addresses.

    **Answer:** Y

1. Dynamic ARP inspection will match an IP address and what other packet detail?<br>
    > ARP inspection will compare the source IP address and MAC address to the binding pair

    **Answer:** MAC Address
