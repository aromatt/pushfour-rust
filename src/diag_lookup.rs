use util::*;

#[derive(Clone, Copy, Debug)]
struct DiagLookup {
    main: [[Coord; BOARD_SIZE]; BOARD_SIZE],
    rot: [[Coord; BOARD_SIZE]; BOARD_SIZE]
}

impl DiagLookup {
    /* This struct contains lookup tables for translating regular board coordinates to
     * coordinates in two diagonal rotations of the board ('main' and 'rot').
     * The diagonal representations are only used for detecting diagonal win states.
     * The lookup tables are only used for *setting* bits in the diagonal representations.
     *
     *       00            02
     *     10  01        01  12
     *   20  11  02    00  11  22   -- Representations ('00' means 'top left on real board')
     *     21  12        10  21
     *       22            22
     *
     *    00 11 22      20 10 00
     *    10 21 31      30 21 11    -- Lookup tables
     *    20 30 40      40 31 22
     *
     *      (1)           (2)
     *
     *    TODO: Alternative lookup table instead of (2), which results in an inversion
     *    of representation (1). This lookup table is obtained by flipping (1)'s
     *    lookup table about its middle row:
     *
     *      20 30 40
     *      10 21 31
     *      00 11 22
     */
    pub fn new(size: usize) -> DiagLookup {
        let mut main =  [[Coord(0, 0); BOARD_SIZE]; BOARD_SIZE];
        let mut rot = [[Coord(0, 0); BOARD_SIZE]; BOARD_SIZE];
        let mut key_row_reset = 1;
        let mut key_col_reset = 1;
        let mut key_row = 0;
        let mut key_col = 0;
        let mut val_row = 0;
        let mut val_col = 0;
        let mut total = 0;
        while total < size * size {
            main[key_row][key_col] = Coord(val_row, val_col);
            let (key_row_rot, key_col_rot) = rotate_cw(size, key_row, key_col);
            rot[key_row_rot][key_col_rot] = Coord(val_row, val_col);

            // Reset from top row to the left column
            if key_row == 0 && key_row_reset < size {
                key_row = key_row_reset;
                key_col = 0;
                key_row_reset += 1;
                val_col = 0;
                val_row += 1;
                // Reset from the right column to the bottom row
            } else if key_col == size - 1 && key_col_reset < size {
                key_col = key_col_reset;
                key_row = size - 1;
                key_col_reset += 1;
                val_col = 0;
                val_row += 1;
                // Normal traversal up and to the right
            } else {
                key_row -= 1;
                key_col += 1;
                val_col += 1;
            }
            total += 1;
        }
        DiagLookup {
            main: main,
            rot: rot,
        }
    }
}

lazy_static! {
    static ref TABLES: [DiagLookup; 7] = [
        DiagLookup::new(2),
        DiagLookup::new(3),
        DiagLookup::new(4),
        DiagLookup::new(5),
        DiagLookup::new(6),
        DiagLookup::new(7),
        DiagLookup::new(8),
    ];
}

pub fn lookup(size: usize, row: usize, col: usize) -> (Coord, Coord) {
    ((*TABLES)[size - 2].main[row][col],(*TABLES)[size - 2].rot[row][col])
}
