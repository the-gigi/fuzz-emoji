# fuzz-emoji

A fuzzy search for emojis. If you're looking for an emoji by name, but you don't know exactly
what's the name fuzz-emoji is here to help.

For example, suppose you want to get the `fire` emoji 🔥, but you don't know it's called `fire`.
You can ask fuzz-emoji for `flame` and it will return the 🔥. How cool is that?

# The matrix

The project implements the same functionality in 3 different programming languages, and it deploys
each implementation as a serverless function on one of the major cloud providers


Language Deployment
Python   AWS lambda
Golang   Google cloud functions
Rust     Azure functions


