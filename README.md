# shamir

This is a simple demonstration of a threshold scheme, using Adi Shamir's [secret
sharing](http://web.mit.edu/6.857/OldStuff/Fall03/ref/Shamir-HowToShareASecret.pdf).

Shamir secret sharing enables us to divide a secret *S*, into *k* shares, such
that *S* is reconstructable from a threshold of *k* pieces, but knowledge of *k* -
1 reveals nothing about *S*. Threshold schemes can be used to mitigate the
threat of authorized insiders abusing their privileged access to a system, since
a malicious insider must compromise a threshold of her colleagues' secret
shares, or induce them to collaborate with her, in order to reconstruct *S*. A
common example of this concept in action is the so-called ["two man
rule"](https://en.wikipedia.org/wiki/Two-man_rule) used to control the use of
nuclear weapons.

Since the security of Shamir secret sharing is dependent upon no single
individual knowing *S*, it is unfortunate that *S* remains vulnerable when *k*
shares are generated, as well as when these shares are used to compute *S*, in
order to authorize some action in a computer system. In order to reduce this
threat, we can compute *S* in a Trusted Execution Environment (TEE), using
[Enarx](https://enarx.dev/), which reduces the size of the trusted computing
base where *S* is exposed at secret sharing and reconstruction time.

A description of Enarx's threat model can be found
[here.](https://enarx.dev/docs/Technical/Threat)

## BUILD

To run shamir in a TEE, first install
[Enarx](https://enarx.dev/docs/Installation/Introduction) and its dependencies.

Compile shamir to a WebAssembly target:

`cargo build --release --target=wasm32-wasi`

## USAGE

To run shamir in an Enarx Keep:

`enarx run target/wasm32-wasi/release/shamir.wasm`

Arguments:

```
shamir --help
shamir 0.1.0

USAGE:
    shamir --launchcode <LAUNCHCODE>

OPTIONS:
    -h, --help                       Print help information
    -l, --launchcode <LAUNCHCODE>    
    -V, --version                    Print version information
```
