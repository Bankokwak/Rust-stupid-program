use std::io::{self, Write};
use std::collections::HashMap;

struct Account {
    username: String,
    isadmin: bool,
    money: f64,
    inventory: HashMap<String, i32>
}

fn log_in(database_login: &HashMap<&str, &str>, username: &str, password: &str) -> bool {
    for (user, pass) in database_login{
        if *user == username && *pass == password {
            return true;
        }
    }
    return false;
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim_end().to_string()
}

fn main() {
    let mut database_login: HashMap<&str, &str> = HashMap::new();
    database_login.insert("admin", "admin");

    let mut database_account: HashMap<&str, Account> = HashMap::new();
    database_account.insert("admin", Account { username: "admin".to_string(), isadmin: true, money: 100.0, inventory: HashMap::new()});

    let mut item_shop: HashMap<&str, f64> = HashMap::new();
    item_shop.insert("Eau", 2.0);

    let username = prompt("Username: ");
    let password = prompt("Password: ");
    if !log_in(&database_login, &username, &password) {
        println!("Connection failed: Login or password are wrong.");
        return;
    }

    connected(&mut database_login, &mut database_account, &mut item_shop,&username);
}

fn connected(mut database_login: &mut HashMap<&str, &str>, mut database_account: &mut HashMap<&str, Account>, mut item_shop: &mut HashMap<&str, f64>, username: &str){
    loop{
        print!("\n");
        println!("Menu:\n   1: Compte\n   2: Magasin\n   3: Casino\n   4: Admin\n   5: Deconnexion");
        match prompt("Choix : ").as_str(){
            "1" => choice_account(&database_account, &username),
            "2" => choice_shop(&mut database_account, &item_shop, &username),
            "3" => println!("Comin soon"),
            "4" => choice_admin(&mut database_login, &mut database_account, &mut item_shop, &username),
            _ => {
                println!("Mauvais choix.");
                prompt("Appuyez sur entrer pour continuer.");
            },
        }
    }
}

fn choice_account(database_account: &HashMap<&str, Account>, username: &str){
    print!("\n");
    if let Some(account) = database_account.get(username) {
        println!("Compte: {}\nArgent: {}€\nInventaire: ", account.username, account.money);
        for (name, number) in &account.inventory {
            println!("{} : {}", name, number);
        }
    } else {
        println!("Compte non trouvé !");
    }
    prompt("Appuyez sur entrer pour continuer.");
}

fn choice_shop(database_account: &mut HashMap<&str, Account>, item_shop: &HashMap<&str, f64>,username: &str){
    loop{
        if let Some(account) = database_account.get_mut(username) {
            print!("\nArgent: {}€\nArticle: \n", account.money);
            let mut index: i32 = 0;
            for (article, prix) in item_shop{
                index += 1;
                println!("  [N°{index}] {} : {}€", article, prix);
            }
            println!("Entrez (exit) pour quitter le shop.");

            let binding = prompt("Quel article voulez vous ? N° de l'article : ");
            let art_index: &str= binding.as_str();

            if art_index == "exit" {
                break;
            }

            if let Ok(num) = art_index.parse::<usize>() {
                let items: Vec<(&str, &f64)> = item_shop.iter().map(|(k,v)| (*k,v)).collect();
                if num == 0 || num > items.len() {
                    println!("Aucun article existe avec ce numéro {}.", num);
                    continue;
                }
                let (article, prix) = items[num - 1];
                println!("Vous avez sélectionné {}, au prix de {}€", article, prix);

                let binding = prompt("Entrer le nombre d'article à acheter sinon entrer N : ");
                let nombre_article: &str= binding.as_str();

                if nombre_article == "N" {
                    continue;
                }else{
                        if let Ok(num) = nombre_article.parse::<i32>(){
                            if account.money > prix * num as f64{
                                println!("Vous avez bien acheté {} {}", num, article);
                                account.money -= prix * num as f64;
                                if account.inventory.contains_key(&article.to_string()){
                                    if let Some(add) = account.inventory.get(&article.to_string()){
                                        account.inventory.insert(article.to_string(), num + add);
                                    }
                                }else{
                                    account.inventory.insert(article.to_string(), num);
                                }
                            }
                        }else{
                            println!("Erreur de conversion en float64.")
                        }
                }
            } else {
                println!("Veuillez entrer un numéro valide !");
                continue;
            }
            prompt("Appuyez sur entrer pour continuer.");
        }else{
            println!("Compte non trouvé !");
        }
    }
}

fn choice_admin(database_login: &mut HashMap<&str, &str>, database_account: &mut HashMap<&str, Account>, item_shop: &mut HashMap<&str, f64>, username: &str){
    loop{
        if let Some(account) = database_account.get(username){
            if !account.isadmin{
                println!("Votre compte n'est pas admin.");
                break;
            }

            println!("Admin!");
            break;
        }else{
            println!("Compte non trouvé !");
        }
    }
    prompt("Appuyez sur entrer pour continuer.");
}