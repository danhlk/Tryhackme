> # Mobile Acquisition

## Summary
- [Summary](#summary)
  - [Task 2 - Mobile Devices Within Digital Forensics](#task-2---mobile-devices-within-digital-forensics)
  - [Task 3 - Challenges With Mobile Device Forensics](#task-3---challenges-with-mobile-device-forensics)
  - [Task 4 - APTs Meet Mobile Devices](#task-4---apts-meet-mobile-devices)
  - [Task 5 - Acquisition Techniques](#task-5---acquisition-techniques)
  - [Task 6 - Specialist Acquisition Techniques](#task-6---specialist-acquisition-techniques)
  - [Task 7 - Practical](#task-7---practical)

### Task 2 - Mobile Devices Within Digital Forensics
1. In what country was it where there is a famous example of mobile devices being used within investigations?<br>
    > For example, in a famous 2013 case in South Africa, health data such as step count was used to discredit a defence argument, showing that the person was indeed active at the time they claimed not to be.

    **Answer:** South Africa

1. What is the technical term for a device that has become the initial access method of an attacker?<br>
    > Entrypoin: Mobile devices are a great initial access method for an attacker.

    **Answer:** Entrypoint

### Task 3 - Challenges With Mobile Device Forensics
1. Which manufacturer protection prevents untrusted code from loading during boot?<br>
    > Secure boot process: It ensures that only trusted and verified code from the manufacturer can load, preventing tampering. An old investigation technique used a custom bootloader that bypassed a wide range of security mechanisms.

    **Answer:** Secure boot process

1. Are encryption keys stored in software or hardware?<br>
    > Android and iOS both use a dedicated hardware component to store encryption keys, which is incredibly tough to retrieve. This is similar to the TPM module on motherboards.

    **Answer:** hardware

### Task 4 - APTs Meet Mobile Devices
1. What app store was found to have malicious applications available to users?<br>
    **Answer:** Google Play

1. What is the name of the sophisticated malware that used a combination of "one click" and "zero click" attacks?<br>
    > Pegasus used a combination of "one click" or "zero click" attacks, known as the "BLASTPASS" exploit chain

    **Answer:** Pegasus

### Task 5 - Acquisition Techniques
1. If I wanted to recover deleted data, what acquisition method would I try?<br>
    **Answer:** Physical

1. Which acquisition method involves using features of the Operating System to extract data?<br>
    **Answer:** Logical acquisition

1. What is the name of the tool that can be used to perform a backup of an Iphone, via the CLI?<br>
    **Answer:** idevicebackup2 

1. What is the name of the tool that can be used to perform a backup of an Android, via the CLI?<br>
    **Answer:** ADB

### Task 6 - Specialist Acquisition Techniques
1. What is the name of the technique that boots the device into a temporary Operating System, often bypassing security mechanisms?<br>
    > Custom Boot Loading: A monitor with red lines depicting the custom boot loading process taking place.This technique involves getting the mobile device to boot into a temporary, custom Operating System that provides low-level access to the device and bypasses security mechanisms.

    **Answer:** Custom Boot Loading

1. What is the name of the technique that exploits a known vulnerability within the device? Granting it full or "root" access?<br>
    > Jailbreaking involves exploiting a known vulnerability within the mobile device's Operating System to provide what's known as "root-level" access, allowing complete control over the device.

    **Answer:** Jailbreaking

### Task 7 - Practical
1. What is the flag displayed once the capture is complete?<br>
    **Answer:** THM{MOBILE_ACQUISITION}
