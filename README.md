# jpegger

## a work in progress to delete a jpeg that is corrupted

It works by checking for the leading two bytes for a jpeg and the absence of the trailing two bytes.

`find . -iname "*" -exec ./jpegger {} \;`
 
