# build_release.sh    Afnan Enayet
# Builds the release version of the shell with cargo and compresses it with 
# gzexe

#!/usr/bin/bash

# Build the executable
cargo build --release

# Compress the executable
strip ./target/release/ensh
gzexe ./target/release/ensh 

# Retain original executable and rename the compressed executable 
mv ./target/release/ensh ./target/release/ensh_compressed
mv ./target/release/ensh~ ./target/release/ensh

