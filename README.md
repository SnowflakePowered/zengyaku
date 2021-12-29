# 善逆 zengyaku

GoodTools reverse engineering research project. 

Tested against GoodNES and GoodSNES V3.23b

## Suite
`zg` tools only work with **unpacked** GoodTools executables. The executable can be unpacked with [x32dbg](https://x64dbg.com) and the Scylla plugin.

### zg-find
Tries to find offsets given the CRC32, SHA1, and name of the first entry. 
The first entry is the first line in the `*Miss.txt` file when GoodTools is ran within an empty folder. 
The hash of the first entry can be found with [OpenGood](https://github.com/SnowflakePowered/opengood), unless if the first entry is missing; in that case, it will have to be determined manually with a reverse engineering suite.

```
GoodTools Address Finder

USAGE:
    zg-find.exe --crc <CRC> --sha1 <SHA1> --name <NAME> <EXE>

ARGS:
    <EXE>
            The name of the executable to dump

OPTIONS:
    -c, --crc <CRC>
            The CRC32 value to search for

    -h, --help
            Print help information

    -n, --name <NAME>
            The name to search for

    -s, --sha1 <SHA1>
            The SHA1 value to search for

    -V, --version
            Print version information
```

### zg-dump
Dumps CRC32 and SHA1 hashes given the known offsets of each table.

```
GoodTools Database Dumper

USAGE:
    zg-dump.exe [OPTIONS] --crc-off <CRC_OFF> --sha1-off <SHA1_OFF> --name-off <NAME_OFF> --known-num <KNOWN_NUM> <EXE> [FORMAT]

ARGS:
    <EXE>       The name of the executable to dump
    <FORMAT>    Output format [default: none] [possible values: none, tsv, xml]

OPTIONS:
    -c, --crc-off <CRC_OFF>        The offset of the CRC32 table
    -e, --extension <EXTENSION>    [default: ]
    -h, --help                     Print help information
    -k, --known-num <KNOWN_NUM>    Total number of known ROMs
    -n, --name-off <NAME_OFF>      The offset of the name table
    -o, --output <OUTPUT>          The name of the executable to dump
    -s, --sha1-off <SHA1_OFF>      The offset of the SHA1 table
    -V, --version                  Print version information
```

## Instructions

1. Unpack the GoodTools executable using your method of choice. [x32dbg](https://x64dbg.com) is suggested but not required.
2. Find the hash details of the first entry with [OpenGood](https://github.com/SnowflakePowered/opengood) or manually with Ghidra or IDA Pro.
3. Use `zg-find` to find the offsets of the first entry.
4. Use `zg-dump` to dump the database once the offsets are found.

## Legal
zengyaku is licensed under the MIT License.

Unlike OpenGood, using zengyaku violates clause 3 of the GoodTools license. zengyaku is provided for educational purposes only.