use std::{
    fs::File,
    io::{BufRead, BufReader},
    str, thread,
    time::Duration,
};

// TODO: configuration?
// TODO: Only write to terminal as needed, use escape codes to move the cursor
const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const ANIMATION_INTERVAL: Duration = Duration::new(0, 500_000_000);

struct Write {
    index: usize,
    live: bool,
}

const LIVE: u8 = '*' as u8;
const DEAD: u8 = '.' as u8;
const LINE_FEED: u8 = '\n' as u8;

fn initial_from_file(file_name: &str) -> Vec<u8> {
    let width = WIDTH;
    let height = HEIGHT;

    // Build grid
    let mut grid = Vec::with_capacity((width + 1) * height);
    let mut line_count = 0;

    let file = File::open(file_name).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        grid.extend_from_slice(line.as_bytes());
        let count = width - line.len();
        for _ in 0..count {
            grid.push(DEAD);
        }
        grid.push(LINE_FEED);
        line_count += 1;
    }

    for _ in 0..(height - line_count) {
        for _ in 0..width {
            grid.push(DEAD);
        }
        grid.push(LINE_FEED);
    }
    grid
}

fn initial_random() -> Vec<u8> {
    let width = WIDTH;
    let height = HEIGHT;
    let mut grid = Vec::with_capacity((width + 1) * height);
    for y in 0..height {
        for x in 0..width {
            grid.push(if rand::random() {
                LIVE
            } else { DEAD });
        }
        grid.push(LINE_FEED);
    }
    grid
}

fn main() {
    let width = WIDTH;
    let height = HEIGHT;

    // Build grid
    let mut grid = {
        if let Some(file_name) = std::env::args().nth(1) {
            initial_from_file(&file_name)
        } else {
            initial_random()
        }
    };

    // Allocate needed
    let row: isize = width as isize + 1;
    let offsets: Vec<isize> = vec![-row - 1, -row, -row + 1, -1, 1, row - 1, row, row + 1];

    let mut writes: Vec<Write> = Vec::with_capacity(width * height);
    loop {
        // Reset screen
        print!("\x1b[1;1H\x1b[0J");
        // Print grid
        print!("{}", unsafe { str::from_utf8_unchecked(&grid) });

        thread::sleep(ANIMATION_INTERVAL);

        // advance
        for x in 0..width {
            for y in 0..height {
                let index = y * (width + 1) + x;
                let cell = grid[index];

                // count live neighbors
                let index_as_isize: isize = index.try_into().unwrap();
                let mut live_count = 0;
                for offset in &offsets {
                    let index = index_as_isize + offset;
                    if let Ok(index) = usize::try_from(index) {
                        if let Some(cell) = grid.get(index) {
                            if cell == &LIVE {
                                live_count += 1;
                            }
                        }
                    }
                }

                // Generation
                if cell == DEAD && live_count == 3 {
                    writes.push(Write { index, live: true });
                }
                // Underpopulation / Overpopulation
                if cell == LIVE && (live_count < 2 || live_count > 3) {
                    writes.push(Write { index, live: false });
                }
            }
        }

        for w in &writes {
            grid[w.index] = if w.live { LIVE } else { DEAD };
        }
        writes.clear();
    }
}
