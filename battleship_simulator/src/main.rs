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


fn validate_input(user_input: String) -> bool{
    //function to validate user input since i didnt validate anything LMAOOO
    let mut is_valid: bool = true;
    //put logic here
    return is_valid;
}
fn main() {
    println!("Battleship Simulator");
    let mut game_ongoing: bool = true;
    let mut attacked_coordinates: Vec<coordinate> = vec![];
    let ship_size = io_ship_size(); //get user inputs
    let grid_size = ship_size * 2;
    let mut get_coordinates: coordinate;
    let mut user_ship_coordinates: Vec<coordinate> =  vec![];
    initial_grid(grid_size, &mut attacked_coordinates,&mut user_ship_coordinates);

    user_ship_coordinates = generate_user_ship(ship_size, grid_size);
    println!("Ship Coordinates");
    println!("      C| R");
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
            game_ongoing = initial_grid(grid_size, &mut attacked_coordinates, &mut user_ship_coordinates)
        }
        

        

        
    }
        
}


fn initial_grid(grid_size: i32, attacked_coordinates: &mut Vec<coordinate>, ship_coordinates: &mut Vec<coordinate>) -> bool{
    //println!("Vector size: {}",attacked_coordinates.len());
    separator();
    let mut columns = grid_size;
    let mut rows = grid_size;

    let grid_box: String = "[O]".to_string();
    let marked_grid_box: String = "[X]".to_string();

    //system messages
    let mut global_hit: bool = false; 

    let mut hit_counter: i32 = 0;
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
                let mut current_attack: coordinate;
                if attacked_coordinates.len() == 0{
                    current_attack = coordinate { 
                        column: c, 
                        row: r };
                }else {
                    current_attack = coordinate { 
                        column: attacked_coordinates[attacked_coordinates.len() - 1].column, 
                        row: attacked_coordinates[attacked_coordinates.len() - 1].row };
                }
                
                for attacked_coordinate in attacked_coordinates.iter_mut(){
                    if (attacked_coordinate.column == c && attacked_coordinate.row == r){
                        let mut hit: bool = false;
                        for index in 0..ship_coordinates.len(){
                            if ship_coordinates[index].column == c && ship_coordinates[index].row == r{
                                print!("{}",marked_grid_box.red());
                                hit = true;
                                hit_counter += 1;
                                if ship_coordinates[index].column == current_attack.column && ship_coordinates[index].row == current_attack.row{
                                    global_hit = true;
                                }
                                break;
                            }
                        }
                        if !hit{
                            print!("{}",grid_box.blue());
                        }
                        
                        //make hit and no hit different colours
                        marked = true;
                    } else {
                        continue;
                    }
                }
                if !marked{
                    
                    print!("{}",grid_box);
                }
                
                //print gridbox
                
            }
            

        }
        //println!(); //leaves space between rows
    }
    separator();
    if hit_counter == ship_coordinates.len().try_into().unwrap(){
        println!("You won!");
        return false;
    } else{
        
        if global_hit{
            println!("Target Hit!");
            global_hit = false;
        } else{
            if attacked_coordinates.len() == 0{
                print!("Game Start!");
            }else {
                print!("Missed!");
            }
        }
        return true;
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

fn generate_user_ship(ship_size: i32,grid_size: i32) -> Vec<coordinate>{
    separator();
    let scalar: [&str;3] = ["Horizontal", "Vertical", "Diagonal(WIP)"];
    let mut valid_coordinate: bool = false;

    println!("Ship size: {}", ship_size);
    //get user input on where he wants to place ship
    //let user select middle coordinate, vertical/horizontal then auto generate for them
    let mut chosen_coordinate_c_io: String = String::new();
    let mut chosen_coordinate_c: i32 = 0;
    let mut chosen_coordinate_r_io: String = String::new();
    let mut chosen_coordinate_r: i32 = 0;

    separator();
    println!("Ship Orientations: ");
    for direction in 0..scalar.len(){
        println!("{}. {}", direction + 1, scalar[direction]);
    } 

    println!("Enter Selection: ");
    let mut ship_orientation_io: String = String::new();
    let mut ship_orientation: i32;
    io().read_line(&mut ship_orientation_io);
    ship_orientation = ship_orientation_io.trim().parse().expect("Invalid Selection.");
    ship_orientation -= 1;
    
    println!("Enter your ship's main coordinate: ");

    println!("Column: ");
    io().read_line(&mut chosen_coordinate_c_io);
    chosen_coordinate_c = chosen_coordinate_c_io.trim().parse().expect("Invalid Column!");

    println!("Row: ");
    io().read_line(&mut chosen_coordinate_r_io);
    chosen_coordinate_r = chosen_coordinate_r_io.trim().parse().expect("Invalid Row!");
 
    let middle_coordinate: coordinate = coordinate { column: chosen_coordinate_c, row: chosen_coordinate_r };
    separator();

    let mut user_ship_coordinates: Vec<coordinate> = vec![];

    //how the algo works
    //1 e.g. ship size 5 -> user inputs middle so we have 4 coordinates to plot
    //2 we can do for 1..middle -> 2 for loops to plot before and after middle coord

    if ship_orientation == 0{ //horizontal - change column
        println!("Vertical");
        //plot before
        for i in 1..(ship_size+1)/2{
            user_ship_coordinates.push(coordinate { column: chosen_coordinate_c - ((ship_size+1)/2 - i), row: chosen_coordinate_r })
        }
        //plot middle (input)
        user_ship_coordinates.push(coordinate { column: chosen_coordinate_c, row: chosen_coordinate_r });
        //plot after
        for i in 1..(ship_size+1)/2{
            user_ship_coordinates.push(coordinate { column: chosen_coordinate_c + i, row: chosen_coordinate_r })
        }
    } else if ship_orientation == 1 { //vertical - change row
        println!("Horizontal");
        for i in 1..(ship_size+1)/2{
            user_ship_coordinates.push(coordinate { column: chosen_coordinate_c, row: chosen_coordinate_r - ((ship_size+1)/2 - i) })
        }
        //plot middle (input)
        user_ship_coordinates.push(coordinate { column: chosen_coordinate_c, row: chosen_coordinate_r });
        //plot after
        for i in 1..(ship_size+1)/2{
            user_ship_coordinates.push(coordinate { column: chosen_coordinate_c, row: chosen_coordinate_r + i })
        }
    } else{
        println!("Diagonal is WIP");
    }

    

    
    
    return user_ship_coordinates;
}

fn game_attack(ship_size: i32) -> coordinate{

    let mut game_attack_c_io: String = String::new();
    println!("Enter Column of Attack: ");
    io().read_line(&mut game_attack_c_io);
    let game_attack_c: i32 = game_attack_c_io.trim().parse().expect("Column invalid! Please Restart!");
    
    
    let mut game_attack_r_io: String = String::new();
    println!("Enter Row of Attack: ");
    io().read_line(&mut game_attack_r_io);

    
    let game_attack_r: i32 = game_attack_r_io.trim().parse().expect("Row invalid! Please Restart!");


    return coordinate { column: game_attack_c, row: game_attack_r }
    //redraw grid to show where they attacked and ask for confirmation
}  