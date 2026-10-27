# Rofitabs

Rofitabs is a minimal extension with the native portion written in rust for switching between tabs in Firefox using Rofi. There are two native executables (`rofitabs` and `rofitabsmediator`). The `rofitabs` executable is the CLI client that can query tab ids and titles from firefox (just run `rofitabs`) and activate them (`rofitabs <tabId>`). 

I've been a long time user of Brotab and this project takes inspiration from Brotab. I realized that I only needed the ability to list tabs and activate them, so I wrote a minimal version.

## Installation

### Native Component

Using nix:

```
nix run github:ccoverstreet/rofitabs#install 
```

### Browser Extension

I have yet to publish the rofitabs extension to Firefox extensions, but the extension can be installed using the about:debugging page and adding a temporary browser extension using the manifest located in `extensions/manifest.json`.
