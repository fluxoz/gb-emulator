#[derive(Clone, Copy)]
enum Colors {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

impl Colors {
    fn into_row(value: u16) -> [Colors; 8] {
        let mut row = [Colors::White; 8];
        let mut value = value;
        for x in &mut row {
           *x = match &value & 3 {
                0 => Colors::White,
                1 => Colors::DarkGrey,
                2 => Colors::LightGrey,
                3 => Colors::Black,
                _ => Colors::White,
           };
           value = value >> 2;
        }
        row
    }
    fn into_tile(data: u128) -> [[Colors;8];8] {
        let mut data = data;
        let mut color_data = [[Colors::White; 8]; 8];
        for x in &mut color_data {
            let row_value: u16 = (data & 0xFF) as u16;
            *x = Self::into_row(row_value);
            data = data >> 16;
        } 
        color_data
    }
}

// display tile
struct Tile {
    data: u128,
    color_data: [[Colors;8];8],
}

impl Tile {
    fn new(data: u128) -> Self {
        Self {
            data,
            color_data: Colors::into_tile(data),
        }        
    }
}

struct Screen {
    // rows x columns 18 rows, 20 columns
    data: [[Colors; 20]; 18]
}
