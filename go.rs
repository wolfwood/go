use vec::*;

enum Liberty{
    Black,
    White,
    Empty
}

impl Liberty{
    fn show(&self){
        io::print(match *self{
            Black => {" *"}
            White => {" O"}
            Empty => {" ."}
        })
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
            k.show();
        }

        io::println("");
    }
}