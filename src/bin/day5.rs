use aoc2020::advent_main;

advent_main!(day5, "data/day5_input.txt");

fn day5(lines: &[String]) -> (usize, usize) {
    let coords: Vec<_> = lines
        .iter()
        .map(|pass| pass_to_coords(pass, 128, 8))
        .collect();
    let mut ids: Vec<_> = coords.iter().map(|(row, col)| row * 8 + col).collect();
    ids.sort_unstable();

    let part1 = ids.iter().max().unwrap();
    let part2 = find_singular_gap(ids.as_slice());
    (*part1, part2)
}

fn pass_to_coords(pass: &str, rows: usize, columns: usize) -> (usize, usize) {
    let mut high_row = rows;
    let mut low_row = 0;
    let mut high_col = columns;
    let mut low_col = 0;

    for command in pass.chars() {
        let mid_row = (high_row + low_row) / 2;
        let mid_col = (high_col + low_col) / 2;

        match command {
            'F' => high_row = mid_row,
            'B' => low_row = mid_row,
            'L' => high_col = mid_col,
            'R' => low_col = mid_col,
            _ => unreachable!("Invalid data"),
        }
    }

    (low_row, low_col)
}

fn find_singular_gap(values: &[usize]) -> usize {
    *values
        .iter()
        .zip(&values[1..])
        .find_map(|(previous, next)| {
            if previous + 2 == *next {
                Some(previous)
            } else {
                None
            }
        })
        .unwrap()
}

#[test]
fn test_coord_conversion() {
    let result = pass_to_coords("FBFBBFFRLR", 128, 8);
    assert_eq!(result, (44, 5));
}
