#!/bin/bash

git submodule update --init --recursive deps/libiop

declare -A submodules
submodules=(
    ["deps/libiop"]="a2ed2ec2f3e85f29b6035951553b02cb737c817a"
    ["deps/libiop/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libiop/depends/benchmark"]="a9beffda0b89a6995372100456a4ad894d29b93b"
    ["deps/libiop/depends/gtest"]="a325ad2db5deb623eab740527e559b81c0f39d65"
    ["deps/libiop/depends/libff"]="9769030a06b7ab933d6c064db120019decd359f1"
    ["deps/libiop/depends/libfqfft"]="7d460caa27b87574fe0e8144e6a3a66b7bcfe770"
    ["deps/libiop/depends/xbyak"]="811f4959ee0dd36a3ccedd2d4d7460472dd19a14"
    ["deps/libiop/depends/libff/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libiop/depends/libff/depends/gtest"]="a325ad2db5deb623eab740527e559b81c0f39d65"
    ["deps/libiop/depends/libff/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
    ["deps/libiop/depends/libfqfft/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libiop/depends/libfqfft/depends/gtest"]="3a4cf1a02ef4adc28fccb7eef2b573b14cd59009"
    ["deps/libiop/depends/libfqfft/depends/libff"]="accdf9e761979ac8c95dced219cac0b4ad4a4799"
    ["deps/libiop/depends/libfqfft/depends/libff/depends/ate-pairing"]="e69890125746cdaf25b5b51227d96678f76479fe"
    ["deps/libiop/depends/libfqfft/depends/libff/depends/gtest"]="a325ad2db5deb623eab740527e559b81c0f39d65"
    ["deps/libiop/depends/libfqfft/depends/libff/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
    ["deps/libiop/depends/libfqfft/depends/xbyak"]="f0a8f7faa27121f28186c2a7f4222a9fc66c283d"
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
