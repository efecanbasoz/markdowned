const SERVICE_NAME: &str = "com.markdowned.app";

pub fn store_api_key(provider: &str, key: &str) -> Result<(), String> {
    let entry =
        keyring::Entry::new(SERVICE_NAME, provider).map_err(|e| format!("Keychain error: {e}"))?;
    entry
        .set_password(key)
        .map_err(|e| format!("Failed to store key: {e}"))
}

pub fn retrieve_api_key(provider: &str) -> Result<Option<String>, String> {
    let entry =
        keyring::Entry::new(SERVICE_NAME, provider).map_err(|e| format!("Keychain error: {e}"))?;
    match entry.get_password() {
        Ok(key) => Ok(Some(key)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to retrieve key: {e}")),
    }
}

pub fn delete_api_key(provider: &str) -> Result<(), String> {
    let entry =
        keyring::Entry::new(SERVICE_NAME, provider).map_err(|e| format!("Keychain error: {e}"))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Failed to delete key: {e}")),
    }
}

pub fn is_available() -> bool {
    let entry = keyring::Entry::new(SERVICE_NAME, "_test_availability");
    match entry {
        Ok(e) => {
            let _ = e.set_password("test");
            let _ = e.delete_credential();
            true
        }
        Err(_) => false,
    }
}
