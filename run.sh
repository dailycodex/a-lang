#!/bin/bash

if [[ $1 == "-h" ]];
then
  echo "<command> [<filepath>] [run] [clean]";
  echo "-h      print this message";
  echo "run     run file and print out of program exit value"
  echo "clean   removes build files after running"
  exit 0
fi


filename=$1;
exefile=${filename%.*}
asmfile=$exefile".asm"

function clean() {
  rm $exefile;
  rm $asmfile;
}

function run() {
  $exefile && echo $? || echo $?;
}

cargo run --release -- $filename && $2 && $3;
