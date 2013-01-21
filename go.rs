// --- imports ---
use vec::*;
use to_str::ToStr;
use io::*;
use str::split_char_nonempty;
use int::from_str;

// --- types ---
enum Player{
    Opaque,
    Transparent
}

struct Colony{
    p: Player,
    liberties: uint
}

enum Liberty{
    Occupied (Player, @mut Colony),
    Empty
}

enum Board = ~[~[Liberty]];

// --- implementations ---
impl Player{
    fn flip(&mut self){
        *self = match *self{
            Opaque => {Transparent}
            Transparent => {Opaque}
        }
    }
}

impl Colony{
    static fn new(p: Player, num_liberties: uint) -> @mut Colony{
        let c = @mut Colony {p:p, liberties:num_liberties};

        c
    }
}

impl Liberty: ToStr{
    pure fn to_str() -> ~str{
        match self{
            Occupied (Opaque, _) => {~" *"}
            Occupied (Transparent, _) => {~" O"}
            Empty => {~" ."}
        }
    }
}

impl Board{
    static fn new(num_rows: uint) -> Board{
        let mut first_row = ~[Empty];
        first_row.grow(num_rows -1, &Empty);

        let mut rows = ~[copy first_row];
        rows.grow(num_rows -1, &first_row);

        Board(rows)
    }

    fn play(&mut self, p: Player, x: uint, y: uint) -> bool{
        match self[x][y] {
            Empty => {}
            Occupied (_, _) => {return false}
        }

        let libs = 4;
        let c = Colony::new(p, libs);

        self[x][y] = Occupied(p, c);

        true
    }

    fn print(){
        let mut i = 0, j = 0;
        for each(*self) |l| {
            io::print(fmt!(" %d ", i));
            j = 0;

            for each(*l) |k| {
                io::print(k.to_str());
                j += 1;
            }

            io::println("");

            i += 1;
        }

        io::println("");
        io::print("   ");
        i = 0;
        while i < j {
            io::print(fmt!(" %d", i));
            i+=1;
        }

        io::println("");
    }
}

// --- the beef ---
fn main(){
    let rows = 9;
    let mut b = Board::new(rows);
    let mut current_player = Opaque;

    loop{
        b.print();

        print(fmt!("\n%?'s move > ", current_player));
        let input = io::stdin().read_line().trim();

        match input {
            ~"?"|~"h"|~"help" => {println(" enter (row column) coordinate of play, (h)elp or (q)uit\n"); loop}
            ~"q"|~"Q"|~"quit" => {break}
            _ => {}
        }

        let inputs = split_char_nonempty(input, ' ');

        if inputs.len() != 2 {loop}

        let x, y;
        match int::from_str(inputs[0]){
            Some (xx) => {x = xx}
            None => {loop}
        }

        match int::from_str(inputs[1]){
            Some (yy) => {y = yy}
            None => {loop}
        }

        if !b.play(current_player, x as uint, y as uint) {loop}

        current_player.flip();
    }
}
