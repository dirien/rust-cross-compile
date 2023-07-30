## Introduction

In this blog post, we will have a look at how to cross-compile your Rust applications using cross-rs and GitHub Actions. We will also have a look at how to use the cross-rs Docker image to cross-compile your Rust applications locally.

But before we dig into the details, let's have a look at multi-platform support in Rust.

### Motivation for multi-platform support

Nowadays, it is common to use different operating systems and architectures. We have IoT devices that run on ARM processors and servers which run on x86 processors. Or Apple computers that run on Apple Silicon processors. And then we have Windows, Linux and macOS. Enough reasons to support multiple platforms when developing software.

But now comes the downside: Most OS APIs are not compatible with each other. This difference in APIs is the reason why have to create platform-dependent code.

### How Rust supports multi-platform

The good news is that Rust makes it easy to write multi-platform code. Rust has a built-in macro called `cfg` which enables us the conditional compilation of code. It supports a lot of options so we can easily write platform-dependent parts of our code depending on the target platform.

[https://doc.rust-lang.org/reference/conditional-compilation.html](https://doc.rust-lang.org/reference/conditional-compilation.html)

Some examples of `cfg`:

```rust
#[cfg(target_os = "linux")]
fn main() {
    println!("This is Linux");
}

#[cfg(target_os = "macos")]
fn main() {
    println!("This is macOS");
}
```

The `cfg` macro also supports `any`, `all` and `not`:

* `any` - If any of the given predicates is true, the code is included.

* `all` - If all of the given predicates are true, the code is included.

* `not` - If the given predicate is false, the code is included.


```rust
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn main() {
    println!("This is Linux or macOS");
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn main() {
    println!("This is Linux on x86_64");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("This is not Windows");
}
```

Rust also supports platform-dependent dependencies. We can use the `target` attribute to specify dependencies for a specific platform. The `cfg` syntax is also supported.

```toml
[dependencies]
# This dependency is only used on Linux
[target.'cfg(unix)'.dependencies]
openssl = "1.0.1"
```

Same for `dev-dependencies` and `build-dependencies`:

```toml
[dev-dependencies]
# This dev-dependency is only used on Linux
[target.'cfg(unix)'.dev-dependencies]
openssl = "1.0.1"

[build-dependencies]
# This build-dependency is only used on Linux
[target.'cfg(unix)'.build-dependencies]
openssl = "1.0.1"
```

### Rust support tiers

Rust is organized in support tiers when it comes to multi-platform support. The Rust team provides different levels of support for different platforms. The support tiers are:

* <mark>Tier 1 </mark> \- Tier 1 platforms are "guaranteed to work", this is the highest level of support

* <mark>Tier 2</mark> - Tier 2 platforms are "guaranteed to build", but not necessarily to pass all tests

* <mark>Tier 3</mark> - Tier 3 platforms are those for which the Rust code has support, but which are not built or tested automatically. So there is no guarantee that they work.


According to the Rust team, [the following platforms are Tier 1](https://doc.rust-lang.org/nightly/rustc/platform-support.html):

* aarch64-unknown-linux-gnu

* i686-pc-windows-gnu

* i686-pc-windows-msvc

* i686-unknown-linux-gnu

* x86\_64-apple-darwin

* x86\_64-pc-windows-gnu

* x86\_64-pc-windows-msvc

* x86\_64-unknown-linux-gnu


### `cross-rs` to the rescue

%[https://github.com/cross-rs/cross] 

With [`cross-rs`](https://github.com/cross-rs/cross) we have an easy way to cross-compile our Rust applications. `cross-rs` works by using pre-made Dockerfiles to build and run your application inside a Docker container. The list of supported platforms is quite long, have a look at the following link for more information:

%[https://github.com/cross-rs/cross/tree/main/docker] 

Here is a short selection of supported platforms:

1. `Dockerfile.x86_64-unknown-linux-gnu`

2. `Dockerfile.x86_64-pc-windows-gnu`

3. `Dockerfile.aarch64-unknown-linux-gnu`

4. `Dockerfile.i686-unknown-linux-gnu`

5. `Dockerfile.i686-pc-windows-gnu`

6. `Dockerfile.armv7-unknown-linux-gnueabihf`

7. `Dockerfile.riscv64gc-unknown-linux-gnu`

8. `Dockerfile.mips64-unknown-linux-gnuabi64`

9. `Dockerfile.powerpc64le-unknown-linux-gnu`

10. `Dockerfile.x86_64-unknown-linux-musl`


Pretty impressive, right?

Let us create a simple demo project to see `cross-rs` in action.

## Prerequisites

* [Rust](https://www.rust-lang.org)

* An IDE or text editor of your choice

* [Docker](https://www.docker.com)

* GitHub account

* [GitHub CLI](https://cli.github.com)


## Initialize the demo project

The demo project is a simple Rust application that prints a [FIGlet](http://www.figlet.org/) text to the console. We use clap to parse the command line arguments. That's it. Nothing fancy!

```bash
cargo init --bin figctl
```

Then we add the `clap` dependency:

```bash
cargo add clap --features derive
```

And then we add a `figlet-rs` dependency:

```bash
cargo add figlet-rs
```

Now we can add the following code to `src/main.rs`:

```rust
use clap::{Parser, Args};
use figlet_rs::FIGfont;

#[derive(Parser, Debug)]
struct FigletCtl {
    message: String,
}

fn main() {
    let args = FigletCtl::parse();
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(args.message.as_str());
    println!("{}", figure.unwrap());
}
```

If you run the application now, you should see the following output:

```bash
➜ cargo run -q -- 'Hello World!'
  _   _          _   _            __        __                 _       _   _ 
 | | | |   ___  | | | |   ___     \ \      / /   ___    _ __  | |   __| | | |
 | |_| |  / _ \ | | | |  / _ \     \ \ /\ / /   / _ \  | '__| | |  / _` | | |
 |  _  | |  __/ | | | | | (_) |     \ V  V /   | (_) | | |    | | | (_| | |_|
 |_| |_|  \___| |_| |_|  \___/       \_/\_/     \___/  |_|    |_|  \__,_| (_)
```

## Cross-compile the demo project

You can install `cross-rs` with the following command:

```bash
cargo install cross --git https://github.com/cross-rs/cross
```

And let test it with a simple example.

```bash
cross build --target aarch64-unknown-linux-gnu
```

Now you should have a `target/aarch64-unknown-linux-gnu/debug/figctl` binary. Let's run it and see what happens.

```bash
➜ ./target/aarch64-unknown-linux-gnu/debug/figctl "Hello World!"
zsh: exec format error: ./target/aarch64-unknown-linux-gnu/debug/figctl
```

The binary is not executable on our host system. But we can run it inside a Docker container.

```bash
docker run --rm -it -v $(pwd):/app  --platform=linux/arm64 -w /app rust ./target/aarch64-unknown-linux-gnu/debug/figctl 'Hello World!'
  _   _          _   _            __        __                 _       _   _ 
 | | | |   ___  | | | |   ___     \ \      / /   ___    _ __  | |   __| | | |
 | |_| |  / _ \ | | | |  / _ \     \ \ /\ / /   / _ \  | '__| | |  / _` | | |
 |  _  | |  __/ | | | | | (_) |     \ V  V /   | (_) | | |    | | | (_| | |_|
 |_| |_|  \___| |_| |_|  \___/       \_/\_/     \___/  |_|    |_|  \__,_| (_)
```

It works! So let's see how we can use `cross-rs` to build our application for multiple platforms with GitHub Actions.

## GitHub Actions

We need to create a file called `.github/workflows/ci.yml` and add the following content:

```yaml
name: build and release

on:
  workflow_dispatch:
  release:
    types: [ created ]

permissions:
  contents: write

jobs:
  build:
    name: ${{ matrix.platform.os_name }} with rust ${{ matrix.toolchain }}
    runs-on: ${{ matrix.platform.os }}
    strategy:
      fail-fast: false
      matrix:
        platform:
          - os_name: Linux-aarch64
            os: ubuntu-20.04
            target: aarch64-unknown-linux-musl
            bin: figctl-linux-arm64
          - os_name: Linux-x86_64
            os: ubuntu-20.04
            target: x86_64-unknown-linux-gnu
            bin: figctl-linux-amd64
          - os_name: Windows-x86_64
            os: windows-latest
            target: x86_64-pc-windows-msvc
            bin: figctl-amd64.exe
          - os_name: macOS-x86_64
            os: macOS-latest
            target: x86_64-apple-darwin
            bin: figctl-darwin-amd64
          - os_name: macOS-aarch64
            os: macOS-latest
            target: aarch64-apple-darwin
            bin: figctl-darwin-arm64
        toolchain:
          - stable
    steps:
      - uses: actions/checkout@v3
      - name: Build binary
        uses: houseabsolute/actions-rust-cross@v0
        with:
          command: "build"
          target: ${{ matrix.platform.target }}
          toolchain: ${{ matrix.toolchain }}
          args: "--locked --release"
          strip: true
      - name: Rename binary (linux and macos)
        run: mv target/${{ matrix.platform.target }}/release/figctl target/${{ matrix.platform.target }}/release/${{ matrix.platform.bin }}
        if: matrix.platform.os_name != 'Windows-x86_64'
      - name: Rename binary (windows)
        run: mv target/${{ matrix.platform.target }}/release/figctl.exe target/${{ matrix.platform.target }}/release/${{ matrix.platform.bin }}
        if: matrix.platform.os_name == 'Windows-x86_64'
      - name: Generate SHA-256
        run: shasum -a 256 target/${{ matrix.platform.target }}/release/${{ matrix.platform.bin }} | cut -d ' ' -f 1 > target/${{ matrix.platform.target }}/release/${{ matrix.platform.bin }}.sha256
      - name: Release binary and SHA-256 checksum to GitHub
        uses: softprops/action-gh-release@v1
        with:
          files: |
            target/${{ matrix.platform.target }}/release/${{ matrix.platform.bin }}
            target/${{ matrix.platform.target }}/release/${{ matrix.platform.bin }}.sha256
```

This workflow will build the application for the following platforms:

* Linux-aarch64

* Linux-x86\_64

* Windows-x86\_64

* macOS-x86\_64

* macOS-aarch64


It will also create a GitHub release for each platform.

Let's push the changes to GitHub and create a new release.

```bash
git add .
git commit -m "Add all the things"
git push
```

Now go to the GitHub repository and create a new release or use the GitHub CLI.

```bash
➜ gh release create v0.1.0 
? Title (optional) My figctl cli
? Release notes Leave blank
? Is this a prerelease? No
? Submit? Publish release
https://github.com/dirien/rust-cross-compile/releases/tag/v0.1.0
```

This will trigger the GitHub Actions workflow and build the application for all the platforms.

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1690752217312/ce86de1f-7439-4eca-a7a6-25ae4b9b1b06.png align="center")

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1690752244717/c4142236-0ede-4ba8-acd5-c52f6c412fc3.png align="center")

## Conclusion

Creating a cross-platform application with Rust is easy as Rust has a great toolchain and ecosystem. But it's not always easy to build the application for multiple platforms. With `cross-rs` and GitHub Actions, we can build our application for multiple platforms and with GitHub Releases, we can distribute the application to our users.

## Links

* https://github.com/cross-rs/cross

* https://doc.rust-lang.org/nightly/rustc/platform-support.html

* https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
