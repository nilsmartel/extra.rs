# extra

Usage:

Reads Text from ```stdin```
and extracts all content found between the text of two given arguments

Example
  if the content of text.txt is
    ```
    links(
     url(cargo.io)
     url(github.com)
    )
    ```
  
  then calling extra like this
  
  ```cat text.txt | extra url( )```
  
  would produce this output
  
  ```
  cargo.io
  github.com
  ```
  
