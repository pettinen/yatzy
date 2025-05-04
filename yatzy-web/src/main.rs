use std::{net::IpAddr, path::PathBuf};

use axum::{
    Json, Router,
    extract::RawQuery,
    http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
    response::{AppendHeaders, IntoResponse},
    routing::get,
};
use clap::Parser;
use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_rational::Ratio;
use pct_str::PctStr;
use regex::Regex;
use rustc_hash::FxBuildHasher;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use tokio::net::{TcpListener, UnixListener};
use yatzy::{Combo, Game, GameOptions};
use yatzy_solver::{
    Choice, GameState, best_choice_0_rerolls, best_choice_1_reroll, best_choice_2_rerolls,
};

lazy_static! {
    static ref DICE_REGEX: Regex =
        Regex::new(r"^[1-6],[1-6],[1-6],[1-6],[1-6]$").expect("invalid regex");
    static ref EXPECTED_VALUES: papaya::HashMap<GameState, Ratio<BigUint>, FxBuildHasher> =
        papaya::HashMap::with_capacity_and_hasher(958_974, FxBuildHasher);
}

#[derive(Clone, Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    config: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct ConfigInput {
    expected_values_path: PathBuf,
    tcp_listen_address: Option<IpAddr>,
    tcp_listen_port: Option<u16>,
    unix_socket_path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
enum Socket {
    Tcp(IpAddr, u16),
    Unix(PathBuf),
}

#[derive(Clone, Debug)]
struct Config {
    expected_values_path: PathBuf,
    socket: Socket,
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
enum ConfigError {
    #[error("missing TCP listener address")]
    MissingTcpAddress,
    #[error("missing TCP listener port")]
    MissingTcpPort,
    #[error("configure a TCP listener or a Unix socket listener")]
    NoListener,
    #[error("configure either a TCP listener or a Unix socket listener, not both")]
    TcpAndUnixListenersMutuallyExclusive,
}

impl TryFrom<ConfigInput> for Config {
    type Error = ConfigError;

    fn try_from(value: ConfigInput) -> Result<Self, Self::Error> {
        let socket = match (
            value.unix_socket_path,
            value.tcp_listen_address,
            value.tcp_listen_port,
        ) {
            (None, None, None) => {
                return Err(ConfigError::NoListener);
            }
            (None, None, Some(_)) => {
                return Err(ConfigError::MissingTcpAddress);
            }
            (None, Some(_), None) => {
                return Err(ConfigError::MissingTcpPort);
            }
            (None, Some(addr), Some(port)) => Socket::Tcp(addr, port),
            (Some(path), None, None) => Socket::Unix(path),
            (Some(_), None, Some(_)) | (Some(_), Some(_), None) | (Some(_), Some(_), Some(_)) => {
                return Err(ConfigError::TcpAndUnixListenersMutuallyExclusive);
            }
        };
        Ok(Self {
            expected_values_path: value.expected_values_path,
            socket,
        })
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let config_string = match std::fs::read_to_string(args.config) {
        Ok(config_string) => config_string,
        Err(error) => {
            eprintln!("failed to read config file: {error}");
            std::process::exit(2);
        }
    };
    let config_input: ConfigInput = match toml::from_str(&config_string) {
        Ok(config_input) => config_input,
        Err(error) => {
            eprintln!("failed to parse config file: {error}");
            std::process::exit(2);
        }
    };
    let config = match Config::try_from(config_input) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("configuration error: {error}");
            std::process::exit(2);
        }
    };

    let expected_values: std::collections::HashMap<GameState, Ratio<BigUint>> =
        match std::fs::read(&config.expected_values_path) {
            Ok(bytes) => match postcard::from_bytes(&bytes) {
                Ok(map) => map,
                Err(error) => {
                    eprintln!(
                        "failed to parse `{}`: {}",
                        config.expected_values_path.display(),
                        error
                    );
                    std::process::exit(3);
                }
            },
            Err(error) => {
                eprintln!(
                    "failed to read `{}`: {}",
                    config.expected_values_path.display(),
                    error
                );
                std::process::exit(3);
            }
        };

    let expected_values_static = EXPECTED_VALUES.pin();
    for (state, value) in expected_values {
        expected_values_static.insert(state, value);
    }

    let app = Router::new().route("/", get(index));

    match config.socket {
        Socket::Tcp(addr, port) => {
            let listener = match TcpListener::bind((addr, port)).await {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("failed to bind TCP listener: {error}");
                    std::process::exit(4);
                }
            };
            match axum::serve(listener, app).await {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("server failed: {error}");
                }
            }
        }
        Socket::Unix(path) => {
            let listener = match UnixListener::bind(path) {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("failed to create Unix socket: {error}");
                    std::process::exit(4);
                }
            };
            match axum::serve(listener, app).await {
                Ok(()) => {}
                Err(error) => {
                    eprintln!("server failed: {error}");
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, thiserror::Error)]
enum ParseIndexQueryStringError {
    #[error("duplicate parameter `{0}`")]
    DuplicateParameter(String),
    #[error("invalid query string")]
    InvalidQueryString,
    #[error("invalid value for parameter `{0}`")]
    InvalidValue(String),
    #[error("missing value for parameter `{0}`")]
    MissingValue(String),
    #[error("unknown parameter `{0}`")]
    UnknownParameter(String),
}

fn parse_index_query_string(query: &str) -> Result<Game, Vec<ParseIndexQueryStringError>> {
    let query = match PctStr::new(query) {
        Ok(query) => query.decode(),
        Err(_) => {
            return Err(vec![ParseIndexQueryStringError::InvalidQueryString]);
        }
    };
    let keys = [
        "dice",
        "rerolls_left",
        "ones",
        "twos",
        "threes",
        "fours",
        "fives",
        "sixes",
        "one_pair",
        "two_pairs",
        "three_of_a_kind",
        "four_of_a_kind",
        "small_straight",
        "large_straight",
        "full_house",
        "chance",
        "yatzy",
    ];

    let mut dice = None;
    let mut rerolls_left = None;
    let mut ones = None;
    let mut twos = None;
    let mut threes = None;
    let mut fours = None;
    let mut fives = None;
    let mut sixes = None;
    let mut one_pair = None;
    let mut two_pairs = None;
    let mut three_of_a_kind = None;
    let mut four_of_a_kind = None;
    let mut small_straight = None;
    let mut large_straight = None;
    let mut full_house = None;
    let mut chance = None;
    let mut yatzy = None;

    let mut errors = Vec::new();

    for key_value in query.split('&') {
        let Some((key, value)) = key_value.split_once('=') else {
            if keys.contains(&key_value) {
                errors.push(ParseIndexQueryStringError::MissingValue(String::from(
                    key_value,
                )));
            } else {
                if key_value != "" || query != "" {
                    errors.push(ParseIndexQueryStringError::UnknownParameter(String::from(
                        key_value,
                    )));
                }
            }
            continue;
        };
        match key {
            "dice" => {
                if dice.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                if value.is_empty() {
                    errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                    continue;
                }
                if !DICE_REGEX.is_match(value) {
                    dice = Some(Err(()));
                    errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                    continue;
                }
                dice = Some(Ok(value
                    .split(',')
                    .map(|die_str| match die_str {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .expect("expected five dice")));
            }
            "rerolls_left" => {
                if rerolls_left.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                rerolls_left = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "0" => Some(Ok(0)),
                    "1" => Some(Ok(1)),
                    "2" => Some(Ok(2)),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "ones" => {
                if ones.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                ones = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "1" => Some(Ok(Some(1))),
                    "2" => Some(Ok(Some(2))),
                    "3" => Some(Ok(Some(3))),
                    "4" => Some(Ok(Some(4))),
                    "5" => Some(Ok(Some(5))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "twos" => {
                if twos.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                twos = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "2" => Some(Ok(Some(2))),
                    "4" => Some(Ok(Some(4))),
                    "6" => Some(Ok(Some(6))),
                    "8" => Some(Ok(Some(8))),
                    "10" => Some(Ok(Some(10))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "threes" => {
                if threes.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                threes = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "3" => Some(Ok(Some(3))),
                    "6" => Some(Ok(Some(6))),
                    "9" => Some(Ok(Some(9))),
                    "12" => Some(Ok(Some(12))),
                    "15" => Some(Ok(Some(15))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "fours" => {
                if fours.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                fours = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "4" => Some(Ok(Some(4))),
                    "8" => Some(Ok(Some(8))),
                    "12" => Some(Ok(Some(12))),
                    "16" => Some(Ok(Some(16))),
                    "20" => Some(Ok(Some(20))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "fives" => {
                if fives.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                fives = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "5" => Some(Ok(Some(5))),
                    "10" => Some(Ok(Some(10))),
                    "15" => Some(Ok(Some(15))),
                    "20" => Some(Ok(Some(20))),
                    "25" => Some(Ok(Some(25))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "sixes" => {
                if sixes.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                sixes = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "6" => Some(Ok(Some(6))),
                    "12" => Some(Ok(Some(12))),
                    "18" => Some(Ok(Some(18))),
                    "24" => Some(Ok(Some(24))),
                    "30" => Some(Ok(Some(30))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "one_pair" => {
                if one_pair.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                one_pair = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "2" => Some(Ok(Some(2))),
                    "4" => Some(Ok(Some(4))),
                    "6" => Some(Ok(Some(6))),
                    "8" => Some(Ok(Some(8))),
                    "10" => Some(Ok(Some(10))),
                    "12" => Some(Ok(Some(12))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "two_pairs" => {
                if two_pairs.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                two_pairs = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "6" => Some(Ok(Some(6))),
                    "8" => Some(Ok(Some(8))),
                    "10" => Some(Ok(Some(10))),
                    "12" => Some(Ok(Some(12))),
                    "14" => Some(Ok(Some(14))),
                    "16" => Some(Ok(Some(16))),
                    "18" => Some(Ok(Some(18))),
                    "20" => Some(Ok(Some(20))),
                    "22" => Some(Ok(Some(22))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "three_of_a_kind" => {
                if three_of_a_kind.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                three_of_a_kind = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "3" => Some(Ok(Some(3))),
                    "6" => Some(Ok(Some(6))),
                    "9" => Some(Ok(Some(9))),
                    "12" => Some(Ok(Some(12))),
                    "15" => Some(Ok(Some(15))),
                    "18" => Some(Ok(Some(18))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "four_of_a_kind" => {
                if four_of_a_kind.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                four_of_a_kind = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "4" => Some(Ok(Some(4))),
                    "8" => Some(Ok(Some(8))),
                    "12" => Some(Ok(Some(12))),
                    "16" => Some(Ok(Some(16))),
                    "20" => Some(Ok(Some(20))),
                    "24" => Some(Ok(Some(24))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "small_straight" => {
                if small_straight.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                small_straight = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "15" => Some(Ok(Some(15))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "large_straight" => {
                if large_straight.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                large_straight = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "20" => Some(Ok(Some(20))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "full_house" => {
                if full_house.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                full_house = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "7" => Some(Ok(Some(7))),
                    "8" => Some(Ok(Some(8))),
                    "9" => Some(Ok(Some(9))),
                    "11" => Some(Ok(Some(11))),
                    "12" => Some(Ok(Some(12))),
                    "13" => Some(Ok(Some(13))),
                    "14" => Some(Ok(Some(14))),
                    "15" => Some(Ok(Some(15))),
                    "16" => Some(Ok(Some(16))),
                    "17" => Some(Ok(Some(17))),
                    "18" => Some(Ok(Some(18))),
                    "19" => Some(Ok(Some(19))),
                    "20" => Some(Ok(Some(20))),
                    "21" => Some(Ok(Some(21))),
                    "22" => Some(Ok(Some(22))),
                    "23" => Some(Ok(Some(23))),
                    "24" => Some(Ok(Some(24))),
                    "26" => Some(Ok(Some(26))),
                    "27" => Some(Ok(Some(27))),
                    "28" => Some(Ok(Some(28))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "chance" => {
                if chance.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                chance = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "5" => Some(Ok(Some(5))),
                    "6" => Some(Ok(Some(6))),
                    "7" => Some(Ok(Some(7))),
                    "8" => Some(Ok(Some(8))),
                    "9" => Some(Ok(Some(9))),
                    "10" => Some(Ok(Some(10))),
                    "11" => Some(Ok(Some(11))),
                    "12" => Some(Ok(Some(12))),
                    "13" => Some(Ok(Some(13))),
                    "14" => Some(Ok(Some(14))),
                    "15" => Some(Ok(Some(15))),
                    "16" => Some(Ok(Some(16))),
                    "17" => Some(Ok(Some(17))),
                    "18" => Some(Ok(Some(18))),
                    "19" => Some(Ok(Some(19))),
                    "20" => Some(Ok(Some(20))),
                    "21" => Some(Ok(Some(21))),
                    "22" => Some(Ok(Some(22))),
                    "23" => Some(Ok(Some(23))),
                    "24" => Some(Ok(Some(24))),
                    "25" => Some(Ok(Some(25))),
                    "26" => Some(Ok(Some(26))),
                    "27" => Some(Ok(Some(27))),
                    "28" => Some(Ok(Some(28))),
                    "29" => Some(Ok(Some(29))),
                    "30" => Some(Ok(Some(30))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            "yatzy" => {
                if yatzy.is_some() {
                    errors.push(ParseIndexQueryStringError::DuplicateParameter(
                        String::from(key),
                    ));
                    continue;
                }
                yatzy = match value {
                    "" => {
                        errors.push(ParseIndexQueryStringError::MissingValue(String::from(key)));
                        continue;
                    }
                    "empty" => Some(Ok(None)),
                    "0" => Some(Ok(Some(0))),
                    "50" => Some(Ok(Some(50))),
                    _ => {
                        errors.push(ParseIndexQueryStringError::InvalidValue(String::from(key)));
                        Some(Err(()))
                    }
                };
            }
            key => {
                errors.push(ParseIndexQueryStringError::UnknownParameter(String::from(
                    key,
                )));
            }
        }
    }

    if dice.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "dice",
        )));
    }
    if rerolls_left.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "rerolls_left",
        )));
    }
    if ones.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "ones",
        )));
    }
    if twos.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "twos",
        )));
    }
    if threes.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "threes",
        )));
    }
    if fours.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "fours",
        )));
    }
    if fives.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "fives",
        )));
    }
    if sixes.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "sixes",
        )));
    }
    if one_pair.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "one_pair",
        )));
    }
    if two_pairs.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "two_pairs",
        )));
    }
    if three_of_a_kind.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "three_of_a_kind",
        )));
    }
    if four_of_a_kind.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "four_of_a_kind",
        )));
    }
    if small_straight.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "small_straight",
        )));
    }
    if large_straight.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "large_straight",
        )));
    }
    if full_house.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "full_house",
        )));
    }
    if chance.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "chance",
        )));
    }
    if yatzy.is_none() {
        errors.push(ParseIndexQueryStringError::MissingValue(String::from(
            "yatzy",
        )));
    }

    if !errors.is_empty() {
        let mut unique_errors = Vec::new();
        for error in errors {
            if !unique_errors.contains(&error) {
                unique_errors.push(error);
            }
        }
        return Err(unique_errors);
    }

    let game = Game::new(GameOptions {
        dice: dice.expect("missing `dice`").expect("invalid `dice`"),
        rerolls_left: rerolls_left
            .expect("missing `rerolls_left`")
            .expect("invalid `rerolls_left`"),
        ones: ones
            .expect("missing combo `ones`")
            .expect("invalid combo `ones`"),
        twos: twos
            .expect("missing combo `twos`")
            .expect("invalid combo `twos`"),
        threes: threes
            .expect("missing combo `threes`")
            .expect("invalid combo `threes`"),
        fours: fours
            .expect("missing combo `fours`")
            .expect("invalid combo `fours`"),
        fives: fives
            .expect("missing combo `fives`")
            .expect("invalid combo `fives`"),
        sixes: sixes
            .expect("missing combo `sixes`")
            .expect("invalid combo `sixes`"),
        one_pair: one_pair
            .expect("missing combo `one_pair`")
            .expect("invalid combo `one_pair`"),
        two_pairs: two_pairs
            .expect("missing combo `two_pairs`")
            .expect("invalid combo `two_pairs`"),
        three_of_a_kind: three_of_a_kind
            .expect("missing combo `three_of_a_kind`")
            .expect("invalid combo `three_of_a_kind`"),
        four_of_a_kind: four_of_a_kind
            .expect("missing combo `four_of_a_kind`")
            .expect("invalid combo `four_of_a_kind`"),
        small_straight: small_straight
            .expect("missing combo `small_straight`")
            .expect("invalid combo `small_straight`"),
        large_straight: large_straight
            .expect("missing combo `large_straight`")
            .expect("invalid combo `large_straight`"),
        full_house: full_house
            .expect("missing combo `full_house`")
            .expect("invalid combo `full_house`"),
        chance: chance
            .expect("missing combo `chance`")
            .expect("invalid combo `chance`"),
        yatzy: yatzy
            .expect("missing combo `yatzy`")
            .expect("invalid combo `yatzy`"),
    })
    .expect("invalid game");
    Ok(game)
}

async fn index(RawQuery(query): RawQuery) -> impl IntoResponse {
    let query = match query {
        Some(query) => query,
        None => String::new(),
    };
    let game = match parse_index_query_string(&query) {
        Ok(game) => game,
        Err(errors) => {
            let mut rv = Map::new();
            rv.insert(
                String::from("errors"),
                Value::Array(
                    errors
                        .into_iter()
                        .map(|error| error.to_string().into())
                        .collect(),
                ),
            );
            return (
                AppendHeaders([(ACCESS_CONTROL_ALLOW_ORIGIN, "*")]),
                Json(rv.into()),
            );
        }
    };
    if game.ended() {
        return (
            AppendHeaders([(ACCESS_CONTROL_ALLOW_ORIGIN, "*")]),
            Json(json!({ "errors": ["game has ended"] })),
        );
    }

    let cache = papaya::HashMap::with_hasher(FxBuildHasher);
    let (choices, _) = match game.rerolls_left() {
        0 => best_choice_0_rerolls::<_, FxBuildHasher, _, Ratio<BigUint>>(
            game,
            &EXPECTED_VALUES,
            &cache,
        ),
        1 => best_choice_1_reroll::<_, FxBuildHasher, _, Ratio<BigUint>>(
            game,
            &EXPECTED_VALUES,
            &cache,
        ),
        2 => best_choice_2_rerolls::<_, FxBuildHasher, _, Ratio<BigUint>>(
            game,
            &EXPECTED_VALUES,
            &cache,
        ),
        _ => unreachable!(),
    };

    let choices_json = choices
        .into_iter()
        .map(|choice| match choice {
            Choice::SelectCombo(combo) => {
                let combo_str = match combo {
                    Combo::Ones => "ones",
                    Combo::Twos => "twos",
                    Combo::Threes => "threes",
                    Combo::Fours => "fours",
                    Combo::Fives => "fives",
                    Combo::Sixes => "sixes",
                    Combo::OnePair => "one_pair",
                    Combo::TwoPairs => "two_pairs",
                    Combo::ThreeOfAKind => "three_of_a_kind",
                    Combo::FourOfAKind => "four_of_a_kind",
                    Combo::SmallStraight => "small_straight",
                    Combo::LargeStraight => "large_straight",
                    Combo::FullHouse => "full_house",
                    Combo::Chance => "chance",
                    Combo::Yatzy => "yatzy",
                };
                json!({
                    "choice": "select_combo",
                    "combo": combo_str,
                })
            }
            Choice::Reroll1(dice) => json!({
                "choice": "reroll",
                "dice": dice,
            }),
            Choice::Reroll2(dice) => json!({
                "choice": "reroll",
                "dice": dice,
            }),
            Choice::Reroll3(dice) => json!({
                "choice": "reroll",
                "dice": dice,
            }),
            Choice::Reroll4(dice) => json!({
                "choice": "reroll",
                "dice": dice,
            }),
            Choice::Reroll5(dice) => json!({
                "choice": "reroll",
                "dice": dice,
            }),
        })
        .collect::<Vec<_>>();

    (
        AppendHeaders([(ACCESS_CONTROL_ALLOW_ORIGIN, "*")]),
        Json(json!(choices_json)),
    )
}
