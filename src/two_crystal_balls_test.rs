#[cfg(test)]
mod two_crystal_balls_test {
    #[test]
    fn two_crystal_balls_test() {
        assert_eq!(crate::two_crystal_balls::find_break(vec!(false, false, false, false)), -1);
        assert_eq!(crate::two_crystal_balls::find_break(vec!(false, false, false, false, true, true, true, true)), 4);
        assert_eq!(crate::two_crystal_balls::find_break(vec!(false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true)), 9);
        assert_eq!(crate::two_crystal_balls::find_break(vec!(false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, )), 17);
        assert_eq!(crate::two_crystal_balls::find_break(vec!(false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, )), 34);
    }
}
