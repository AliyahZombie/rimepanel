#!/bin/sh
# Sets up Rust + zig (as C compiler/linker) in a bare sandbox without gcc.
set -e
curl -sSf https://sh.rustup.rs -o /tmp/rustup.sh
sh /tmp/rustup.sh -y --profile minimal --default-toolchain stable --no-modify-path >/tmp/rustup.log 2>&1
pip3 install --user -q ziglang
mkdir -p ~/bin
mk() { # name zig-target
cat > ~/bin/$1 <<PY
#!/usr/bin/env python3
import sys, subprocess
skip={'-Wl,--no-undefined-version','-Wl,--fix-cortex-a53-843419'}
args=[a for a in sys.argv[1:] if not a.startswith('--target=') and a not in skip and not a.startswith('-Wl,-plugin')]
sys.exit(subprocess.call([sys.executable,'-m','ziglang','cc','-target','$2']+args))
PY
chmod +x ~/bin/$1
}
mk cc "$(uname -m)-linux-gnu.2.28"
mk zcc-x86_64-musl x86_64-linux-musl
mk zcc-aarch64-musl aarch64-linux-musl
printf '#!/bin/sh\nexec python3 -m ziglang ar "$@"\n' > ~/bin/ar; chmod +x ~/bin/ar
cp ~/bin/ar ~/bin/zar
~/.cargo/bin/rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl >/dev/null 2>&1
echo toolchain-ready
