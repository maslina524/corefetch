use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::borrow::ToOwned;
use alloc::format;

#[derive(Debug, Clone)]
struct Arg {
    long: String,
    short: Option<char>,
    takes_value: bool,
}

#[derive(Debug, Clone)]
pub struct ArgsParser {
    command_name: String,
    description: String,
    arguments: Vec<Arg>,
}

#[derive(Debug, Clone)]
pub struct ArgResult {
    pub map: BTreeMap<String, String>,
    pub solo: Vec<String>,
}

impl ArgsParser {
    pub fn new(command_name: &str, description: &str) -> Self {
        Self {
            command_name: command_name.to_owned(),
            description: description.to_owned(),
            arguments: Vec::new(),
        }
    }

    pub fn add_arg(&mut self, long: &str, short: Option<char>, takes_value: bool) {
        self.arguments.push(Arg {
            long: long.to_owned(),
            short,
            takes_value,
        });
    }

    pub fn parse(&self, args: &[String]) -> Result<ArgResult, String> {
        let mut map = BTreeMap::new();
        let mut solo = Vec::new();
        let mut stop_parsing = false;
        let mut i = 0;

        while i < args.len() {
            let arg = &args[i];

            if !stop_parsing && arg == "--" {
                stop_parsing = true;
                i += 1;
                continue;
            }

            if !stop_parsing && arg.starts_with("--") && arg.len() > 2 {
                let (name, value_from_arg) = arg.find('=').map_or_else(|| {
                    let name = &arg[2..];
                    (name, None)
                }, |eq_pos| {
                    let name = &arg[2..eq_pos];
                    let value = &arg[eq_pos + 1..];
                    (name, Some(value))
                });

                if let Some(arg_def) = self.arguments.iter().find(|a| a.long == name) {
                    if arg_def.takes_value {
                        let value = if let Some(val) = value_from_arg {
                            val.to_owned()
                        } else if i + 1 < args.len() {
                            let next = &args[i + 1];
                            if !next.starts_with('-') || next == "--" {
                                i += 1;
                                next.clone()
                            } else {
                                return Err(format!("option --{name} requires a value, but next argument is an option"));
                            }
                        } else {
                            return Err(format!("option --{name} requires a value, but none provided"));
                        };
                        map.insert(name.to_owned(), value);
                    } else {
                        if value_from_arg.is_some() {
                            return Err(format!("flag --{name} does not take a value"));
                        }
                        map.insert(name.to_owned(), "true".to_owned());
                    }
                } else {
                    return Err(format!("unknown long option: --{name}"));
                }
                i += 1;
                continue;
            }

            if !stop_parsing && arg.starts_with('-') && arg.len() > 1 {
                let chars: Vec<char> = arg[1..].chars().collect();
                let mut j = 0;
                
                while j < chars.len() {
                    let c = chars[j];
                    if let Some(arg_def) = self.arguments.iter().find(|a| a.short == Some(c)) {
                        if arg_def.takes_value {
                            if j + 1 < chars.len() {
                                let value: String = chars[j + 1..].iter().collect();
                                map.insert(arg_def.long.clone(), value);
                                break;
                            } else if i + 1 < args.len() {
                                let next = &args[i + 1];
                                if !next.starts_with('-') || next == "--" {
                                    map.insert(arg_def.long.clone(), next.clone());
                                    i += 1;
                                } else {
                                    return Err(format!("Option -{c} requires a value, but next argument is an option"));
                                }
                            } else {
                                return Err(format!("Option -{c} requires a value, but none provided"));
                            }
                        } else {
                            map.insert(arg_def.long.clone(), "true".to_owned());
                        }
                    } else {
                        return Err(format!("Unknown short option: -{c}"));
                    }
                    j += 1;
                }
                i += 1;
                continue;
            }

            solo.push(arg.clone());
            i += 1;
        }

        Ok(ArgResult { map, solo })
    }
}