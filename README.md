# tester

## Install

    cargo install --git https://github.com/Nugine/tester.git

## Example

    $ tester ./test/hello
    Hello, world!
    code: 0
    time: 0 ms
    memory: 1384 KB

    $ tester ~/a+b < a+b.in
    3
    code: 0
    time: 0 ms
    memory: 1660 KB

    $ tester ~/a+b < a+b.in > a+b.out
    code: 0
    time: 0 ms
    memory: 1692 KB

## Usage

    tester 0.2.0
    Nugine <Nugine@163.com>

    USAGE:
        tester <target>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <target> 

## Todo

more test cases!