pub fn run() {
    println!("{greet} my age is {age} and your is also {age}", greet = "Aoa", age = 22);
}

pub fn run_loop(){
   for x in 1..6{ // 6 is not inclusive
      if x==3 {
         continue;
      }
      println!("x is {}",x);
   }
}

pub fn get_pi() -> f64 {
   22.0/7.0
}