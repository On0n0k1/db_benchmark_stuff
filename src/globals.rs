#![allow(unused)]
pub mod db {
    pub const HOST: &str = "sqlserver";
    pub const PORT: u16 = 1433;
    pub const ADDRESS: &str = "sqlserver:1433";
    pub const USERNAME: &str = "SA";
    pub const PASSWORD: &str = "p@ssw0rD!";
    pub const DATABASE: &str = "RestoredData";
    pub const ADO: &str = "server=tcp:sqlserver,1433;user=SA;pwd=p@ssw0rD!;database=RestoredData;IntegratedSecurity=true;TrustServerCertificate=true";
}
