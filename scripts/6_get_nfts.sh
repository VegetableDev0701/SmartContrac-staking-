#!/bin/bash
input="./1"
while read -r line
do
  echo "$line"
done < "$input"