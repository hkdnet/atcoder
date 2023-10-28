#!/bin/bash -eu

for l in $(ls src/bin/*.rs); do
  file=$(basename $l)
  cd "${file%%.*}"
  make test
  cd ../
done
