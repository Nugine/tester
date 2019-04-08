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

    $ tester -j ping -- www.baidu.com -c 5 > /dev/null 
    {"code":0,"signal":null,"time":{"real":4051,"user":4,"sys":4},"memory":2748}

## Usage

    tester 0.3.1
    Nugine <nugine@foxmail.com>

    USAGE:
        tester [FLAGS] [OPTIONS] <target> [-- <args>...]

    FLAGS:
        -h, --help       Prints help information
        -j, --json       Json output
        -V, --version    Prints version information

    OPTIONS:
        -o, --output <output>    output file path (default stderr)

    ARGS:
        <target>     command to run
        <args>...    arguments to be passed

## Output declaration

```typescript
declare type TesterOutput = ({
    code: number
    signal: null
} | {
    code: null,
    signal: string
}) & {
    time: {
        real: number,
        user: number,
        sys: number
    },
    memory: number
}
```

## Changelog

+ Add option `--output` on `0.3.1`

+ Break changes on `0.3.0`

    delete `--arg`

    pass arguments by `[-- <args>...]`

## Todo

more test cases!