# 善逆 zengyaku

GoodTools reverse engineering research project. 

## Instructions

1. Unpack the GoodTools executable using your method of choice. [x32dbg](https://x64dbg.com) is suggested but not required.
2. Find the hash details of the first entry with [OpenGood](https://github.com/SnowflakePowered/opengood) or manually with Ghidra or IDA Pro. If the tool you are dumping is an old-style database, you may only need the CRC32 or name of the first entry.
3. Use `zg-find` to find the offsets of the first entry.
4. Use `zg-dump` to dump the database once the offsets are found.

`zg` tools only work with **unpacked** GoodTools executables. The executable can be unpacked with [x32dbg](https://x64dbg.com) and the Scylla plugin.

### zg-find

Helper to find offsets for the embedded database. For 'old-style' databases (generally prior to 3.2x), only the CRC32 or the name of the first entry is needed. For 'new style databases (3.2x+), all of the CRC32, SHA1, and name of the first entry is needed.
The first entry is the entry listed in first line in the `*Miss.txt` file when GoodTools is ran within an empty folder. 
Any necessary hashes for 'new-style' databases can be looked up with [OpenGood](https://github.com/SnowflakePowered/opengood) unless if the first entry is missing; in that case, it will have to be determined manually with a reverse engineering suite.

For example, to find offsets for GoodWSx (new-style)
```bash
$ zg-dump "GoodWSx_unpacked.exe" new --crc "2cbe41a6" --sha1 "28a0e1bccc4c10a57379f87c67c6c5ecf07fb0f4" --name "#Wonderwitch Promo Beta Demo by Dox (PD)"     
```

For GoodPico (old-style)
```bash
$ zg-find "GoodPico_unpacked.exe" old --crc "d62e3372"
```

A line of command arguments that can be pasted directly to `zg-dump` can be outputted with the `-C` flag.

### zg-dump
Dumps CRC32 and SHA1 hashes given the known offsets of each table and the number of known ROMs. Generally you should find offsets with `zg-find -C` and pipe the resulting command line arguments into `zg-dump`. 
`zg-dump` can also output Logiqx XML or TSV with the `-f` flag and `-o` flag to write output to a file. See `zg-dump --help` for more information.

## Frequently Asked Questions

### Doesn't this violate the GoodTools license?
There is nothing really *specific* to GoodTools in zengyaku's code; at its core it just searches an executable for some patterns, then tries to make sense of it. If you feed `zg-find` or `zg-dump` other executables in all likelyhood it will dump out some (albeit questionable) output. However, if you unpack a GoodTools EXE with a debugger, then run zengyaku on the result, that pretty clearly violates clause 3, if you care. For that reason, I do not publish any database information extracted directly from the GoodTools executable here.

### Should I use this?
Outside of intellectual curiosity, most definitely not. GoodTools does not contain any size or file extension information that [OpenGood](https://github.com/SnowflakePowered/opengood) provides for the same set of ROMs. OpenGood is also clearly not in violation of the GoodTools license as the information there was not directly extracted from the GoodTools executable, whereas zengyaku can be used to do so.

### Will you provide executables?
No, zengyaku is a source-only distribution. If you have trouble setting up a Rust toolchain, please take a look at [OpenGood](https://github.com/SnowflakePowered/opengood) and see if that would better fit your use case.

### Will you publish the extracted database?
Please take a look at [OpenGood](https://github.com/SnowflakePowered/opengood) and see if that would better fit your use case. While there are some missing ROMs owing to the nature of how OpenGood's data was collected, OpenGood provides additional information like file size and MD5 hashes as well. Publishing a database extracted directly from GoodTools is clearly in violation of clause 3 of the GoodTools license.

### What's the difference between an old-style and new-style database?
As a rule of thumb, new-style databases are used in V3.2x-tools, with older tools using the old-style database. To know for sure, you may need to try `zg-find` a few times until the resulting offsets make sense. New-style databases store hashes in separate contiguous binary arrays, whereas old-style databases is an array of strings containing ASCII-encoded hash information. `zg-dump` can handle both formats, but the proper offsets must be supplied. 

## Legal
zengyaku is licensed under the MIT License and is provided for educational purposes only.
