use  std::{io, vec};

fn main() {
    println!("Battleship Simulator" );
    // separator();
    // initial_grid();
    // separator();
    let ship_size = io_ship_size(); //get user inputs
    let grid_size = ship_size * 2;
    initial_grid(grid_size);
}


fn initial_grid(grid_size: i32){
    let mut columns = grid_size;
    let mut rows = grid_size;

    //input
    let mut ship_size = 3; //let user choose between ship sizes
    println!("Columns: {columns}\nRows: {rows}");

    //let gridbox: &str = "[ ]";

    //let row_grid = gridbox.repeat(rows);


    //need to set ID for each gridbox
    //format: C1R1 represents column 1 row 1 gridbox
    let grid_numbers: usize = grid_size.to_string().len();
    for r in 0..(rows+1){
        //this loop to passover row number to nested loop
        println!();
        for c in 0..(columns+1){
            //generate gridbox
            let current_coordinate = coordinate{
                column: c,
                row: r
            };
            
            //need to create an object instead so we can access the column and row number
            //let mut grid_id = 0;
            let is_label: bool = if (c == 0 || r == 0) {true} else {false};
            if is_label{
                let is_true: Vec<String> = vec![r.to_string(),if (r.to_string().len() < grid_numbers) {"  ".to_string()} else {" ".to_string()}];
                let is_false: Vec<String> = vec![c.to_string(),if (c.to_string().len() < grid_numbers) {" ".repeat(grid_numbers+1-c.to_string().len()).to_string()} else {" ".repeat(grid_numbers+1).to_string()}];
                let zero_axis: Vec<String> = vec![c.to_string()," ".to_string(),if (grid_numbers > 1) {" ".repeat(grid_numbers).to_string()} else {" ".to_string()}];

                let label_coordinate: String = if (c == 0) {if (r == 0) {zero_axis.concat()} else {is_true.concat()}} else {is_false.concat()};
                //let label_coordinate: String = if (c == 0) {is_true.concat()} else {is_false.concat()};

                print!("{label_coordinate}");
            } else {
                //print gridbox
                let grid_box: Vec<&str> = vec!["[] "];
                print!("{}",grid_box.concat());
            }

        }
        println!();
    }
}

fn separator(){ 
    let spacing = "-".repeat(10);
    println!("{spacing}");
}


    //input: where they want to place the ship
    //input: where they want to attack
    //output: if they hit a ship
    //output: change grid size (+ sign shape [up down left right]) - temporarily disable diagonal placements, only x-y axis
struct coordinate{
    column: i32,
    row: i32
}

struct ship_coordinates{
    coordinates: [coordinate;1]
    //to create
    // let ship = ship_coordinates{
    //     coordinates:
    //     [coordinate{
    //         column: c,
    //         row: r
    //     }]
    // };
}

fn io_ship_size() -> i32{
    let ship_sizes: [&str; 3] = ["3","4","5"]; //set grid limit here
    println!("Choose Ship Size:");
    
    for i in ship_sizes{
        println!("{i}");
    }
    separator();
    let mut acceptable_input: bool = false;
    let mut ship_size: String = String::new();
    while !acceptable_input{
        
        let stdin = io::stdin(); //register IO
        stdin.read_line(&mut ship_size);
        //let check_ship: String = ship_size;
        let found: bool = ship_sizes.iter().any(|v| v == &ship_size.trim()); //.trim() removes trailing whitespaces
        println!("{found}");
        if found{
            println!("Chosen Ship Size: {}",ship_size);
            acceptable_input = true;
        } else{
            println!("Invalid Input!");
        }
    }

    
    return ship_size.trim().parse().expect("Cannot parse Ship Size!");
    
}