use std::time;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

const MAXN:u16 = 500;


fn time_seed() -> u32 {
    let now = time::SystemTime::now();
    let duration = now.duration_since(time::UNIX_EPOCH).expect("Time went backwards");

    let microseconds = duration.subsec_micros();

    return microseconds;
}

fn parameters() -> (u16, u64){
    let args: Vec<String> = std::env::args().collect();
    let seed:u64;
    let n:u16;
    
    if args.len() >= 2 {
        n = args[1].parse().unwrap();
        if (n < 1) || (n > MAXN) {
            println!("N = {} is out of range.", n);
            std::process::exit(1);
        }
    }
    else {
        println!("Usage: {} <matrixdimension> [randomseed]", args[0]);
        std::process::exit(1);
    }
    
    if args.len() == 3 {
        seed = args[2].parse().unwrap();
        
    }
    else {
        seed = time_seed() as u64;
    }
     println!("Random seed = {}", seed);
    
    println!("\nMatrix dimension N = {}.", n);
    
    return (n, seed);
}

fn initialize_inputs(n:u16, seed:u64) -> ([[f32; MAXN as usize]; MAXN as usize], [f32; MAXN as usize], [f32; MAXN as usize]) {
    println!("\nInitializing...");

    let mut r = StdRng::seed_from_u64(seed);
    
    let mut a = [[0.0; MAXN as usize]; MAXN as usize];
    let mut b = [0.0; MAXN as usize];
    let mut x = [0.0; MAXN as usize];

    for col in 0..(n as usize) {
        for row in 0..(n as usize){
            a[row][col] = r.random::<f32>() * 32768.0;
        }
        b[col] = r.random::<f32>() * 32768.0;
        x[col] = 0.0;
    }

    return (a, b, x);

}

fn print_inputs(n:u16, a:[[f32; MAXN as usize]; MAXN as usize], b:[f32; MAXN as usize]) {
    if n < 10 {
        print!("\nA =\n\t");
        for row in 0..(n as usize) {
            for col in 0..(n as usize) {
                print!("{:.2}{}", a[row][col], if col < (n as usize) -1 {", "} else{";\n\t"});
            }
        }

        print!("\nB = [");
        for col in 0..(n as usize) {
            print!("{:.2}{} ", b[col], if col < (n as usize) -1 {';'} else{']'});
        }
        println!("\n");
    }
}

fn print_x(n:u16, x:[f32; MAXN as usize]) {
    if n < 10{
        print!("\nX = [");
        for col in 0..(n as usize) {
            print!("{:.2}{} ", x[col], if col < (n as usize) -1 {';'} else{']'});
        }
        println!("\n");
    }
}
fn main() {

    let time:u32 = time_seed();
    println!("time is {}", time);
    let (n, seed) = parameters();
    
    let (a, b, mut x) = initialize_inputs(n, seed);
    print_inputs(n, a.clone(), b.clone());

    println!("Starting clock.");
    let start = time::Instant::now();
    
    x = gauss(n, a, b, x);

    let duration = start.elapsed();
    
    print_x(n, x);
    println!("Elapsed time: {:.9} ms.", duration.as_secs_f64() * 1000.0);

   

}

fn gauss(n:u16, mut a:[[f32; MAXN as usize]; MAXN as usize], mut b:[f32; MAXN as usize] , mut x:[f32; MAXN as usize]) -> [f32; MAXN as usize] {


    let n = n as usize;
    for norm in 0..(n - 1) {
        for row in (norm + 1)..n {
            let multiplier = a[row][norm] / a[norm][norm];
            for col in norm..n {
                a[row][col] -= a[norm][col] * multiplier;
            }
            b[row] -= b[norm] * multiplier;
        }
    }

    for row in (0..n).rev() {
        x[row] = b[row];
        for col in (row + 1)..n {
            x[row] -= a[row][col] * x[col];
        }
        x[row] /= a[row][row];
    }


    return x;
}