Hello All,

I am writing a command-line tool for Bcache This was prompted by Bcache's 
difficulting handling suspend/resume[[1]][[2]], and by my personal experience with 
managing Bcache setups. This is very much a work-in-progress. I've never written
a command-line tool, and I'm teaching myself Rust. My hope is to make easier 
Bcache managment by providing a central location for device configurations, and
an easy to use tool.

I call the tool 'bcachectl'. As I implied, it's written in Rust. I use TOML
for the configuration markup as supported by the `toml` crate. I've only tested
this on KDE Neon with 4.15.0 as the kernel. I make no guarintees that it will
work on other systems. Having said that, since bcachectl wraps Bcache's sysfs
interface, it should work on any kernel version that implements the same
interface as 4.15.0.

My current TODO list is:
 - Fully impliment the Bcache interface
 - Implement the `stat` subcommand. This command will return the current state
 of the given device.
 - Write the man page

The following is the current documetation as of 0.3.2:
```
bcachectl 0.3.2
A program to control Bcache devices

USAGE:
    bcachectl <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    load       Load configuration file
    set        Directly control a device
    suspend    Suspend all caching devices. Use 'load' when resuming
```

```
Directly control a device

USAGE:
    bcachectl set [OPTIONS] <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --cache-mode <cache-mode>
            Set caching mode. Modes are writethrough, writeback, writearound, and none

    -s, --sequential-cutoff <sequential-cutoff>    Set cutoff for sequential reads and writes

ARGS:
    <name>    The name of the device to control. Ex. bcache0
```

```
Load a configuration file

USAGE:
    bcachectl load [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>    Path to configuration file [default: /etc/bcache/bcache.conf]
```

```
Suspend all caching devices. Use 'load' when resuming

USAGE:
    bcachectl suspend [path]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path>    Path to configuration file [default: /etc/bcache/bcache.conf]
```

[1]: https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=823860
[2]: https://bugs.launchpad.net/ubuntu/+source/bcache-tools/+bug/1515780
