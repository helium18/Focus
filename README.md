# Focus
Keep distractive apps at bay allowing you to focus!

---
## What it does.
The apps specified are killed automatically after a period of 5 seconds till the clock runs out. Once its done, `focus` closes on its own.

![Focus](https://i.imgur.com/P4LiTwT.png)

## Why?
For some, it may just prevent them from opening distractive apps. For others, its just an added layer of trouble (so that lazy people like me lose interest from opening the app all together)

---
## Is it heavy? 
No, it eats barely a few kbs of memory.

---
## Install 
Download the deb package from [here](https://github.com/amd176/Focus/releases/tag/Beta)

---
## Build

- Dependencies: [Rust](https://www.rust-lang.org/tools/install) and `pkill`

```
git clone https://github.com/amd176/Focus.git
cd ~/Focus/src
cargo build --release
cd ..
cd target/release
./focus
```

`focus` can be run from the `~/Focus/target/release/` directory or you can just paste the rust binary in one of the `$PATH` folders so that it can opened quickly.

---
## Uninstall :(

Built: Just remove the `~/Focus` directory, or the rust binary which you've pasted in one of the `$PATH` folders. If you're facing a problem, consider raising an issue.
deb: Uninstall it via the original deb file.

---
## Support 

Star the repo 😳.

---
