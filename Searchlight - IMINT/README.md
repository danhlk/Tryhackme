> # Searchlight - IMINT - Tryhackme

# Summary
* [Task 1 - Welcome to the Searchlight IMINT room!](#task-1---welcome-to-the-searchlight-imint-room)
* [Task 2 - Your first challenge!](#task-2---your-first-challenge)
* [Task 3 - Just Google it!](#task-3---just-google-it)
* [Task 4 - Keep at it!](#task-4---keep-at-it)
* [Task 5 - Coffee and a light lunch](#task-5---coffee-and-a-light-lunch)
* [Task 6 - Reverse your thinking](#task-6---reverse-your-thinking)
* [Task 7 - Locate this sculpture](#task-7---locate-this-sculpture)
* [Task 8 - ...and justice for all](#task-8---and-justice-for-all)
* [Task 9 - The view from my hotel room](#task-9---the-view-from-my-hotel-room)

## Task 1 - Welcome to the Searchlight IMINT room!
1. Did you understand the flag format?<br>
    > The flag format is: sl{flag}<br>
    The answer to the question below is: ready

    **Answer:** sl{ready}

## Task 2 - Your first challenge!
First tool - **YOUR EYES**
1. What is the name of the street where this image was taken?<br>
    Open the image, you will see the name of street on Welcome Gate.<br>
    ![](images/1.png)<br>
    **Answer:** sl{carnaby street}

## Task 3 - Just Google it!
Google Dorking.
1. Which city is the tube station located in?<br>
    Search on Google with `public subway underground location`.<br>
    ![](images/2.png)<br>
    **Answer:** sl{london}

1. Which tube station do these stairs lead to?<br>
    In original image, we see string `Circus` below the Gate.
    ![](images/3.png)<br>
    So I find on Google with key `circus public subway underground` and received the answer.<br>
    ![](images/4.png)<br>
    **Answer:** sl{Piccadilly Circus}

1. Which year did this station open?<br>
    We have already known the name of this station, so search when this station opened.<br>
    ![](images/5.png)<br>
    **Answer:** sl{1906}

1. How many platforms are there in this station?<br>
    Continue search on Google based on the question.<br>
    ![](images/6.png)<br>
    **Answer:** sl{4}

## Task 4 - Keep at it!
1. Which building is this photo taken in?<br>
    From original image, we see the banner `YVR Connect`
    ![](images/7.png)<br>
    So search on Google with this keyword.<br>
    ![](images/8.png)<br>
    **Answer:** sl{Vancouver International Airport}

1. Which country is this building located in?<br>
    ![](images/9.png)<br>
    **Answer:** sl{Canada}

1. Which city is this building located in?<br>
    From above question, we have the answer.<br>
    **Answer:** sl{Richmond}

## Task 5 - Coffee and a light lunch
1. Which city is this coffee shop located in?<br>
    In attached image, in front of this coffee is a shop-sign with name which was above and a white narrow.<br>
    ![](images/10.png)<br>
    Search on Google with `Scotland` country.<br>
    ![](images/11.png)<br>
    Follow link of this image, we will see a description below.<br>
    ![](images/12.png)<br>
    **Answer:** sl{Blairgowrie}

1. Which street is this coffee shop located in?<br>
    Search on Google map with information found on previous question.<br>
    ![](images/13.png)<br>
    **Answer:** sl{Allan Street}

1. What is their phone number?<br>
    Select the Wee coffee shop to see phone number.<br>
    ![](images/14.png)<br>
    **Answer:** sl{+4478788 39128}

1. What is their email address?<br>
    Follow their Facebook page, you will see the email address.<br>
    ![](images/15.png)<br>
    **Answer:** sl{theweecoffeeshop@aol.com}

1. What is the surname of the owners?<br>
    Search on Google, it immediately display the answer but this information is old (in 2019).<br>
    ![](images/16.png)<br>
    Below is a new link refer to needed information in 2021.<br>
    ![](images/17.png)<br>
    **Answer:** sl{Cochrane}

## Task 6 - Reverse your thinking
1. Which restaurant was this picture taken at?<br>
    Upload thsi image to google lens.<br>
    ![](images/18.png)<br>
    Follow this image in red area.<br>
    ![](images/19.png)<br>
    **Answer:** sl{Katz's Deli}

1. What is the name of the Bon Appétit editor that worked 24 hours at this restaurant?<br>
    Search on google.<br>
    ![](images/20.png)<br>
    Follow this link.<br>
    ![](images/21.png)<br>
    **Answer:** sl{Andrew Knowlton}

## Task 7 - Locate this sculpture
1. What is the name of this statue?<br>
    Upload this image to Google Lens.<br>
    ![](images/22.png)<br>
    Follow image in red area.<br>
    ![](images/23.png)<br>
    **Answer:** sl{Rudolph the chrome nosed reindeer}

1. Who took this image?<br>
    Location of this statue is on Oslo, Norway. So based on the hint, I use [https://www.visitoslo.com/](https://www.visitoslo.com/) to search for the photographer.<br>
    ![](images/24.png)<br>
    In this article, I found an image the same as the material image belongs with credit.<br>
    ![](images/25.png)<br>
    **Answer:** sl{Kjersti Stensrud}

## Task 8 - ...and justice for all
1. What is the name of the character that the statue depicts?<br>
    Upload this image to Google Lens.<br>
    ![](images/26.png)<br>
    **Answer:** sl{Lady Justice}

1. where is this statue located?<br>
    Based on this link [https://www.americanprogress.org/article/past-time-congress-expand-lower-courts/](https://www.americanprogress.org/article/past-time-congress-expand-lower-courts/), I saw a small description of this statue, I refer to U.S. Courthouse in Alexandria, Virginia.<br>
    ![](images/27.png)<br>
    Search on Google maps [link](https://www.google.com/maps/place/US+District+Court/@38.8027573,-77.065776,3a,75y,228.03h,105.12t/data=!3m6!1e1!3m4!1sf69L_nrg348GWTxd-nviuQ!2e0!7i16384!8i8192!4m7!3m6!1s0x89b7b1a8d97cbdbb:0x376540cc3ca906d9!8m2!3d38.8026292!4d-77.0658933!14m1!1BCgIgARICCAI), this image assure that we found the right address and red area above is then answer based on the hint.<br>
    ![](images/28.png)<br>
    **Answer:** sl{alexandria, virginia}

1. What is the name of the building opposite from this statue?<br>
    Follow 360 view, we will see the name of building opposite from this statue.<br>
    ![](images/29.png)<br>
    **Answer:** sl{The Westin Alexandria Old Town}

## Task 9 - The view from my hotel room
1. 
    At 00:08s, we will see a row colorful buildings, drop this frame then upload to Google Lens.<br>
    ![](images/30.png)<br>
    Follow this image, we will know the address of these buildings.<br>
    ![](images/31.png)<br>
    Search `Clarke quay` on Google Map, you will an image the same as these building in this [link](https://www.google.com/maps/place/Clarke+Quay/@1.2905984,103.8447243,2a,90y,29.83h,99.82t/data=!3m6!1e1!3m4!1sbnBBrRfobm9l7LCKAubTbg!2e0!7i13312!8i6656!4m14!1m6!3m5!1s0x31da19a038007c9d:0x20e5acd1e8ccf02!2sClarke+Quay!8m2!3d1.2906024!4d103.8464742!3m6!1s0x31da19a038007c9d:0x20e5acd1e8ccf02!8m2!3d1.2906024!4d103.8464742!14m1!1BCgIgARICCAI).<br>
    ![](images/32.png)<br>
    THe building opposite these buidlings have a banner `Liang Court`. Search it on Google Earth, name of it is `Novotel`.<br>
    ![](images/34.png)<br>
    Drop image for this building then search on Google Lens for finding the fullname.<br>
    ![](images/33.png)<br>
    In this [link](https://fishand.tips/en/hotel/Novotel_Clarke_Quay/), we will see the full name of this building.<br>
    ![](images/35.png)<br>
    **Answer:** sl{Novotel Singapore Clarke Quay}

> ***Done***
![](images/36.png)<br>
