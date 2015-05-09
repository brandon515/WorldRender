extern crate getopts;
extern crate PGMap;
use getopts::Options;
use std::env;
use PGMap::world::World;
use PGMap::tile::Type;

fn print_usage(program: &String, opts: &Options){
    let brief = format!("Usage: {} [options]", program);
    println!("{}", opts.usage(&brief));
}

fn render_world(world_name: &String){
    let mut path_to_world = env::current_dir().unwrap();
    path_to_world.push(world_name);
    let world_to_render = World::from_file(path_to_world.as_path()).unwrap();
    for x in 0..world_to_render.width(){
        for y in 0..world_to_render.height(){
            let objects = world_to_render.objects_at(x,y).unwrap();
            if objects.len() == 0{
                print!("'");
            }
            for obj in objects.iter(){
                match obj.obj{
                    Type::HorizontalWall    =>  print!("|"),
                    Type::VerticalWall      =>  print!("-"),
                    Type::Floor             =>  print!("."),
                    Type::MainCharacter     =>  print!("@"),
                    Type::Door              =>  print!("+"),
                }
            }
        }
        println!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    let mut opts = Options::new();
    opts.reqopt("w", "world", "The world to be rendered", "WORLD");
    let matches = match opts.parse(&args[1..]){
        Ok(x)   =>  x,
        Err(e)  =>  {   println!("{}", e);
                        print_usage(&program, &opts);
                        return;
        }
    };
    let world_name = match matches.opt_str("w"){
        Some(x) =>  x,
        None    =>  {   print_usage(&program, &opts);
                        return;
        }
    };
    render_world(&world_name);
}
