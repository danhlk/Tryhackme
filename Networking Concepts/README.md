> # Networking Concepts

## Summary
- [Summary](#summary)
  - [Task 2 - OSI Model](#task-2---osi-model)
  - [Task 3 - TCP/IP Model](#task-3---tcpip-model)
  - [Task 4 - IP Addresses and Subnets](#task-4---ip-addresses-and-subnets)
  - [Task 5 - UDP and TCP](#task-5---udp-and-tcp)
  - [Task 6 - Encapsulation](#task-6---encapsulation)
  - [Task 7 - Telnet](#task-7---telnet)

### Task 2 - OSI Model
1. Which layer is responsible for connecting one application to another?<br>
    Layer 4 include TCP and UDP will responsible for connecting one application to another.<br>
    > End-to-end communication and data segmentation

    **Answer:** Layer 4

1. Which layer is responsible for routing packets to the proper network?<br>
    > Layer 3: Logical addressing and routing between networks

    **Answer:** Layer 3

1. In the OSI model, which layer is responsible for encoding the application data?<br>
    > Layer 6: Data encoding, encryption, and compression

    **Answer:** Layer 6

1. Which layer is responsible for transferring data between hosts on the same network segment?<br>
    > Layer 2: Reliable data transfer between adjacent nodes
    
    **Answer:** Layer 2

### Task 3 - TCP/IP Model
1. To which layer does HTTP belong in the TCP/IP model?<br>
    HTTP belongs to Application Layer.<br>
    **Answer:** Application Layer
1. How many layers of the OSI model does the application layer in the TCP/IP model cover?<br>
    **Answer:** 3

### Task 4 - IP Addresses and Subnets
1. Which of the following IP addresses is not a private IP address?<br>
    The tree range private IP address are start with 10.0.0.0, 172.16.0.0, 192.168.0.0.<br>
    **Answer:** 49.69.147.197

1. Which of the following IP addresses is not a valid IP address?<br>
    In 192.168.305.19, the value of octer 3 is greater than 255.<br>
    **Answer:** 192.168.305.19

### Task 5 - UDP and TCP
1. Which protocol requires a three-way handshake?<br>
    > A TCP connection is established using what’s called a three-way handshake

    **Answer:** TCP

1. What is the approximate number of port numbers (in thousands)?<br>
    Because 65535//1000 = 65.<br>
    **Answer:** 65

### Task 6 - Encapsulation
1. On a WiFi, within what will an IP packet be encapsulated?<br>
    > Data link frame: The Ethernet or WiFi receives the IP packet and adds the proper header and trailer, creating a frame

    **Answer:** frame

1. What do you call the UDP data unit that encapsulates the application data?<br>
    > Transport protocol segment or datagram: The transport layer, such as TCP or UDP, adds the proper header information and creates the TCP segment (or UDP datagram). This segment is sent to the layer below it, the network layer.

    **Answer::** datagram

1. What do you call the data unit that encapsulates the application data sent over TCP?<br>
    **Answer:** segment

### Task 7 - Telnet
1. Use telnet to connect to the web server on MACHINE_IP. What is the name and version of the HTTP server?<br>
    ```shell
    ~$ telnet 10.10.85.81 80
    Trying 10.10.85.81...
    Connected to 10.10.85.81.
    Escape character is '^]'.

    HTTP/1.0 400 Bad Request
    Content-Type: text/html
    Content-Length: 345
    Connection: close
    Date: Sat, 02 Nov 2024 05:30:54 GMT
    Server: lighttpd/1.4.63

    <?xml version="1.0" encoding="iso-8859-1"?>
    <!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN"
            "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
    <html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
    <head>
    <title>400 Bad Request</title>
    </head>
    <body>
    <h1>400 Bad Request</h1>
    </body>
    </html>
    Connection closed by foreign host.
    ```
    **Answer:** lighttpd/1.4.63

1. What flag did you get when you viewed the page?<br>
    Use `curl` to retrive the web page content.<br>
    ```shell
    ~$ curl http://10.10.85.81
    THM{TELNET_MASTER}
    ```
    **Answer:** THM{TELNET_MASTER}
