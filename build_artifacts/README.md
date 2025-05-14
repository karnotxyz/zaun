> _This file contains pre-compiled binaries for all the smart contract which `zaun` uses_.

## Compile from source

To simplify building each contract across many different external dependencies
and versions, a `Dockerfile` is provided to run the build in a containerized
environment. To build from scratch, simply run:

```bash
make artifacts
```

> [!CAUTION]
> This will **delete** all existing build artifacts!

This will pull the repositories for all external contracts, download and
configure external dependencies and versions, compile each contract and copy
the resulting binaries into this folder.
