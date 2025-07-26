# Scryfall Processer
This set of programs is desgined to download and hash all MTG cards for the dropkick sorter.

> [!CAUTION]  
> This will download and proccess all 506,000 cards which will take up 500GB of disk space.\
> Please make sure you have enough space to process.\
> If you are not testing please run in release mode, it will be a lot faster as it is compute heavy.

> [!WARNING]  
> This does not use defalut phash settings, read below for more details

## Export
The exporter will download the latest all card file from [ScryFall](https://scryfall.com/docs/api/bulk-data). Then download all 500,000 cards.

## Image Process
The image processer will phash all cards and export it to a hashes.json file to be used for the dropkick scanner. \
It is normal for images to error out on load, these are normally placeholder images ScryFall uses.

# Phash Details
The hasher uses the [img_hash](https://docs.rs/img_hash/latest/img_hash/) create to hash all images. It does not use default settings instead is uses a 16X16 sized hash with a Gradient algorithm. \
If you would like to use it with another project please make sure it supports the same phash algorithms.