pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}
impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1
        // 🎁 Your code here! 🎁
        let split_fields: Vec<&str> = csv_row.split(",").collect();
        if split_fields.len() != 3 {
            return Err("insufficient length");
        }
        let name = split_fields[0];
        let Ok(good_deeds) = split_fields[1].parse::<u32>() else {
            return Err("invalid parse");
        };
        let Ok(bad_deeds) = split_fields[2].parse::<u32>() else {
            return Err("invalid parse");
        };
        Ok(Self::new(name.to_string(), good_deeds, bad_deeds))
    }
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };
        Self { name, niceness }
    }
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }
        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
        let ratio = good_deeds / (good_deeds + bad_deeds);
        ratio >= 0.75
    }
}
