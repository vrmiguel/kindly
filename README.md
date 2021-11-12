# kindly

`kindly` is a (hopefully) well-commented Rust implementation of a set-user-ID-_root_ program, similar to `sudo` but in a much reduced way.

## Building

```shell
# Let's get the code and build it
git clone https://github.com/vrmiguel/kindly
cd kindly && cargo build --release

# We know need to make `kindly` officially a set-user-ID-root program by enabling the set-user-ID bit for it
sudo chown root:root target/release/kindly && sudo chmod u+s target/release/kindly
```