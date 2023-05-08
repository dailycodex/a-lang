#!/bin/bash
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
