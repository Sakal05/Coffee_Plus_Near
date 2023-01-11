// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use std::collections::HashMap;
use near_sdk::{ env, near_bindgen, AccountId };
use near_sdk::serde::{ Serialize, Deserialize };

// Define the contract structure
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub struct Contract {
    //Map receipt's Id to Coupon's Info
    listed_coupon: HashMap<String, Coupon>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Coupon {
    coupon_id: String,
    creator: AccountId,
    cup_number: String,
    discount: String,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - gets coupon id and generate Discount Coupon
    pub fn coupon_generator(&mut self, r_id: String, cup_num: i32) -> Coupon{ //r_id: receipt id, cup_num: number of coffee bought
        let c_id = r_id.clone() + "C";
        let d;
        if cup_num >= 2 && cup_num <= 5 {
            d = 5;
            //d = 5 + 2 * (cup_num - 1);
        }
        else if cup_num > 5 && cup_num <= 10 {
            d = 15;
        }
        else if cup_num > 10 && cup_num <= 15 {
            d = 30;
        }
        else if cup_num > 15 {
            d = 30 + 2 * (cup_num - 1);
        }
        else {
            panic!("Can't Proceed, minimum cup should be 2!");
        }
        let c_info = Coupon {
            creator: env::signer_account_id(),
            coupon_id: c_id,
            cup_number: cup_num.to_string(),
            discount: d.to_string(),
        };
        self.listed_coupon.insert(r_id, c_info.clone());
        env::log_str("Coupon has succesfully generated!");
        env::log_str(&format!("Coupon: {:?}", c_info));
        return c_info;
    }

    // Public view method - view all coupon inside hashmap
    pub fn get_coupons(&self) -> Vec<Coupon> {
        let all_coupon = self.listed_coupon.values().cloned().collect::<Vec<Coupon>>();
        env::log_str("Got All Coupons succesfully!");
        env::log_str(&format!("Coupon: {:?}", all_coupon));
        return all_coupon;
    }

    /* Public View Method - View coupon by ID */
    pub fn get_coupon(&self, c_id: String) -> Option<Coupon> {
        match self.listed_coupon.get(&c_id) {
            Some(result) => { Some(result.clone()) }
            None => None,
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success_coupon_generator() {
        let sakal_accountid: AccountId = "bob.near".parse().unwrap();
        let c_info1 = Coupon {
            creator: sakal_accountid,
            coupon_id: "001C".to_string(),
            cup_number: "6".to_string(),
            discount: "15".to_string(),
        };

        let map = HashMap::from([("1".to_string(), c_info1.clone())]);

        //let h_map: HashMap<String, Coupon> = HashMap::new();
        let mut contract_info: Contract = Contract {
            listed_coupon: map,
        };

        let contract = Contract::coupon_generator(&mut contract_info, "001".to_string(), 6);

        assert_eq!(contract, c_info1);
    }

    #[test]
    fn get_coupons() {
        let sakal_accountid: AccountId = "sakal.testnet".parse().unwrap();
        let alice_accountid: AccountId = "alice.testnet".parse().unwrap();
        let c_info1 = Coupon {
            creator: sakal_accountid,
            coupon_id: "1".to_string(),
            cup_number: "5".to_string(),
            discount: "20".to_string(),
        };
        let c_info2 = Coupon {
            creator: alice_accountid,
            coupon_id: "2".to_string(),
            cup_number: "10".to_string(),
            discount: "20".to_string(),
        };
        let map = HashMap::from([
            ("1".to_string(), c_info1),
            ("2".to_string(), c_info2),
        ]);
        let contract_info: Contract = Contract {
            listed_coupon: map,
        };
        
        let contract = Contract::get_coupons(&contract_info);
        let mut vec_coupon: Vec<Coupon> = Vec::new();
        for value in contract_info.listed_coupon.values() {
            vec_coupon.push(value.clone());
        }

        assert_eq!(contract, vec_coupon);
    }
    #[test]
    fn get_coupons_id() {
        let sakal_accountid: AccountId = "sakal.testnet".parse().unwrap();

        let c_info1 = Coupon {
            creator: sakal_accountid,
            coupon_id: "1".to_string(),
            cup_number: "5".to_string(),
            discount: "20".to_string(),
        };
        let c = c_info1.clone();
        let map = HashMap::from([("1".to_string(), c_info1)]);
        let contract_info: Contract = Contract {
            listed_coupon: map,
        };

        let contract_pass = Contract::get_coupon(&contract_info, "1".to_string());
        let contract_fail = Contract::get_coupon(&contract_info, "3".to_string());
        assert_eq!(contract_pass, Some(c));
        assert_eq!(contract_fail, None);
    }
    #[test]
    #[should_panic]
    fn panic_coupon_generator() {
        let sakal_accountid: AccountId = "sakal.testnet".parse().unwrap();
        let c_info1 = Coupon {
            creator: sakal_accountid,
            coupon_id: "1".to_string(),
            cup_number: "1".to_string(),
            discount: "20".to_string(),
        };

        let map = HashMap::from([("1".to_string(), c_info1)]);

        //let h_map: HashMap<String, Coupon> = HashMap::new();
        let mut contract_info: Contract = Contract {
            listed_coupon: map,
        };

        let _contract = Contract::coupon_generator(&mut contract_info, "001".to_string(), 1);
        //assert_eq!(contract, env::panic_str("Can't Proceed, minimum cup should be 2!"));
    }
}