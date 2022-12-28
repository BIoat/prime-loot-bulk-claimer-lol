cargo build --release --target x86_64-pc-windows-gnu
cargo build --release 
mv -f "./target/x86_64-pc-windows-gnu/release/captchasolver.exe" ../out/captchasolver.exe
mv -f "./target/release/captchasolver" ../out/captchasolver
cd ../out && strip captchasolver captchasolver.exe && upx --lzma captchasolver captchasolver.exe
