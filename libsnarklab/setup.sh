#!/bin/bash

git submodule update --init --recursive deps/libsnark

declare -A submodules
submodules=(
    ["deps/libsnark"]="6c705e3135f585c222813654caedc86520fda1f6"
    ["deps/libsnark/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libsnark/depends/gtest"]="3a4cf1a02ef4adc28fccb7eef2b573b14cd59009"
    ["deps/libsnark/depends/libff"]="176f3f42fdef791f12b24417a400c4b6d386863c"
    ["deps/libsnark/depends/libff/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libsnark/depends/libff/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
    ["deps/libsnark/depends/libfqfft"]="7e1e957d0e84accadcf92e88162510c0ad886709"
    ["deps/libsnark/depends/libfqfft/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libsnark/depends/libfqfft/depends/gtest"]="3a4cf1a02ef4adc28fccb7eef2b573b14cd59009"
    ["deps/libsnark/depends/libfqfft/depends/libff"]="176f3f42fdef791f12b24417a400c4b6d386863c"
    ["deps/libsnark/depends/libfqfft/depends/libff/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libsnark/depends/libfqfft/depends/libff/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
    ["deps/libsnark/depends/libfqfft/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
    ["deps/libsnark/depends/libsnark-supercop"]="b04a0ea2c7d7422d74a512ce848e762196f48149"
    ["deps/libsnark/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
)

for path in "${!submodules[@]}"; do
    commit="${submodules[$path]}"
    if [ -d "$path" ]; then
        echo "Checking out $path to commit $commit"
        cd "$path" || exit
        git checkout "$commit"
        cd - > /dev/null
    else
        echo "Warning: Directory $path does not exist!"
    fi
done

echo "All submodules checked out to specified commits."
