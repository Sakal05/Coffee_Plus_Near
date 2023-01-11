# Coffee Incentive Program on NEAR

The smart contract exposes 3 methods to generate and get coupon(s) in the NEAR network.

```rust
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize, Clone)]
pub struct Contract {
    //Map receipt's Id to Coupon's Info
    listed_coupon: HashMap<String, Coupon>
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
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
    pub fn coupon_generator(&mut self, r_id: String, cup_num: i32) {
        let c_id = r_id.clone() + "R";
        let d;
        if cup_num >= 2 {
            d = 5 + 2*(cup_num - 1);
        }
        else {
            panic!("Can't Proceed, minimum cup should be 2!");
        }
        let c_info = Coupon {
            creator: env::signer_account_id(),
            coupon_id: c_id,
            cup_number: cup_num.to_string(),
            discount: d.to_string()
        };
        
        self.listed_coupon.insert(r_id, c_info);
        env::log_str("Coupon has succesfully generated!");
    }

    // Public view method - view all coupon inside hashmap
    pub fn get_coupons(&self) -> Vec<Coupon> {
        let all_coupon = self.listed_coupon.values().cloned().collect::<Vec<Coupon>>();
        env::log_str("Got All Coupons succesfully!");
        return all_coupon;
    }

    /* Public View Method - View coupon by ID */
    pub fn get_coupon(&self, id: String) -> Option<Coupon> {
        match self.listed_coupon.get(&id) {
            Some(result) => {
                Some(result.clone())
            },
            None => None
        }
    }
}
```

<br />

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

<br />

## 1. Build and Deploy the Contract
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
./deploy.sh
```

Once finished, check the `neardev/dev-account` file to find the address in which the contract was deployed:

```bash
cat ./neardev/dev-account
# e.g. dev-1672242250468-70699655227233
```

<br />

## 2. Retrieve all Coupons

`get_coupons` is a read-only method (aka `view` method).

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the greeting
near view <dev-account> get_greeting
```

<br />

## 3. Generate New Coupon
`generate_coupon` changes the contract's state, for which it is a `change` method.

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction.

```bash
# Use near-cli to set a new greeting
near call <dev-account> generate_coupon '{"r_id": "9", "cup_num": 4}' --accountId <dev-account>
```

**Tip:** If you would like to call `generate_coupon` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.