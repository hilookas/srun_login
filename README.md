# SRun Login for Rust

Command line login for SRun Auth Gateway.

## Usage

`./srun_login AUTH_GW_ENDPOINT AC_ID LOCAL_IP USERNAME PASSWORD`

eg:

`./srun_login "http://10.1.1.2" 1 10.233.2.97 200034322 password`

## Build

Take openwrt on mipsel as example.

Build stage needs Linux environment (like Ubuntu) to run openwrt toolchain (linker needed)

```bash
apt update && apt install build-essential -y

cd
wget https://downloads.openwrt.org/snapshots/targets/ramips/mt7621/openwrt-toolchain-ramips-mt7621_gcc-11.2.0_musl.Linux-x86_64.tar.bz2
tar xvf openwrt-toolchain-ramips-mt7621_gcc-11.2.0_musl.Linux-x86_64.tar.bz2

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add mipsel-unknown-linux-musl

git clone https://github.com/hilookas/srun_login.git
cd srun_login
CARGO_TARGET_MIPSEL_UNKNOWN_LINUX_MUSL_LINKER=~/openwrt-toolchain-ramips-mt7621_gcc-11.2.0_musl.Linux-x86_64/toolchain-mipsel_24kc_gcc-11.2.0_musl/bin/mipsel-openwrt-linux-musl-gcc cargo build --release --target=mipsel-unknown-linux-musl

# build result is placed at ~/srun_login/target/mipsel-unknown-linux-musl/release/srun_login
```