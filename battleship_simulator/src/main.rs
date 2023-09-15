fn main() {
    println!("Battleship Simulator" );
    separator();
    initial_grid();
    separator();
}


fn initial_grid(){
    let mut columns = 2;
    let mut rows = 4;

    //input
    let mut ship_size = 3; //let user choose between ship sizes
    println!("Columns: {columns}\nRows: {rows}");

    //let gridbox: &str = "[ ]";

    //let row_grid = gridbox.repeat(rows);


    //need to set ID for each gridbox
    //format: C1R1 represents column 1 row 1 gridbox
    for r in 1..(rows+1){
        //this loop to passover row number to nested loop
        println!();
        for c in 1..(columns+1){
            //generate gridbox
            //need to create an object instead so we can access the column and row number
            //let mut grid_id = 0;
            print!("[C:{c}R:{r}]");
        }
        println!();
    }
}

fn separator(){
    let spacing = "-".repeat(10);
    println!("{spacing}");
}

fn io(){
    //input: where they want to place the ship
    //input: where they want to attack
    //output: if they hit a ship
    //output: change grid size (+ sign shape [up down left right]) - temporarily disable diagonal placements, only x-y axis

}