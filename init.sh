#!/bin/sh

# check is the command exist in the computer
exist() {
    which $1 > /dev/null
    return $?
}

if ! exist cargo ; then
    echo "The project need rust (and cargo more specifically), please install it!"
    exit 1
fi

# install cargo watch
cargo install cargo-watch
npm i
