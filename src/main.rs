use std::io;

use std::collections::{HashMap, hash_map};

fn main() {
    
    println!("Enter your address to create a wallet");

    let mut address = String::new();
    io::stdin().read_line(&mut address);

    let mut balances_of_user = HashMap::new();  //users wallet balances can be updated here
    balances_of_user.insert(Token::USDT, 1000.0);
    // when type is value , no error but when type is used for key it must be hashable. So, Hash and Eq must be used in derive


    let tokenA = Balances { token:Token::SOL, balance:10000.0 };
    let tokenB = Balances { token:Token::DOT, balance:10000.0 };

    let mut my_pool = Pool::Create_pool(tokenA, tokenB);


    // Creating a Wallet 

    let mut users_wallet = Wallet::create_wallet(address, balances_of_user); //balance_of_user is now owned by users_wallet. Ownership is transferred. 

    println!("Your wallet has been created");


    // Printing Balance  by creating a method inside wallet impl
loop {
       
    println!("1-Check my Balance");
    println!("2-see the Market Status");
    println!("3-Buying a token");
    println!("4- Sell a Token");
    println!("5- Swap Token");
    let mut ch = String::new();
    io::stdin().read_line(&mut ch);

    let choice:i32 = ch.trim().parse().unwrap();
    match choice {

        1 =>{
            users_wallet.check_my_balance() ;
        },
        2 => {
            Token::show_market_status();

        },
        3 => {
               println!("Enter the name of the token you want to buy");
               Token::show_market_status();

               let mut token_name = String::new();
               io::stdin().read_line(&mut token_name);

            //    Token name is stored in string

            let buying_token = Token::return_token(token_name.trim());
            // trim() converts string to str.
              

            println!("Enter the amount you want to buy");
            let mut token_amount = String::new();
            io::stdin().read_line(&mut token_amount);

            let parsed_amount:f64 = token_amount.trim().parse().unwrap();

            Token::buy_token(buying_token, parsed_amount, &mut users_wallet.balances );

            // &mut balances_of_user cant be used above for balance. instead users_wallet.balance can be used
        },
        4 => {
            println!("Enter the name of the token you want to sell");
               Token::show_market_status();

               let mut token_name = String::new();
               io::stdin().read_line(&mut token_name);

               let selling_token = Token::return_token(token_name.trim());
               let selling_token2 = Token::return_token(token_name.trim());

               println!("Enter the amount you want to sell");
            let mut token_amount = String::new();
            io::stdin().read_line(&mut token_amount);

            let parsed_amount:f64 = token_amount.trim().parse().unwrap();

            Token::sell_token(&selling_token,&selling_token2, parsed_amount, &mut users_wallet.balances );
        },

        5 => {

            println!("Enter TokenA amount");
           let mut token_a_amount = String::new();
           io::stdin().read_line(&mut token_a_amount);

           let parsed_token_a_amount = token_a_amount.trim().parse().unwrap();
           Pool::swap_a_to_b(&my_pool, parsed_token_a_amount ,& mut  users_wallet.balances);


        }
        _ => {
            println!("Invalid Option")
        }
        
    };

}






}


//Listing Tokens
#[derive(Debug,PartialEq, Eq,Hash,Clone, Copy)]
enum Token {
    BTC,
    ETH,
    SOL,
    DOT,
    USDT
}

impl Token {
    fn show_market_status() {
        println!("BTC: price:{:?}",Self::return_price(&Token::BTC) ) ;
        println!("ETH: price:{:?}",Self::return_price(&Token::ETH) ) ;
        println!("SOL: price:{:?}",Self::return_price(&Token::SOL) ) ;
        println!("DOT: price:{:?}",Self::return_price(&Token::DOT) ) ;
        println!("USDT: price:{:?}",Self::return_price(&Token::USDT) ) ;
    }

    fn return_token(input:&str) ->Self {
        
        match input {
            "btc" => Token::BTC,
            "eth" => Token::ETH,
            "sol" => Token::SOL,
            "dot" => Token::DOT,
            "usdt" => Token::USDT,
            _ => Token::BTC
            
        }
    }

    fn return_price(&self) -> f64 {
       match self {
            Token::BTC => 30000.0,
            Token::ETH => 1000.0,
            Token::SOL => 35.0,
            Token::DOT => 8.0,
            Token::USDT =>1.0
        }
        
    }
    
    // Buy token (token:Token , amount:f64, balances)
    fn buy_token(self, amount:f64, balances:&mut HashMap<Token,f64>) {
        
    //    1000 usdt initial
    // usdt >= current price(self) * amount

    // get usdt of balance of user
    let users_usdt_bal = balances.get(&Token::USDT).unwrap();
    
    // calculating the price
    let price_of_token = Token::return_price(&self);
    let calculated_price_of_token = price_of_token*amount;

     if users_usdt_bal >= &calculated_price_of_token {
        // deduct usdt first
        balances.insert(Token::USDT, users_usdt_bal-&calculated_price_of_token ); 
        
        if balances.contains_key(&self) {
            let prev_bal_of_token = balances.get(&self).unwrap();
            balances.insert(self, &amount + prev_bal_of_token);
        }
        else{
            balances.insert(self, amount);
        }
       
        println!("Transaction Successful :)")

     }else {
         println!("Transaction declined due to insufficient balance");
     }
    
    }

    fn sell_token(&self,sell_token:&Token, amount:f64, balances:&mut HashMap<Token,f64>) {
        
        // let copy_sell_token = &self;
        let bal_of_token = balances.get(&self).unwrap();
        if bal_of_token >= &amount {
            
            balances.insert(*self, bal_of_token-amount);
            let prev_bal_of_usdt = balances.get(&Token::USDT).unwrap();
            let price_of_token = Self::return_price(&sell_token);
            let calculated_price_of_token = price_of_token*amount;
            let calc_usdt_bal = prev_bal_of_usdt + calculated_price_of_token ;
            balances.insert(Token::USDT, calc_usdt_bal);

            println!("Transaction Successful")

        }else{
            println!("Invalid Amount, Transaction declined");
        }
    } 



}

     #[derive(Debug,Clone, Copy)]
    struct Pool {

    TokenA:Balances,
    TokenB:Balances
    }
    
    impl Pool {
        fn Create_pool(TokenA:Balances,TokenB:Balances) -> Self {
           
            Self { TokenA,TokenB }
        } 
        //  Direction of swap
        fn swap_a_to_b(&self,amount_of_a:f64,balances:&mut HashMap<Token,f64>) {

            let token_a = self.TokenA.token;  //INSERT
            let token_b = self.TokenB.token;  //RETURN
            
          // (A_total + A_in) * (B_total - B_out) = invariant
          // (100 + 10) * (5,000 - B_out) = 500,000
          // 5,000 - B_out = 500,000 / 110
          // 5,000 - (500,000 / 110) = B_out
          // B_out = 454.5454...

           let users_bal_of_token_a = balances.get(&token_a).unwrap();

        //    now checking if the balance is more than the amount he is ready to swap

          if users_bal_of_token_a >= &amount_of_a {
              
            let a_init = &self.TokenA.balance;  //Pool's initial value of Token A
            let b_init = &self.TokenB.balance;  //Pool's initial value of Token B

            // x*y = k
            let invarient:f64 = a_init*b_init;

            // (A_total + A_in) * (B_total - B_out) = invariant
            let a_cal = a_init+amount_of_a;

            let div = invarient/a_cal;
            let mut b_out = b_init-div; //Amount of b iam getting from a_to_b swap

            println!("Estimated amount of {:?} is {:?} , do you want to proceed this swap",token_b,b_out);

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer);

            if buffer.trim() == "yes" {

                balances.insert(token_a, users_bal_of_token_a-amount_of_a);
      
                // if users have token b already, then 

                if balances.contains_key(&token_b) {
                    
                    let prev_bal_of_b = balances.get(&token_b).unwrap();
                    balances.insert(token_b, b_out + prev_bal_of_b);

                    println!("Swap successful");
                
                }else {
                    balances.insert(token_b, b_out);
                    println!("Swap successful");
                
                }
                

            }else {
                println!("Swap Cancelled");
            }




          }  

        }
    }



// hashmap <Token, f64>

// Wallet (Address:string & Balances)
#[derive(Debug)]
struct Wallet {
    address:String,
    balances:HashMap<Token,f64>

}

impl Wallet {
    fn create_wallet(address:String,balances:HashMap<Token, f64>) -> Self {
        Self { address,balances }
        
    }

    fn check_my_balance(&self) {

        println!("{:?}", self.balances); //
        
    }
}

// Balances( Token:Balance )

#[derive(Debug,PartialEq,Clone, Copy)]
struct Balances {
    token:Token,
    balance:f64
}
