use crate::Error;

use super::testcase::TestPackage;

pub async fn generate_tests(name: String, amount: u32, time_limit: u128) -> Result<(), Error> {
    let mut test_package = TestPackage::new(name, time_limit);

    test_package.generate_tests(amount).await?;
    test_package.save()?;

    Ok(())
}
