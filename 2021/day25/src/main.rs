use utils::*;

#[allow(dead_code)]
fn print_floor(floor: &[char], width: usize) {
    for row in floor.chunks(width) {
        for ch in row.iter() {
            print!("{ch}");
        }
        println!();
    }
}

fn main() {
    // let input = "v...>>.vv>
    // .vv>>.vv..
    // >>.>v>...v
    // >>v>>.>.v.
    // v>v.vv.v..
    // >.>>..v...
    // .vv..>.>v.
    // v.v..>>v.v
    // ....v..v.>";
    let input = read_puzzle_input!().unwrap();

    let lines: Vec<_> = input.lines()
        .map(|line| line.trim())
        .collect();
    
    let width = lines[0].len();
    let height = lines.len();
    let mut floor: Vec<_> = lines.iter()
        .flat_map(|line| line.chars())
        .collect();
    
    let mut steps = 0;
    loop {
        let mut moved = false;

        // Move Easterlings
        for row in floor.chunks_mut(width) {
            let current = row.to_vec();
            for x in 0..width {
                let x2 = (x+1)%width;
                if current[x] == '>' && current[x2] == '.' {
                    row[x] = '.';
                    row[x2] = '>';
                    moved = true;
                }
            }
        }

        // Move Southerlings
        for col in 0..width {
            let current: Vec<_> = floor.iter()
                .skip(col)
                .step_by(width)
                .copied()
                .collect();
            let mut col: Vec<_> = floor.iter_mut()
                .skip(col)
                .step_by(width)
                .collect();
            
            for y in 0..height {
                let y2 = (y+1)%height;
                if current[y] == 'v' && current[y2] == '.' {
                    *col[y] = '.';
                    *col[y2] = 'v';
                    moved = true;
                }
            }
        }

        steps += 1;

        if !moved {
            break;
        }
    }

    println!("Cucumbers stopped moving at {steps} steps");
}
