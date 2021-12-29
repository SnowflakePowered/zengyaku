# 善逆 zengyaku

GoodTools reverse engineering research project. 

## Suite
`zg` tools only work with **unpacked** GoodTools executables. The executable can be unpacked with [x32dbg](https://x64dbg.com) and the Scylla plugin.

### zg-find
Tries to find offsets given the CRC32, SHA1, and name of the first entry. 
The first entry is the first line in the `*Miss.txt` file when GoodTools is ran within an empty folder. 
The hash of the first entry can be found with [OpenGood](https://github.com/SnowflakePowered/opengood), unless if the first entry is missing; in that case, it will have to be determined manually with a reverse engineering suite.

```
zengyaku-find: GoodTools Address Finder

USAGE:
    zg-find.exe [OPTIONS] --crc <CRC> --sha1 <SHA1> --name <NAME> <EXE>

ARGS:
    <EXE>    The path to the executable to search

OPTIONS:
    -c, --crc <CRC>      The CRC32 value to search for
    -C, --print-args     Output command-line arguments for zg-dump
    -h, --help           Print help information
    -n, --name <NAME>    The name string to search for
    -s, --sha1 <SHA1>    The SHA1 value to search for
    -V, --version        Print version information
```

### zg-dump
Dumps CRC32 and SHA1 hashes given the known offsets of each table and the number of known ROMs.

```
zengyaku-dump: GoodTools database dumper

USAGE:
    zg-dump.exe [OPTIONS] --crc-off <CRC_OFF> --sha1-off <SHA1_OFF> --name-off <NAME_OFF> --known-num <KNOWN_NUM> <EXE>

ARGS:
    <EXE>    The path to the executable to dump

OPTIONS:
    -c, --crc-off <CRC_OFF>        The offset of the CRC32 table
    -e, --extension <EXTENSION>    The extension to use when saving an Logiqx XML file; if omitted, emits no file extensions in the resulting `rom` entries [default: ]
    -f, --format <FORMAT>          The format to output results [default: none] [possible values: none, tsv, xml]
    -h, --help                     Print help information
    -k, --known-num <KNOWN_NUM>    The total number of known ROMs
    -n, --name-off <NAME_OFF>      The offset of the name table
    -o, --output <OUTPUT>          The path to write output; if omitted, outputs to stdout
    -s, --sha1-off <SHA1_OFF>      The offset of the SHA1 table
    -V, --version                  Print version information
```

## Instructions

1. Unpack the GoodTools executable using your method of choice. [x32dbg](https://x64dbg.com) is suggested but not required.
2. Find the hash details of the first entry with [OpenGood](https://github.com/SnowflakePowered/opengood) or manually with Ghidra or IDA Pro.
3. Use `zg-find` to find the offsets of the first entry.
4. Use `zg-dump` to dump the database once the offsets are found.

## Frequently Asked Questions

### Doesn't this violate the GoodTools license?
There is nothing really *specific* to GoodTools in zengyaku's code; at its core it just searches an executable for some patterns, then tries to make sense of it. If you feed `zg-find` or `zg-dump` other executables in all likelyhood it will dump out some (albeit questionable) output. However, if you unpack a GoodTools EXE with a debugger, then run zengyaku on the result, that pretty clearly violates clause 3, if you care. For that reason, I do not publish any database information extracted directly from the GoodTools executable here.

### Should I use this?
Outside of intellectual curiosity, most definitely not. GoodTools does not contain any size or file extension information that [OpenGood](https://github.com/SnowflakePowered/opengood) provides for the same set of ROMs. OpenGood is also clearly not in violation of the GoodTools license as the information there was not directly extracted from the GoodTools executable, whereas zengyaku can be used to do so.

### Will you provide executables?
No, zengyaku is a source-only distribution. If you have trouble setting up a Rust toolchain, please take a look at [OpenGood](https://github.com/SnowflakePowered/opengood) and see if that would better fit your use case.

### Will you publish the extracted database?
Please take a look at [OpenGood](https://github.com/SnowflakePowered/opengood) and see if that would better fit your use case. While there are some missing ROMs owing to the nature of how OpenGood's data was collected, OpenGood provides additional information like file size and MD5 hashes as well. Publishing a database extracted directly from GoodTools is clearly in violation of clause 3 of the GoodTools license.

## Legal
zengyaku is licensed under the MIT License and is provided for educational purposes only.
