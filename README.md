# Host1x UAPI test

This is a test suite to test proper functionality of the Host1x and TegraDRM kernel-user interfaces,
as well as internal driver functionality.

## Compilation

### Prerequisites

Ensure that you have the Rust compiler installed. You can either cross-compile, or build the test suite natively. The easiest way to install the toolchain is by using rustup (http://rustup.rs).

If you are cross-compiling, make sure to install the cross-compilation target.

* For arm64: `rustup target add aarch64-unknown-linux-musl`
* For armv7: `rustup target add armv7-unknown-linux-musleabi`

Also ensure you have an appropriate linker for the target, most likely GCC.

### Setting up the Makefile

Edit `Makefile` and adjust the `LINKERS` variable to match the names of the linkers you want to use.

If you want to be able to execute the test suite remotely, also set up the other `REMOTE_` variables accordingly.

### Building and running

Run `make build-32` or `make build-64` to build the test suite. You should get a binary called `uapi-test-32` or `uapi-test-64` in the project root directory.

If you set up remote access, you can use `make run-remote-32` or `make run-remote-64` to run the test suite on the target system directly. You can append `ARGS="your arguments"` to run the test suite with command line parameters.

## Contributing

The project is licensed under the MIT license. To contribute, you need to add a Signed-off-by
tag to each of your commits, certifying the following:

### Developer’s Certificate of Origin 1.1

By making a contribution to this project, I certify that:

* The contribution was created in whole or in part by me and I have the right to submit it under the open source license indicated in the file; or
* The contribution is based upon previous work that, to the best of my knowledge, is covered under an appropriate open source license and I have the right under that license to submit that work with modifications, whether created in whole or in part by me, under the same open source license (unless I am permitted to submit under a different license), as indicated in the file; or
* The contribution was provided directly to me by some other person who certified (a), (b) or \(c\) and I have not modified it.
* I understand and agree that this project and the contribution are public and that a record of the contribution (including all personal information I submit with it, including my sign-off) is maintained indefinitely and may be redistributed consistent with this project or the open source license(s) involved.

(This is the same process as for the Linux kernel itself.)
