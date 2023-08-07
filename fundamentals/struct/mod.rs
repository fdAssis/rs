use std::io;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    password: String,
    age: u8,
    money_in_wallet: f32,
}

impl User {
    pub fn new(name: String, email: String, password: String, age: u8) -> User {
        User {
            name,
            email,
            age,
            password,
            money_in_wallet: 10.0,
        }
    }

    fn take_in_wallet_money(&mut self) {
        let mut amount = String::new();
        println!("Informe o valor do saque: ");
        io::stdin()
            .read_line(&mut amount)
            .expect("Is not valid value");

        let amount = match amount.trim().parse::<f32>() {
            Ok(amount) => amount,
            Err(_) => 0.0,
        };
        assert!(amount < self.money_in_wallet, "Saldo insuficiente");
        self.money_in_wallet -= amount;
    }

    fn push_money_in_wallet(&mut self) {
        let mut amount = String::new();
        println!("Informe o valor do deposito");
        io::stdin()
            .read_line(&mut amount)
            .expect("Value is invalid");

        let amount = match amount.trim().parse::<f32>() {
            Ok(amount) => amount,
            Err(_) => 0.0,
        };

        self.money_in_wallet += amount;
    }

    fn show_account(&self) {
        println!("\n****** User Info *******");
        print!("Nome: {}", self.name);
        print!("E-mail: {}", self.email);
        println!("Idade: {}", self.age);
        println!("Saldo: {}", self.money_in_wallet);
        println!("*************************")
    }
}

// fn main() {
//   let mut name = String::new();
//   let mut password = String::new();
//   let mut email = String::new();
//   let mut age = String::new();

//   println!("\n\t******* hack system *******\n");
//   println!("You name:");
//   io::stdin().read_line(&mut name).expect("Erro with name");
//   println!("You email:");
//   io::stdin().read_line(&mut email).expect("Erro with email");

//   println!("You password:");
//   io::stdin()
//       .read_line(&mut password)
//       .expect("Erro with password");

//   println!("You age:");
//   io::stdin().read_line(&mut age).expect("Erro with age");

//   let age: u8 = match age.trim().parse::<u8>() {
//       Ok(age) => age,
//       Err(_) => 0,
//   };

//   let mut new_user: User = User::new(name, email, password, age);

//   loop {
//       let mut option = String::new();

//       println!("\n\t****** MENU *******");
//       println!("\t 1 - Fazer Deposito");
//       println!("\t 2 - Fazer Saque");
//       println!("\t 3 - Ver minhas informações");
//       println!("\t*********************");

//       println!("Escolha uma opção");
//       io::stdin()
//           .read_line(&mut option)
//           .expect("Is not valid option");

//       let option = match option.trim().parse::<u8>() {
//           Ok(option) => option,
//           Err(_) => 0,
//       };

//       match option {
//           1 => new_user.push_money_in_wallet(),
//           2 => new_user.take_in_wallet_money(),
//           3 => new_user.show_account(),
//           _ => println!("Feature in development"),
//       };
//   }
// }
