# XEcryptionBreaker
Bruteforces messages encrypted with the XEcryption algorithm. Also generates a viable key to re-encrypt it

<h2>Help message:</h2>

USAGE<br>
> XEcryptionBreaker [operation] (key)
  
<b>Decryption</b> (usage -> 'XEcryptionBreaker [file name]')<br>
Put the to-be-decrypted message into a file and pass the file's location on to this program.

This program currently has two limitations with its decryption
  a) It can only decrypt messages with spaces in them
  b) It can not automatically detect the correct encrypted message
After it is done decrypting, it will output all possible decrypted messages along with \ntheir respective key. Look through all of the messages until you find the correctly decypted one \n(This shouldn't be too difficult since the rest will be nonsense!)
  
<b>Key Generation</b> (usage -> 'XEcryptionBreaker key-gen [key-value]')<br>
If you want to modify the encrypted message and re-encrypt it using the same key, you can use \nthis handy feature. XEcryption does not need the same key to en/decrypt a message,\nit only needs the same sum of the ascii values of all the characters in the key to match up.
  
<b>Help</b> (usage -> 'XEcryptionBreaker help')<br>
Displays this page.
  
Credits:
  - Understanding of algorithm and approach to breaking it:
    https://mvddvm.blogspot.com/2007/04/breaking-xecryption.html
  
