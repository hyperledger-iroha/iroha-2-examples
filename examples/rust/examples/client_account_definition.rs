use eyre::Error;

fn main() {
    account_definition_test().expect("Account definition example is expected to work correctly");

    println!("Account definition example works!");
}

fn account_definition_test() -> Result<(), Error> {
    // #region account_definition_comparison
    use iroha_data_model::prelude::AccountId;

    // Create an `iroha_data_model::AccountId` instance
    // with a DomainId instance and a Domain ID for an account
    let longhand_account_id = AccountId::new("white_rabbit".parse()?, "looking_glass".parse()?);
    let account_id: AccountId = "white_rabbit@looking_glass"
        .parse()
        .expect("Valid, because the string contains no whitespace, has a single '@' character and is not empty after");

    // Check that two ways to define an account match
    assert_eq!(account_id, longhand_account_id);

    // #endregion account_definition_comparison

    // Finish the test successfully
    Ok(())
}
