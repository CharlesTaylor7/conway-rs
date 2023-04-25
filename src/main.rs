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
const ANIMATION_INTERVAL: Duration = Duration::new(0, 200_000_000);

struct Write {
    index: usize,
    live: bool,
}

const LIVE: u8 = 42;
const DEAD: u8 = 46;

fn main() {
    let width = WIDTH;
    let height = HEIGHT;

    // Build grid
    let mut grid = String::with_capacity((width + 1) * height);
    let mut line_count = 0;

    let file = File::open("conway.txt").unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        grid.push_str(&line);
        let count = width - line.len();
        for _ in 0..count {
            grid.push('.');
        }
        grid.push('\n');
        line_count += 1;
    }

    for _ in 0..(height - line_count) {
        for _ in 0..width {
            grid.push('.');
        }
        grid.push('\n');
    }

    // Allocate needed
    let row: isize = isize::try_from(width).unwrap() + 1;
    let offsets: Vec<isize> = vec![-row - 1, -row, -row + 1, -1, 1, row - 1, row, row + 1];

    let bytes = unsafe { grid.as_bytes_mut() };
    let mut writes: Vec<Write> = Vec::with_capacity(width * height);
    loop {
        // Reset screen
        print!("\x1b[1;1H\x1b[0J");
        // Print grid
        print!("{}", unsafe { str::from_utf8_unchecked(bytes) });

        thread::sleep(ANIMATION_INTERVAL);

        // advance
        for x in 0..width {
            for y in 0..height {
                let index = y * (width + 1) + x;
                let cell = bytes[index];

                // count live neighbors
                let index_as_isize: isize = index.try_into().unwrap();
                let mut live_count = 0;
                for offset in &offsets {
                    let index = index_as_isize + offset;
                    if let Ok(index) = usize::try_from(index) {
                        if let Some(cell) = bytes.get(index) {
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
            bytes[w.index] = if w.live { LIVE } else { DEAD };
        }
        writes.clear();
    }
}

#[allow(dead_code)]
fn write(x: usize, y: usize, c: char) {
    print!("\x1b[{};{}H{}", y + 1, x + 1, c);
}
