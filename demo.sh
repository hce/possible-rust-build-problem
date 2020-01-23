#!/bin/sh
echo Running BUILDING FOR THE 1ST TIME
echo ----------------------------------------------------------------------
cargo run
echo
echo Deleting demo.txt
rm demo.txt
echo Running BUILDING FOR THE 2ND TIME
echo ----------------------------------------------------------------------
cargo run
