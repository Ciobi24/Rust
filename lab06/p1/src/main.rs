use anyhow::Result;
use std::fs;
use std::io;

trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[&str]) -> Result<()>;
}

struct CpCommand;

impl Command for CpCommand {
    fn get_name(&self) -> &str {
        "count"
    }

    fn exec(&mut self, args: &[&str]) -> Result<()> {
        println!("{}", args.len());
        Ok(())
    }
}

struct TimesCommand {
    count: u32,
}

impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        self.count += 1;
        println!("{}", self.count);
        Ok(())
    }
}

struct PingCommand;

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }

    fn exec(&mut self, _args: &[&str]) -> Result<()> {
        println!("pong");
        Ok(())
    }
}
struct User;
//hello user command, write your username and the program will say hi to you


impl Command for User{
    fn get_name(&self)->&str{
        "user"
    }
    fn exec(&mut self, args:&[&str])->Result<()>{
        if args.len()==0{
            return Err(anyhow::anyhow!("user command must have at least one argument"));
        }
    let formatted_string = args.join(" ");
        println!("Hi, {} !",formatted_string);
        Ok(())
    }
}

struct Terminal {
    commands: Vec<Box<dyn Command>>,
}

impl Terminal {
    fn new() -> Terminal {
        Terminal {
            commands: Vec::<Box<dyn Command>>::new(),
        }
    }

    fn register(&mut self, b: Box<dyn Command>) {
        self.commands.push(b);
    }

    fn run(&mut self) -> Result<()> {
        let mut stop: bool = false;
        let s = fs::read_to_string("src/comands.txt")?;
        for line in s.lines() {
            if stop == false {
                let mut com = line.split_whitespace();
                let command_name = com.next().unwrap_or_default() ;
                    if command_name == "stop" {
                        stop = true;
                    } else {
                        let mut found:bool=false;
                        let args: Vec<&str> = com.collect();
                        for command in &mut self.commands {
                            if command_name == command.get_name() {
                                command.exec(&args)?;
                                found=true;
                            }
                        }
                        if found==false && command_name!=""{
                            return Err(anyhow::anyhow!("Command {} doesn't exist",command_name));
                        }
                    }
                }
        
        }
        Ok(())
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(CpCommand));
    terminal.register(Box::new(User));

    if let Err(e) = terminal.run() {
        eprintln!("Error: {}", e);
    }
}
