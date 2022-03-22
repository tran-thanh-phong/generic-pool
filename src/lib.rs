use near_sdk::{near_bindgen, env};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::collections::{UnorderedMap};

type PoolId = u64;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct SimplePool {
    prop_a: u32,
}

// Comment this line for the first deployment
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ComplexPool {
    prop_b: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Pool {
    SimplePool(SimplePool),
    // Comment this line for the first deployment
    ComplexPool(ComplexPool),
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pools: UnorderedMap<PoolId, Pool>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            pools: UnorderedMap::new(b"m".to_vec()),
        }   
    }
}

#[near_bindgen]
impl Contract {
    pub fn create_simple_pool(&mut self) {
        let pool_id = self.pools.len() + 1;
        self.pools.insert(&pool_id, &Pool::SimplePool(SimplePool {prop_a: pool_id as u32}));
    }

    // Comment this function for the first deployment
    pub fn create_complex_pool(&mut self) {
        let pool_id = self.pools.len() + 1;
        self.pools.insert(&pool_id, &Pool::ComplexPool(ComplexPool {prop_b: pool_id as u32}));
    }

    pub fn get_pools(&self) {
        for (pool_id, pool) in self.pools.to_vec() {
            env::log(format!("Pool id: {}, Pool: {:?}", pool_id, pool).as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
