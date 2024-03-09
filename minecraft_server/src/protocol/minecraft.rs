use crate::server::prelude::{LoginPlayer, LoginServer};

use super::v1_20_4::v1_20_4::{HandShakingPlayer, V1_20_4};

pub struct Minecraft;

super::protocols!(100_000, LoginServer, LoginPlayer, V1_20_4,);
