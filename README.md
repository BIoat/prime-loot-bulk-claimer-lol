# fuck this
i'm too dumb for rust

# amazon prime loot-bulk-claimer-lol

## mass amazon prime loot claimer for league of legends

WIP, not functional yet...

### Todo

- ~~solution for GIF captcha~~
- iterate through twitch and amazon accounts (bulk claim)
- mark completed accounts and export to claimed.txt
- error handling & tests
- headless geckodriver window
- workaround for IP/GEO based amazon redirection/lang

### Download pre-built binaries

- [windows](/out/lolclaimeramazon.exe)
- [linux](/out/lolclaimeramazon)


### Prerequisites / Building / Usage

##### Requirements

- [geckodriver](https://github.com/mozilla/geckodriver/releases) (with firefox installed)
- amazon prime account(s) with unclaimed gaming loot for league of legends
- riot account(s) which have not received loot yet

##### Build from source

- `sh ./build.sh`

##### Usage

###### Windows

- download [geckodriver](https://github.com/mozilla/geckodriver/releases) and install firefox
- Move "geckodriver.exe" to C:/Windows/System32/
- open cmd.exe and type `geckodriver` (should start on port 4444)
- run lolclaimeramazon.exe

###### Linux

- launch geckodriver
- execute the binary with `chmod +x ./lolclaimeramazon && ./lolclaimeramazon`
