use error::{parse_error, Result};

#[derive(Clone, Debug)]
pub struct Group {
    /// Group name
    pub group: String,
    // Unique group id
    pub gid: u32,
    // Group members usernames
    pub users: Vec<String>,
}

impl Group {
    pub fn parse(line: &str) -> Result<Group> {
        let mut parts = line.split(';');

        let group = parts.next().ok_or(parse_error("expected group"))?;
        let gid = parts.next().ok_or(parse_error("expected gid"))?.parse::<u32>()?;
        //Allow for an empty users field. If there is a better way to do this, do it
        let users_str = parts.next().unwrap_or(" ");
        let users = users_str.split(',').map(|u| u.into()).collect();

        Ok(Group {
            group: group.into(),
            gid: gid,
            users: users
        })
    }
}
