> # Intermediate Nmap

# Find the flag!
Scan for the opening ports first.<br>
![](images/1.png)<br>

We'have found three opening ports are 22, 2222, 31337. Based on the description of the task, may be we have to find the credential of SSH.<br>
I have try to find information about port 2222 but don't see anything.<br>
When try with port 31337, I found a interested things may be the credential of the target.<br>
![](images/2.png)<br>
I try to login to the target and success. The flag is in path /home/user/flag.txt.<br>
![](images/3.png)<br>
**Answer:** flag{\<redacted\>}
