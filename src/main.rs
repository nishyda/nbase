use std::{env, str::from_utf8};
mod database;
use database::DB;
mod databases;
use databases::DBs;

fn cli() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!()
    }
    match args[1].as_str() {
        "init"=>{
            if args.len() != 3 {
                println!("Wrong number of arguments");
                return
            }
            println!("Creating {}.ndb . . .",args[2]);
            DB::new(args[2].clone()).write_to_file();
            println!("Success!");
            return
        },
        "inits"=>{
            if args.len() != 3 {
                println!("Wrong number of arguments");
                return
            }
            println!("Creating {}.ndbs . . .",args[2]);
            DBs::new(args[2].clone()).write_to_file();
            println!("Success!");
            return
        },
        "with"=>{
            println!("Trying to read from {}.ndb . . .",args[2]);
            let mut db = DB::read_from_file(args[2].clone());
            println!("Read successful!");
            match args[3].as_str() {
                "json"=>{
                    db.to_json_file();
                }
                "push"=>{
                    if args.len() != 6 {
                        println!("Wrong number of arguments");
                        return
                    }
                    if db.add(args[4].clone(), args[5].as_bytes().to_vec()) {
                        println!("Push successful, writing into file . . .");
                        db.write_to_file()
                    } else {
                        println!("Push ignored");
                    }
                },
                "pushmany"=>{
                    if args.len() < 6 {
                        println!("Wrong number of arguments");
                        return
                    } 
                    if args.len()%2 != 0 {
                        println!("Wrong number of arguments");
                        return
                    }
                    if args.len() == 6 {
                        println!("Please use push instead for better performance.");
                    } 
                    let mut j = 0;
                    let mut w = false;
                    for i in (4..args.len()).step_by(2) {
                        j += 1;
                        if db.add(args[i].clone(), args[i+1].as_bytes().to_vec()) {
                            println!("Push {} successful", j);
                            w = true;
                        } else {
                            println!("Push {} ignored", j);
                        }
                    }
                    if w {
                        println!("Push many successful, writing into file . . .");
                        db.write_to_file()
                    } else {
                        println!("Push many ignored");
                    }  
                },
                "pull"=>{
                    if args.len() != 5 {
                        println!("Wrong number of arguments");
                        return
                    }
                    let (o, _) = db.get(args[4].clone());
                    if o != None {
                        println!("{}", from_utf8(&o.unwrap()).expect("Failed to convert data to utf8 string"))
                    } else {
                        println!("No value associated with key {}", args[3])
                    }
                },
                "edit"=>{
                    if args.len() != 6 {
                        println!("Wrong number of arguments");
                        return
                    }
                    if db.upd(args[4].clone(), args[5].as_bytes().to_vec()) {
                        println!("Edit successful, writing into file . . .");
                        db.write_to_file()
                    } else {
                        println!("Edit ignored, does the key exist?")
                    }
                },
                "pop"=>{
                    if args.len() != 5 {
                        println!("Wrong number of arguments");
                        return
                    }
                    if db.rem(args[4].clone()) {
                        println!("Pop successful, writing into file . . .");
                        db.write_to_file()
                    } else {
                        println!("Pop ignored, does the key exist?")
                    }
                },
                _=>println!("Invalid argument: {}",args[3])
            }
        },
        "withs"=>{
            println!("Trying to read from {}.ndbs . . .",args[2]);
            let mut dbs = DBs::read_from_file(args[2].clone());
            println!("Read successful!");
            match args[3].as_str() {
                "json"=>{
                    dbs.to_json_file();
                },
                "add"=>{
                    println!("Trying to read from {}.ndb . . .",args[4]);
                    let db = DB::read_from_file(args[4].clone());
                    println!("Read successful!");
                    dbs.db_list.push(db);
                    dbs.write_to_file();
                    println!("{}.ndb successfully added to {}.ndbs",args[4],args[2]);
                },
                _=>println!("Invalid argument: {}",args[3])
            }
        },
        _=>panic!()
    }
}

fn main() {
    /*
    let mut nishy = DB::new("Nishy".to_string());
    let mut linuxcat = DB::new("LinuxCat".to_string());
    let mut nefa = DB::new("Nefa".to_string());
    nishy.add("haha".to_string(), "jonathan".as_bytes().to_vec());
    linuxcat.add("money".to_string(), "very much a lot".as_bytes().to_vec());
    nefa.add("grades".to_string(), "very good".as_bytes().to_vec());
    let mut many = DBs::new("many".to_string());
    many.db_list.push(nishy);
    many.db_list.push(linuxcat);
    many.db_list.push(nefa);
    many.write_to_file();
    let nishy2 = DBs::read_from_file("many".to_string());
    nishy2.to_json_file();
    */
    cli()
}