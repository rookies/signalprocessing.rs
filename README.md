# signalprocessing.rs

[![Build Status](https://travis-ci.org/rookies/signalprocessing.rs.svg?branch=master)](https://travis-ci.org/rookies/signalprocessing.rs)

## Overview
This is a small Rust library for Signal Processing algorithms, my first Rust project and mostly just for me to get a bit more familiar with the language. Maybe it will become bigger, but I won't promise anything. Feel free to use it for your own projects, see LICENSE.

## Data Structures
* **ZeroPaddedSignal**: models an infinite, zero-padded signal; simplifies the implementation of various algorithms

## Algorithms
* **Linear prediction**: takes a *ZeroPaddedSignal* and a vector of coefficients and creates a new *ZeroPaddedSignal*; see [Wikipedia: Linear prediction](https://en.wikipedia.org/wiki/Linear_prediction)
