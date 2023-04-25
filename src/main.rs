use std::time::Duration;
use std::thread::sleep;
use std::str;

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
    // TODO: read from file
    let mut initial = vec![
        ".*.",
        "..*",
        "***",
    ];
    
    let height = 25;
    let width = 80;

    let mut grid = String::new();

    // Pad
    for _ in 0..(height - initial.len()) {
        initial.push("");
    }

    for line in initial {
        grid.push_str(line);
        let count = width - line.len();
        for _ in 0..count {
            grid.push('.');
        }
        grid.push('\n');
    }

    // Advance
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
        sleep(Duration::new(0, 400_000_000));
        writes.clear();
        for index in 0..bytes.len() {
            let cell = bytes[index];
            if cell == LINE_FEED {
                continue;
            }
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
    }
}
