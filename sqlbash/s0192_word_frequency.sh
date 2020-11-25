#!/usr/bin/env zsh
# Read from the file words.txt and output the word frequency list to stdout.
cat words.txt | tr -s ' ' '\n' | sort | uniq -c | sort -r -n -k 1 |awk '{print $2, $1}'
