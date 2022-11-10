use std::fs::{self};
use log::{info, warn, debug};
use simplelog::*;

use rust_tga::TGA;
use rust_tga::TGA::{run_task, multiply, subtract, screen, overlay};

fn part1() {
    // Part 1 (provided example)
    info!("Part 1 started");
    debug!("Loading layer1.tga");
    let layer1 = TGA::load_nocereal("./input/layer1.tga");
    debug!("Loading pattern1.tga");
    let pattern1 = TGA::load_nocereal("./input/pattern1.tga");
    
    debug!("Running multiply blend on layer1 and pattern1");
    let mult = run_task(&layer1, &pattern1, |top, bottom, _range|  {
        return multiply(top, bottom, (0,255))
    }, None);
    info!("Writing part1 output");
    fs::write("./output/part1.tga", &mult.cereal()).expect("Cannot write output file.");
}

fn part2() {
    info!("Part 2 started");
    debug!("Loading layer2.tga");
    let layer2 = TGA::load_nocereal("./input/layer2.tga");
    debug!("Loading car.tga");
    let car = TGA::load_nocereal("./input/car.tga");

    debug!("Running subtract blend on layer2 and car");
    let sub = run_task(&layer2, &car, |top, bottom, _range|  {
        return subtract(top, bottom, (0,255))
    }, None);
    info!("Writing part2 output");
    fs::write("./output/part2.tga", &sub.cereal()).expect("Cannot write output file.");
}

fn part3() {
    info!("Part 3 started");
    debug!("Loading layer1.tga");
    let layer1 = TGA::load_nocereal("./input/layer1.tga");
    debug!("Loading pattern2.tga");
    let pattern2 = TGA::load_nocereal("./input/pattern2.tga");
    debug!("Loading text.tga");
    let text = TGA::load_nocereal("./input/text.tga");

    debug!("Running multiply blend on layer1 and pattern2");
    let mult = run_task(&layer1, &pattern2, |top, bottom, _range|  {
        return multiply(top, bottom, (0,255))
    }, None);
    debug!("Running screen blend on text and (layer1 and pattern2)");
    let scrn = run_task(&text, &mult, |top, bottom, _range| {
        return screen(top, bottom, (0,255))
    }, None);
    info!("Writing part3 output");
    fs::write("./output/part3.tga", &scrn.cereal()).expect("Cannot write output file.");
}

fn part4() {
    info!("Part 4 started");
    debug!("Loading layer2.tga");
    let layer2 = TGA::load_nocereal("./input/layer2.tga");
    debug!("Loading circles.tga");
    let circles = TGA::load_nocereal("./input/circles.tga");
    debug!("Loading pattern2.tga");
    let pattern2 = TGA::load_nocereal("./input/pattern2.tga");

    debug!("Running multiply blend on layer2 and circles");
    let mult = run_task(&layer2, &circles, |top, bottom, _range|  {
        return multiply(top, bottom, (0,255))
    }, None);
    debug!("Running subtract blend on pattern2 and (layer2 and circles)");
    let sub = run_task(&pattern2, &mult, |top, bottom, _range| {
        return subtract(top, bottom, (0,255))
    }, None);
    info!("Writing part4 output");
    fs::write("./output/part4.tga", &sub.cereal()).expect("Cannot write output file.");
}

fn part5() {
    //TODO: fixme
    info!("Part 5 started");
    debug!("Loading layer1.tga");
    let layer1 = TGA::load_nocereal("./input/layer1.tga");
    debug!("Loading pattern1.tga");
    let pattern1 = TGA::load_nocereal("./input/pattern1.tga");

    debug!("Running overlay blend on layer1 and pattern1");
    let over = run_task(&layer1, &pattern1, |top, bottom, _range|  {
        return overlay(top, bottom, (0,255))
    }, None);
    info!("Writing part5 output");
    fs::write("./output/part5.tga", &over.cereal()).expect("Cannot write output file.");
}

fn part6() {
    info!("Part 6 started");
    debug!("Loading car.tga");
    let car = TGA::load_nocereal("./input/car.tga");

    debug!("Adding 200 to green channel in each pixel");
    let over = run_task(&car, &car, |top, _bottom, _range|  {
        return (
            top.0,
            top.1.saturating_add(200),
            top.2,
        )
    }, None);
    info!("Writing part6 output");
    fs::write("./output/part6.tga", &over.cereal()).expect("Cannot write output file.");
}

fn part7() {
    info!("Part 7 started");
    debug!("Loading car.tga");
    let car = TGA::load_nocereal("./input/car.tga");

    debug!("Scaling red by 4 and blue by 0 for each pixel");
    let over = run_task(&car, &car, |top, _bottom, _range|  {
        return (
            top.0.saturating_mul(0),
            top.1,
            top.2.saturating_mul(4),
        )
    }, None);
    info!("Writing part7 output");
    fs::write("./output/part7.tga", &over.cereal()).expect("Cannot write output file.");
}

fn part8() {
    info!("Part 8 started");
    debug!("Loading car.tga");
    let car = TGA::load_nocereal("./input/car.tga");

    debug!("Splitting off blue channel");
    let blue = run_task(&car, &car, |top, _bottom, _range|  {
        return (
            top.0,
            top.0,
            top.0,
        )
    }, None);
    debug!("Splitting off green channel");
    let green = run_task(&car, &car, |top, _bottom, _range|  {
        return (
            top.1,
            top.1,
            top.1,
        )
    }, None);
    debug!("Splitting off red channel");
    let red = run_task(&car, &car, |top, _bottom, _range|  {
        return (
            top.2,
            top.2,
            top.2,
        )
    }, None);
    info!("Writing part8 outputs");
    fs::write("./output/part8_r.tga", &red.cereal()).expect("Cannot write output file.");
    fs::write("./output/part8_g.tga", &green.cereal()).expect("Cannot write output file.");
    fs::write("./output/part8_b.tga", &blue.cereal()).expect("Cannot write output file.");
}

fn part9() {
    info!("Part 9 started");
    debug!("Loading layer_red.tga");
    let layer_red = TGA::load_nocereal("./input/layer_red.tga");
    debug!("Loading layer_green.tga");
    let layer_green = TGA::load_nocereal("./input/layer_green.tga");
    debug!("Loading layer_blue.tga");
    let layer_blue = TGA::load_nocereal("./input/layer_blue.tga");

    debug!("Combining RGB layers into single file");
    let over = run_task(&layer_red, &layer_green, |top, bottom, range|  {
        return (
            range.unwrap().0,
            bottom.1,
            top.2,
        )
    }, Some(&layer_blue));
    info!("Writing part9 output");
    fs::write("./output/part9.tga", &over.cereal()).expect("Cannot write output file.");
}

fn part10() {
    info!("Part 10 started");
    debug!("Loading text2.tga");
    let text2 = TGA::load_nocereal("./input/text2.tga");

    debug!("Reversing pixel array to rotate 180");
    let mut over = text2;
    over.pixels.reverse();
    info!("Writing part10 output");
    fs::write("./output/part10.tga", &over.cereal()).expect("Cannot write output file.");
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ]
    ).unwrap();
    debug!("Initialized logging");
    warn!("Program will act destructively on ./output, so if you didn't clear it, your SOL.");

    info!("Running part1...");
    part1();

    info!("Running part2...");
    part2();

    info!("Running part3...");
    part3();

    info!("Running part4...");
    part4();

    info!("Running part5...");
    part5();
    
    info!("Running part6...");
    part6();

    info!("Running part7...");
    part7();

    info!("Running part8...");
    part8();

    info!("Running part9...");
    part9();

    info!("Running part10...");
    part10();

    warn!("Done!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_input_and_serialize() {
        let layer1 = TGA::load_nocereal("./input/layer1.tga");
        fs::write("./output/fstest.tga", layer1.cereal()).expect("Could not write test file");
        assert_eq!(fs::read("./output/fstest.tga").expect("Could not read test file"), fs::read("./input/layer1.tga").expect("Could not read layer1"));
    }

    #[test]
    fn test_part1_tga_multiply() {
        part1();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part1.tga"), TGA::load_nocereal("./output/part1.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
        
    }

    #[test]
    fn test_part2_tga_subtract() {
        part2();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part2.tga"), TGA::load_nocereal("./output/part2.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part3_tga_screen() {
        part3();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part3.tga"), TGA::load_nocereal("./output/part3.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part4() {
        part4();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part4.tga"), TGA::load_nocereal("./output/part4.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part5_tga_overlay() {
        part5();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part5.tga"), TGA::load_nocereal("./output/part5.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part6() {
        part6();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part6.tga"), TGA::load_nocereal("./output/part6.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }
    
    #[test]
    fn test_part7() {
        part7();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part7.tga"), TGA::load_nocereal("./output/part7.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part8() {
        part8();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part8_r.tga"), TGA::load_nocereal("./output/part8_r.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }


        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part8_g.tga"), TGA::load_nocereal("./output/part8_g.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }


        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part8_b.tga"), TGA::load_nocereal("./output/part8_b.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part9() {
        part9();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part9.tga"), TGA::load_nocereal("./output/part9.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }

    #[test]
    fn test_part10() {
        part10();
        let (example, test) = (TGA::load_nocereal("./examples/EXAMPLE_part10.tga"), TGA::load_nocereal("./output/part10.tga"));
        for (index, _pixel) in example.pixels.iter().enumerate() {
            assert_eq!(example.pixels[index], test.pixels[index]);
        }
    }
}