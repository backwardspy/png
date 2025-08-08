# PNG

*pigeon's nice graphics format*

a hand-made, ground-breaking, future-proof, game-changing image format the likes of which has never been seen.

**note:** this is *not* the same as PNG the "portable network graphics" format from 1996. IANA won't let me take the name, despite mine being obviously a better format. still working on this. in the meantime, PNG uses the `.png_real` extension.

## usage

this implementation provides three tools to allow even you, a mere mortal, to work with PNG images. they also serve as reference implementations for you to draw from when you inevitably need to support PNG in your operating system, video game, or other important project.

### makepng

`makepng` allows you to take an inferior image and turn it into a PNG.

usage:

```console
$ makepng --help
Usage: makepng <PATH>

Arguments:
  <PATH>  

Options:
  -h, --help  Print help

$ makepng my_image.tiff
wrote 512x512 PNG into my_image.png_real
```

### unmakepng

i don't know why you'd want this, but `unmakepng` lets you, if you must, turn a PNG into an inferior format.

usage:

```console
$ unmakepng --help
Usage: unmakepng --format <FORMAT> <PATH>

Arguments:
  <PATH>  

Options:
  -f, --format <FORMAT>  [possible values: avif, bmp, cr2, dng, eps, exr, gif, heic, jpeg, jpg, jxl, kra, pdf, png, psd, raw, svg, tif, tiff, webp, xcf]
  -h, --help             Print help

$ unmakepng -f heic my_image.png_real
read 512x512 PNG in a blazing 43 seconds
error: selected format "heic" is too simple to adequately contain a PNG. please try a better format.
```

**note:** as of yet i've been unable to find an image format that can adequately contain a PNG, so i can't really demo `unmakepng` properly.

### viewpng

for some reason none of the existing image viewers i've been able to find can display PNG images, so i've provided a perfectly cromulent one here.

usage:

```console
$ viewpng --help
Usage: viewpng <PATH>

Arguments:
  <PATH>  

Options:
  -h, --help  Print help

$ viewpng my_image.png_real
```

output:

![a beautifully rendered image of an F-16 fighter jet in PNG format, albeit saved as a webp since github doesn't support PNG yet :(](assets/sample.webp)

## details

the PNG format is eminently simple, yet perfectly ready to tackle the challenges of tomorrow.

modern image and display formats want you to be comfortable, even wowed, by a 12-bit colour depth.

"you can have 68,719,476,736 colours!!!" they brag.

what's PNG's approach to this? 1,024-bit colour depth.

you read that right. each colour channel in a PNG image is a full 1kb wide. and that's not all; PNG has four colour channels per pixel.

"oh, that's not impressive, everything uses 4-channel RGBA now anyway!" you say.

wrong. dead wrong. PNG has four *colour* channels, and three *alpha* channels. how is this possible? observe:

- red: for wavelengths in the 625-750nm range.
- green: for wavelengths in the 495-570nm range.
- blue: for wavelengths in the 450-495nm range.
- glorp: for new undiscovered wavelengths *that we can't even see yet*.
- alpha-3: regular (albeit 1kb wide) alpha channel for three-dimensional beings.
- alpha-4: a new alpha channel for four-dimensional beings.
- alpha-5: *another* new alpha channel for five-dimensional beings.

as you can see, PNG has every possible contingency already in place, no matter what the future holds.

so, 3x12-bit colour channels for 68.7 billion representable colours? try 4x1,024-bit colour channels for *1,044,388,881,413,152,506,691,752,710,716,624,382,579,964,249,047,383,780,384,233,483,283,953,907,971,557,456,848,826,811,934,997,558,340,890,106,714,439,262,837,987,573,438,185,793,607,263,236,087,851,365,277,945,956,976,543,709,998,340,361,590,134,383,718,314,428,070,011,855,946,226,376,318,839,397,712,745,672,334,684,344,586,617,496,807,908,705,803,704,071,284,048,740,118,609,114,467,977,783,598,029,006,686,938,976,881,787,785,946,905,630,190,260,940,599,579,453,432,823,469,303,026,696,443,059,025,015,972,399,867,714,215,541,693,835,559,885,291,486,318,237,914,434,496,734,087,811,872,639,496,475,100,189,041,349,008,417,061,675,093,668,333,850,551,032,972,088,269,550,769,983,616,369,411,933,015,213,796,825,837,188,091,833,656,751,221,318,492,846,368,125,550,225,998,300,412,344,784,862,595,674,492,194,617,023,806,505,913,245,610,825,731,835,380,087,608,622,102,834,270,197,698,202,313,169,017,678,006,675,195,485,079,921,636,419,370,285,375,124,784,014,907,159,135,459,982,790,513,399,611,551,794,271,106,831,134,090,584,272,884,279,791,554,849,782,954,323,534,517,065,223,269,061,394,905,987,693,002,122,963,395,687,782,878,948,440,616,007,412,945,674,919,823,050,571,642,377,154,816,321,380,631,045,902,916,136,926,708,342,856,440,730,447,899,971,901,781,465,763,473,223,850,267,253,059,899,795,996,090,799,469,201,774,624,817,718,449,867,455,659,250,178,329,070,473,119,433,165,550,807,568,221,846,571,746,373,296,884,912,819,520,317,457,002,440,926,616,910,874,148,385,078,411,929,804,522,981,857,338,977,648,103,126,085,903,001,302,413,467,189,726,673,216,491,511,131,602,920,781,738,033,436,090,243,804,708,340,403,154,190,336* representable colours.

that number's so big, they don't even have a name for it on wolfram alpha yet. i humbly suggest to mr. wolfram that it be called "1 PNGillion".

### the elephant in the room

if you've made it this far, you're likely thinking to yourself something along these lines:

> wow, PNG is so cool, but surely the processing and storage requirements of
> such an incredible format are practically impossible to meet with today's
> technology?

you're right, PNG *is* so cool, but the good news is that despite their frankly incredible capabilities, PNG images are usable and storeable on contemporary computer hardware.

to prove this seemingly impossible claim, here is a terminal session showcasing the conversion & subsequent display of a 512x512 TIFF format image into the PNG format on a regular computer that i certainly didn't steal from a top-secret government alien research facility:

```console
$ file my_image.tiff
my_image.tiff: TIFF image data, big-endian, direntries=10, height=512, bps=12, compression=none, PhotometricInterpretation=RGB, width=512

$ makepng my_image.tiff
wrote 512x512 PNG into my_image.png_real

$ du -h my_image.{tiff,png_real}
772K    my_image.tiff
225M    my_image.png_real

$ viewpng my_image.png_real
decoding my_image.png_real...
decoded 512x512 PNG in a blazing 43 seconds
```

as you can see, the 512x512 TIFF went from 772K to 225M. if you ignore the letters, that's a reduction of almost 3.5 times.

## FAQs

- Q: this takes so long to build
    - A: yeah lol
