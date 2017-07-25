#!/usr/local/bin/bash

# Afnan Enayet    July 2017
# Script that runs unit tests for ensh and deletes artifacts

rm -f test_*
cargo test
rm -f test_*

