# monochrome

Convert color images to monochrome images with several algorithms.

## Instalation

```sh
$ cargo install --git https://github.com/shrhdk/monochrome
```

## Usage

```sh
$ monochrome <algorithm> <input> <output>
```

## Example

```sh
$ monochrome floyd lena.jpg floyd.png  
$ monochrome bayer lena.jpg bayer.png
```

### Original
![Original Lena](img/lena.jpg)

### Floyd-Steinberg Dithering

![Image dithered by Floyd-Steinberg method](img/floyd.png)

### Bayer Dithering

![Image dithered by Bayer method](img/bayer.png)
