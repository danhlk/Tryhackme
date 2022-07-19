> # Nmap Live Host Discovery

# Summary
* [Summary](#summary)
   * [Task 2 - Subnetworks](#task-2---subnetworks)
   * [Task 3 - Enumerating Targets](#task-3---enumerating-targets)
   * [Task 4 - Discovering Live Hosts](#task-4---discovering-live-hosts)
   * [Task 5 - Nmap Host Discovery Using ARP](#task-5---nmap-host-discovery-using-arp)
   * [Task 6 - Nmap Host Discovery Using ICMP](#task-6---nmap-host-discovery-using-icmp)
   * [Task 7 - Nmap Host Discovery Using TCP and UDP](#task-7---nmap-host-discovery-using-tcp-and-udp)
   * [Task 8 - Using Reverse-DNS Lookup](#task-8---using-reverse-dns-lookup)
   
## Task 2 - Subnetworks
1. How many devices can see the ARP Request?<br>
    **Answer:** 4 (computer1, 2, 3, and router)

1. Did computer6 receive the ARP Request? (Y/N)<br>
    **Answer:** N (not in the same subnet)

1. How many devices can see the ARP Request?<br>
    **Answer:** 4 (computer4, 5, 6 and router)

1. Did computer6 reply to the ARP Request? (Y/N)<br>
    **Answer:** Y (same subnet)

## Task 3 - Enumerating Targets
1. What is the first IP address Nmap would scan if you provided `10.10.12.13/29` as your target?<br>
    ```sh
    $ nmap -sL -n 10.10.12.13/29
    Starting Nmap 7.80 ( https://nmap.org ) at 2022-07-18 15:02 +07
    Nmap scan report for 10.10.12.8
    Nmap scan report for 10.10.12.9
    Nmap scan report for 10.10.12.10
    Nmap scan report for 10.10.12.11
    Nmap scan report for 10.10.12.12
    Nmap scan report for 10.10.12.13
    Nmap scan report for 10.10.12.14
    Nmap scan report for 10.10.12.15
    Nmap done: 8 IP addresses (0 hosts up) scanned in 0.00 seconds
    ```
    **Answer:** 10.10.12.8

1. How many IP addresses will Nmap scan if you provide the following range `10.10.0-255.101-125`? <br>
    256 * 25<br>
    **Answer:** 6400

## Task 4 - Discovering Live Hosts
1. What is the type of packet that computer1 sent before the ping?<br>
    **Answer:** ARP Request

1. What is the type of packet that computer1 received before being able to send the ping?<br>
    **Answer:** ARP Response

1. How many computers responded to the ping request?
    **Answer:** 1

1. What is the name of the first device that responded to the first ARP Request?<br>
    **Answer:** router

1. What is the name of the first device that responded to the second ARP Request?<br>
    **Answer:** computer5

1. Send another Ping Request. Did it require new ARP Requests? (Y/N)<br>
    **Answer:** N

## Task 5 - Nmap Host Discovery Using ARP
1. How many devices are you able to discover using ARP requests?<br>
    **Answer:** 3 (computer1, 2, 3)

## Task 6 - Nmap Host Discovery Using ICMP
1. What is the option required to tell Nmap to use ICMP Timestamp to discover live hosts?<br>
    **Answer:** -PP

1. What is the option required to tell Nmap to use ICMP Address Mask to discover live hosts?<br>
    **Answer:** -PM

1. What is the option required to tell Nmap to use ICMP Echo to discover life hosts?<br>
    **Anwer:** -PE

## Task 7 - Nmap Host Discovery Using TCP and UDP
1. Which TCP ping scan does not require a privileged account?<br>
    **Answer:** TCP SYN Ping

1. Which TCP ping scan requires a privileged account?<br>
    **Answer:** TCP ACK Ping

1. What option do you need to add to Nmap to run a TCP SYN ping scan on the telnet port?<br>
    **Answer:** -PS23

## Task 8 - Using Reverse-DNS Lookup
1. We want Nmap to issue a reverse DNS lookup for all the possibles hosts on a subnet, hoping to get some insights from the names. What option should we add?<br>
    **Answer:** -R
