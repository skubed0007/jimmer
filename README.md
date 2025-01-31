# Jimmer Timer
================

A customizable timer application with a quote widget, built using Rust and the TUI library.

## Table of Contents
-----------------

* [Features](#features)
* [Installation](#installation)
* [Usage](#usage)
* [Configuration](#configuration)
* [Themes](#themes)
* [Quote Widget](#quote-widget)
* [Audio](#audio)
* [Troubleshooting](#troubleshooting)
* [Contributing](#contributing)
* [License](#license)

## Features
-----------

* Customizable timer duration in minutes and seconds
* 10 pre-defined themes for the timer and quote widgets
* Quote widget with a collection of inspirational quotes
* Audio playback during the timer (loop and end sounds)
* User-friendly interface with keyboard navigation

## Installation
------------
``curl -sSfL https://raw.githubusercontent.com/skubed0007/jimmer/master/ci.sh | sudo bash``
## Usage
-----

To use the Jimmer Timer, simply run the application and follow the on-screen instructions.

* Use the `--minute` and `--second` options to set the timer duration.
* Use the `--theme` option to choose a theme (1-10).
* Press `P` to pause the timer.
* Press `Q` to quit the application.

## Configuration
-------------

The Jimmer Timer can be configured using the following options:

* `--minute=<value>`: Set the timer duration in minutes.
* `--second=<value>`: Set the timer duration in seconds.
* `--theme=<number>`: Choose a theme for the timer and quotes (1-10).

## Themes
--------

The Jimmer Timer comes with 10 pre-defined themes for the timer and quote widgets. You can choose a theme using the `--theme` option.

## Quote Widget
-------------

The Jimmer Timer features a quote widget that displays inspirational quotes. The quotes are randomly selected from a collection of quotes.

## Audio
------

The Jimmer Timer plays audio during the timer (loop and end sounds). You can customize the audio by providing your own audio files , they shall be in the ``current working directory`` and there shall be 2 files - ``audio.mp3`` and ``end.mp3``  , by default jimmer installation will ship with audio.mp3 and places it at ``/usr/local/bin`` but it also looks for one in current dir, atleast ``audio.mp3`` shall be there

## Contributing
------------

Contributions to the Jimmer Timer are welcome! If you have any ideas or bug fixes, please submit a pull request.

## License
-------

The Jimmer Timer is licensed under the MIT License.
