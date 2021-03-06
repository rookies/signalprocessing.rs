# signalprocessing.rs

[![Build Status](https://travis-ci.org/rookies/signalprocessing.rs.svg?branch=master)](https://travis-ci.org/rookies/signalprocessing.rs)

## Overview
This is a small Rust library for Signal Processing algorithms, my first Rust project and mostly just for me to get a bit more familiar with the language. Maybe it will become bigger, but I won't promise anything. Feel free to use it for your own projects, see LICENSE.

## Data Structures
* **ZeroPaddedSignal**: models an infinite, causal, zero-padded signal; simplifies the implementation of various algorithms; see [Wikipedia: Causal system](https://en.wikipedia.org/wiki/Causal_system)
* **MaximumLengthSequence**: models a maximum length sequence generator; see [Wikipedia: Maximum length sequence](https://en.wikipedia.org/wiki/Maximum_length_sequence)

## Algorithms
* **linear prediction**: takes a *ZeroPaddedSignal* and a vector of coefficients and creates a new *ZeroPaddedSignal*; see [Wikipedia: Linear prediction](https://en.wikipedia.org/wiki/Linear_prediction)

## Planned features
* calculating linear prediction coefficients using covariance method
* calculating linear prediction coefficients using autocorrelation method
* discrete Fourier transform
* inverse discrete Fourier transform
* pre-defined polynoms for *MaximumLengthSequence*
* *ZeroPaddedSignal*-like functions for *MaximumLengthSequence*
* operators (Add,Sub,Mul,Div,...) for *ZeroPaddedSignal*
* *findpeaks* function
