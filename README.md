# Rofitabs

Rofitabs is a minimal extension with the native portion written in rust for switching between tabs in Firefox using Rofi. There are two native executables (`rofitabs` and `rofitabsmediator`). The `rofitabs` executable is the CLI client that can query tab ids and titles from firefox (just run `rofitabs`) and activate them (`rofitabs <tabId>`). 

I've been a long time user of Brotab and this project takes inspiration from Brotab. 

## Installation

### From this repo

Using nix

```
nix run .#install 
```
