# nth_rs
Return or exclude the `nth` lines supplied from stdin as output on stdout. See the Python version of this utility here: [nth_py](https://github.com/alexdelorenzo/nth_py).

# Install

```bash
cargo install nth_rs
```

# Usage

```bash
$ dmesg | nth_rs 0 1 2 3
[    4.095065] xhci-hcd xhci-hcd.3.auto: xHCI Host Controller
[    4.100328] xhci-hcd xhci-hcd.3.auto: new USB bus registered, assigned bus number 4
[    4.107985] xhci-hcd xhci-hcd.3.auto: Host supports USB 3.0  SuperSpeed
[    4.109677] mmc_host mmc0: Bus speed (slot 0) = 50000000Hz (slot req 52000000Hz, actual 50000000HZ div = 0)

```

To better illustrate, let's enumerate each line of stdin:

```bash
$ dmesg | count | nth_rs 0 1 2 3
     0  [    4.095065] xhci-hcd xhci-hcd.3.auto: xHCI Host Controller
     1  [    4.100328] xhci-hcd xhci-hcd.3.auto: new USB bus registered, assigned bus number 4
     2  [    4.107985] xhci-hcd xhci-hcd.3.auto: Host supports USB 3.0  SuperSpeed
     3  [    4.109677] mmc_host mmc0: Bus speed (slot 0) = 50000000Hz (slot req 52000000Hz, actual 50000000HZ div = 0)
```

## Help
```bash
$ nth_rs --help
nth 0.2.0
AlexDeLorenzo.dev
Return the contents of stdin from the line numbers supplied as arguments.

USAGE:
    nth_rs [FLAGS] <LINES>...

FLAGS:
    -h, --help       Prints help information
    -r, --reverse    Write every line, except the line numbers supplied as LINES, from stdin to stdout.
    -V, --version    Prints version information

ARGS:
    <LINES>...    Line numbers to select

```

# License
See `LICENSE`. If you'd like to use this project with a different license, please get in touch.
