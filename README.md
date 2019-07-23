# monochrome

Convert color images to monochrome images with several algorithms.

## Instalation

```sh
$ cargo install --git https://github.com/shrhdk/monochrome
```

## Usage

```sh
$ monochrome <algorithm> <input> <output> --gamma <gamma>
```

## Example

```sh
$ monochrome floyd lena.jpg floyd.png
$ monochrome bayer lena.jpg bayer.png
$ monochrome bayer lena.jpg bayer.png --gamma 0.5
```

### Original
![Original Lena](img/lena.jpg)

### Floyd-Steinberg Dithering

![Image dithered by Floyd-Steinberg method](img/floyd.png)

### Bayer Dithering

![Image dithered by Bayer method](img/bayer.png)

### Bayer Dithering with Gamma Correction (&gamma; = 0.5)

![Image dithered by Bayer method and gamma correction (&gamma; = 0.5)](img/bayer-gamma-0.5.png)
