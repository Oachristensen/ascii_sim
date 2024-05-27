use std::{array, char, thread::sleep, vec};
use rand::{thread_rng, Rng};
use std::time::Duration;


const Y_SIZE: usize = 100;
const X_SIZE: usize = 100;
const MATRIX_SIZE: usize = Y_SIZE*X_SIZE;
const NUM_SIM: usize = 1000;

// This doesnt work unless it is a square
fn x_y(x: i32, y: i32) -> i32{
x+(y*(Y_SIZE as i32-1))-1
}

#[derive(Clone, PartialEq)]
struct Pixel {
    x : i32,
    y : i32,
    value : char,
}

fn move_pixel(mut pixel_array : Vec<Pixel>, x: i32, y: i32, index : usize) -> Vec<Pixel> {
    pixel_array[index].x = pixel_array[index].x +x;
    pixel_array[index].y = pixel_array[index].y + y;
    pixel_array
}

fn check_pixel_left(mut pixel_array : Vec<Pixel>, index : usize) -> (bool,Vec<Pixel>) {
    if pixel_array[index].x == 0 {
        return (false,pixel_array);
    }
    for i in 0..NUM_SIM {
        
        if !(pixel_array[index] == pixel_array[i]) {
            if !(pixel_array[i].x == X_SIZE as i32) {
                if pixel_array[index].y == pixel_array[i].y {
                    if pixel_array[index].x == pixel_array[i].x+1 {
                        return (false,pixel_array)
                    }
                }
            }
        }
    }
    (true, pixel_array)
}


fn check_pixel_right(mut pixel_array : Vec<Pixel>, index : usize) -> (bool,Vec<Pixel>) {
    if pixel_array[index].x == X_SIZE as i32 {
        return (false,pixel_array);
    }
    for i in 0..NUM_SIM {
        
        if !(pixel_array[index] == pixel_array[i]) {
            if !(pixel_array[i].x == 0) {
                if pixel_array[index].y == pixel_array[i].y {
                    if pixel_array[index].x == pixel_array[i].x-1 {
                        return (false, pixel_array)
                    }
                }
            }
        }
    }
    (true, pixel_array)
}


fn check_pixel_down(mut pixel_array : Vec<Pixel>, index : usize) -> (bool,Vec<Pixel>) {
    if pixel_array[index].y == Y_SIZE as i32 {
        return (false,pixel_array);
    }
    for i in 0..pixel_array.len()  {
        if !(pixel_array[index] == pixel_array[i]) {
            if  !(pixel_array[i].y == 0) {            
                if pixel_array[index].x == pixel_array[i].x {
                    if pixel_array[index].y == pixel_array[i].y-1 {
                        return (false,pixel_array);
                    }
                }
            }
        }
    }
    (true, pixel_array)
}


fn draw_matrix(arr: &Vec<char>) {
    let mut cur_pixel : i32 = 0;
    loop {
        print!("{}", &arr[cur_pixel as usize]);
        cur_pixel+=1;
        if cur_pixel % X_SIZE as i32 == 0 {
            println!();
        }
        if cur_pixel == MATRIX_SIZE as i32 {
            break;
        }
    }
}


fn run_sim(mut pixel_array : Vec<Pixel>) ->  Vec<Pixel>{
    for index in 0..NUM_SIM {
        let down : bool;
        let left : bool;
        let right : bool;

        (down , pixel_array) = check_pixel_down(pixel_array, index);
        (left , pixel_array) = check_pixel_left(pixel_array, index);
        (right , pixel_array) = check_pixel_right(pixel_array, index);
        if down {
           pixel_array = move_pixel(pixel_array, 0, 1, index);
        }
        if left && right {
            let coin_toss : bool = rand::random();
            if coin_toss {
                pixel_array = move_pixel(pixel_array, -1, 0, index);
            }
            else {
                pixel_array = move_pixel(pixel_array, 1, 0, index);
            }
        }
        else if left { 
           pixel_array = move_pixel(pixel_array, -1, 0, index);
        }
        else if right {
            pixel_array = move_pixel(pixel_array, 1, 0, index);
        }
    }
    pixel_array
}

fn reset_matrix(mut matrix_array: Vec<char>) -> Vec<char> {
    for i in 0..MATRIX_SIZE {
        matrix_array[i] = '#';
    }
    matrix_array
}

fn update_matrix(mut matrix_array: Vec<char>, mut pixel_array : Vec<Pixel>) -> (Vec<char>, Vec<Pixel>) {
    for i in 0.. NUM_SIM {
        matrix_array[x_y(pixel_array[i].x, pixel_array[i].y) as usize] = '0';
    }
    (matrix_array, pixel_array)
}

fn main() {
    // Create Matrix
    let mut pixel = Pixel{ x: (0), y: (0), value: ('0')};
    let mut matrix_array: Vec<char> = vec!['#'; X_SIZE * Y_SIZE];
    let mut pixel_array: Vec<Pixel> = vec![pixel.clone(); NUM_SIM ];

    //Generate random pixels
    // Needs better rand checking, can generate same num twice
    for i in 0..NUM_SIM {
        pixel_array[i].x = rand::thread_rng().gen_range(0..X_SIZE as i32);
        pixel_array[i].y = rand::thread_rng().gen_range(0..Y_SIZE as i32);
    }
    //Populate random pixels
    for i in 0..NUM_SIM {
        matrix_array[x_y(pixel_array[i].x, pixel_array[i].y) as usize] = pixel_array[i].value;
    }
    
    loop {
        matrix_array = reset_matrix(matrix_array);
        pixel_array = run_sim(pixel_array);
        (matrix_array,pixel_array) = update_matrix(matrix_array, pixel_array);
        draw_matrix(&matrix_array);
        println!();
        sleep(Duration::new(0, 100000000));
    }
    
}
