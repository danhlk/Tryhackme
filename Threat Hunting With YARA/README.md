> # Threat Hunting With YARA

## Summary
- [Summary](#summary)
  - [Task 2 - Scenario Description](#task-2---scenario-description)
  - [Task 3 - Opportunities for Threat Hunting](#task-3---opportunities-for-threat-hunting)
  - [Task 4 - YARA: Introduction](#task-4---yara-introduction)
  - [Task 5 - YARA: Strings and Conditions](#task-5---yara-strings-and-conditions)
  - [Task 7 - YARA: How To Use YARA Rules To Hunt for Indicators of Compromise](#task-7---yara-how-to-use-yara-rules-to-hunt-for-indicators-of-compromise)
  - [Task 8 - Indicators of Compromise Detected - Now What](#task-8---indicators-of-compromise-detected---now-what)
  - [Task 9 - YARA: Hands-on Exercise](#task-9---yara-hands-on-exercise)

### Task 2 - Scenario Description
1. What technique does ID T1134 describe?<br>
    **Answer:** Access Token Manipulation

1. What does the detection rule M_APT_Dropper_Rootsaw_Obfuscated detect?<br>
    **Answer:** Detects obfuscated ROOTSAW payloads

### Task 3 - Opportunities for Threat Hunting
1. Which threat hunting style is proactive and uses indicators of attack and TTPs?<br>
    > Structured Hunting:: This hunting style uses Indicators of Attack and TTPs (Tactics, Techniques, and Procedures) to look for possible attacks from threat actors. 

    **Answer:** Structured Hunting

1. In which phase of the threat hunting process, tools like YARA or Volatility are used?<br>
    > Investigation: A specific trigger is selected and used as a starting point for hunting activities. The threat hunter can use various tools to support the hunt for anomalies, such as YARA rules, Volatility, malware scanners, packet analyzers like Wireshark, and many more. 

    **Answer:** Investigation

1. You have received a threat intelligence report consisting only of Indicators of Compromise. What threat hunting style do you recommend to use?<br>
    > Unstructured Hunting: This hunting style uses Indicators of Compromise to fuel a search in the environment. 

    **Answer:** Unstructured Hunting

### Task 4 - YARA: Introduction
1. Apart from the rule name, which other section is also required in a YARA rule?<br>
    > Condition: In this part of the rule, a matching condition is defined using the identifiers defined in the strings part.
    **Answer:** Condition

### Task 5 - YARA: Strings and Conditions
1. What modifier should be used if you want to search for 2-byte encoded characters?<br>
    **Answer:** wide

1. What condition should be used if you want to exclude the defined strings from the matching process?<br>
    > none of them // Matches only when none of the defined strings are present.

    **Answer:** none of them

### Task 7 - YARA: How To Use YARA Rules To Hunt for Indicators of Compromise
1. What option do you need to pass to ensure you scan all directories recursively?<br>
    **Answer:** -r

### Task 8 - Indicators of Compromise Detected - Now What
1. What does DAIR stand for?<br>
    **Answer:** Dynamic Approach to Incident Response

### Task 9 - YARA: Hands-on Exercise
1. What is the flag found in exercise 1?<br>
    The condition was used `$s = /THM\{.*\}/`.<br>
    ```powershell
    PS C:\TMP> yara64 C:\TMP\YARARULES\myfirstrule.yar C:\TMP\Exercise1\
    warning: rule "myfirstrule" in C:\TMP\YARARULES\myfirstrule.yar(8): $s contains .*, .+ or .{x,} consider using .{,N}, .{1,N} or {x,N} with a reasonable value for N
    myfirstrule C:\TMP\Exercise1\\file26.txt
    PS C:\TMP> Get-Content C:\TMP\Exercise1\\file26.txt | Select-String "THM{"

    THM{Threathuntingisawesome}
    ```
    **Answer:** THM{Threathuntingisawesome}

1. What is the filename found in exercise 2? (Format: filename.extension)<br>
    The condition was used.<br>
    ```
    strings:
        $s1 = "Yet another" wide
	   $s2 = "Ridiculous acronym" wide
    condition:
       all of them
    ```
    **Answer:** file10.txt

1. What is the filename found in exercise 3? (Format: filename.extension)<br>
    The condition was used `$s = "THM{This was a really fun exercise}" base64`.<br>
    **Answer:** file13.txt

1. What was the XOR key used for encryption in exercise 4?<br>
    The condition was used `"THM{FoundSomethingHidden}" xor`. Then uwe `-r, -X and -s` print all the needed string.<br>
    ```
    PS C:\TMP> yara64 C:\TMP\YARARULES\myfirstrule.yar C:\TMP -r -X -s
    myfirstrule C:\TMP\Exercise4\10test.txt
    0x1ade:$s:xor(0x01,THM{FoundSomethingHidden}): UILzGntoeRnlduihofIheedo|
    ```
    **Answer:** 0x01

1. What encrypted string did you find in exercise 4?<br>
    **Answer:** UILzGntoeRnlduihofIheedo|
