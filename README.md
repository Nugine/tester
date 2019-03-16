# tester

## Install

    cargo install --git https://github.com/Nugine/tester.git

## Example

    $ tester ./test/hello
    Hello, world!
    code: 0
    real time: 3 ms
    user time: 0 ms
    sys time: 2 ms
    memory: 1460 KB

    $ echo hey | tester cat
    hey
    code: 0
    real time: 5 ms
    user time: 0 ms
    sys time: 4 ms
    memory: 2124 KB

    $ tester ping -a=-c --arg=5 -a=www.baidu.com > /dev/null
    code: 0
    real time: 4184 ms
    user time: 3 ms
    sys time: 6 ms
    memory: 2936 KB

## Usage

    tester 0.2.2
    Nugine <Nugine@163.com>

    USAGE:
        tester [OPTIONS] <target>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -a, --arg <args>...    

    ARGS:
        <target>

## Todo

more test cases!