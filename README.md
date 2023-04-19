# uv5rtool
A small tool to dump or upload the memory of Baofeng UV-5R radios


## What it does?
This tool downloads and uploads the memory of Baofeng UV-5R radios. It allows users to save the settings of the UV-5R, and to upload back the image file. Image files are compatible to those created by Chirp, and can be edited with Chirp.


## But why?
Chirp is an awesome application that offers a lot of features. With Chirp, you can not only back up your settings but also configure your radio. I've used Chirp successfully with my Baofeng radio version BFB297. However, when I got a new UV-5R version BFB298, Chirp (until at least version 20230418) couldn't upload a complete image to the radio and caused it to lose everything in the AUX memory area. After that, Chirp couldn't open the Other settings area, and the radio started behaving strangely. The problem is most likely caused by the firmware in the new BFB298 version. Therefore, I created this tool after experimenting a bit, which can both download and upload complete images to the UV-5R. I've tested the tool with multiple UV-5Rs BFB298 and it seems to work correctly.


## How to use?
This tool allows you to dump the UV-5R memory and upload an image file back to the radio. With this tool alone you can't actually make any changes to your settings. However, since the dumped images are compatible with the Chirp file format, you can do this instead:
* Dump the memory of your Baofeng BFB298 using uv5rtool
* Open the dumped file in Chirp and make whatever settings you want, then save the chnages
* Use uv5rtool to upload the edited image back to your radio


## What if I've already broken my radio?
I created this tool specifically for that. However, I was lucky enough to have saved the "original" dump from the radio, that I created using Chirp. So your best bet, if you don't have the "original" memory file, is to try to find an image that matches your model, specifically a dump from the BFB298 version. Once you have that, you can easily upload it to your radio using uv5rtool and your radio should be fixed.


## Usage
Depending on your operating system, the command line is slightly different. 

**On Windows**

Dump (download) image:

`uv5rtool -p COM5 -f image.img -m dump`

or

`uv5rtool --port COM5 --file image.img --mode dump`

or just:

`uv5rtool --port COM5`
(this defaults to mode *dump* and file name *dump.img*)
 
Upload image:

`uv5rtool -p COM5 -f image.img -m upload`

or

`uv5rtool --port COM5 --file image.img --mode upload`

**On Linux**

Dump (download) image:

`./uv5rtool -p /dev/ttyUSB0 -f image.img -m dump`

or

`./uv5rtool --port /dev/ttyUSB0 --file image.img --mode dump`

or just:

`./uv5rtool --port /dev/ttyUSB0`
(this defaults to mode *dump* and file name *dump.img*)
 
Upload image:

`./uv5rtool -p /dev/ttyUSB0 -f image.img -m upload`

or

`./uv5rtool --port /dev/ttyUSB0 --file image.img --mode upload`

**On MacOS**

Dump (download) image:

`./uv5rtool -p /dev/tty.usbserial-1410 -f image.img -m dump`

or

`./uv5rtool --port /dev/tty.usbserial-1410 --file image.img --mode dump`

or just:

`./uv5rtool --port /dev/tty.usbserial-1410`
(this defaults to mode *dump* and file name *dump.img*)

Upload image:

`./uv5rtool -p /dev/tty.usbserial-1410 -f image.img -m upload`

or

`./uv5rtool --port /dev/tty.usbserial-1410 --file image.img --mode upload`

So the only difference is the way you specify the COM port. Of course, replace COM5, /dev/ttyUSB0 or /dev/tty.usbserial-1410 with whatever port your programming cable is connected to. You can get that from Chirp.

On any OS you can use:
`uv5rtool --help`


## How to build
* Make sure you have Rust installed (https://www.rust-lang.org/tools/install)
* Clone the repository, open a terminal and change to the uv5rtool directory
* Do `cargo build --release`

You can of course use the prebuilt version as well.


## Disclaimer
This tool is provided as is. Although I tested it as much as I could and it seems to function properly, use on your own risk. **My recommendation is to use this for version BFB298 only, and use Chirp for any other versions or radios.** DO NOT USE with any other radio except the UV-5R! This tool has been tested on Baofeng UV-5R version BFB297 and BFB298 only!
