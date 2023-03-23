//Pong
/*MIT License
Copyright (c) 2023 Darwin Geiselbrecht
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use sdl2::video::Window;
use sdl2::render::Canvas;

use rand::Rng;

const SPEED: f32 = 4.;
const X_MAX: u32= 1650;             // size of window in x direction
const Y_MAX: u32 = 1000;            // size of window in y direction
const NUM_BALL_TYPES: usize = 4;      // number of types of balls
const NUM_BALLS:usize = 4;          // number of each type of balls

#[derive(Clone,Copy,PartialEq)]
enum Role{
    SEEKER,
    COWARD, 
    TARGET,
    STINKER
}

#[derive(Clone,Copy)]
struct Ball {
    size: u32,
    x: f32,
    y: f32,
    speed: f32,
    direction: f32,
    role: Role,                    // indicates role as seeker, target or npc
    color: Color,
}

impl Ball {
    /*fn new () -> Ball {
        Ball { 
            size: 10, x: 0., y: 10., speed: SPEED, direction: 0., seek: false, color:Color::CYAN}
    } */

    fn move_ball(&mut self) {
        self.x += self.speed * self.direction.cos() ;
        if self.x >= (X_MAX - self.size) as f32 {
            self.direction = 3.1416- self.direction;
            self.x = (X_MAX - self.size) as f32 ;
        }    

        if self.x <= 0.0 { 
            self.direction = 3.1416 - self.direction;
            self.x =  0.;
        }
        
        self.y += self.speed * self.direction.sin();
        if self.y >= (Y_MAX - self.size) as f32 { 
            self.direction = 6.283 - self.direction;
            self.y = (Y_MAX - self.size) as f32;
        }
        if self.y <= 0.0 {
            self.direction = 6.283 - self.direction;
            self.y = 0.0;
        }
    }
    fn draw(&mut self,canvas:&mut Canvas<Window>){
        let x_pos:i32 = self.x as i32;
        let y_pos:i32 = self.y as i32;
        
        canvas.set_draw_color(self.color);               // redraw in updated location
        canvas.fill_rect(Rect::new(x_pos,y_pos,self.size,self.size)).unwrap(); 
    }
    fn randomize(&mut self) {                               //randomize starting position and direction
        let mut rng = rand::thread_rng();
        self.x = rng.gen_range(0.0, X_MAX as f32);
        self.y = rng.gen_range(0.0 ,Y_MAX as f32);
        self.direction = rng.gen_range(0.,6.28);
    }
    fn carom(&mut self) {                                   // bounce off at a random direction
        let mut rng = rand::thread_rng();
        self.direction = rng.gen_range(0.,6.28);
    }
}

fn check_collisions(balls:&mut Vec<Ball>) {


    for j in 0 .. balls.len() {
        //let deltax_a =   balls[j].x;
        //let deltay_a = balls[j].y;
            for k in 0 .. balls.len() {
                if j != k {
                    //let deltax_b = balls[k].x;
                    //let deltay_b = balls[k].y;
                    let delta_x = (balls[j].x - balls[k].x).abs();
                    let delta_y = (balls[j].y - balls[k].y).abs();
                    if (delta_x < balls[j].size as f32) && (delta_y < balls[k].size as f32) {
                        balls[j].carom();                    // collided carom off randomlly
                        balls[k].carom();
                    }

                }
            }
    }
}
// set the direction of the seekers and the cowards
fn seek(balls:&mut Vec<Ball>) {

    // put all the targets in two vectors - one for the seekers and one for the cowards
    let mut target_balls = vec![];
    let mut avoided_balls = vec![];
    let mut stinker_balls = vec![];    
    for i in 0 .. balls.len() {
        match balls[i].role {
            Role::TARGET => {
                target_balls.push(i);
                avoided_balls.push(i);
            },
            Role::STINKER => {
                stinker_balls.push(i);
            },
            _ => {},
        }
    }

    for idx in 0 .. balls.len() {             // go through all balls finding seekers and cowards
        match balls[idx].role {
            Role::SEEKER => {
                let tar_idx:usize = target_balls.pop().unwrap(); 
                balls[idx].direction = find_direction_to_meet(balls,idx,tar_idx); 
            },
            Role::COWARD => {
                let tar_idx:usize = avoided_balls.pop().unwrap(); 
                balls[idx].direction = find_direction_to_meet(balls,idx,tar_idx);
                let tar_ball = balls[tar_idx].clone(); // to beat the borrow checker
                if find_distance_between_balls(balls[idx],tar_ball) < 100. {
                    balls[idx].direction -= 3.1416;          // too close for comfort, go in the opposite direction
                    if balls[idx].direction < 0.0 { 
                        balls[idx].direction += 6.283;
                    }
                }
            },
            _ => {},
        }
    }   
 
    for idx in 0 .. balls.len() {             // go through all balls avoiding close stinky ones
        if balls[idx].role != Role::STINKER {
            for i in 0 .. stinker_balls.len() {
                let tar_ball = balls[stinker_balls[i]].clone(); // to beat the borrow checker
                if find_distance_between_balls(balls[idx],tar_ball) < 100. {
                    balls[idx].direction -= 3.1416 ;          // too close for comfort, go in the opposite direction
                    if balls[idx].direction < 0.0 {           // keep direction positive
                        balls[idx].direction += 6.283;
                    }
                }                    
            
            }
        } 
    }    

}

// finds what direction to turn a seeker to meet up with a target ball in the future
fn find_direction_to_meet(balls:&mut Vec< Ball>, seek_idx:usize,tar_idx:usize)   -> f32{

    let mut tar_ball: Ball = balls[tar_idx].clone(); // to beat the borrow checker
    let steps:i32 = 
        ( find_distance_between_balls( balls[seek_idx],tar_ball) /
        tar_ball.speed )as i32;
    for _i in 0 .. steps {
        tar_ball.move_ball();                   // use move to extrapolate position
    } 
        find_direction_between_balls(balls[seek_idx], tar_ball)
}

// find the distance in pixels between two sets of x,y coordinates
fn find_distance (x1:f32,y1:f32,x2:f32,y2:f32) -> f32 {
    let xdist = x2 - x1;
    let ydist = y2 - y1;
    ( (xdist * xdist) + (ydist * ydist) ).sqrt()
}
// find the distance between two balls
fn find_distance_between_balls (ball_1:Ball,ball_2: Ball) -> f32 {
    find_distance(ball_1.x,ball_1.y,ball_2.x,ball_2.y)
}

// find the direction in radians between two sets of x,y coordinates
fn find_direction (x1:f32,y1:f32,x2:f32,y2:f32) -> f32 {
    let xdist = x2 - x1;
    let ydist = y2 - y1;
    ydist.atan2(xdist)
}

// find direction between balls
fn find_direction_between_balls (  ball_1:Ball, ball_2: Ball) -> f32 {
    find_direction(ball_1.x,ball_1.y,ball_2.x,ball_2.y)
}

fn main() -> Result<(), String> {


    let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Moving Box", X_MAX, Y_MAX)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().present_vsync().build()
        .expect("could not make a canvas");

    let mut event_pump = sdl_context.event_pump()?;
    
    
    let mut balls = Vec::with_capacity(NUM_BALLS * NUM_BALL_TYPES);

    for _i in 0 ..NUM_BALLS {
       balls.push(Ball {size:10,x:0.,y:0.,speed:SPEED*2.,direction:0.,role: Role::TARGET,color:Color::BLACK});
       balls.push(Ball {size:10,x:0.,y:0.,speed:SPEED,direction:0.,role: Role::SEEKER,color:Color::RED});
       balls.push(Ball {size:10,x:0.,y:0.,speed:SPEED,direction:0.,role: Role::COWARD,color:Color::YELLOW});
       balls.push(Ball {size:10,x:0.,y:0.,speed:SPEED/2.,direction:0.,role: Role::STINKER,color:Color::CYAN});
     }


    for  i in 0 .. balls.len(){                    // Randomize starting location and direction
        balls[i].randomize();                           
    }

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        // Update

        //canvas.set_draw_color(Color::RGB(175,175,175));             // paint over the previous locations
        
        canvas.set_draw_color(Color::GRAY);  
        canvas.clear();

        for  i in 0 .. balls.len(){                                 // Update all the balls
            //let ball: &mut Ball  = balls[i];
            balls[i].move_ball();
            balls[i].draw(&mut canvas);
        };

        //seek(&mut balls);
        //check_collisions(&mut balls);
        check_collisions(&mut balls);
        seek (&mut balls);

        canvas.present();

        // Time management! not needed, synced to vsync
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    } //running loop

    Ok(())
}