use crate::key_bind::KeyBind;
use crate::log::LogLevel;
use crate::Key;
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[cfg(test)]
use serde::Serialize;

#[derive(StructOpt, Debug)]
pub struct CliConfig {
    /// Set the config file
    #[structopt(long, short, global = true)]
    config_path: Option<std::path::PathBuf>,

    /// Set the key bind file
    #[structopt(long, short, global = true)]
    key_bind_path: Option<std::path::PathBuf>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReadConfig {
    pub conn: Vec<Connection>,
    #[serde(default)]
    pub log_level: LogLevel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub conn: Vec<Connection>,
    #[serde(default)]
    pub key_config: KeyConfig,
    #[serde(default)]
    pub log_level: LogLevel,
}

#[derive(Debug, Deserialize, Clone)]
enum DatabaseType {
    #[serde(rename = "mysql")]
    MySql,
    #[serde(rename = "postgres")]
    Postgres,
    #[serde(rename = "sqlite")]
    Sqlite,
}

impl fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MySql => write!(f, "mysql"),
            Self::Postgres => write!(f, "postgres"),
            Self::Sqlite => write!(f, "sqlite"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            conn: vec![Connection {
                r#type: DatabaseType::MySql,
                name: None,
                user: Some("root".to_string()),
                host: Some("localhost".to_string()),
                port: Some(3306),
                path: None,
                password: None,
                database: None,
                unix_domain_socket: None,
                limit_size: 200,
                timeout_second: 5,
            }],
            key_config: KeyConfig::default(),
            log_level: LogLevel::default(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Connection {
    r#type: DatabaseType,
    name: Option<String>,
    user: Option<String>,
    host: Option<String>,
    port: Option<u64>,
    path: Option<std::path::PathBuf>,
    password: Option<String>,
    unix_domain_socket: Option<std::path::PathBuf>,
    pub database: Option<String>,
    #[serde(default = "default_limit_size")]
    pub limit_size: usize,
    #[serde(default = "default_timeout_second")]
    pub timeout_second: u64,
}

fn default_limit_size() -> usize {
    200
}

fn default_timeout_second() -> u64 {
    5
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(Serialize, PartialEq))]
pub struct KeyConfig {
    pub scroll_up: Key,
    pub scroll_down: Key,
    pub scroll_right: Key,
    pub scroll_left: Key,
    pub sort_by_column: Key,
    pub move_up: Key,
    pub move_down: Key,
    pub copy: Key,
    pub enter: Key,
    pub exit: Key,
    pub quit: Key,
    pub exit_popup: Key,
    pub focus_right: Key,
    pub focus_left: Key,
    pub focus_above: Key,
    pub focus_connections: Key,
    pub open_help: Key,
    pub filter: Key,
    pub scroll_down_multiple_lines: Key,
    pub scroll_up_multiple_lines: Key,
    pub scroll_to_top: Key,
    pub scroll_to_bottom: Key,
    pub move_to_head_of_line: Key,
    pub move_to_tail_of_line: Key,
    pub extend_selection_by_one_cell_left: Key,
    pub extend_selection_by_one_cell_right: Key,
    pub extend_selection_by_one_cell_up: Key,
    pub extend_selection_by_one_cell_down: Key,
    pub extend_selection_by_horizontal_line: Key,
    pub tab_records: Key,
    pub tab_columns: Key,
    pub tab_constraints: Key,
    pub tab_definition: Key,
    pub tab_foreign_keys: Key,
    pub tab_indexes: Key,
    pub tab_sql_editor: Key,
    pub tab_properties: Key,
    pub extend_or_shorten_widget_width_to_right: Key,
    pub extend_or_shorten_widget_width_to_left: Key,
}

impl Default for KeyConfig {
    fn default() -> Self {
        Self {
            scroll_up: Key::Char('k'),
            scroll_down: Key::Char('j'),
            scroll_right: Key::Char('l'),
            scroll_left: Key::Char('h'),
            sort_by_column: Key::Char('s'),
            move_up: Key::Up,
            move_down: Key::Down,
            copy: Key::Char('y'),
            enter: Key::Enter,
            exit: Key::Ctrl('c'),
            quit: Key::Char('q'),
            exit_popup: Key::Esc,
            focus_right: Key::Right,
            focus_left: Key::Left,
            focus_above: Key::Up,
            focus_connections: Key::Char('c'),
            open_help: Key::Char('?'),
            filter: Key::Char('/'),
            scroll_down_multiple_lines: Key::Ctrl('d'),
            scroll_up_multiple_lines: Key::Ctrl('u'),
            scroll_to_top: Key::Char('g'),
            scroll_to_bottom: Key::Char('G'),
            move_to_head_of_line: Key::Char('^'),
            move_to_tail_of_line: Key::Char('$'),
            extend_selection_by_one_cell_left: Key::Char('H'),
            extend_selection_by_one_cell_right: Key::Char('L'),
            extend_selection_by_one_cell_down: Key::Char('J'),
            extend_selection_by_horizontal_line: Key::Char('V'),
            extend_selection_by_one_cell_up: Key::Char('K'),
            tab_records: Key::Char('1'),
            tab_properties: Key::Char('2'),
            tab_sql_editor: Key::Char('3'),
            tab_columns: Key::Char('4'),
            tab_constraints: Key::Char('5'),
            tab_foreign_keys: Key::Char('6'),
            tab_indexes: Key::Char('7'),
            tab_definition: Key::Char('8'),
            extend_or_shorten_widget_width_to_right: Key::Char('>'),
            extend_or_shorten_widget_width_to_left: Key::Char('<'),
        }
    }
}

impl Config {
    pub fn new(config: &CliConfig) -> anyhow::Result<Self> {
        let config_path = if let Some(config_path) = &config.config_path {
            config_path.clone()
        } else {
            get_app_config_path()?.join("config.toml")
        };

        let key_bind_path = if let Some(key_bind_path) = &config.key_bind_path {
            key_bind_path.clone()
        } else {
            get_app_config_path()?.join("key_bind.ron")
        };

        if let Ok(file) = File::open(config_path) {
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents)?;
            let config: Result<ReadConfig, toml::de::Error> = toml::from_str(&contents);
            match config {
                Ok(config) => return Ok(Config::build(config, key_bind_path)),
                Err(e) => panic!("fail to parse connection config file: {}", e),
            }
        }

        Ok(Config::default())
    }

    fn build(read_config: ReadConfig, key_bind_path: PathBuf) -> Self {
        let key_bind = KeyBind::load(key_bind_path).unwrap();
        Config {
            conn: read_config.conn,
            log_level: read_config.log_level,
            key_config: KeyConfig::from(key_bind),
        }
    }
}

impl Connection {
    pub fn database_url(&self) -> anyhow::Result<String> {
        let password = self
            .password
            .as_ref()
            .map_or(String::new(), |p| p.to_string());
        self.build_database_url(password)
    }

    fn masked_database_url(&self) -> anyhow::Result<String> {
        let password = self
            .password
            .as_ref()
            .map_or(String::new(), |p| p.to_string());

        let masked_password = "*".repeat(password.len());
        self.build_database_url(masked_password)
    }

    fn build_database_url(&self, password: String) -> anyhow::Result<String> {
        match self.r#type {
            DatabaseType::MySql => {
                let user = self.user.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "type mysql needs the user field in Connection::build_database_url"
                    )
                })?;
                let host = self.host.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "type mysql needs the host field in Connection::build_database_url"
                    )
                })?;
                let port = self.port.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "type mysql needs the port field in Connection::build_database_url"
                    )
                })?;
                let unix_domain_socket = self
                    .valid_unix_domain_socket()
                    .map_or(String::new(), |uds| format!("?socket={}", uds));

                match self.database.as_ref() {
                    Some(database) => Ok(format!(
                        "mysql://{user}:{password}@{host}:{port}/{database}{unix_domain_socket}",
                        user = user,
                        password = password,
                        host = host,
                        port = port,
                        database = database,
                        unix_domain_socket = unix_domain_socket
                    )),
                    None => Ok(format!(
                        "mysql://{user}:{password}@{host}:{port}{unix_domain_socket}",
                        user = user,
                        password = password,
                        host = host,
                        port = port,
                        unix_domain_socket = unix_domain_socket
                    )),
                }
            }
            DatabaseType::Postgres => {
                let user = self.user.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "type postgres needs the user field in Connection::build_database_url"
                    )
                })?;
                let host = self.host.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "type postgres needs the host field in Connection::build_database_url"
                    )
                })?;
                let port = self.port.as_ref().ok_or_else(|| {
                    anyhow::anyhow!(
                        "type postgres needs the port field in Connection::build_database_url"
                    )
                })?;

                if let Some(unix_domain_socket) = self.valid_unix_domain_socket() {
                    match self.database.as_ref() {
                        Some(database) => Ok(format!(
                            "postgres://?dbname={database}&host={unix_domain_socket}&user={user}&password={password}",
                            database = database,
                            unix_domain_socket = unix_domain_socket,
                            user = user,
                            password = password,
                        )),
                        None => Ok(format!(
                            "postgres://?host={unix_domain_socket}&user={user}&password={password}",
                            unix_domain_socket = unix_domain_socket,
                            user = user,
                            password = password,
                        )),
                    }
                } else {
                    match self.database.as_ref() {
                        Some(database) => Ok(format!(
                            "postgres://{user}:{password}@{host}:{port}/{database}",
                            user = user,
                            password = password,
                            host = host,
                            port = port,
                            database = database,
                        )),
                        None => Ok(format!(
                            "postgres://{user}:{password}@{host}:{port}",
                            user = user,
                            password = password,
                            host = host,
                            port = port,
                        )),
                    }
                }
            }
            DatabaseType::Sqlite => {
                let path = self.path.as_ref().map_or(
                    Err(anyhow::anyhow!(
                        "type sqlite needs the path field in Connection::build_database_url"
                    )),
                    |path| {
                        expand_path(path).ok_or_else(|| {
                            anyhow::anyhow!(
                                "cannot expand file path in Connection::build_database_url"
                            )
                        })
                    },
                )?;

                Ok(format!("sqlite://{path}", path = path.to_str().unwrap()))
            }
        }
    }

    pub fn database_url_with_name(&self) -> anyhow::Result<String> {
        match self.masked_database_url() {
            Ok(url) => Ok(match &self.name {
                Some(name) => format!("[{name}] {database_url}", name = name, database_url = url),
                None => url,
            }),
            Err(e) => Err(anyhow::anyhow!(e)
                .context("Failed to masked_database_url in Connection::database_url_with_name")),
        }
    }

    pub fn is_mysql(&self) -> bool {
        matches!(self.r#type, DatabaseType::MySql)
    }

    pub fn is_postgres(&self) -> bool {
        matches!(self.r#type, DatabaseType::Postgres)
    }

    fn valid_unix_domain_socket(&self) -> Option<String> {
        if cfg!(windows) {
            // NOTE:
            // windows also supports UDS, but `rust` does not support UDS in windows now.
            // https://github.com/rust-lang/rust/issues/56533
            return None;
        }
        return self.unix_domain_socket.as_ref().and_then(|uds| {
            let path = expand_path(uds)?;
            let path_str = path.to_str()?;
            if path_str.is_empty() {
                return None;
            }
            Some(path_str.to_owned())
        });
    }
}

pub fn get_app_config_path() -> anyhow::Result<std::path::PathBuf> {
    let mut path = if cfg!(target_os = "macos") {
        dirs_next::home_dir().map(|h| h.join(".config"))
    } else {
        dirs_next::config_dir()
    }
    .ok_or_else(|| anyhow::anyhow!("failed to find os config dir."))?;

    path.push("zhobo");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

fn expand_path(path: &Path) -> Option<PathBuf> {
    let mut expanded_path = PathBuf::new();
    let mut path_iter = path.iter();
    if path.starts_with("~") {
        path_iter.next()?;
        expanded_path = expanded_path.join(dirs_next::home_dir()?);
    }
    for path in path_iter {
        let path = path.to_str()?;
        expanded_path = if cfg!(unix) && path.starts_with('$') {
            expanded_path.join(std::env::var(path.strip_prefix('$')?).unwrap_or_default())
        } else if cfg!(windows) && path.starts_with('%') && path.ends_with('%') {
            expanded_path
                .join(std::env::var(path.strip_prefix('%')?.strip_suffix('%')?).unwrap_or_default())
        } else {
            expanded_path.join(path)
        }
    }
    Some(expanded_path)
}

#[cfg(test)]
mod test {
    use super::{
        expand_path, CliConfig, Config, Connection, DatabaseType, KeyConfig, Path, PathBuf,
    };
    use serde_json::Value;
    use std::env;

    #[test]
    fn test_load_config() {
        let cli_config = CliConfig {
            config_path: Some(Path::new("examples/config.toml").to_path_buf()),
            key_bind_path: Some(Path::new("examples/key_bind.ron").to_path_buf()),
        };

        assert_eq!(Config::new(&cli_config).is_ok(), true);
    }

    #[test]
    #[cfg(unix)]
    fn test_database_url() {
        let mysql_conn = Connection {
            r#type: DatabaseType::MySql,
            name: None,
            user: Some("root".to_owned()),
            host: Some("localhost".to_owned()),
            port: Some(3306),
            path: None,
            password: Some("password".to_owned()),
            database: Some("city".to_owned()),
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        let mysql_result = mysql_conn.database_url().unwrap();
        assert_eq!(
            mysql_result,
            "mysql://root:password@localhost:3306/city".to_owned()
        );

        let postgres_conn = Connection {
            r#type: DatabaseType::Postgres,
            name: None,
            user: Some("root".to_owned()),
            host: Some("localhost".to_owned()),
            port: Some(3306),
            path: None,
            password: Some("password".to_owned()),
            database: Some("city".to_owned()),
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        let postgres_result = postgres_conn.database_url().unwrap();
        assert_eq!(
            postgres_result,
            "postgres://root:password@localhost:3306/city".to_owned()
        );

        let sqlite_conn = Connection {
            r#type: DatabaseType::Sqlite,
            name: None,
            user: None,
            host: None,
            port: None,
            path: Some(PathBuf::from("/home/user/sqlite3.db")),
            password: None,
            database: None,
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        let sqlite_result = sqlite_conn.database_url().unwrap();
        assert_eq!(sqlite_result, "sqlite:///home/user/sqlite3.db".to_owned());
    }

    #[test]
    fn test_overlappted_key() {
        let value: Value =
            serde_json::from_str(&serde_json::to_string(&KeyConfig::default()).unwrap()).unwrap();
        if let Value::Object(map) = value {
            let mut values: Vec<String> = map
                .values()
                .map(|v| match v {
                    Value::Object(map) => Some(format!("{:?}", map)),
                    _ => None,
                })
                .flatten()
                .collect();
            values.sort();
            let before_values = values.clone();
            values.dedup();
            pretty_assertions::assert_eq!(before_values, values);
        }
    }

    #[test]
    #[cfg(unix)]
    fn test_dataset_url_in_unix() {
        let mut mysql_conn = Connection {
            r#type: DatabaseType::MySql,
            name: None,
            user: Some("root".to_owned()),
            host: Some("localhost".to_owned()),
            port: Some(3306),
            path: None,
            password: Some("password".to_owned()),
            database: Some("city".to_owned()),
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        assert_eq!(
            mysql_conn.database_url().unwrap(),
            "mysql://root:password@localhost:3306/city".to_owned()
        );

        mysql_conn.unix_domain_socket = Some(Path::new("/tmp/mysql.sock").to_path_buf());
        assert_eq!(
            mysql_conn.database_url().unwrap(),
            "mysql://root:password@localhost:3306/city?socket=/tmp/mysql.sock".to_owned()
        );

        let mut postgres_conn = Connection {
            r#type: DatabaseType::Postgres,
            name: None,
            user: Some("root".to_owned()),
            host: Some("localhost".to_owned()),
            port: Some(3306),
            path: None,
            password: Some("password".to_owned()),
            database: Some("city".to_owned()),
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        assert_eq!(
            postgres_conn.database_url().unwrap(),
            "postgres://root:password@localhost:3306/city".to_owned()
        );
        postgres_conn.unix_domain_socket = Some(Path::new("/tmp").to_path_buf());
        assert_eq!(
            postgres_conn.database_url().unwrap(),
            "postgres://?dbname=city&host=/tmp&user=root&password=password".to_owned()
        );

        let sqlite_conn = Connection {
            r#type: DatabaseType::Sqlite,
            name: None,
            user: None,
            host: None,
            port: None,
            path: Some(PathBuf::from("/home/user/sqlite3.db")),
            password: None,
            database: None,
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        let sqlite_result = sqlite_conn.database_url().unwrap();
        assert_eq!(sqlite_result, "sqlite:///home/user/sqlite3.db".to_owned());
    }

    #[test]
    #[cfg(windows)]
    fn test_database_url_in_windows() {
        let mut mysql_conn = Connection {
            r#type: DatabaseType::MySql,
            name: None,
            user: Some("root".to_owned()),
            host: Some("localhost".to_owned()),
            port: Some(3306),
            path: None,
            password: Some("password".to_owned()),
            database: Some("city".to_owned()),
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        assert_eq!(
            mysql_conn.database_url().unwrap(),
            "mysql://root:password@localhost:3306/city".to_owned()
        );

        mysql_conn.unix_domain_socket = Some(Path::new("/tmp/mysql.sock").to_path_buf());
        assert_eq!(
            mysql_conn.database_url().unwrap(),
            "mysql://root:password@localhost:3306/city".to_owned()
        );

        let mut postgres_conn = Connection {
            r#type: DatabaseType::Postgres,
            name: None,
            user: Some("root".to_owned()),
            host: Some("localhost".to_owned()),
            port: Some(3306),
            path: None,
            password: Some("password".to_owned()),
            database: Some("city".to_owned()),
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        assert_eq!(
            postgres_conn.database_url().unwrap(),
            "postgres://root:password@localhost:3306/city".to_owned()
        );
        postgres_conn.unix_domain_socket = Some(Path::new("/tmp").to_path_buf());
        assert_eq!(
            postgres_conn.database_url().unwrap(),
            "postgres://root:password@localhost:3306/city".to_owned()
        );

        let sqlite_conn = Connection {
            r#type: DatabaseType::Sqlite,
            name: None,
            user: None,
            host: None,
            port: None,
            path: Some(PathBuf::from("/home/user/sqlite3.db")),
            password: None,
            database: None,
            unix_domain_socket: None,
            limit_size: 200,
            timeout_second: 5,
        };

        let sqlite_result = sqlite_conn.database_url().unwrap();
        assert_eq!(
            sqlite_result,
            "sqlite://\\home\\user\\sqlite3.db".to_owned()
        );
    }

    #[test]
    #[cfg(unix)]
    fn test_expand_path() {
        let home = env::var("HOME").unwrap();
        let test_env = "baz";
        env::set_var("TEST", test_env);

        assert_eq!(
            expand_path(&Path::new("$HOME/foo")),
            Some(PathBuf::from(&home).join("foo"))
        );

        assert_eq!(
            expand_path(&Path::new("$HOME/foo/$TEST/bar")),
            Some(PathBuf::from(&home).join("foo").join(test_env).join("bar"))
        );

        assert_eq!(
            expand_path(&Path::new("~/foo")),
            Some(PathBuf::from(&home).join("foo"))
        );

        assert_eq!(
            expand_path(&Path::new("~/foo/~/bar")),
            Some(PathBuf::from(&home).join("foo").join("~").join("bar"))
        );
    }

    #[test]
    #[cfg(windows)]
    fn test_expand_patha() {
        let home = std::env::var("HOMEPATH").unwrap();
        let test_env = "baz";
        env::set_var("TEST", test_env);

        assert_eq!(
            expand_path(&Path::new("%HOMEPATH%/foo")),
            Some(PathBuf::from(&home).join("foo"))
        );

        assert_eq!(
            expand_path(&Path::new("%HOMEPATH%/foo/%TEST%/bar")),
            Some(PathBuf::from(&home).join("foo").join(test_env).join("bar"))
        );

        assert_eq!(
            expand_path(&Path::new("~/foo")),
            Some(PathBuf::from(&dirs_next::home_dir().unwrap()).join("foo"))
        );

        assert_eq!(
            expand_path(&Path::new("~/foo/~/bar")),
            Some(
                PathBuf::from(&dirs_next::home_dir().unwrap())
                    .join("foo")
                    .join("~")
                    .join("bar")
            )
        );
    }
}
