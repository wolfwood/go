use vec::*;
use to_str::ToStr;

enum Liberty{
    Black,
    White,
    Empty
}

impl Liberty: ToStr{
    pure fn to_str() -> ~str{
        match self{
            Black => {~" *"}
            White => {~" O"}
            Empty => {~" ."}
        }
    }
}



fn main(){
    //let rows = 9;
    let mut board = [[ Empty, ..9 ], ..9 ];

    for each_mut(board) |l| {
        for each_mut(*l) |k| {
            *k = White;
        }
    }


    for each(board) |l| {
        for each(*l) |k| {
            io::print(k.to_str());
        }

        io::println("");
    }
}