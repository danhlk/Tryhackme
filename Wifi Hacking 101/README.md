> # Wifi Hacking 101 - Tryhackme

# Summary
* [Summary](#summary)
   * [Task 1 - The basics - An Intro to WPA](#task-1---the-basics---an-intro-to-wpa)
   * [Task 2 - You're being watched - Capturing packets to attack](#task-2---youre-being-watched---capturing-packets-to-attack)
   * [Task 3 - Aircrack-ng - Let's Get Cracking<br>](#task-3---aircrack-ng---lets-get-cracking)

## Task 1 - The basics - An Intro to WPA
Key Terms

- SSID: The network "name" that you see when you try and connect
- ESSID: An SSID that *may* apply to multiple access points, eg a company office, normally forming a bigger network. For Aircrack they normally refer to the network you're attacking.
- BSSID: An access point MAC (hardware) address
- WPA2-PSK: Wifi networks that you connect to by providing a password that's the same for everyone
- WPA2-EAP: Wifi networks that you authenticate to by providing a username and password, which is sent to a RADIUS server.
- RADIUS: A server for authenticating clients, not just for wifi.
The core of WPA(2) authentication is the 4 way handshake.

Most home WiFi networks, and many others, use WPA(2) personal. If you have to log in with a password and it's not WEP, then it's WPA(2) personal. WPA2-EAP uses RADIUS servers to authenticate, so if you have to enter a username and password in order to connect then it's probably that.

Previously, the WEP (Wired Equivalent Privacy) standard was used. This was shown to be insecure and can be broken by capturing enough packets to guess the key via statistical methods.

The 4 way handshake allows the client and the AP to both prove that they know the key, without telling each other. WPA and WPA2 use practically the same authentication method, so the attacks on both are the same.

The keys for WPA are derived from both the ESSID and the password for the network. The ESSID acts as a salt, making dictionary attacks more difficult. It means that for a given password, the key will still vary for each access point. This means that unless you precompute the dictionary for just that access point/MAC address, you will need to try passwords until you find the correct one.
1. What type of attack on the encryption can you perform on WPA(2) personal?<br>
    > If you have to log in with a password and it's not WEP, then it's WPA(2) personal. 

    **Answer:** Brute force

1. Can this method be used to attack WPA2-EAP handshakes? (Yea/Nay)<br>
    > The ESSID acts as a salt, making dictionary attacks more difficult. 

    **Answer:** Nay

1. What three letter abbreviation is the technical term for the "wifi code/password/passphrase"?<br>
    > WPA2-PSK: Wifi networks that you connect to by providing a password that's the same for everyone

    **Answer:** PSK

1. What's the minimum length of a WPA2 Personal password?<br>
    Based on the passwords which we have typed.
    **Answer:** 8

## Task 2 - You're being watched - Capturing packets to attack
1. How do you put the interface “wlan0” into monitor mode with Aircrack tools? (Full command)<br>
    Follow the helpful [https://www.aircrack-ng.org/doku.php?id=airmon-ng](https://www.aircrack-ng.org/doku.php?id=airmon-ng)<br>
    **Answer:** airmon-ng start wlan0

1. What is the new interface name likely to be after you enable monitor mode?<br>
    **Answer:** wlan0mon

1. What do you do if other processes are currently trying to use that network adapter? <br>
    **Answer:** airmon-ng check kill

1. What tool from the aircrack-ng suite is used to create a capture?<br>
    Follow this [https://www.aircrack-ng.org/doku.php?id=airodump-ng](https://www.aircrack-ng.org/doku.php?id=airodump-ng)<br>
    **Answer:** airodump-ng

1. What flag do you use to set the BSSID to monitor?<br>
    From this link [https://www.aircrack-ng.org/doku.php?id=newbie_guide](https://www.aircrack-ng.org/doku.php?id=newbie_guide)
    > The “--bssid” combined with the AP MAC address limits the capture to the one AP. The “--bssid” option is only available on new versions of airodump-ng.

    **Answer:** --bssid

1. And to set the channel?<br>
    From help page of `airodump-ng`.
    ```
    $ airodump-ng --help
    [---SNIP---]
    --channel <channels>  : Capture on specific channels
    [---SNIP---]
    ```
    **Answer:** --channel

1. And how do you tell it to capture packets to a file?<br>
    ```
    $ airodump-ng --help
    [---SNIP---]
        --write      <prefix> : Dump file prefix
        -w                    : same as --write
    [---SNIP---]
    ```
    **Answer:** -w

## Task 3 - Aircrack-ng - Let's Get Cracking<br>
1. What flag do we use to specify a BSSID to attack?<br>
    See in help page of aircrack-ng.<br>
    ```
    $ aircrack-ng --help | grep bssid
      -b <bssid> : target selection: access point's MAC
    ```
    **Answer:** -b

1. What flag do we use to specify a wordlist?<br>
    ```
    $ aircrack-ng --help | grep wordlist
    -w <words> : path to wordlist(s) filename(s)
    ```
    **Answer:** -w

1. How do we create a HCCAPX in order to use hashcat to crack the password?<br>
    ```
    $ aircrack-ng --help | grep HCCAPX
    -j <file>  : create Hashcat v3.6+ file (HCCAPX)
    ```
    **Answer:** -j

1. Using the rockyou wordlist, crack the password in the attached 
capture. What's the password?<br>
    Crack with aircrack-ng.<br>
    ```
    $ aircrack-ng -b 02:1A:11:FF:D9:BD -w /usr/share/wordlists/rockyou.txt  NinjaJc01-01.cap
    Aircrack-ng 1.6 

      [00:00:13] 124412/14344391 keys tested (9436.38 k/s) 

      Time left: 25 minutes, 6 seconds                           0.87%

                        KEY FOUND! [ greeneggsandham ]


      Master Key     : 71 5F 17 D1 D7 9E 70 4D 6E 2E 9C AD 46 F5 45 F5 
                       AF 5E 43 48 16 F9 5B AA 14 8F 39 AA FC 5E EB 3B 

      Transient Key  : 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
                       00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
                       00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 
                       00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 

      EAPOL HMAC     : 9A 6A 56 EE E4 4E 42 A3 14 71 26 9F E0 E2 93 04
    ```
    **Answer:** greeneggsandham

1. Where is password cracking likely to be fastest, CPU or GPU?<br>
    **Answer:** GPU
