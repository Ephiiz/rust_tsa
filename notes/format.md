The binary tga file format broken down. Scheme is {data_name}::{size_in_bytes}::{description}

# Header::18 bytes total

id_length::1::Size of the image ID field
color_map_type::1::Is a color map included?
image_type::1::Compressed? True Color? Grayscale?

## Color map specs (5 bytes across 3 vars)
color_map_origin::2::Color map origin (0 in our case)
color_map_length::2::Color map length (0 in our case)
color_map_depth::1::Color map depth (0 in our case)

## Image specs (10 bytes across 6 vars)
x_origin::2::XOrigin
y_origin::2::YOrigin
image_width::2::Image width
image_height::2::Image Height
pixel_depth::1::Pixel depth - typically 8, 16, 24, or 32
image_descriptor::1::Image descriptor

# Image Data (variable length)
The image data is stored in a contiguous block of pixels equal to `(image_width * image_height)` The contents of a single pixel can vary depending of the file's properties, but for this project we are using images with 24-bit color. This means that each pixel would contain:
blue::1
green::1
red::1

Each of those bytes with contain a value between 0-255, which makes u8 perfect to store them.
Each pixel could be stored in a 3 val tuple. The values are stored in reverse, (blue, green red), order. The first pixel in the file represents the BOTTOM LEFT pixel in the image, with the last pixel representing the TOP RIGHT pixel.

Example:
[Example of pixel format](./8x4_pixelexample.png)


# Optional Footer
Unused for this project



# Project Details
The files will be 24-bit true color, uncompressed images. Should really only need image_width and image_height.

## Tasks
1. Load files from the input folder.
2. Perform ops of the loaded files
3. Write the results to a new tga file (named part#.tga) in the output folder. Test outputs against provided examples for testing suites, files should be identical (hopefully).

Task Example:
1. Load the file





(00 00 00) (00 00 00)




137 * 239 / 255