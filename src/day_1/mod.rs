static INPUT_1: &'static str = include_str!("./input_1.txt");

///
/// As they're making the final adjustments, they discover that their calibration document
/// (your puzzle input) has been amended by a very young Elf who was apparently just excited
/// to show off her art skills. Consequently, the Elves are having trouble reading the
/// values on the document.
///
/// The newly-improved calibration document consists of lines of text; each line originally
/// contained a specific calibration value that the Elves now need to recover. On each line,
/// the calibration value can be found by combining the first digit and the last digit
/// (in that order) to form a single two-digit number.
///
/// For example:
/// ```
/// 1abc2
/// pqr3stu8vwx
/// a1b2c3d4e5f
/// treb7uchet
/// ```
/// In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.
///
/// Consider your entire calibration document. What is the sum of all of the calibration values?
/// takes an input str and determines the "calibration value"
fn solve_1(input: &str) -> usize {
    let mut o = 0usize;
    for line in input.lines() {
        let mut iter = line
            .trim()
            .chars()
            .filter_map(|c| c.is_digit(10).then(|| c.to_digit(10)).flatten())
            .map(|c| c as usize);
        match (iter.next(), iter.last()) {
            (Some(first), Some(last)) => {
                o += first * 10;
                o += last
            }
            (Some(first), _) => {
                o += first * 10;
                o += first
            }
            _ => {}
        }
    }

    return o;
}

/// Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters:
/// one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
//
// Equipped with this new information, you now need to find the real first and last digit on each line. For example:
fn solve_2(input: &str) -> usize {
    let mut output = 0usize;
    static DIGIT_SPELLINGS: [&'static str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn check(str: &str) -> Option<usize> {
        // check if the str contains any digits
        str.chars()
            .find(|d| d.is_digit(10))
            .map(|character| character.to_digit(10))
            .flatten()
            .map(|d| d as usize)
            .or_else(|| {
                DIGIT_SPELLINGS
                    .iter()
                    .enumerate()
                    .find(|(idx, d)| str.contains(*d))
                    .map(|(idx, _)| idx)
            })
    }

    // accumulate forward and backward, in two separate iterators
    // each iterator is an iterator of lines containing an iterator of chars, one going forwards the other going backwards
    let forwards = input.lines().map(|l| l.trim()).map(|l| l.chars());
    let backwards = input.lines().map(|l| l.trim()).map(|l| l.chars().rev());
    let zipper = forwards.zip(backwards);

    // zip both iterators together, this represents a single line with one iterator going forward and another going backward\
    // We know both iterators are the same length because they both originate from `input.lines()`
    for (forwards, reverse) in zipper {
        let mut digits = (None, None);
        let mut forward = String::new();
        let mut backward = String::new();
        // realistically there's an optimization here where the zip can stop if the pointers touch
        // but i can't be bothered
        for (char_f, char_b) in forwards.zip(reverse) {
            if digits.0.is_some() && digits.1.is_some() {
                break;
            }
            digits = (
                digits.0.or({
                    forward.push(char_f);

                    check(&forward)
                }),
                digits.1.or({
                    backward.insert(0, char_b);

                    check(&backward)
                }),
            );
        }

        if let (Some(tens), Some(ones)) = digits {
            output += tens * 10;
            output += ones;
        } else {
            panic!("Could not find two digits")
        }
    }

    return output;
}
#[test]
fn sample_case() {
    let output = solve_1(
        r"
          1abc2
          pqr3stu8vwx
          a1b2c3d4e5f
          treb7uchet
          ",
    );

    assert_eq!(output, 142)
}

#[test]
fn full_case_1() {
    let output = solve_1(&INPUT_1);

    assert_eq!(output, 54081);
}

#[test]
fn full_case_2() {
    let output = solve_2(&INPUT_1);

    assert_eq!(output, 54081);
}
