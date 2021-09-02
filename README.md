# brute-force

Password dictionary generator made in RUST.

The application creates as many entries as possible until reaching a maximum word length.

Chunk the dictionary into several files by providing the maximum file size in Mb.

```
# cargo run -- --file /home/bruteforce/out --length 8 --size 100
```

## License
This project is licensed under the terms of the MIT license. 
Check the [LICENSE](LICENSE.md) file out for license rights and limitations.