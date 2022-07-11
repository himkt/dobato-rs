## Dobato - 堂鳩

[Dobato (堂鳩)](https://www.suntory.co.jp/eco/birds/encyclopedia/detail/4642.html)
is a simple tool to send a message to Discord. This is the Rust implementation of
https://github.com/himkt/dobato


### Install

(TBD)
Download the appropriate binary for your environment.
Place the binary where executable programs are located.


### Configure discord webhook

Run `dobato setup`.

```bash
> ./dobato-rs setup
webhook URL: https://webhook.example.com
Stored webhook to /Users/himkt/.config/dobato/webhook.txt
```


### Notify as a command

```bash
dobato-rs post --text "Hello, World."
```
