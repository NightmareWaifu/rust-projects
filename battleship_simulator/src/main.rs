use std::{io, vec}; 
use rand::Rng;
use colored::Colorize;

// fn update_attacked_coordinates(coordinates: coordinate) ->Vec<coordinate>{
    
//     const ATTACKED_COORDINATES: Vec<coordinate> = vec![];
// }
#[derive(Clone,Copy,PartialEq)]
struct coordinate{
    column: i32,
    row: i32
}

impl coordinate {

    fn new(column: i32, row: i32) -> Self {
        //create new coord
        coordinate { column, row } 
    }
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
fn main() {
    println!("Battleship Simulator");
    let mut game_ongoing: bool = true;
    let mut ATTACKED_COORDINATES: Vec<coordinate> = vec![];
    let ship_size = io_ship_size(); //get user inputs
    let grid_size = ship_size * 2;
    let mut get_coordinates: coordinate;
    initial_grid(grid_size, &mut ATTACKED_COORDINATES);
    while game_ongoing{
        separator();
         //send request to input attack
         get_coordinates = io__user_attack();
        let mut coordinate_invalid: bool = if (ATTACKED_COORDINATES.contains(&get_coordinates) || (get_coordinates.column > grid_size || get_coordinates.row > grid_size)) {true} else {false};
        if coordinate_invalid{
            separator();
            println!("Coordinate has been previously attacked or is out of grid!");
        } else{
            let mut bind_coordinates: coordinate = coordinate::new(
                get_coordinates.column, 
                get_coordinates.row);
            ATTACKED_COORDINATES.push(bind_coordinates); 
        }
        

        

        initial_grid(grid_size, &mut ATTACKED_COORDINATES)
    }
        
}


fn initial_grid(grid_size: i32, attacked_coordinates: &mut Vec<coordinate>){
    //println!("Vector size: {}",attacked_coordinates.len());
    let mut columns = grid_size;
    let mut rows = grid_size;

    let grid_box: Vec<&str> = vec!["[O]"];
    let marked_grid_box: String = "[X]".to_string();

    //input
    let mut ship_size = 3; //let user choose between ship sizes
    //println!("Columns: {columns}\nRows: {rows}");

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
            // let current_coordinate = coordinate{
            //     column: c,
            //     row: r
            // };
            
            //need to create an object instead so we can access the column and row number
            //let mut grid_id = 0;
            let is_label: bool = if (c == 0 || r == 0) {true} else {false};
            if is_label{
                //generate legend
                let is_true: Vec<String> = vec![r.to_string(),if (r.to_string().len() < grid_numbers) {"  ".to_string()} else {" ".to_string()}];
                let is_false: Vec<String> = vec![c.to_string(),if (c.to_string().len() < grid_numbers) {" ".repeat(grid_numbers+1-c.to_string().len()).to_string()} else {" ".repeat(grid_numbers+1).to_string()}];
                let zero_axis: Vec<String> = vec![c.to_string()," ".to_string(),if (grid_numbers > 1) {" ".repeat(grid_numbers).to_string()} else {" ".to_string()}];

                let label_coordinate: String = if (c == 0) {if (r == 0) {zero_axis.concat()} else {is_true.concat()}} else {is_false.concat()};

                print!("{label_coordinate}");
            } else {
                let mut marked: bool = false;
                //check if coordinate has been attacked
                for attacked_coordinate in attacked_coordinates.iter_mut(){
                    //println!("C: {} R: {}", attacked_coordinate.column, attacked_coordinate.row);
                    if (attacked_coordinate.column == c && attacked_coordinate.row == r){
                        print!("{}",marked_grid_box.red());
                        //make hit and no hit different colours
                        marked = true;
                    } else {
                        continue;
                    }
                }
                if !marked{
                    
                    print!("{}",grid_box.concat());
                }
                
                //print gridbox
                
            }

        }
        //println!(); //leaves space between rows
    }
}

fn separator(){ 
    let spacing = "-".repeat(10);
    println!("\n{}\n",spacing);
}


    //input: where they want to place the ship
    //input: where they want to attack
    //output: if they hit a ship
    //output: change grid size (+ sign shape [up down left right]) - temporarily disable diagonal placements, only x-y axis


//const STDIN: io::Stdin = io::stdin()

fn io() -> io::Stdin{
    let stdin = io::stdin();

    return stdin;
}

//register IO
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
        
        io().read_line(&mut ship_size);
        //let check_ship: String = ship_size;
        let found: bool = ship_sizes.iter().any(|v| v == &ship_size.trim()); //.trim() removes trailing whitespaces
        //println!("{found}");
        if found{
            println!("Chosen Ship Size: {}",ship_size);
            acceptable_input = true;
        } else{
            println!("Invalid Input!");
        }
    }

    
    return ship_size.trim().parse().expect("Cannot parse Ship Size!");
    
}

fn io__user_attack() -> coordinate{
    let mut allowed_io: bool = true;
    //invalid inputs not handled yet
    let mut user_attack_c_io: String = String::new();
    println!("Enter Column of Attack: ");
    io().read_line(&mut user_attack_c_io);
    let user_attack_c: i32 = user_attack_c_io.trim().parse().expect("Column invalid! Please Restart!");
    
    
    let mut user_attack_r_io: String = String::new();
    println!("Enter Row of Attack: ");
    io().read_line(&mut user_attack_r_io);

    
    let user_attack_r: i32 = user_attack_r_io.trim().parse().expect("Row invalid! Please Restart!");


    return coordinate { column: user_attack_c, row: user_attack_r }
    //redraw grid to show where they attacked and ask for confirmation
}   