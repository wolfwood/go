// --- imports ---
use vec::*;
use to_str::ToStr;
use io::*;

// --- types ---
enum Liberty{
    Black,
    White,
    Empty
}

enum Board = ~[~[Liberty]];


// --- implementations ---
impl Liberty: ToStr{
    pure fn to_str() -> ~str{
        match self{
            Black => {~" *"}
            White => {~" O"}
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
    let b = Board::new(rows);

    loop{
        b.print();

        let input = io::stdin().read_line().trim();

        match input {
            ~"?"|~"h"|~"help" => println(" enter [row, column] coordinate of play, or q to quit\n"),
            ~"q"|~"Q"|~"quit" => break,
            _ => loop
        }
    }
}
