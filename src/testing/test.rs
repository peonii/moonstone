use crate::Error;

use super::testcase::TestPackage;

pub async fn test_package(name: String) -> Result<(), Error> {
    let test_package = TestPackage::load(name.as_str())?;

    test_package.test().await?;

    Ok(())
}
