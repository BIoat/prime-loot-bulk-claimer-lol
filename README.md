# amazon prime loot-bulk-claimer-lol

mass amazon prime loot claimer for league of legends
project is still WIP. Non functional

### Todo

- Manual captcha solving
- iterate through twitch and amazon accounts (bulk claim)
- error handling
- headless geckodriver window

### Prerequisites / Building / Usage

##### Requirements

- [geckodriver](https://github.com/mozilla/geckodriver/releases) (with firefox installed)
- amazon prime account(s) with unclaimed gaming loot for league of legends
- riot account(s) which have not received loot yet

##### Build from source

- `bash ./build.sh`

##### Download

- [windows](/out/geckodriver.exe)
- [linux](/out/geckodriver)

##### Usage

###### Windows

- Download geckodriver from The Requirements link above
- Move The executable to C:/Windows/System32/
- open cmd.exe and type "geckodriver" (should start on port 4444)
- run lolamazonclaimer.exe

###### Linux

- launch geckodriver
- execute the binary with `bash chmod +x ./lolamazonclaimer && ./lolamazonclaimer`
