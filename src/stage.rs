use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone)]
pub struct Stage {
    corner_end_points: [u16; 4],
    pixels: [[u8; 4]; 789],
    mode: Mode
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, EnumString, EnumIter, Display)]
pub enum Mode {
    #[strum(serialize = "off")]
    Off,
    #[strum(serialize = "tealwave")]
    TealWave,
    #[strum(serialize = "white")]
    White,
    #[strum(serialize = "red")]
    Red,
    #[strum(serialize = "orange")]
    Orange,
    #[strum(serialize = "yellow")]
    Yellow,
    #[strum(serialize = "green")]
    Green,
    #[strum(serialize = "blue")]
    Blue,
    #[strum(serialize = "pink")]
    Pink,
    #[strum(serialize = "purple")]
    Purple,
    #[strum(serialize = "rainbow")]
    Rainbow
}

impl Mode {
    pub fn lower(&self) -> &'static str {
        match self {
            Mode::Off => "off",
            Mode::TealWave => "tealwave",
            Mode::White => "white",
            Mode::Red => "red",
            Mode::Orange => "orange",
            Mode::Yellow => "yellow",
            Mode::Green => "green",
            Mode::Blue => "blue",
            Mode::Pink => "pink",
            Mode::Purple => "purple",
            Mode::Rainbow => "rainbow",
        }
    }

    pub fn css_button_color(&self) -> &'static str {
        match self {
            Mode::Off => "black",
            Mode::TealWave => "teal",
            Mode::White => "white",
            Mode::Red => "red",
            Mode::Orange => "orange",
            Mode::Yellow => "yellow",
            Mode::Green => "green",
            Mode::Blue => "blue",
            Mode::Pink => "hotpink",
            Mode::Purple => "purple",
            Mode::Rainbow => "white",
        }
    }

    pub fn css_text_color(&self) -> &'static str {
        match self {
            Mode::Off => "aqua",
            Mode::TealWave => "#800000",
            Mode::White => "black",
            Mode::Red => "#00ffff",
            Mode::Orange => "#0059ff",
            Mode::Yellow => "#0000ff",
            Mode::Green => "#800080",
            Mode::Blue => "#ffff00",
            Mode::Pink => "#69ffb4",
            Mode::Purple => "#008000",
            Mode::Rainbow => "black",
        }
    }
}

impl Stage {
    pub fn init(corner_end_points: [u16; 4], pixels: [[u8; 4]; 789]) -> Self {
        Self { corner_end_points, pixels, mode: Mode::TealWave }
    }

    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn get_full_stage(&self) -> &[[u8; 4]] {
        &self.pixels
    }

    // pub fn get_stage_subset(&self, mut start: i16, mut end: i16) -> Result<Vec<[u8; 4]>, &str> {
    //     while start < 0 {
    //         start += 789;
    //     }
        
    //     while end < 0 {
    //         end += 789;
    //     }
        
    //     if start < 0 || end < 0 {
    //         return Err("Invalid start and/or end index");
    //     }
        
    //     // println!("{}", format!("{} -> {}", start, end));
    //     let start_i: usize = start.try_into().unwrap();
    //     let end_i: usize = end.try_into().unwrap();

    //     if end < 0 {
    //         let mut final_vec = Vec::new();
    //         final_vec.extend_from_slice(&self.pixels[end_i..789]);
    //         final_vec.extend_from_slice(&self.pixels[0..start_i]);
    //         return Ok(final_vec.clone());
    //     } else {
    //         return Ok(self.pixels[start_i..end_i].to_vec());
    //     }
    // }

    // pub fn get_wall(&self, wall: usize) -> Result<Vec<[u8; 4]>, &str> {
    //     if wall >= self.corner_end_points.len() {
    //         return Err("Invalid wall index");
    //     }
    //     let start = if wall == 0 { 1 } else { self.corner_end_points[wall - 1] + 1 };
    //     let end = self.corner_end_points[wall];
    //     if end <= 789 {
    //         return self.get_stage_subset(start.try_into().unwrap(), end.try_into().unwrap());
    //     } else {
    //         Err("End index out of bounds")
    //     }
    // }

    // pub fn get_corner_area(&self, corner: usize) -> Result<Vec<[u8; 4]>, &str> {
    //     if corner >= 4 {
    //         return Err("Invalid corner value");
    //     }

    //     let start: i16 = if corner == 0 {
    //         -99
    //     } else {
    //         (
    //             if corner % 2 == 0 {
    //                 self.corner_end_points[corner - 1] - 98
    //             } else {
    //                 self.corner_end_points[corner - 1] - 99
    //             }
    //         ).try_into().unwrap()
    //     };

    //     let end: i16 = if corner == 0 {
    //         97
    //     } else {
    //         ((self.corner_end_points[corner - 1] - 1) + if corner % 2 == 0 { 100 } else { 99 })
    //             .try_into()
    //             .unwrap()
    //     };

    //     // println!("RUNNING {}", corner);

    //     return self.get_stage_subset(start, end);
    // }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
}

/*
1 99 98 0
99     99
99     99
2 98 99 3
 */

/*

  1         short 197         0
   196                       0
  l                           l
  o                           o
  n                           n
  g                           g

  1                           1
  9                           9
  9                           9
   394                     591
  2         short 198         3

*/
