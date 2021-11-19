# kindly

`kindly` is a (hopefully) well-commented Rust implementation of a set-user-ID-_root_ program, similar to `sudo` but in a much reduced way.

__Notice__: this is not a security-hardened application (although it does take a few security measures, described below) and does not aim to replace battle-hardened tools like `sudo` or `doas`.

* Locks all memory pages mapped into its address space in order to avoid leaking information if sent to swap
* Reads the password from a tty using [`rpassword`](https://crates.io/crates/rpassword)
* Zeroes and drops the unencrypted password as soon as it is no longer needed through non-elidable operations
* Attempts to avoid timing attacks through ["constant-time"](https://www.chosenplaintext.ca/articles/beginners-guide-constant-time-cryptography.html) byte comparisons

## Building

```shell
# Let's get the code and build it
git clone https://github.com/vrmiguel/kindly
cd kindly && cargo build --release

# We now need to make `kindly` officially a set-user-ID-root program by enabling the set-user-ID bit for it
sudo chown root:root target/release/kindly && sudo chmod u+s target/release/kindly
```
