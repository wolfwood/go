// --- imports ---
use vec::*;
use to_str::ToStr;
use io::*;

// --- types ---
enum Player{
    Opaque,
    Transparent
}

enum Liberty{
    Occupied (Player),
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
impl Liberty: ToStr{
    pure fn to_str() -> ~str{
        match self{
            Occupied (Opaque) => {~" *"}
            Occupied (Transparent) => {~" O"}
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

    fn print(&self){
        let mut i = 0, j = 0;
        for each(**self) |l| {
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
            ~"?"|~"h"|~"help" => {println(" enter <row, column> coordinate of play, or q to quit\n"); loop}
            ~"q"|~"Q"|~"quit" => {break}
            ~"1 1" => {println("played\n")}
            _ => {loop}
        }

        current_player.flip();
    }
}
