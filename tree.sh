#!/bin/bash

# I got this file on internet, bug I forgot who is the author. sorry!

SEDMAGIC='s;[^/]*/;|____;g;s;____|; |;g'

if [ "$#" -gt 0 ] ; then
   dirlist="$@"
else
    dirlist="."
fi

for x in $dirlist; do
    find "$x" -print | sed -e "$SEDMAGIC"
done
