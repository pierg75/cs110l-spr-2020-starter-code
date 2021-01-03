use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;
use std::cmp::max;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
//#[allow(unused)] // TODO: delete this line when you implement this function
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let mut vec_strings: Vec<String> = Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        vec_strings.push(line?);
    }
    println!("vec_strings {:?}", vec_strings);
    Ok(vec_strings)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    let mut lcs_grid = Grid::new(seq1.len()+1, seq2.len()+1);
    // here, the .., like in Rust, is inclusive on the left bound and exclusive on
    // the right bound
    for i in 0..seq1.len()+1 {
        lcs_grid.set(i,0,0).unwrap();
    }
    for j in 0..seq2.len()+1 {
        lcs_grid.set(0,j,0).unwrap();
    }
    for i in 0..seq1.len() {
        for j in 0..seq2.len() {
            if seq1.get(i) == seq2.get(j) {
                lcs_grid.set(i+1,j+1, lcs_grid.get(i,j).unwrap_or(0) + 1).unwrap();
            } else {
                lcs_grid.set(i+1,j+1, max(lcs_grid.get(i+1,j).unwrap_or(0), lcs_grid.get(i,j+1).unwrap_or(0))).unwrap();
            }
        }
    }
    lcs_grid
}

fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    // let C be the grid computed by lcs()
    // let X and Y be sequences
    // i and j specify a location within C that you want to look at when reading out
    // the diff. (This makes more sense if you understand the LCS algorithm, but
    // it's not important.) When you call this function initially, just pass
    // i=len(X) and j=len(Y).
    // function print_diff(C, X, Y, i, j)
    if i > 0 && j > 0 && lines1.get(i-1) == lines2.get(j-1) {
        print_diff(lcs_table, lines1, lines2, i-1, j-1);
        println!("  {:?}", lines1.get(i-1).unwrap_or(&"".to_owned()));
    } else if j > 0 && (i == 0 || lcs_table.get(i,j-1) >= lcs_table.get(i-1,j)) {
        print_diff(lcs_table, lines1, lines2, i, j-1);
        println!("> {:?}", lines2.get(j-1).unwrap_or(&"".to_owned()));
    } else if i > 0 && (j == 0 || lcs_table.get(i,j-1) < lcs_table.get(i-1,j)) {
        print_diff(lcs_table, lines1, lines2, i-1, j);
        println!("< {:?}", lines1.get(i-1).unwrap_or(&"".to_owned()));
    } else {
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];
    
    let vec_file1 = read_file_lines(filename1).unwrap();
    let vec_file2 = read_file_lines(filename2).unwrap();
    let lcs_grid = lcs(&vec_file1, &vec_file2);
    print_diff(&lcs_grid, &vec_file1, &vec_file2, vec_file1.len(), vec_file2.len());

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
