use crate::Solution;
use paste::paste;

fn read_answers(day: usize, puzzle: usize) -> Option<String> {
    std::fs::read_to_string(format!("answers/{day}.txt"))
        .ok()?
        .lines()
        .map(|line| line.to_string())
        .nth(puzzle)
}

macro_rules! answer_tests {
    ($day:literal) => {
        paste! {
            #[test]
            fn [<day_ $day _part_1>]() {
                $crate::init_test_logging();
                let Some(expected) = read_answers($day, 0) else { return };
                let input = $crate::read_input($day, "1");
                let ans = $crate::[<day $day>]::Solution.solve_1(input);
                assert_eq!(expected, ans);
            }

            #[test]
            fn [<day_ $day _part_2>]() {
                $crate::init_test_logging();
                let Some(expected) = read_answers($day, 1) else { return };
                let input = $crate::read_input($day, "2");
                let ans = $crate::[<day $day>]::Solution.solve_2(input);
                assert_eq!(expected, ans);
            }
        }
    };
}

answer_tests!(1);
answer_tests!(2);
answer_tests!(3);
answer_tests!(4);
answer_tests!(5);
answer_tests!(6);
answer_tests!(7);
answer_tests!(8);
answer_tests!(9);
answer_tests!(10);
answer_tests!(11);
answer_tests!(12);
answer_tests!(13);
answer_tests!(14);
answer_tests!(15);
answer_tests!(16);
answer_tests!(17);
answer_tests!(18);
answer_tests!(19);
answer_tests!(20);
answer_tests!(21);
answer_tests!(22);
answer_tests!(23);
answer_tests!(24);
answer_tests!(25);
