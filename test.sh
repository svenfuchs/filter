#!/bin/bash

version=$1

if [[ $version = ruby ]]; then
  cmd="ruby filter_2.rb"
else
  cmd="./filter"
fi

exec > >(
  export A=111 B=666
  $cmd A B
) 2>&1

str=01112345666789

while read -n1 char; do
  echo -n $char
  sleep 0.1
done < <(echo -n "$str")

echo
