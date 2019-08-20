## wasm-twitter-card

A humble pre-release library that does one thing only: provides a `generate_text` function for creating a twitter card-size image (630 x 1200 px) returned as a Uint8Array of pixels.

`generate_text` accepts two arguments: a title and author. It creates an image with white monospace text with an empty background, e.g.:

![gatsby-remark-twitter-cards in action](https://i.imgur.com/UGFRs9g.png)

It was made for the use case described in [`gatsby-remark-twitter-cards`](https://github.com/alessbell/gatsby-remark-twitter-cards).
