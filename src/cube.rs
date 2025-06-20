pub static YELLOW_BG: &str = "\x1b[43m";
pub static ORANGE_BG: &str = "\x1b[48;2;255;165;0m"; // true color (24-bit) for orange
pub static WHITE_BG: &str = "\x1b[47m";
pub static BLUE_BG: &str = "\x1b[44m";
pub static RED_BG: &str = "\x1b[41m";
pub static GREEN_BG: &str = "\x1b[42m";
pub static RESET: &str = "\x1b[0m";

#[derive(Debug, Clone, Copy)]
enum ColorPiece {
    Yellow,
    Green,
    White,
    Blue,
    Orange,
    Red,
    None,
}

impl ColorPiece {
    fn display(self) {
        match self {
            ColorPiece::Yellow => print!("{}  {} ", YELLOW_BG, RESET),
            ColorPiece::Green => print!("{}  {} ", GREEN_BG, RESET),
            ColorPiece::White => print!("{}  {} ", WHITE_BG, RESET),
            ColorPiece::Blue => print!("{}  {} ", BLUE_BG, RESET),
            ColorPiece::Orange => print!("{}  {} ", ORANGE_BG, RESET),
            ColorPiece::Red => print!("{}  {} ", RED_BG, RESET),
            ColorPiece::None => print!("{}", RESET),
        }
    }
}

pub struct RubicsCube {
    // all sides in one array
    sides: [ColorPiece; 54],
}

impl RubicsCube {
    pub fn new() -> Self {
        Self {
            sides: [
                // Front
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                ColorPiece::Red,
                // Right
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                ColorPiece::Blue,
                // Back
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                ColorPiece::Orange,
                // Left
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                ColorPiece::Green,
                // Top
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                ColorPiece::White,
                // Bottom
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
                ColorPiece::Yellow,
            ],
        }
    }

    pub fn show(&self) {
        println!("  FRONT      RIGHT      BACK       LEFT        TOP       DOWN");
        for row in 0..3 {
            for side in 0..6 {
                let left = self.sides[side * 9 + row * 3]; // X--
                let middle = self.sides[side * 9 + row * 3 + 1]; // -X-
                let right = self.sides[side * 9 + row * 3 + 2]; // --X
                left.display();
                middle.display();
                right.display();
                print!("  "); // a couple of spaces between sides
            }
            println!(""); // newline per row
        }
    }

    pub fn rotate_down(&mut self, col: usize) {
        let offset = col; // bottom side offset
        // backup front side
        let front_side_backup: [ColorPiece; 3] = [
            self.sides[offset],
            self.sides[offset + 3],
            self.sides[offset + 6],
        ];
        // rewrite front with top
        self.sides[offset] = self.sides[offset + 36];
        self.sides[offset + 3] = self.sides[offset + 39];
        self.sides[offset + 6] = self.sides[offset + 42];

        // rewrite top with back
        let back_offset = 18 + col; // back side offset
        self.sides[offset + 36] = self.sides[back_offset];
        self.sides[offset + 39] = self.sides[back_offset + 3];
        self.sides[offset + 42] = self.sides[back_offset + 6];

        // rewrite back with bottom
        let bottom_offset = 45 + col; // bottom side offset
        self.sides[back_offset] = self.sides[bottom_offset];
        self.sides[back_offset + 3] = self.sides[bottom_offset + 3];
        self.sides[back_offset + 6] = self.sides[bottom_offset + 6];

        // rewrite bottom with front backup
        self.sides[bottom_offset] = front_side_backup[0];
        self.sides[bottom_offset + 3] = front_side_backup[1];
        self.sides[bottom_offset + 6] = front_side_backup[2];

        // rotate right side
        if col == 2 {
            let offset: usize = 9;
            let right_side: [ColorPiece; 9] = self.sides[9..18]
                .try_into()
                .expect("can't backup bottom side");

            self.sides[offset] = right_side[2]; // +2
            self.sides[offset + 1] = right_side[5]; // +4
            self.sides[offset + 2] = right_side[8]; // +6
            self.sides[offset + 3] = right_side[1]; // -2
            //  0 : 4 is 4
            self.sides[offset + 5] = right_side[7]; // +2
            self.sides[offset + 6] = right_side[0]; // -6
            self.sides[offset + 7] = right_side[3]; // -4
            self.sides[offset + 8] = right_side[6]; // -2
        }
        // rotate left side
        if col == 0 {
            let offset = 27;
            let left_side: [ColorPiece; 9] = self.sides[27..36]
                .try_into()
                .expect("can't backup top side");

            self.sides[offset] = left_side[6]; // -6
            self.sides[offset + 1] = left_side[3]; // +2
            self.sides[offset + 2] = left_side[0]; // -2
            self.sides[offset + 3] = left_side[7]; // +4
            //  0 : 4 is 4
            self.sides[offset + 5] = left_side[1]; // -4
            self.sides[offset + 6] = left_side[8]; // +2             
            self.sides[offset + 7] = left_side[5]; // -2
            self.sides[offset + 8] = left_side[2]; // -6
        }
    }

    pub fn rotate_up(&mut self, col: usize) {
        let offset = col; // front side offset
        // backup front side
        let front_side_backup: [ColorPiece; 3] = [
            self.sides[offset],
            self.sides[offset + 3],
            self.sides[offset + 6],
        ];
        // rewrite front with bottom
        self.sides[offset] = self.sides[offset + 45];
        self.sides[offset + 3] = self.sides[offset + 48];
        self.sides[offset + 6] = self.sides[offset + 51];

        // rewrite bottom with back
        let back_offset = 18 + col; // back side offset
        self.sides[offset + 45] = self.sides[back_offset];
        self.sides[offset + 48] = self.sides[back_offset + 3];
        self.sides[offset + 51] = self.sides[back_offset + 6];

        // rewrite back with top
        let top_offset = 36 + col; // top side offset
        self.sides[back_offset] = self.sides[top_offset];
        self.sides[back_offset + 3] = self.sides[top_offset + 3];
        self.sides[back_offset + 6] = self.sides[top_offset + 6];

        // rewrite top with front backup
        self.sides[top_offset] = front_side_backup[0];
        self.sides[top_offset + 3] = front_side_backup[1];
        self.sides[top_offset + 6] = front_side_backup[2];

        // rotate right side
        if col == 2 {
            let offset: usize = 9;
            let right_side: [ColorPiece; 9] = self.sides[9..18]
                .try_into()
                .expect("can't backup bottom side");

            self.sides[offset] = right_side[6]; // +6
            self.sides[offset + 1] = right_side[3]; // +2
            self.sides[offset + 2] = right_side[0]; // -2
            self.sides[offset + 3] = right_side[7]; // +4
            //  0 : 4 is 4
            self.sides[offset + 5] = right_side[1]; // -4
            self.sides[offset + 6] = right_side[8]; // +2
            self.sides[offset + 7] = right_side[5]; // -2
            self.sides[offset + 8] = right_side[2]; // -6
        }
        // rotate left side
        if col == 0 {
            let offset = 27;
            let left_side: [ColorPiece; 9] = self.sides[27..36]
                .try_into()
                .expect("can't backup top side");

            self.sides[offset] = left_side[2]; // +2
            self.sides[offset + 1] = left_side[5]; // +4
            self.sides[offset + 2] = left_side[8]; // +6
            self.sides[offset + 3] = left_side[1]; // -2
            //  0 : 4 is 4
            self.sides[offset + 5] = left_side[7]; // +2
            self.sides[offset + 6] = left_side[0]; // -6             
            self.sides[offset + 7] = left_side[3]; // -4
            self.sides[offset + 8] = left_side[6]; // -2
        }
    }

    pub fn rotate_right(&mut self, row: usize) {
        let side = 3;
        let offset = side * 9 + row * 3;
        // backup left side
        let left_side_backup: [ColorPiece; 3] = [
            self.sides[offset],
            self.sides[offset + 1],
            self.sides[offset + 2],
        ];
        // move front->right->back->left
        for side in 0..3 {
            let dst_offset = (3 - side) * 9 + row * 3;
            let src_offset = dst_offset - 9;
            for i in 0..3 {
                self.sides[dst_offset + i] = self.sides[src_offset + i];
            }
        }
        // restore front from left backup
        for i in 0..3 {
            self.sides[row * 3 + i] = left_side_backup[i];
        }

        // rotate top side
        if row == 0 {
            let offset = 36;
            let top_side: [ColorPiece; 9] = self.sides[36..45]
                .try_into()
                .expect("can't backup top side");

            self.sides[offset] = top_side[2]; // +2
            self.sides[offset + 1] = top_side[5]; // +4
            self.sides[offset + 2] = top_side[8]; // +6
            self.sides[offset + 3] = top_side[1]; // -2
            //  0 : 4 is 4
            self.sides[offset + 5] = top_side[7]; // +2
            self.sides[offset + 6] = top_side[0]; // -6
            self.sides[offset + 7] = top_side[3]; // -4
            self.sides[offset + 8] = top_side[6]; // -2
        }
        // rotate bottom side
        if row == 2 {
            let offset = 45;
            let bottom_side: [ColorPiece; 9] = self.sides[45..54]
                .try_into()
                .expect("can't backup bottom side");

            self.sides[offset] = bottom_side[6]; // +6
            self.sides[offset + 1] = bottom_side[3]; // +2
            self.sides[offset + 2] = bottom_side[0]; // -2
            self.sides[offset + 3] = bottom_side[7]; // +4
            //  0 : 4 is 4
            self.sides[offset + 5] = bottom_side[1]; // -4
            self.sides[offset + 6] = bottom_side[8]; // +2
            self.sides[offset + 7] = bottom_side[5]; // -2
            self.sides[offset + 8] = bottom_side[2]; // -6
        }
    }

    pub fn rotate_left(&mut self, row: usize) {
        let offset = row * 3;
        // backup front side
        let front_side_backup: [ColorPiece; 3] = [
            self.sides[offset],
            self.sides[offset + 1],
            self.sides[offset + 2],
        ];
        // move front<-right<-back<-left
        for side in 0..3 {
            let dst_offset = side * 9 + row * 3;
            let src_offset = dst_offset + 9;
            for i in 0..3 {
                self.sides[dst_offset + i] = self.sides[src_offset + i];
            }
        }
        // restore front from left backup
        for i in 0..3 {
            self.sides[27 + row * 3 + i] = front_side_backup[i];
        }
        // rotate top side
        if row == 0 {
            let offset = 36;
            let top_side: [ColorPiece; 9] = self.sides[36..45]
                .try_into()
                .expect("can't backup top side");

            self.sides[offset] = top_side[6]; // +6
            self.sides[offset + 1] = top_side[3]; // +2
            self.sides[offset + 2] = top_side[0]; // -2
            self.sides[offset + 3] = top_side[7]; // +4
            //  0 : 4 is 4
            self.sides[offset + 5] = top_side[1]; // -4
            self.sides[offset + 6] = top_side[8]; // +2
            self.sides[offset + 7] = top_side[5]; // -2
            self.sides[offset + 8] = top_side[2]; // -6
        }
        // rotate bottom side
        if row == 2 {
            let offset = 45;
            let bottom_side: [ColorPiece; 9] = self.sides[45..54]
                .try_into()
                .expect("can't backup bottom side");

            self.sides[offset] = bottom_side[2]; // +2
            self.sides[offset + 1] = bottom_side[5]; // +4
            self.sides[offset + 2] = bottom_side[8]; // +6
            self.sides[offset + 3] = bottom_side[1]; // -2
            //  0 : 4 is 4
            self.sides[offset + 5] = bottom_side[7]; // +2
            self.sides[offset + 6] = bottom_side[0]; // -6
            self.sides[offset + 7] = bottom_side[3]; // -4
            self.sides[offset + 8] = bottom_side[6]; // -2
        }
    }
}
