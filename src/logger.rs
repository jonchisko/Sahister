use std::fmt::Display;

pub fn log<T>(displayable: T)
where T: Display 
{
    println!("{}", displayable);
}