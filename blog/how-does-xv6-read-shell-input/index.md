<!--Attributes
title: How does xv6 read shell input?
link: how-does-xv6-read-shell-input
published_date: 2024-02-09
meta_description: How often does the shell read?
tags: kernel, syscall, xv6 
-->

# How many read syscalls are involved in reading from the shell?

## Background

I completed a project of [CS537](tab:https://pages.cs.wisc.edu/~remzi/Classes/537/Spring2018/) which is to add a trivial system call to the xv6 kernel that keeps track of the amount of times the `read()` syscall has been called.

```c
int getreadcount(void)
```

> Your system call returns the value of a counter (perhaps called readcount or something like that) which is incremented every time any process calls the read() system call. That's it!

## Instigator

```c
// getread.c
#include "types.h"
#include "user.h"
#include "stat.h"

int main(int argc, char* argv[])
{
	printf(1, "%d\n", getreadcount());
	exit();
}
```

Upon running the above program on a newly created xv6 shell, I found that the program outputs a number greater than 0. However, it does not call `read` anywhere. So why the discrepancy?

The answer turns out to be the very program I use to run my program ie. the xv6 shell. The shell runs processes by `fork`ing and then using a variant of `exec` upon the newly created child process.

But to run a given process, it must first fetch that from standard input (`stdin`), thus making use of the `read` syscall.

## Discovery

When the `getread` program is called with no arguments we get an increment in the number of `read` calls by the amount `program_name.length + 1`.

```text
Booting from Hard Disk..xv6...
cpu0: starting 0
sb: size 1000 nblocks 941 ninodes 200 nlog 30 logstart 2 inodestart 32 bmap start 58
init: starting sh
$ getread
8
$
```

Assuming the number of calls to `read` was 0 when the shell was initialized, the shell performed 8 calls to `read` in executing our program. This makes sense as `"getread".length = 7` and a string in C is terminated by a null character (`\0`), thus making it 8 characters that have to be read by the prompt.

Thus, without reading the shell's source code, we assume that the shell reads only a single character at a time.

## Further Experiments

I was curious how the newline (`\n`) and `^D` (`Ctrl+d`) characters factor into the calls to `read`. As it turns out they do not appear to factor into the number of times `read` is called when parsing input from the shell.

```text
Booting from Hard Disk..xv6...
cpu0: starting 0
sb: size 1000 nblocks 941 ninodes 200 nlog 30 logstart 2 inodestart 32 bmap start 58
init: starting sh
$ getread    // 8 chars
8
$ wc         // 3 chars
a            // 'a' + '\0' + '\n' = 2 chars (newline didn't factor)
1 1 2        // ^D pressed here (didn't factor)
$ getread    // 8 chars
21
$ wc         // 3 chars
a0 1 1       // 'a' + '\0' + ^D = 2 chars (^D didn't factor)
$ getread    // 8 chars
34
$ 
```

Thus, it seems likely that our assumption for the shell is true ie. it reads input one character at a time, ignoring newlines and `^D`s when calling `read`.

*One interesting thing to note is that if one gives multi-character input to `wc`, the number of calls to `read` will be lower than expected since most UNIX utlities read `N` characters at a time. `N = 512/1024/2048/4096` depending on the program author and is usually the target machine's page size.*

**Kindly communicate improvements via [e-mail](mailto:saumay03pro@gmail.com).**
