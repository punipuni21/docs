use lombok::*;

#[derive(Getter, GetterMut, Setter, NoArgsConstructor, AllArgsConstructor, ToString, Clone)]
pub struct ConnectParams {
    host: String,
    port: u16,
    dbname: String,
    user: String,
    password: String,
}
impl ConnectParams {
    pub fn connect_string(&self) -> String {
        format!(
            "host={} port={} dbname={} user={} password={}",
            self.host, self.port, self.dbname, self.user, self.password
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    #[test]
    fn connect_string() -> Result<()> {
        let params = ConnectParams::new(
            "localhost".to_owned(),
            5432,
            "rust_sample".to_owned(),
            "postgres".to_owned(),
            "postgres".to_owned(),
        );
        println!("{:?}", params);
        Ok(())
    }
}
