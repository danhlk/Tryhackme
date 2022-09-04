> # Windows x64 Assembly - Tryhackme

# Summary

## Task 2 - Number Systems
1. What is 0xA in decimal?<br>
    **Answer:** 10

1. What is decimal 25 in hexadecimal? Include the prefix for hexadecimal.<br>
    **Answer:** 0x19

## Task 3 - Bits and Bytes
1. How many bytes is a WORD?<br>
    **Answer:** 2

1. How many bits is a WORD?<br>
    **Answer:** 2*8 = 16

## Task 4 - Binary Operations
1. What is the result of the binary operation: 1011 AND 1100?<br>
    **Answer:** 1000

1. What is the result of the binary operation: 1011 NAND 1100? Include leading zeroes.<br>
    **Anwer:** 0111

## Task 5 - Registers
1. How many bytes is RAX?<br>
    **Answer:** 8

1. How many bytes is EAX?<br>
    **Answer:** 4

## Task 6 - Instructions
1. What instruction returns from a function?<br>
    **Answer:** ret

1. What instruction will call/execute a function?<br>
    **Answer:** call

1. What instruction could be used to save a register in a way that it can later be restored?<br>
    `push` instruction will push the value of register to the top of stack but still hold the value in register.<br>
    **Answer:** push

# Task 7 - Flags
1. If two equal values are compared to each other, what will ZF be set to as result of the comparison?<br>
    If two equal values are compared, the result is 0 so ZF is set to 1.<br>
    **Answer:** 1

# Task 8 - Calling Conventions
1. In fastcall, what 64-bit register will hold the return value of a function?
    **Answer:** RAX

1. In fastcall, what register is the first function parameter passed in?<br>
    Không có dấu chấm động theo thứ tự RCX, RDX, R8, R9, nếu có sẽ là XMM0-3.<br>
    Nếu có nhiều hơn 4 tham số, cứ đẩy vào 4 thanh ghi và push dần vào stack.<br>
    **Answer:** RCX

# Task 9 - Memory Layout
1. In what order is data taken off of or put onto the stack? Provide the acronym.<br>
    Stack is Last In First Out
    **Answer:** LIFO

