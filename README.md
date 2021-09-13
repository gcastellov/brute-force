[![Rust](https://github.com/gcastellov/brute-force/actions/workflows/rust.yml/badge.svg)](https://github.com/gcastellov/brute-force/actions/workflows/rust.yml)

# brute-force

Password dictionary generator made in RUST.

The application creates as many entries as possible until reaching a maximum word length.

## Options:

|Keyword|Description|Optional|Default|
|-------|-----------|-------|-------|
|--length|Executes until reaching the word length.|
|--file|The output file name path.|
|--size|Sets the maximum file size in Mb.|
|--start-with|Starts composing words from certain length.| true | 0 |
|--start-with-char|Starting character when composing words. Can be used in combination of --start-with. | true | ' ' |
|--verbose|Indicates whether it has to show output info or not. | true | false |

## Execute

```
# cargo run -- --file /home/bruteforce/out --length 8 --size 100
```

or

```
# brute-force --file /home/bruteforce/out --length 8 --size 100 --verbose true
```

It will show progress while creating entries.

## License
This project is licensed under the terms of the MIT license. 
Check the [LICENSE](LICENSE.md) file out for license rights and limitations.
