> # Cryptography for Dummies - Tryhackme

# Summary

## Task 2 - Types of cryptography
1. What type of cryptography is more secure?<br>
    > Note : Symmetric cryptography is faster than asymmetric, but asymmetric is more secure.

    **Answer:** asymmetric

1. What type of cryptography is faster?<br>
    > Note : Symmetric cryptography is faster than asymmetric, but asymmetric is more secure.

    **Answer:** symmetric

1. What type of cryptography will a Bank site use?<br>
    Because Bank store our money, so they must have a extremely secure.<br>
    **Answer:** asymmetric

1. What will you use to encrypt your messages in asymmetric cryptography?<br>
    > The encryption key can be used only for encryption, not for decryption and vice versa. The encryption key is called Public key and the decryption key is called Private key.

    **Answer:** public key

1. What will you use to decrypt messages in asymmetric cryptography?<br>
    **Answer:** private key

1. Does symmetric cryptography use two different keys for encryption/decryption? (aye/nay)<br>
    **Answer:** nay

## Task 3 - What is a hash?
1. What's the MD5 hash of "hashes are cool"?<br>
    Use built-in command `md5sum`.<br>
    ```
    $ echo -n "hashes are cool" | md5sum
    f762d32e3c160900d94b683e927555b9  -
    ```
    **Answer:** f762d32e3c160900d94b683e927555b9

1. What does MD5 stand for?<br>
    Search on Google.<br>
    **Answer:** Message Digest 5

1. Who created MD5?<br>
    Continue search on Google.<br>
    **Answer:** Ronald Rivest

## Task 4 - Decoding/encoding
1. Encode the string "cryptographyisuseful" with Base64<br>
    ```
     echo -n cryptographyisuseful | base64
    Y3J5cHRvZ3JhcGh5aXN1c2VmdWw=
    ```
    **Answer:** Y3J5cHRvZ3JhcGh5aXN1c2VmdWw=

1. Decode the string "dGhlIHNlY3JldCB3b3JkIGlzIDogd2F0ZXJtZWxvbg==". What's the secret word?<br>
    ```
    $ echo -n "dGhlIHNlY3JldCB3b3JkIGlzIDogd2F0ZXJtZWxvbg==" | base64 -d
    the secret word is : watermelon
    ```
    **Answer:** watermelon

