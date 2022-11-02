use super::{path::CONFIG, PemfileReader};

pub fn pem_reader() -> PemfileReader {
  let mut filepath = CONFIG.clone();
  filepath.push("identity.pem");
  PemfileReader { filepath }
}
