use std::{io, vec}; 
use rand::Rng;
use colored::Colorize;

// fn update_attacked_coordinates(coordinates: coordinate) ->Vec<coordinate>{
    
//     const attacked_coordinates: Vec<coordinate> = vec![];
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
    coordinates: Vec<coordinate>
    //to create
    // let ship = ship_coordinates{
    //     coordinates:
    //     [coordinate{
    //         column: c,
    //         row: r
    //     }]
    // };
}

fn validate_input(user_input: String){
    //function to validate user input since i didnt validate anything LMAOOO
}
fn main() {
    println!("Battleship Simulator");
    let mut game_ongoing: bool = true;
    let mut attacked_coordinates: Vec<coordinate> = vec![];
    let ship_size = io_ship_size(); //get user inputs
    let grid_size = ship_size * 2;
    let mut get_coordinates: coordinate;
    initial_grid(grid_size, &mut attacked_coordinates);

    let mut user_ship_coordinates: Vec<coordinate> = generate_user_ship(ship_size, grid_size).coordinates;
    for i in 0..user_ship_coordinates.len(){
        println!("Ship: {}, {}",user_ship_coordinates[i].column,user_ship_coordinates[i].row);
    }
    while game_ongoing{
        separator();
         //send request to input attack
        get_coordinates = io__user_attack();
        let mut coordinate_invalid: bool = if (attacked_coordinates.contains(&get_coordinates) || (get_coordinates.column > grid_size || get_coordinates.row > grid_size)) {true} else {false};
        if coordinate_invalid{
            separator();
            println!("Coordinate has been previously attacked or is out of grid!");
        } else{
            let mut bind_coordinates: coordinate = coordinate::new(
                get_coordinates.column, 
                get_coordinates.row);
            attacked_coordinates.push(bind_coordinates); 
        }
        

        

        initial_grid(grid_size, &mut attacked_coordinates)
    }
        
}


fn initial_grid(grid_size: i32, attacked_coordinates: &mut Vec<coordinate>){
    //println!("Vector size: {}",attacked_coordinates.len());
    separator();
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
    let ship_sizes: [&str; 2] = ["3","5"]; //set grid limit here
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

fn generate_user_ship(ship_size: i32,grid_size: i32) -> ship_coordinates{
    separator();
    let scalar: [&str;3] = ["Vertical", "Horizontal", "Diagonal(WIP)"];
    let mut valid_coordinate: bool = false;

    println!("Ship size: {}", ship_size);
    //get user input on where he wants to place ship
    //let user select middle coordinate, vertical/horizontal then auto generate for them
    let mut chosen_coordinate_c_io: String = String::new();
    let mut chosen_coordinate_c: i32 = 0;
    let mut chosen_coordinate_r_io: String = String::new();
    let mut chosen_coordinate_r: i32 = 0;
    println!("Enter your ship's main coordinate: ");

    while !valid_coordinate{
        println!("Column: ");
        io().read_line(&mut chosen_coordinate_c_io);
        chosen_coordinate_c = chosen_coordinate_c_io.trim().parse().expect("Invalid Column!");
        if chosen_coordinate_c > grid_size{
            println!("Invalid Column Value!");
        } else{
            break;
        }
    }
    

    while !valid_coordinate{
        println!("Row: ");
        io().read_line(&mut chosen_coordinate_r_io);
        chosen_coordinate_r = chosen_coordinate_r_io.trim().parse().expect("Invalid Column!");
        if chosen_coordinate_r > grid_size{
            println!("Invalid Row Value!");
        } else{
            break;
        }
    }

    let middle_coordinate: i32 = (ship_size + 1)/2;

    let mut user_ship_coordinates: ship_coordinates;
    return ship_coordinates{
        coordinates: vec![coordinate{
            column: chosen_coordinate_c,
            row: chosen_coordinate_r
        },
        coordinate{
            column: 2,
            row: 2
        }
        ]
    };
}