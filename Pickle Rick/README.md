> # Pickle Rick

1. What is the first ingredient Rick needs?<br>
    Check the home page first, we have to find the information about credential of the target to find 3 ingredients.<br>
    ![](images/1.png)<br>
    When I looked at page source, I found the username is R1ckRul3s.<br>
    ![](images/2.png)<br>
    Almost page will have the file named `robots.txt`, I checked it and found a string may be the password.<br>
    With `dirsearch` tool, I also found the path to `login.php`, use R1ckRul3s:Wubbalubbadubdub to login and success.<br>
    It will redirect you to `portal.php` where you can run command on the target.<br>
    ![](images/4.png)<br>
    List files in current directory.<br>
    ![](images/5.png)<br>
    The 1st ingredient is in file Sup3r..., but the `cat` command is disable in the target, so I must find another tool to read it, so I use `tac` command and successfully open this file.<br>
    ![](images/6.png)<br>
    **Answer:** mr. meeseek hair

1. Whats the second ingredient Rick needs?<br>
    Another file named `clue.txt` hint that we have to look around file system to find the others.<br>
    ![](images/7.png)<br>
    List `/home` directoty, in the rick's home folder you will see the 2nd ingredient.<br>
    ![](images/8.png)<br>
    **Answer:** 1 jerry tear

1. Whats the final ingredient Rick needs?<br>
    The current user in this shell is www-data, but when I run `sudo -l`, this user can run all command without password.<br>
    ![](images/9.png)<br>
    The 3rd ingredient is in `/root` folder, read it with `sudo` before.<br>
    ![](images/10.png)<br>
    **Answer:** fleeb juice
