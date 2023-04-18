title: What is a file?
link: what-is-a-file
published_date: 2023-04-16
tags: linux, html, xxd, binary, hexadecimal
___

# How to convert a base64 data stream into an image?

## Intro: What's in a bookmark?

I exported my Chrome bookmarks to an HTML file and saw that it has a basic HTML structure: I could open it in my browser like a standard HTML file.  
The thing that caught my attention, however, was the structure of the bookmark tags - they are arranged in a table as data in the form of anchor tag links with additional attributes `ADD_DATE` and `ICON` apart from the typical `HREF` (the file had everything capitalized). While these attributes don't affect how these links are displayed in the browser, they do serve their own purposes:

* `ADD_DATE` contains the timestamp since the UNIX epoch and
* `ICON` wraps the data for the favicon to be displayed beside the bookmark in the browser.

![src](https://imgur.com/nLmatzd.png)

> Wait, shouldn't `ICON` contain a path to some image online?

No, in fact...

## Iconic Data: Internals of `ICON`

...`ICON` stores the *complete information* needed to display a PNG image, in the form of base64-encoded data. We can tell by looking at the helpful `data:image/png;base64` descriptor at the start of the attribute value. This descriptor is what allows the browser to display the favicon. Even a non-Chromium browser like Firefox can decode this data if it fits the encoding Firefox expects from a favicon. To test, we simply paste the contents of `ICON` in the address bar:

![base64](https://imgur.com/5pXgmY0.png)

(All modern browsers should expect the same type of data in the same encoding, if not, these bookmark files won't be cross-browser compatible.)

## Let's make a PNG file from base64 data!

How does the browser go from reading base64-encoded data to displaying a PNG file as the favicon?  
Let's try to create a file that would display the same image in Firefox.

The naïve approach would be to paste the contents of `ICON` into a file and try opening that with the browser.

```text
$ echo 'data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAACIklEQVQ4jYWSS0iUURTHf/fe8RvHooE2VlT2FNqUGWmNEYUR9lhEEVJhUIsoXOQuap1Rq6KHNQt3LaPAIOxhlNTChUwLMU3NR1CklUzg6xvPd1ro2KhTHjjcA/e8/uf/hzmmqsUiEheRLhHxp/2TiDxQ1aK5+ZmFeSJSrwuYiMRVNZKuMxnFz51zu9T3GX/6iPGmRqS/F5WAUMEawuUVRI5UYjwPEWl2zlUYY8YMgIjUW2vPBkPfSV6uYbKvJ+uW3rZSojfuABAEQdw5d96oajHQqr7P8IUqpL8X43lEjp3EK4mBtfgt75l4+4po7U3cytWZPbcyjUlTidv642ipDu7foX7bh2zgs92jDhHpUlWdbNmuEw15OvqweqE7ZjboCAEFADrSjs1LkRM7NAt3+bWRebfYudFx9XguwFqbwePs9z/mT/6NLdAHMBpex28W0/C1Y1Zy05VFM75nUwiAZVGT/v5sgdcA3UurOPUrxvXOFhJD7fOmdn4LeNc5NbpkfWimv5mWZ8KXFKdfXqInOYBnc6gsPEjZ8mKssbQOtvEkMczYl0oK8z3un4lgppbYkhZS3Fp7bnD0Jxeba+lODmTFviFcxq29NeRHDUEQ1DnnqtNSjohIo3Nutx+keNz9gmf9zfQkB0ChYMkK9q2KcaLwMJFQGFV9Y4w5YIwZzyBBI2lRLcD9PVXN/SdFqlokInUi0iEiE9P+UUTuqurmufl/AKTzsFGmvUNUAAAAAElFTkSuQmCC' > ff.png
```

This approach will fail as the descriptor and data are intended for the address bar and we have no reason to assume that reading a file would involve the same process used to decode an address.  
Nevertheless, feel free to try. To open a file in the browser, press Ctrl+O or enter its path into the address bar.

![failed](https://imgur.com/lEsMv4z.png)

The next approach would be to remove the descriptor and let the browser decide how to process the file. After removing the descriptor, purely base64 data is left.

```text
$ cat ff.png 
iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAACIklEQVQ4jYWSS0iUURTHf/fe8RvHooE2VlT2FNqUGWmNEYUR9lhEEVJhUIsoXOQuap1Rq6KHNQt3LaPAIOxhlNTChUwLMU3NR1CklUzg6xvPd1ro2KhTHjjcA/e8/uf/hzmmqsUiEheRLhHxp/2TiDxQ1aK5+ZmFeSJSrwuYiMRVNZKuMxnFz51zu9T3GX/6iPGmRqS/F5WAUMEawuUVRI5UYjwPEWl2zlUYY8YMgIjUW2vPBkPfSV6uYbKvJ+uW3rZSojfuABAEQdw5d96oajHQqr7P8IUqpL8X43lEjp3EK4mBtfgt75l4+4po7U3cytWZPbcyjUlTidv642ipDu7foX7bh2zgs92jDhHpUlWdbNmuEw15OvqweqE7ZjboCAEFADrSjs1LkRM7NAt3+bWRebfYudFx9XguwFqbwePs9z/mT/6NLdAHMBpex28W0/C1Y1Zy05VFM75nUwiAZVGT/v5sgdcA3UurOPUrxvXOFhJD7fOmdn4LeNc5NbpkfWimv5mWZ8KXFKdfXqInOYBnc6gsPEjZ8mKssbQOtvEkMczYl0oK8z3un4lgppbYkhZS3Fp7bnD0Jxeba+lODmTFviFcxq29NeRHDUEQ1DnnqtNSjohIo3Nutx+keNz9gmf9zfQkB0ChYMkK9q2KcaLwMJFQGFV9Y4w5YIwZzyBBI2lRLcD9PVXN/SdFqlokInUi0iEiE9P+UUTuqurmufl/AKTzsFGmvUNUAAAAAElFTkSuQmCC
```

As you may have guessed, this approach too will fail for the same reason, but we are on the right track!

## Covering all bases: What is data after all?

We now know that the data is in base64, which is simply a way to encode binary data. Since all data is ultimately binary at the lowest level, it's safe to assume that given this binary data, the browser will know how to display the file even without any descriptors.  

We need a hexadecimal processor to create a binary file from this base64-encoded data. We don't manipulate binary data directly because it's extremely cumbersome to represent and work with, instead, we use hexadecimal which lets us store much more information with fewer bits and greater convenience. Once we have hexadecimal data, it's very simple to convert it into binary. A widely available Linux utility, `xxd` can both encode and decode hexadecimal files.

But before all this, we must convert the base64 data into hexadecimal first. A simple method is to just go to [RapidTables](https://www.rapidtables.com/convert/number/ascii-hex-bin-dec-converter.html) for the conversion from base64 to hexadecimal. Copy the contents of your file, paste them into the base64 box, and copy the resulting hexadecimal data into a file.

```text
$ echo '89 50 [..omitted for brevity..] 60 82' > hexadecimal.txt
```

This file contains a lot of unnecessary spaces (`' '`) which need to be removed to convert it into pure hexadecimal.

```text
$ cat hexadecimal.txt | tr -d ' ' > hexmod.txt
$ cat hexmod.txt
89504E47[..omitted for brevity..]AE426082
```

Now we have a purely hexadecimal file, which we need to convert into binary.

```text
$ xxd -r -p hexmod.txt > newff.png
```

* Here, `-r` denotes that we are reverting the file from hexadecimal to binary,
* `-p` denotes that there is no information in the file for how the hexadecimal is to be represented (relevant to `xxd`),
* `>` means to take the output (stdout) of this command and store that in the file `newff.png`. Any data stored inside `newff.png` is overwritten. We do this to store the binary data because the terminal doesn't know how to print it - hence, we can't just copy it from the output.

## Moment of Truth

Once all this is done, just open newff.png in your browser of choice - you'll see that the same image is displayed as the content of ICON.

![success](https://imgur.com/rnQCujC.png)

Not only that, this file is, for all intents and purposes, a PNG file - and will be treated as such by the OS, even showing the file's preview on the Desktop.

![desktop](https://imgur.com/zrhJ3lC.png)

## Outro

A file can just be thought of as instructions that the program designed to handle it has to perform to show us the expected outcome. In the case of an image, the instructions are to manipulate the pixels of the screen such that we perceive it as an image. Given certain descriptors and methods of evaluation, a file can be processed effectively by a program carrying out these instructions.

**Kindly communicate improvements via [e-mail](mailto:saumay03pro@gmail.com).**

<!--

```text
$ xxd png.png | head -5
00000000: 6956 424f 5277 304b 4767 6f41 4141 414e  iVBORw0KGgoAAAAN
00000010: 5355 6845 5567 4141 4142 4141 4141 4151  SUhEUgAAABAAAAAQ
00000020: 4341 5941 4141 4166 382f 3968 4141 4143  CAYAAAAf8/9hAAAC
00000030: 496b 6c45 5156 5134 6a59 5753 5330 6955  IklEQVQ4jYWSS0iU
00000040: 5552 5448 662f 6665 3852 7648 6f6f 4532  URTHf/fe8RvHooE2
```

```text
$ xxd png.png | xxd -r > newpng.png
$ diff png.png newpng.png
    # nothing is printed #
```

```text
$ xxd png.png > ff.png
$ firefox ff.png &
```
So we try:
```text
$ xxd png.png -p > ff.png
$ firefox ff.png &
```

We go to [RapidTables](https://www.rapidtables.com/convert/number/ascii-hex-bin-dec-converter.html) for the conversion from base64 to hexadecimal.

-->
