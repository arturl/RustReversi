use all_asserts;

#[cfg(test)]
mod test {

    use crate::board::*;
    use crate::color::*;
    use crate::stat::Stat;
    use crate::analysis::*;

    fn get_starting_board_precorner() -> Board {
        let mut b = Board::new();
        b.set_at_c('C', 1, Color::Black);
        b.set_at_c('D', 1, Color::Black);
        b.set_at_c('E', 1, Color::White);
        b
    }

    #[test]
    fn test_precorner_analysis(){

        let b = get_starting_board_precorner();
        let moves = b.get_available_moves_for(Color::White).collect::<Vec<_>>();
        assert_eq!(moves.iter().count(), 1);

        let mut stat = Stat::new();
        let (pos, score) = find_best_move(&b, Color::White, 0, 1, &mut stat).unwrap();

        assert_eq!(pos, Pos2D::new(1,1));
        all_asserts::assert_lt!(score, 0);
    }

    fn get_starting_board_corner() -> Board {
        let mut b = Board::new();
        b.set_at_c('F', 7, Color::White);
        b.set_at_c('G', 7, Color::Black);
        b
    }

    #[test]
    fn test_corner_analysis(){

        let b = get_starting_board_corner();
        let moves = b.get_available_moves_for(Color::White).collect::<Vec<_>>();
        assert_eq!(moves.iter().count(), 1);

        let mut stat = Stat::new();
        let (pos, score) = find_best_move(&b, Color::White, 0, 1, &mut stat).unwrap();

        assert_eq!(pos, Pos2D::new(7,7));
        all_asserts::assert_gt!(score, 0);
    }

    fn get_starting_board_corner2() -> Board {
        let mut b = Board::new();
        b.set_at_c('B', 0, Color::White);
        b.set_at_c('C', 0, Color::Black);
        b.set_at_c('F', 7, Color::Black);
        b.set_at_c('G', 7, Color::Black);
        b.set_at_c('E', 7, Color::White);
        b
    }

    #[test]
    fn test_two_corners_analysis(){

        let b = get_starting_board_corner2();
        let moves = b.get_available_moves_for(Color::White).collect::<Vec<_>>();
        assert_eq!(moves.iter().count(), 2);

        let mut stat = Stat::new();
        let (pos, score) = find_best_move(&b, Color::White, 0, 0, &mut stat).unwrap();

        assert_eq!(pos, Pos2D::new(7,7));
        all_asserts::assert_gt!(score, 0); // must be slightly better
    }
}
