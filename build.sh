cargo build --release --target x86_64-pc-windows-gnu
cargo build --release 
mv -f "./target/x86_64-pc-windows-gnu/release/lolclaimeramazon.exe" ./out/lolclaimeramazon.exe
mv -f "./target/release/lolclaimeramazon" ./out/lolclaimeramazon
cd out && strip lolclaimeramazon lolclaimeramazon.exe && upx --lzma lolclaimeramazon lolclaimeramazon.exe
