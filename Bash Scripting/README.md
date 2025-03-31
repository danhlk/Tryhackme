> # Bash Scripting

## Summary
- [Summary](#summary)
  - [Task 2 - Our first simple bash scripts](#task-2---our-first-simple-bash-scripts)
  - [Task 3 - Variables](#task-3---variables)
  - [Task 4 - Parameters](#task-4---parameters)
  - [Task 5 - Arrays](#task-5---arrays)
  - [Task 6 - Conditionals](#task-6---conditionals)

### Task 2 - Our first simple bash scripts
1. What piece of code can we insert at the start of a line to comment out our code?<br>
    **Answer:** #

1. What will the following script output to the screen, echo “BishBashBosh”<br>
    **Answer:** BishBashBosh

### Task 3 - Variables
1. What would this code return?<br>
    **Answwer:** Jammy is 21 years old

1. How would you print out the city to the screen?<br>
    **Answer:** echo $city

1. How would you print out the country to the screen?<br>
    **Answer:** echo $country

### Task 4 - Parameters
1. How can we get the number of arguments supplied to a script?<br>
    Do a littel search, you will see the answer.<br>
    **Answer:** $#

1. How can we get the filename of our current script(aka our first argument)?<br>
    **Answer:** $0

1. How can we get the 4th argument supplied to the script?<br>
    **Answer:** $4

1. If a script asks us for input how can we direct our input into a variable called ‘test’ using “read”<br>
    **Answer:** read test

1. What will the output of “echo $1 $3” if the script was ran with “./script.sh hello hola aloha”<br>
    **Answer:** hello aloha

### Task 5 - Arrays
1. What would be the command to print audi to the screen using indexing.<br>
    **Answer:** echo "${cars[1]}"

1. If we wanted to remove tesla from the array how would we do so?<br>
    **Answer:** unset cars[3]

1. How could we insert a new value called toyota to replace tesla?<br>
    **Answer:** cars[3]='toyota'

### Task 6 - Conditionals
1. What is the flag to check if we have read access to a file?<br>
    **Answer:** -r

1. What is the flag to check to see if it's a directory?<br>
    **Answer:** -d
