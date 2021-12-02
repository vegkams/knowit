use crate::solutions::utils::*;

// Struct for input data and result for luke 1
pub struct Luke1 {
    num_string: String,
    pub answer: String,
}


impl Luke1 {
    
    // Create the struct with the file path and input string
    pub fn new() -> Luke1 {
        let file_path = "/home/vegard/src/knowit/inputfiles/luke1.txt";
        let mut number_string = String::new();

        if let Ok(mut file) = File::open(file_path) {
             match file.read_to_string(&mut number_string) {
                 Ok(_) => (),
                 Err(_e) => panic!("Unable to read file for Luke1..."),
             };
        } else {
            panic!("Unable to read inputfile!");
        }

        Luke1{
            num_string: number_string,
            answer: String::new(),
        }
    }

    /// Find the max sum of numbers written as a string of numbers in Norwegian (in the input file)
    pub fn solve(&mut self) -> Result<(), Error>  {
        let data_len = self.num_string.len();
        let num_slice: &str = &self.num_string;
        let mut data_pos = 0;
        let mut sum = 0;
        let map = self.create_map();

        while data_pos < data_len - 1 {
            let mut best_num = 0;
            let mut best_pair_sum = 0;
            // Iterate in reverse over all possible numbers
            for num in (1..51).rev() {
                let num_str = map.get(&(num as u8)).unwrap();
                // See if the remaining string slice starts with this number
                if num_slice[data_pos..].starts_with(num_str) {
                    let new_data_pos = data_pos + num_str.len();
                    if new_data_pos >= data_len {
                        data_pos = new_data_pos;
                        sum += num;
                        best_num = num;
                        break;
                    }
                    else {
                        for next_num in (1..51).rev() {
                            let next_num_str = map.get(&(next_num as u8)).unwrap();
                            if num_slice[new_data_pos..].starts_with(next_num_str) && num + next_num > best_pair_sum {
                                best_pair_sum = num + next_num;
                                best_num = num;
                            }            
                        }
                    }
                }
            }
            if best_num >= 0 {
                data_pos += map.get(&(best_num as u8)).unwrap().len();
                sum += best_num;
            }
        }
        if sum == 0 {
            return Err(Error::SolverError);
        }
        self.answer = format!("{}", sum);
        Ok(())
    }

    /// Create a hashmap of keys 1-50 as u8, and value as corresponding Norwegian string representation of number
    fn create_map(&self) -> HashMap<u8, String> {
        let mut map = HashMap::new();
        for (i, num) in self.combine_map(0, "") {
            map.insert(i, num);
        }
        map.insert(10, String::from("ti"));
        map.insert(11, String::from("elleve"));
        map.insert(12, String::from("tolv"));
        map.insert(13, String::from("tretten"));
        map.insert(14, String::from("fjorten"));
        map.insert(15, String::from("femten"));
        map.insert(16, String::from("seksten"));
        map.insert(17, String::from("sytten"));
        map.insert(18, String::from("atten"));
        map.insert(19, String::from("nitten"));
        map.insert(20, String::from("tjue"));
        for (i, num) in self.combine_map(20, "tjue") {
            map.insert(i, num);
        }
        map.insert(30, String::from("tretti"));
        for (i, num) in self.combine_map(30, "tretti") {
            map.insert(i, num);
        }
        map.insert(40, String::from("førti"));
        for (i, num) in self.combine_map(40, "førti") {
            map.insert(i, num);
        }
        map.insert(50, String::from("femti"));
        map
    }


    fn combine_map(&self, radix: u8, base: &str) -> HashMap<u8, String> {
        let mut map: HashMap<u8, String> = HashMap::new();
        map.insert(radix+1, format!("{}en", base));
        map.insert(radix+2, format!("{}to", base));
        map.insert(radix+3, format!("{}tre", base));
        map.insert(radix+4, format!("{}fire", base));
        map.insert(radix+5, format!("{}fem", base));
        map.insert(radix+6, format!("{}seks", base));
        map.insert(radix+7, format!("{}sju", base));
        map.insert(radix+8, format!("{}åtte", base));
        map.insert(radix+9, format!("{}ni", base));
        map
    }
}

#[cfg(test)]
mod tests_luke1 {
    use super::*;
    #[test]
    fn input1() {
        let mut test_input = Luke1 {
            num_string: "entotrefirefem".to_string(),
            answer: String::new(),
        };
        test_input.solve().ok();
        assert_eq!("15", test_input.answer);
    }
    #[test]
    fn input2() {
        let mut test_input = Luke1 {
            num_string: "sjufirenitrettentrettitretrettitre".to_string(),
            answer: String::new(),
        };
        test_input.solve().ok();
        assert_eq!("99", test_input.answer);
    }
}