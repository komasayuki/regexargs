# regexargs

CLI for executing commands from standard input for regex user


## Install for Mac

```
$ brew tap komasayuki/regexargs
$ brew install regexargs
```



## Usage

usage: regexargs RegularExpression ExecuteCommand

- RegularExpression should be surrounded by ""

## Special String in ExecuteCommand
- {0}: inputed line
- {1}: 1st group in regular expression
- {2}: 2nd group in regular expression
- {x}: x nd group in regular expression


## Example

Changing the file extension of files in the folder

```
$ ls -1 | regexargs "(.*)\.(.*)" mv {0} {1}.csv
```


## Build

```
$ cargo build
```



## Test

```
$ cargo test
```