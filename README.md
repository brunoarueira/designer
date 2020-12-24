# designer

[![Build Status](https://travis-ci.org/brunoarueira/designer.svg?branch=master)](https://travis-ci.org/brunoarueira/designer)
[![codecov](https://codecov.io/gh/brunoarueira/designer/branch/master/graph/badge.svg)](https://codecov.io/gh/brunoarueira/designer)

Does nice things with images, colors and generate palette colors too

It's a cli app made with rust and can be integrated with other apps or whitelabel app, which most of the time the customer supply a brand logo and someone extract colors or a palette colors to customize the app or the web app.

## Features

- List a collor palette on rgb or hex from a brand logo;
- Return the dominant color from a brand logo (on rgb or hex);
- Output the collor palette to a file joined each color with commas.

## Usage

```
designer -h
```

## TO-DO

- Tests;
- Add CI;
- Pass a template logo, screen template and with the new brand logo, change the palette of the screen template as a kind of preview (useful for analyze the brand logo on a screen before someone custom the app);
- Pass a json template file, template logo, brand logo and generates a new json file with the same color keys, but with custom colors, to easily replace, substitute or automate the process of create new apps or insert somewhere to an app/web app pick up;
- Probably improve the quality of the brand logo, in some cases the customer will supply a brand logo with a poor quality and the final delivery will suffer.
