use opencv::{
    core::{Mat, MatTraitConst, Size},
    imgproc,
};

pub struct AsciiProcessor {
    target_width: i32,
    target_height: i32,
    size: Size,
    gray_frame: Mat,
    pub frame: Mat,
    ascii_lookup: Vec<char>,
}

impl AsciiProcessor {
    pub fn new(target_width: i32, target_height: i32) -> Self {
        let ascii_chars = " ._,:;ox=%#@";
        let ascii_lookup = (0..=255)
            .map(|intensity| {
                ascii_chars
                    .chars()
                    .nth((intensity as usize * (ascii_chars.len() - 1)) / 255)
                    .unwrap_or(' ')
            })
            .collect();

        AsciiProcessor {
            target_width,
            target_height,
            size: Size::new(target_width, target_height),
            frame: Mat::default(),
            gray_frame: Mat::default(),
            ascii_lookup,
        }
    }

    pub fn process(&mut self) {
        self.process_frame();
        self.print_terminal();
    }

    fn process_frame(&mut self) {
        //
        imgproc::cvt_color(
            &self.frame,
            &mut self.gray_frame,
            imgproc::COLOR_BGR2GRAY,
            0,
        )
        .expect("Failed to convert to grayscale");

        imgproc::resize(
            &self.gray_frame,
            &mut self.frame,
            self.size,
            0.0,
            0.0,
            imgproc::INTER_LINEAR,
        )
        .expect("Failed to resize the frame");
    }

    fn print_terminal(&self) {
        print!("{}[2J", 27 as char);

        for y in 0..self.target_height {
            for x in 0..self.target_width {
                let intensity = self.frame.at_2d::<u8>(y, x).unwrap();
                let ascii_char = self.ascii_lookup[*intensity as usize];
                print!("{}", ascii_char);
            }
            println!();
        }
    }
}
