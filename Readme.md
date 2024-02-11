Remote Clipboard

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
