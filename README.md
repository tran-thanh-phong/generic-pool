# generic-pool
An example of generic data type, providing different types of pool. We can implement new type of pool without do migrating.  

# Step 1 - Deploy contract with the single pool

### Comment the declaration of ComplexPool in enum Pool and the related function
```rust
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Pool {
    SimplePool(SimplePool),
    // Comment this line for the first deployment
    // ComplexPool(ComplexPool),
}
```

```rust
    // Comment this function for the first deployment
    // pub fn create_complex_pool(&mut self) {
    //     let pool_id = self.pools.len() + 1;
    //     self.pools.insert(&pool_id, &Pool::ComplexPool(ComplexPool {prop_b: pool_id as u32}));
    // }
```

### Build and deploy contract
``` 
build.sh
deploy.sh 
```

### Create a simple pool

```
near call dev-1647959336462-77896213135645 create_simple_pool --accountId  dev-1647959336462-77896213135645
```

### View pools
```
 near view dev-1647959336462-77896213135645 get_pools
```

# Step 2 - Add the complex pool and redeploy
### Uncomment two block of code above & redeploy

### Create a complex pool

```
near call dev-1647959336462-77896213135645 create_simple_pool --accountId  dev-1647959336462-77896213135645
```

### View pools
```
 near view dev-1647959336462-77896213135645 get_pools
```