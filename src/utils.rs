// // Define the color wheel function
// pub fn color_wheel_256(pos: u16) -> (u8, u8, u8) {
//     let pos: u8 = (pos % 256).try_into().unwrap(); // Ensure pos is within the valid range
    
//     if pos < 85 {
//         // Red to Green
//         println!("R->G");
//         return (255 - pos * 3, pos * 3, 0);
//     } else if pos < 170 {
//         // Green to Blue
//         println!("G->B");
//         let pos = pos - 85;
//         return (0, 255 - pos * 3, pos * 3);
//     } else {
//         // Blue to Red
//         println!("B->R");
//         let pos = pos - 170;
//         return (pos * 3, 0, 255 - pos * 3);
//     }
// }

// Define the color wheel function
pub fn color_wheel_768(pos: u16) -> (u8, u8, u8) {
    let pos: u16 = pos % 768; // Ensure pos is within the valid range
    
    if pos < 256 {
        // Red to Green
        // println!("R->G");
        let pos: u8 = pos as u8;
        return (255 - pos, pos, 0);
    } else if pos < 512 {
        // Green to Blue
        // println!("G->B");
        let pos: u8 = (pos - 256) as u8;
        return (0, 255 - pos, pos);
    } else {
        // Green to Blue
        // println!("G->B");
        let pos: u8 = (pos - 512) as u8;
        return (pos, 0, 255 - pos);
    }
}

// // Define the color wheel function
// pub fn color_wheel_6144(pos: u16) -> (u8, u8, u8) {
//     let pos = pos % 6144; // Ensure pos is within the valid range
    
//     if pos < 1024 {
//         // Red to Yellow
//         println!("R->Y");
//         return (255, (pos / 4) as u8, 0);
//     } else if pos < 2048 {
//         // Yellow to Green
//         println!("Y->G");
//         let pos = pos - 1024;
//         return (255 - (pos / 4) as u8, 255, 0);
//     } else if pos < 3072 {
//         // Green to Cyan
//         println!("G->C");
//         let pos = pos - 2048;
//         return (0, 255, (pos / 4) as u8);
//     } else if pos < 4096 {
//         // Cyan to Blue
//         println!("C->B");
//         let pos = pos - 3072;
//         return (0, 255 - (pos / 4) as u8, 255);
//     } else if pos < 5120 {
//         // Blue to Magenta
//         println!("B->M");
//         let pos = pos - 4096;
//         return ((pos / 4) as u8, 0, 255);
//     } else {
//         // Magenta to Red
//         println!("M->R");
//         let pos = pos - 5120;
//         return (255, 0, 255 - (pos / 4) as u8);
//     }
// }
