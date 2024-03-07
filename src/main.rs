use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Jogo: Advinhe seu número");
    let mut guesses:u32 = 0;
    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Informe um número: ");
        let mut guess:String =String::new();
    
        io::stdin().read_line(&mut guess) 
              .expect("Falha ao executar loop");

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=> num,
            Err(_) => continue,
        };

        guesses+=1;
        match guess.cmp(&secret_number){
            Ordering::Equal=> {
                print!("Você ganhou!");
                break;
            },
            Ordering::Less=>{
                print!("Está para cima!");
                break;
            },
            Ordering::Greater=>{
                print!("Está para baixo!");
                break;
            },
        };
    }
    print!("Você usou {guesses} tentativas!");
}
