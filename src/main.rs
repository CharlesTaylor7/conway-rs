use std::{time::Duration, thread, str, fs::File, io::{BufRead, BufReader}};

// TODO: configuration? 
const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const ANIMATION_INTERVAL: Duration = Duration::new(0, 200_000_000);

struct Write {
    pub index: usize,
    pub live: bool,
}

const LIVE: u8 = 42;
const DEAD: u8 = 46;
const LINE_FEED: u8 = 10;

fn main() {
    unsafe {
        unsafe_main();
    }
}

unsafe fn unsafe_main() {
    let file = File::open("conway.txt").unwrap();
    let mut initial = Vec::new();
    for line in BufReader::new(file).lines() {
        initial.push(line.unwrap()); 
    }

    let width = WIDTH;    
    let height = HEIGHT;    
    let mut grid = String::new();

    // Pad
    for _ in 0..(height - initial.len()) {
        initial.push(String::new());
    }

    for line in &initial {
        grid.push_str(&line);
        let count = width - line.len();
        for _ in 0..count {
            grid.push('.');
        }
        grid.push('\n');
    }

    // Allocate needed
    let row: isize = isize::try_from(width).unwrap() + 1;
    let offsets: Vec<isize> = vec![
        -row - 1, -row, -row + 1,
        -1, 1,
        row -1, row, row + 1,
    ];

    let bytes = grid.as_mut_vec();
    let mut writes: Vec<Write> = Vec::with_capacity(width * height);

    loop {
        // Print current state
        print!("\x1b[1;1H\x1b[0J");
        print!("{}", str::from_utf8_unchecked(bytes));
        thread::sleep(ANIMATION_INTERVAL);

        // advance
        for index in 0..bytes.len() {
            let cell = bytes[index];
            if cell == LINE_FEED {
                continue;
            }

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
        for write in &writes {
            bytes[write.index] = if write.live { LIVE } else { DEAD };
        }
        writes.clear();
    }
}
