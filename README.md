# Provenance

Tools for generating and validating the provenance of content on Humm.

A Humm provenance consists of several components:

- A version number of the provenance
- sha512 of content that provenance is being established for
- A whitelist of public keys of Roughtime protocol timestamping servers
- A public key of a server that can issue JWTs
- An asymmetric signed JWT including:
 - `aud` of the pub key of a device
 - `exp` time that is relatively short lived
 - sha512 of the whitelist of roughtime pub keys
- A signature from the device specified in `aud` of the sha512 hash of all the above
- A roughtime chain from several whitelisted servers of all the above

In JSON format it would look something like this for content `"foo"` with base64 hashes:

```javascript
{
 "version": "1",
 "content": "9/u6bgY2+JDlb7vzKD5STG+jIErimDgtYkdB0NxmODJuKCxBvl5CVNiCB3LFUYosWowMf37aGVlKfrU5RT4e1w==",
 "timers": {
  "pubs": [
    "iBVjxg/1j7y1+kQUTBYdTabxCppesU/07D4PMDJk2WA=",
    "bbT+RPS7zKX6w71ssPibzmwWqU9ffRV5oj2OresSmhE=",
    "gD63hSj3ScS+wuOeGrubXlq35N1c5Lby/S+T7MNTjxo=",
    "gD63hSj3ScS+wuOeGrubXlq35N1c5Lby/S+T7MNTjxo=",
    "gD63hSj3ScS+wuOeGrubXlq35N1c5Lby/S+T7MNTjxo=",
    "cj8GsiNlRkqiDElAeNMSBBMwrAl15hYPgX50+GWX/lA="
  ],
  "chain": [
   {
    "nonce": "aRMZDNcIqIAb5gaWKZX/Q//+zHtj160K0qKtjMQKW7ZhV9u3q62Zs8CVtXhZ0s9zeOfwQBOr6EbtWE/AE1IroA==",
    "pub": "gD63hSj3ScS+wuOeGrubXlq35N1c5Lby/S+T7MNTjxo=",
    "time": "2020-05-16T16:31:58Z",
    "sig": "nCz3owURBhNxRMSvhvgceghyYE7zgwJaUf28zMebMnd2lQW6soesNiAZc-XuXlopRTdqyXpio-A-n9WBiGvrHw",
   },
   {
    "nonce": "nvW0ov7ja7xWS/LO4ST3tE0qknk5AsgpP7mvxV5itf6japeUUGUccmwOCST0oWDWn9PXQxLvsrbKgj2bS2RjDg==",
    "pub": "bbT+RPS7zKX6w71ssPibzmwWqU9ffRV5oj2OresSmhE=",
    "time": "2020-05-16T16:31:59Z",
    "sig": "Mc5CGhRAh9G2X5j4vzFCJuUlr4DOij-ugZpzgZWee8ulZP0c6U0YFEXvA4SB4cZpJiY59SbGDGs8QICJoD92FQ",
   },
   {
    "nonce": "Vq4ETlFeoIILQoxQyIqADu9daioy7/FVHkkFf1JvoPgpf0uBGs5PX7MZ7qVmSbj+5bWB+MhUd9R6NnrBhGbJ0w==",
    "pub": "cj8GsiNlRkqiDElAeNMSBBMwrAl15hYPgX50+GWX/lA=",
    "time": "2020-05-16T16:31:60Z",
    "sig": "N7RSA8jua0woC8Wc1hkhEDD5Tg5Bugb6YriisAylZNqQNN0NuAv-t3WsU9HOK9PxPaozf1kLCgA57gcuT-uAWg",
   },
  ]
 }
 "jwt": {
  "pub": "AAAAC3NzaC1lZDI1NTE5AAAAIOKOtIB9W3NN9ignpYSXuYnkUYLT7QtLdfq/G0mkHcqx",
  "token": "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWUsImlhdCI6MTUxNjIzOTAyMn0.tyh-VfuzIxCyGYDlkBA7DfyjrqmSHu6pQ2hoZuFqUSLPNY2N0mpHb3nk5K17HWP_3cYHBw7AhHale5wky6-sVA",
 },
 "device": {
  "sig": "NkU4l2kxOJMOubW-gF-lfNJ8ZACONihXetKE7RvYcOdP5HQYM_YcjqjbnhkUcikn1Qr3HVXrwFoQ-MsH60S8Gw"
 }
}
```

The time server list was pulled from https://github.com/cloudflare/roughtime/blob/master/ecosystem.json

Note that _every_ time server listed in the ecosystem as at the time of writing is:

- UDP protocol
- ed25519 signed
- surrounded by metadata not needed to verify proofs

Other than the signature, most of the data in the ecosystem JSON is about getting
time proofs, not verifying them.

If we kick the signing can down the road to version 2+ of provenance then we can
simply assume `ed25519` for everything, again this will never break for existing
proofs, it simply might make it necessary to upgrade for new proofs at some point.

Also note that `ed25519` is a valid signature algorithm in JWT, listed as `EdDSA`.

Also note that `ed25519` has a pure rust implementation in dalek

This means we can use a single signing scheme for both times and JWTs, which is
neat.
