use std::fs::{self};

#[allow(non_snake_case)]
pub mod TGA {
    use super::*;


    #[derive(Debug, PartialEq)]
    pub struct TGAFile {
        // Header ( 18 bytes total)
        // Flags ( 3 Bytes )
        id_length: u8,              // Size of the image ID field
        color_map_type: u8,          // Is a color map included?
        image_type: u8,             // Compressed? True Color? Grayscale?
    
        // Color Map Specifications ( 5 Bytes )
        color_map_origin: u16,       // Color map origin point (should be 0 in our case)
        color_map_length: u16,
        color_map_depth: u8,
    
        // Image Specifications ( 10 Bytes )
        x_origin: u16,
        y_origin: u16,
        pub image_width: u16,
        pub image_height: u16,
        pixel_depth: u8,
        image_descriptor: u8,

        // Image Data (variable length based on image_width * image_height)
        pub pixels: Vec<(u8, u8, u8)>,

        // Footer; unused in this project
        _footer: Vec<u8>,
    }

    impl TGAFile {
        pub fn cereal(&self) -> Vec<u8> {
            //TODO: make this less gross, verbose for testing reasons
            let mut output: Vec<u8> = Vec::new();
            output.push(self.id_length);
            output.push(self.color_map_type);
            output.push(self.image_type);

            //Color map
            output.push(self.color_map_origin.to_le_bytes()[0]);
            output.push(self.color_map_origin.to_le_bytes()[1]);

            output.push(self.color_map_length.to_le_bytes()[0]);
            output.push(self.color_map_length.to_le_bytes()[1]);

            output.push(self.color_map_depth);

            //Image Specs
            output.push(self.x_origin.to_le_bytes()[0]);
            output.push(self.x_origin.to_le_bytes()[1]);

            output.push(self.y_origin.to_le_bytes()[0]);
            output.push(self.y_origin.to_le_bytes()[1]);

            output.push(self.image_width.to_le_bytes()[0]);
            output.push(self.image_width.to_le_bytes()[1]);

            output.push(self.image_height.to_le_bytes()[0]);
            output.push(self.image_height.to_le_bytes()[1]);

            output.push(self.pixel_depth);
            output.push(self.image_descriptor);

            for pixel in &self.pixels {
                output.push(pixel.0);
                output.push(pixel.1);
                output.push(pixel.2);
            }

            for byte in &self._footer {
                output.push(*byte);
            }


            return output;
        }
    }

    pub fn multiply(top: &(u8,u8,u8), bottom: &(u8,u8,u8), _range: (usize, usize)) -> (u8, u8, u8) {
        let out: (u8, u8, u8) = (
            ((top.0 as f32) * (bottom.0 as f32) / 255.0).round() as u8,
            ((top.1 as f32) * (bottom.1 as f32) / 255.0).round() as u8,
            ((top.2 as f32) * (bottom.2 as f32) / 255.0).round() as u8,
        );
        return out;

    }

    pub fn subtract(top: &(u8,u8,u8), bottom: &(u8,u8,u8), _range: (usize, usize)) -> (u8, u8, u8) {
        let out = (
            bottom.0.saturating_sub(top.0),
            bottom.1.saturating_sub(top.1),
            bottom.2.saturating_sub(top.2),
        );
        return out;
    }

    pub fn screen(top: &(u8,u8,u8), bottom: &(u8,u8,u8), _range: (usize, usize)) -> (u8, u8, u8) {
        let out: (u8, u8, u8) = (
            ((1.0 - ((1.0 - (top.0 as f32/255.0)) * (1.0 - (bottom.0 as f32/255.0)))) * 255.0).round() as u8,
            ((1.0 - ((1.0 - (top.1 as f32/255.0)) * (1.0 - (bottom.1 as f32/255.0)))) * 255.0).round() as u8,
            ((1.0 - ((1.0 - (top.2 as f32/255.0)) * (1.0 - (bottom.2 as f32/255.0)))) * 255.0).round() as u8,
        );
        return out;

    }

    fn over_screen(p1: u8, p2: u8) -> u8 {
        return ((1.0 - ((1.0 - (p1 as f32/255.0)) * (1.0 - (p2 as f32/255.0)))) * 255.0).round() as u8;
    }
    fn over_multiply(p1: u8, p2: u8) -> u8 {
        return (2.0 *((p1 as f32) * (p2 as f32) / 255.0)).round() as u8;
    }

    pub fn overlay(top: &(u8,u8,u8), bottom: &(u8,u8,u8), _range: (u8,u8)) -> (u8, u8, u8) {
        //TODO: i have no clue what im doing wrong here, but it all has to get rewritten later
        let out: (u8, u8, u8) = (
            (|| {
                if top.0 as f32 / 255.0 > 0.5 {
                    let val = (255.0-top.0 as f32)/127.5;
                    let min = top.0 as f32 - (255.0 - top.0 as f32);
                    return ((bottom.0 as f32 * val) + min) as u8
                } else {
                    let val = (top.0 as f32) / 127.5;
                    return (bottom.1 as f32 * val) as u8;
                }
            })(),
            (|| {
                if top.1 as f32 / 255.0 > 0.5 {
                    let val = (255.0-top.1 as f32)/127.5;
                    let min = top.1 as f32 - (255.0 - top.1 as f32);
                    return ((bottom.1 as f32 * val) + min) as u8
                } else {
                    let val = (top.1 as f32) / 127.5;
                    return (bottom.1 as f32 * val) as u8;
                }
            })(),
            (|| {
                if top.2 as f32 / 255.0 > 0.5 {
                    let val = ((255.0-top.2 as f32)/127.5);
                    let min = top.2 as f32 - (255.0 - top.2 as f32);
                    return ((bottom.2 as f32 * val) + min) as u8
                } else {
                    let val = (top.2 as f32) / 127.5;
                    return (bottom.2 as f32 * val) as u8;
                }
            })(),
            //((top.1 as f32) * (bottom.1 as f32) / 255.0).round() as u8,
            //((top.2 as f32) * (bottom.2 as f32) / 255.0).round() as u8,
        );
        return out;

    }


    pub fn load_nocereal(relative_path: &str) -> TGAFile {
        // Load the file into a byte array
        let file: Vec<u8> = fs::read(relative_path).expect(&format!("Couldnt load tga file {}", relative_path));

        let contents: TGAFile = TGAFile {
            // Load file flags
            id_length: file[0],                                             // 0x00
            color_map_type: file[1],                                        // 0x00
            image_type: file[2],                                            // 0x02


            // Color map
            color_map_origin: u16::from_le_bytes([file[3], file[4]]),       // 0x0000
            color_map_length: u16::from_le_bytes([file[5], file[6]]),       // 0x0000
            color_map_depth: file[7],                                       // 0x00

            // Image Specs
            x_origin: u16::from_le_bytes([file[8], file[9]]),               // 0x0000
            y_origin: u16::from_le_bytes([file[10], file[11]]),             // 0x0000
            image_width: u16::from_le_bytes([file[12], file[13]]),          // 0x0002 = 512
            image_height: u16::from_le_bytes([file[14], file[15]]),         // 0x0002 = 512
            pixel_depth: file[16],                                          // 0x18
            image_descriptor: file[17],                                     // 0x00

            pixels: (|| {
                let image_width = u16::from_le_bytes([file[12], file[13]]) as usize;
                let image_height = u16::from_le_bytes([file[14], file[15]]) as usize;
                let pixel_count = (image_width * image_height) as usize;
                let mut output: Vec<(u8, u8, u8)> = Vec::new();
                for pixel_offset in 0 .. pixel_count {
                    //println!("{}", String::from("ran"));
                    let pixel = (file[(18+(pixel_offset*3))], file[(18+(pixel_offset*3))+1], file[(18+(pixel_offset*3))+2]);
                    output.push(pixel);
                }
                return output;
            })(),
            
            _footer: Vec::new(),
            
        };
        //println!("{:?}", contents);
        return contents;
    }

    pub fn run_task<P>(top: &TGAFile, bottom: &TGAFile, f: P, range: Option<&TGAFile>) -> TGAFile
        where P: Fn(&(u8, u8, u8), &(u8, u8, u8), Option<&(u8,u8,u8)>) -> (u8, u8, u8) {
        // Take the R B and G values and multiply by the associated value on the second layer
        //TODO: stop assuming top and bottom are same dimensions
        // C=A*B
        let output: TGAFile = TGAFile {
            id_length: top.id_length,
            color_map_type: top.color_map_type,
            image_type: top.image_type,

            color_map_origin: top.color_map_origin,
            color_map_length: top.color_map_length,
            color_map_depth: top.color_map_depth,

            x_origin: top.x_origin,
            y_origin: top.y_origin,
            image_width: top.image_width,
            image_height: top.image_height,
            pixel_depth: top.pixel_depth,
            image_descriptor: top.image_descriptor,

            pixels: (|| {
                let mut out: Vec<(u8,u8,u8)> = Vec::new();
                for (index, pixel) in top.pixels.iter().enumerate() {
                    out.push(f(pixel, &bottom.pixels[index], (|| {
                        match range {
                            Some(image) => {
                                return Some(&image.pixels[index]);
                            },
                            _ => {
                                return None;
                            }
                        }
                    })()))
                }
                return out;
            })(),

            _footer: top._footer.clone(),
        };
        return output;
    }



}
