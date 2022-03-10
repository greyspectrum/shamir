# shamir

This is a simple demonstration of threshold scheme, using Adi Shamir's [secret
sharing](http://web.mit.edu/6.857/OldStuff/Fall03/ref/Shamir-HowToShareASecret.pdf).

Shamir secret sharing enables us to divide a secret *S*, into *k* shares, such
that *S* is reconstructable from a threshold of *k* pieces, but knowledge of
*k* - 1 reveals nothing about *S*. Threshold schemes can be used to mitigate
the threat of authorized insiders abusing their privileged access to a system,
since a malicious insider must compromise a threshold of her colleagues' secret
shares, or induce them to collaborate with her, in order to reconstruct *S*. A
common example of this concept in action is the so-called ["two man
rule"](https://en.wikipedia.org/wiki/Two-man_rule) used to control the use of
nuclear weapons.

Since the security of Shamir secret sharing is dependent upon no single
individual knowing *S*, it is unfortunate that *S* remains vulnerable when *k*
shares are generated, as well as when these shares are used to compute *S*, in
order to authorize some action in a computer system. In order to reduce this
threat, we can compute *S* in a Trusted Execution Environment (TEE), using
[Enarx](https://enarx.dev/).

## BUILD

``` cargo build --release --target=wasm32-wasi ```
