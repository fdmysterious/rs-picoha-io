picoha: Simple Hardware Abstraction on Raspberry pico
========================================================

- Florian Dupeyron
- June - October 2023

Introduction
------------

Picoha (pico hardware abstraction) is a set of various firmware for the raspberry pico that implements
various basic hardware interfaces (I/Os, encoder, PWM, protocols like SPI, i2c, etc.). It is built as
a simple, robust set of probes that can be inserted in automated test benches for embedded systems. A simple
python library is as well provided for easy interaction with the hardware. The ultimate goal to set up a probe is:

1. Download the correct `.uf2` file and download it to your pico ;
2. Use the python library to automate your bench.

This project is inspired by the
[`picoha-io`](https://github.com/Panduza/picoha-io) project from
[panduza](https://panduza.github.io/panduza-doc).


Available firmwares
-------------------

- `firmware/picoha-io`: Simple GPIO control.
