# monochrome

Convert color images to monochrome images with several algorithm.

## Instalation

```sh
$ cargo install --git https://github.com/shrhdk/monochrome
```

## Usage

```sh
$ monochrome <algorithm> <input> <output> --gamma <gamma>
```

## Algorithm

| Option Value | Name                                               |
| ------------ | -------------------------------------------------- |
| `floyd`      | Floyd-Steinberg Dithering                          |
| `floyd2`     | Floyd-Steinberg Dithering with Serpentine Scanning |
| `bayer`      | Bayer dithering (4x4 mask)                         |

## Example

```sh
$ monochrome floyd lena.jpg floyd.png
$ monochrome floyd2 lena.jpg floydw.png
$ monochrome bayer lena.jpg bayer.png
$ monochrome bayer lena.jpg bayer.png --gamma 0.5
```

### Original

![Original Lena](img/lena.jpg)

### Floyd-Steinberg Dithering

![Image dithered by Floyd-Steinberg method](img/floyd.png)

### Floyd-Steinberg Dithering with Serpentine Scanning

![Image dithered by Floyd-Steinberg method with Serpentine Scanning](img/floyd2.png)

### Bayer Dithering

![Image dithered by Bayer method](img/bayer.png)

### Bayer Dithering with Gamma Correction (&gamma; = 0.5)

![Image dithered by Bayer method and gamma correction (&gamma; = 0.5)](img/bayer-gamma-0.5.png)
