> # Insecure Randomness

## Sumary
- [Sumary](#sumary)
  - [Task 2 - Few Important Concepts](#task-2---few-important-concepts)
  - [Task 3 - Types of Random Number Generators](#task-3---types-of-random-number-generators)
  - [Task 4 - Weak or Insufficient Entropy](#task-4---weak-or-insufficient-entropy)
  - [Task 5 - Predictable Seed in PRNGs](#task-5---predictable-seed-in-prngs)
  - [Task 6 - Mitigation Measures](#task-6---mitigation-measures)

### Task 2 - Few Important Concepts
1. What measures the amount of randomness or unpredictability in a system?<br>
    > Entropy represents the amount of randomness or unpredictability in a system and is often used to assess the security of cryptographic keys, tokens, or random values. 

    **Answer:** Entropy

1. Is it a good practice to keep the same seed value for all cryptographic functions? (yea/nay)<br>
    **Answer:** nay

### Task 3 - Types of Random Number Generators
1. You prepare a game involving immediate interaction and random event simulation but with no critical security requirements. Which type of RNG would be most appropriate for this purpose? Write the correct option only.<br>
    > tatistical PRNGs are designed to produce numbers that pass statistical randomness tests, meaning the numbers appear random and lack obvious patterns. These generators are widely used in non-security applications such as simulations, statistical sampling, and gaming

    **Answer:** b

### Task 4 - Weak or Insufficient Entropy
1. What is the flag value after logging in as the victim user?<br>
    Follow the instruction from the document, you will get the corret token to reset the victim's password.<br>
    ![](images/1.png)<br>
    **Answer:** THM{VICTIM_SIGNED_IN}

1. What is the flag value after logging in as the master user? <br>
    Repeat the steps above.<br>
    ![](images/2.png)<br>
    **Answer:** THM{ADMIN_SIGNED_IN007}

1. What is the PHP function used to create the token variable in the code above?<br>
    **Answer:** time()

### Task 5 - Predictable Seed in PRNGs
1. What is the flag value after logging in as magic@mail.random.thm?<br>
    Follow the instruction the login as magic.<br>
    ![](images/3.png)<br>
    **Answer:** THM{MAGIC_SIGNED_IN11010}

1. What is the flag value after logging in as hr@mail.random.thm?<br>
    You can write a script to automacally get the result from ther predicted token.<br>
    **Answer:** THM{HR_SIGNED_IN1337}

1. What is the PHP function used to seed the RNG in the code above?<br>
    **Answer:** mt_srand

### Task 6 - Mitigation Measures
1. Which of the following can be considered as a weak seed value? Write the correct letter only.<br>
    **Answer:** d
