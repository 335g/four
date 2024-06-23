use std::path::{Path, PathBuf};

use cargo_lambda_build::{zip_binary, BinaryArchive, BinaryData};
use cargo_lambda_metadata::cargo::main_binary;

use crate::lambda::error::LambdaError;

pub fn create_archive<P: AsRef<Path>>(manifest_path: P) -> Result<BinaryArchive, LambdaError> {
    let manifest_path = manifest_path.as_ref();
    let target_name = main_binary(&manifest_path)?;
    let data = BinaryData::Function(&target_name);
    let bootstrap_dir = cargo_lambda_metadata::cargo::target_dir(&manifest_path)
        .unwrap_or(PathBuf::from("target"))
        .join("lambda")
        .join(data.binary_location());
    let binary_path = bootstrap_dir.join(data.binary_name());
    if !binary_path.exists() {
        return Err(LambdaError::BinaryMissing(binary_path.clone()));
    }

    zip_binary(binary_path, bootstrap_dir, &data, None)
        .map_err(|report| LambdaError::CannotZipArchive(report.root_cause().to_string()))
}
