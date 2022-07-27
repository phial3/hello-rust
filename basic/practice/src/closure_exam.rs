#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                _ => println!("nothing matched ...")
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory() {
        let store = Inventory {
            shirts: vec!(ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue),
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveway1 = store.giveway(user_pref1);
        println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
    }
}



