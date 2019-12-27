#[cfg(test)]
mod test {

    use crate::board::*;
    use crate::color::*;

    #[test]
    fn test_pos2d() {
        let p = Pos2D::new(3,4);
        assert_eq!(p.i, 3);
        assert_eq!(p.j, 4);
    }

    #[test]
    fn test_board_cloning(){
        let mut b = Board::new();
        let b2 = Board::new_from(&b);
        b.set_at_c('c', 4, Color::Black);
        assert_eq!(b.get_at_c('c', 4), Color::Black);
        assert_eq!(b2.get_at_c('c', 4), Color::Empty);
    }

    fn get_starting_board() -> Board {
        let mut b = Board::new();
        b.set_at_c('D', 3, Color::Black);
        b.set_at_c('D', 4, Color::White);
        b.set_at_c('E', 3, Color::White);
        b.set_at_c('E', 4, Color::Black);
        b
    }

    #[test]
    fn test_get_available_moves_for_black(){

        let b = get_starting_board();
        let mut moves = b.get_available_moves_for(Color::Black).collect::<Vec<_>>();

        assert_eq!(moves.iter().count(), 4);

        moves.sort_by(|a, b| (a.i*100+a.j).cmp(&(b.i*100+b.j)));

        assert_eq!(moves[0], Pos2D::new(2,4));
        assert_eq!(moves[1], Pos2D::new(3,5));
        assert_eq!(moves[2], Pos2D::new(4,2));
        assert_eq!(moves[3], Pos2D::new(5,3));
    }

    #[test]
    fn test_get_available_moves_for_white(){

        let b = get_starting_board();
        let mut moves = b.get_available_moves_for(Color::White).collect::<Vec<_>>();

        assert_eq!(moves.iter().count(), 4);

        moves.sort_by(|a, b| (a.i*100+a.j).cmp(&(b.i*100+b.j)));

        assert_eq!(moves[0], Pos2D::new(2,3));
        assert_eq!(moves[1], Pos2D::new(3,2));
        assert_eq!(moves[2], Pos2D::new(4,5));
        assert_eq!(moves[3], Pos2D::new(5,4));
    }
}
