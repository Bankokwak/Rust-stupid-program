use std::io::{self, Write};
use std::collections::HashMap;

struct Account {
    username: String,
    isadmin: bool,
    money: f64,
    inventory: HashMap<String, i32>
}

fn log_in(database_login: &HashMap<String, String>, username: &str, password: &str) -> bool {
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
    let mut database_login: HashMap<String, String> = HashMap::new();
    database_login.insert("admin".to_string(), "admin".to_string());

    let mut database_account: HashMap<String, Account> = HashMap::new();
    database_account.insert("admin".to_string(), Account { username: "admin".to_string(), isadmin: true, money: 100.0, inventory: HashMap::new()});

    let mut item_shop: HashMap<String, f64> = HashMap::new();
    item_shop.insert("Eau".to_string(), 2.0);

    login(&mut database_login, &mut database_account, &mut item_shop);
}

fn login(mut database_login: &mut HashMap<String, String>, mut database_account: &mut HashMap<String, Account>, mut item_shop: &mut HashMap<String, f64>){
    print!("\n");
    let username = prompt("Username: ");
    let password = prompt("Password: ");
    if !log_in(&database_login, &username, &password) {
        println!("Connection failed: Login or password are wrong.");
        return;
    }

    connected(&mut database_login, &mut database_account, &mut item_shop,&username);
}

fn connected(mut database_login: &mut HashMap<String, String>, mut database_account: &mut HashMap<String, Account>, mut item_shop: &mut HashMap<String, f64>, username: &str){
    loop{
        print!("\n");
        println!("Menu:\n   1: Compte\n   2: Magasin\n   3: Casino\n   4: Admin\n   5: Deconnexion");
        match prompt("Choix : ").as_str(){
            "1" => choice_account(&database_account, &username),
            "2" => choice_shop(&mut database_account, &item_shop, &username),
            "3" => println!("Coming soon"),
            "4" => choice_admin(&mut database_login, &mut database_account, &mut item_shop, &username),
            "5" => login(&mut database_login, &mut database_account, &mut item_shop),
            _ => {
                println!("Mauvais choix.");
                prompt("Appuyez sur entrer pour continuer.");
            },
        }
    }
}

fn choice_account(database_account: &HashMap<String, Account>, username: &str){
    print!("\n");
    if let Some(account) = database_account.get(username) {
        println!("--- Compte ---\nCompte: {}\nArgent: {}€\nInventaire: ", account.username, account.money);
        for (name, number) in &account.inventory {
            println!("{} : {}", name, number);
        }
    } else {
        println!("Compte non trouvé !");
    }
    prompt("Appuyez sur entrer pour continuer.");
}

fn choice_shop(database_account: &mut HashMap<String, Account>, item_shop: &HashMap<String, f64>,username: &str){
    loop{
        print!("\n");
        if let Some(account) = database_account.get_mut(username) {
            print!("\n--- Magasin ---\nArgent: {}€\nArticle: \n", account.money);
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
                let items: Vec<(&String, &f64)> = item_shop.iter().collect();
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
                            if account.money >= prix * num as f64{
                                println!("Vous avez bien acheté {} {}", num, article);
                                account.money -= prix * num as f64;
                                let art_name = article.clone();
                                *account.inventory.entry(art_name).or_insert(0) += num;
                            }else{
                                println!("Vous n'avez pas assez de fond pour vous achetez {} {} pour {}€", num, article, prix * num as f64);
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

fn choice_admin(database_login: &mut HashMap<String, String>, database_account: &mut HashMap<String, Account>, item_shop: &mut HashMap<String, f64>, username: &str){
    loop{
        print!("\n");
        if let Some(account) = database_account.get(username){
            if !account.isadmin{
                println!("Votre compte n'est pas admin.");
                break;
            }

            println!("Admin:\n   1: Article Manager\n   2: Compte Manager\n   3: Login Manager\n   4: Exit Admin");
            let buffer: String = prompt("   Choix: ");
            match buffer.as_str(){
                "1" => choice_admin_item(item_shop),
                "2" => choice_admin_account(database_account),
                "3" => choice_admin_login(database_login, database_account),
                "4" => break,
                _ => {
                    println!("Mauvais choix.");
                    prompt("Appuyez sur entrer pour continuer.");
                },
            }
        }else{
            println!("Compte non trouvé !");
        }
    }
    prompt("Appuyez sur entrer pour continuer.");
}

fn choice_admin_item(item_shop: &mut HashMap<String, f64>){
    loop{
        print!("\n");
        println!("Admin:\n   Article Manager:\n      1: Afficher article\n      2: Ajouter article\n      3: Supprimer article\n      4: Modifier article\n      5: Exit Item Manager");
        let buffer: String = prompt("      Choix: ");

        match buffer.as_str(){
            "1" => {
                println!("\nListe des articles:");
                for (name, prix) in &mut *item_shop{
                    println!("   {} : {}€", name, prix);
                }
            },
            "2" => {
                loop{
                    println!("\nAjout d'article:");
                    let art_name: String = prompt("Nom de l'article ou entrer (cancel) pour annuler: ");
                    let art_prix_dirty: String = prompt("Prix de l'aricle ou entrer (cancel) pour annuler: ");

                    if art_name == "cancel" || art_prix_dirty == "cancel" {
                        break;
                    }

                    if let Ok(art_prix_f64) = art_prix_dirty.parse::<f64>(){
                        if item_shop.contains_key(&art_name){
                            println!("{} est déjà present dans les articles.", art_name);
                            return;
                        }
                        println!("Le produit {} a bien été ajouté pour le prix de {}€.",art_name, art_prix_f64);
                        item_shop.insert(art_name, art_prix_f64);
                        break;
                    }else{
                        println!("Le prix n'est pas correct.");
                        prompt("Appuyez sur entrer pour continuer.");
                    }
                }
            },
            "3" => {
                loop{
                    println!("\nSupprimer un article:");
                    let art_name: String = prompt("Nom de l'article à supprimer ou entrer (cancel) pour annuler: ");
                    if art_name == "cancel" {
                        break;
                    }

                    if !item_shop.contains_key(&art_name){
                        println!("Ce produit n'existe pas.");
                        continue;
                    }
                    
                    println!("{} a bien été supprimer des articles.", &art_name);
                    item_shop.remove(&art_name);
                    break;
                }
            },
            "4" => {
                loop {
                    println!("\nModifier un article:");
                    let art_name: String = prompt("Nom de l'article à modifier ou entre (cancel) pour annuler: ");

                    if art_name == "cancel" {
                        break;
                    }

                    if !item_shop.contains_key(&art_name) {
                        println!("Ce produit n'existe pas.");
                        continue;
                    }

                    print!("{}", &art_name);
                    let buffer: String = prompt(" entrez un nouveau prix : ");
                    if let Ok(num) = buffer.parse::<f64>(){
                        println!("{} est bien maitenant à {}€", art_name, num);
                        item_shop.insert(art_name, num);
                    }else{
                        println!("Le prix n'est pas correct.");
                        prompt("Appuyez sur entrer pour continuer.");
                    }
                    break;
                }
            }
            "5" => break,
            _ => println!("Mauvais choix."),
        }
        prompt("Appuyez sur entrer pour continuer.");
    }

    prompt("Appuyez sur entrer pour continuer.");
}

fn choice_admin_account(database_account: &mut HashMap<String, Account>){
    loop {
        print!("\n");
        println!("Admin:\n   Compte Manager:\n      1: Afficher tout les comptes\n      2: Modifier un compte\n      3: Exit Compte Manager");
        let buffer: String = prompt("      Choix: ");
        match buffer.as_str(){
            "1" => {
                println!("\nListe des comptes:");
                for (name, account) in &mut *database_account{
                    if account.inventory.is_empty(){
                        println!("   username: {}\n         Money: {}\n         Inventory: Empty\n         IsAdmin: {}", name, account.money, account.isadmin);
                    }else{
                        println!("   username: {}\n         Money: {}\n         Inventory:", name, account.money);
                        for (name, number) in &account.inventory{
                            println!("            Nom: {}\n            Quantité: {}", &name, &number)
                        }
                        println!("         IsAdmin: {}", account.isadmin)
                    }
                }
            }
            "2" => {
                println!("\nModifier un compte:");
                let account_name: String = prompt("Username du compte à modifier ou entre (cancel) pour annuler: ");
                if account_name == "cancel" {
                    continue;
                }

                if !database_account.contains_key(&account_name) {
                    println!("Ce compte n'existe pas.");
                    continue;
                }
                loop {
                    println!("{}\n   1: Money\n   2: IsAdmin\n   3: Inventaire", &account_name);
                    if let Some(account) = database_account.get_mut(&account_name){
                        let buffer: String = prompt("Entrez un champ à modifier : ");
                        print!("\n");
                        match buffer.as_str(){
                            "1" => {
                                println!("Ce compte a {}€", account.money);
                                let buffer: String = prompt("A combien d'argent voulez vous mettre ce compte ou entre (cancel) pour annuler : ");

                                if buffer == "cancel" {
                                    break;
                                }

                                if let Ok(num) = buffer.parse::<f64>(){
                                    account.money = num;
                                    println!("Le compte {} a maintenant {}€", account_name, account.money);
                                }else{
                                    println!("Le chiffre n'est pas correct.");
                                    prompt("Appuyez sur entrer pour continuer.");
                                }
                            },
                            "2" => {
                                println!("Ce compte a IsAdmin = {}", account.isadmin);
                                let buffer: String = prompt("Status du compte (true or false) ou entre (cancel) pour annuler : ");

                                if buffer == "cancel" {
                                    break;
                                }

                                if let Ok(num) = buffer.parse::<bool>(){
                                    account.isadmin = num;
                                    println!("Le compte {} a IsAdmin = {}", account_name, account.isadmin);
                                }else{
                                    println!("Le boolean n'est pas correct.");
                                    prompt("Appuyez sur entrer pour continuer.");
                                }
                            },
                            "3" => {
                                println!("Inventaire:");
                                for (name, number) in &account.inventory {
                                    println!("   {} : {}", name, number);
                                }
                                let buffer: String = prompt("Que voulez vous faire.\n   1: Ajouter un item\n   2: Supprimer un item\nChoix: ");
                                match buffer.as_str(){
                                    "1" => {
                                        println!("Inventaire:\n   Ajout item:");
                                        let name_item: String = prompt("Nom de l'item à rajouter ou entre (cancel) pour annuler : ");
                                        print!("Nombre de {}", name_item);
                                        let number_item: String = prompt(" à rajouter ou entre (cancel) pour annuler : ");
                                        
                                        if name_item == "cancel" || number_item == "cancel" {
                                            break;
                                        }

                                        if let Ok(num) = number_item.parse::<i32>(){
                                            account.inventory.insert(name_item.clone(), num);
                                            println!("Vous avez bien ajouter {} {}", num, name_item);
                                        }else{
                                            println!("Le chiffre n'est pas correct.");
                                            prompt("Appuyez sur entrer pour continuer.");
                                        }
                                    },
                                    "2" => {
                                        println!("Inventaire:\n   Suppression item:");
                                        let name_item: String = prompt("Nom de l'item à supprimer ou entre (cancel) pour annuler : ");
                                        print!("Nombre de {}", name_item);
                                        let number_item: String = prompt(" à supprimer ou entre (cancel) pour annuler : ");
                                        
                                        if name_item == "cancel" || number_item == "cancel" {
                                            break;
                                        }

                                        if !account.inventory.contains_key(&name_item){
                                            println!("Aucun item existe avec le nom {}", name_item);
                                            continue;
                                        }

                                        if let Some(item_number) = account.inventory.get_mut(&name_item){
                                            if let Ok(num) = number_item.parse::<i32>(){                                
                                                let number_delete: i32 = *item_number - num;
                                                if number_delete <= 0{
                                                    account.inventory.remove(&name_item);
                                                }else{
                                                    account.inventory.insert(name_item.clone(), number_delete);
                                                }
                                                println!("Vous avez supprimer {} {}", number_delete, name_item);
                                            }else{
                                                println!("Le chiffre n'est pas correct.");
                                                prompt("Appuyez sur entrer pour continuer.");
                                            }
                                        }
                                    },
                                    _ => {
                                        println!("Mauvais choix.");
                                        prompt("Appuyez sur entrer pour continuer.");
                                    },
                                }
                            },
                            _ => {
                                println!("Mauvais choix.");
                                prompt("Appuyez sur entrer pour continuer.");
                            },
                        }
                    }
                }
                break;
            },
            "3" => break,
            _ => {
                println!("Mauvais choix.");
                prompt("Appuyez sur entrer pour continuer.");
                continue;
            },
        }
        prompt("Appuyez sur entrer pour continuer.");
    }
}

fn choice_admin_login(database_login: &mut HashMap<String, String>, database_account: &mut HashMap<String, Account>){
    loop{
        println!("\nAdmin\n   Login Manager\n      1: Afficher tout les comptes\n      2: Ajouter un login\n      3: Supprimer un login\n      4: Exit Login Manager");
        let buffer: String = prompt("      Choix: ");

        match buffer.as_str(){
            "1" => {
                println!("\nAccount:");
                for (username, password) in &mut *database_login{
                    println!("   username: {}, password: {}", username, password);
                }
            },
            "2" => {
                println!("\nAjouter un compte:");
                let account_name: String = prompt("Username du compte à ajouter ou entre (cancel) pour annuler: ");
                let account_pass: String = prompt("Password du compte à ajouter ou entre (cancel) pour annuler: ");

                if account_name == "cancel" || account_pass == "cancel" {
                    continue;
                }

                if database_login.contains_key(&account_name){
                    println!("Ce compte existe déjà");
                    continue;
                }

                database_login.insert(account_name.clone(),account_pass.clone());
                database_account.insert(account_name.clone(), Account { username: account_name.clone(), isadmin: false, money: 100.0, inventory: HashMap::new() });
                println!("Ce compte a bien été ajouter.");
            }
            "4" => break,
            _ => {
                println!("Mauvais choix.");
                prompt("Appuyez sur entrer pour continuer.");
            },
        }
        prompt("Appuyez sur entrer pour continuer.");
    }
}