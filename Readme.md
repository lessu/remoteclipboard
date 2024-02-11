Remote Clipboard

# Limitation
- This version is uncompleted. you may occur **BUGS**.

- Failsafe is not done carefully.

- The max clipboard buffer size is limited to a mtu size (1400).
as it use tcp to comminucate,and multi package is not included yet.

- Multi-user/Multi-slot is not completed(would be added in furture)

- No encrypt and no safety guaranteed. Please use it in LAN.
A safe alternative is to use ssh.(see srcc/srcp in this project folder as an example)

# Description
Remote clipboard has 3 bins.
- `rcp`: remote-clipboard paste
- `rcc`: remote-clipboard copy
- `rcd`: remote-clipboard daemon(server)

## `rcd`
```
A remote server for rcc,rcp
usage:
    rcd [OPTIONS]
    options:
        -h: help
        -l <addr>: listen address,eg 0.0.0.0, default localhost
        -p <port>: listen port, default 3298
```
## `rcc`
```
It will save clipboard content from <Text> to remote.
usage:
    rcc [OPTIONS] <Text>
    options:
        -h: help
        -p <profile_path>: a conf file containing the rcd server address
```

## `rcp`
```
It will output clipboard content to std.
usage:
    rcp [OPTIONS]
    options:
        -h: help
        -p <profile_path>: a conf file containing the rcd server address
```

## config file
Default
```
host=localhost
port=3298
```
# Build
```
cargo build --release
```

cross compile
```
W.I.P
```

# Install
copy
`target/release/{rcp,rcc,rcd}` to you path


# Example
## On server
```
rcd -l 0.0.0.0 -p 3298
```

## On copy client

edit `~/.rclip_profile`
```
host=<to-server>
port=3298
```

```
rcc 12345
```

## On paste client
edit `~/.rclip_profile` too
```
rcp 
```
output:
```
12345
```