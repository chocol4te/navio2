#!/bin/sh
scp $2 $1:~/
ssh -t $1 "./$(basename -- $2)"
