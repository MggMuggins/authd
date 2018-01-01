use argon2rs::{Argon2, Variant};
use argon2rs::verifier::Encoded;

use error::{parse_error, Result};

#[derive(Clone, Debug)]
pub struct User {
    /// Username
    pub user: String,
    /// Hashed password
    pub hash: String,
    /// User id
    pub uid: u32,
    /// Group id
    pub gid: u32,
    /// Real name
    pub name: String,
    /// Home directory path
    pub home: String,
    /// Shell path
    pub shell: String
}

impl User {
    pub fn parse(line: &str) -> Result<User> {
        let mut parts = line.split(';');

        let user = parts.next().ok_or(parse_error("expected user"))?;
        let hash = parts.next().ok_or(parse_error("expected hash"))?;
        let uid = parts.next().ok_or(parse_error("expected uid"))?.parse::<u32>()?;
        let gid = parts.next().ok_or(parse_error("expected uid"))?.parse::<u32>()?;
        let name = parts.next().ok_or(parse_error("expected real name"))?;
        let home = parts.next().ok_or(parse_error("expected home directory path"))?;
        let shell = parts.next().ok_or(parse_error("expected shell path"))?;

        Ok(User {
            user: user.into(),
            hash: hash.into(),
            uid: uid,
            gid: gid,
            name: name.into(),
            home: home.into(),
            shell: shell.into()
        })
    }
    
    pub fn set_passwd(&mut self, password: &str, salt: &str) -> Result<()> {
        let a2 = Argon2::new(10, 1, 4096, Variant::Argon2i)?;
        let e = Encoded::new(a2, password.as_bytes(), salt.as_bytes(), &[], &[]);
        self.hash = String::from_utf8(e.to_u8())?;
        Ok(())
    }

    pub fn verify_passwd(&self, password: &str) -> Result<bool> {
        let e = Encoded::from_u8(self.hash.as_bytes())?;
        Ok(e.verify(password.as_bytes()))
    }
}
