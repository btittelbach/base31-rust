# About

Implements Base31 encoding and decoding, which is useful to represent
large integers in a short, case-insensitive, alphanumeric format. 

The difference to Base36 is that in Base31 certain characters that are 
often misread (0, 1, i, I, l, L, o, 0) are not used.

# Motivation

1. I used kmanley's excellent [base31 golang lib](https://github.com/kmanley/base31) for a while now to generate readable serial numbers
2. ported the library to rust in process of learning rust

# License
[MIT](LICENSE)
