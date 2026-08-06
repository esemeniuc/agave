use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientId {
    SolanaLabs,
    JitoLabs,
    Frankendancer,
    Agave,
    AgavePaladin,
    Firedancer,
    AgaveBam,
    Sig,
    Bam,
    // If new variants are added, update From<u16> and TryFrom<ClientId>.
    Unknown(u16),
}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::SolanaLabs => write!(f, "SolanaLabs"),
            Self::JitoLabs => write!(f, "JitoLabs"),
            Self::Frankendancer => write!(f, "Frankendancer"),
            Self::Agave => write!(f, "Agave"),
            Self::AgavePaladin => write!(f, "AgavePaladin"),
            Self::Firedancer => write!(f, "Firedancer"),
            Self::AgaveBam => write!(f, "AgaveBam"),
            Self::Sig => write!(f, "Sig"),
            Self::Bam => write!(f, "Bam"),
            Self::Unknown(id) => write!(f, "Unknown({id})"),
        }
    }
}

impl From<u16> for ClientId {
    fn from(client: u16) -> Self {
        match client {
            0u16 => Self::SolanaLabs,
            1u16 => Self::JitoLabs,
            2u16 => Self::Frankendancer,
            3u16 => Self::Agave,
            4u16 => Self::AgavePaladin,
            5u16 => Self::Firedancer,
            6u16 => Self::AgaveBam,
            7u16 => Self::Sig,
            12u16 => Self::Bam,
            _ => Self::Unknown(client),
        }
    }
}

impl TryFrom<ClientId> for u16 {
    type Error = String;

    fn try_from(client: ClientId) -> Result<Self, Self::Error> {
        match client {
            ClientId::SolanaLabs => Ok(0u16),
            ClientId::JitoLabs => Ok(1u16),
            ClientId::Frankendancer => Ok(2u16),
            ClientId::Agave => Ok(3u16),
            ClientId::AgavePaladin => Ok(4u16),
            ClientId::Firedancer => Ok(5u16),
            ClientId::AgaveBam => Ok(6u16),
            ClientId::Sig => Ok(7u16),
            ClientId::Bam => Ok(12u16),
            ClientId::Unknown(client) if ClientId::is_known(client) => {
                Err(format!("Invalid client: {client}"))
            }
            ClientId::Unknown(client) => Ok(client),
        }
    }
}

impl ClientId {
    fn is_known(client: u16) -> bool {
        !matches!(Self::from(client), Self::Unknown(_))
    }

    pub const fn this_client() -> Self {
        // FIREDANCER: Report client as Firedancer to gossip
        Self::Frankendancer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_client_id() {
        let known_clients = [
            (0u16, ClientId::SolanaLabs),
            (1u16, ClientId::JitoLabs),
            (2u16, ClientId::Frankendancer),
            (3u16, ClientId::Agave),
            (4u16, ClientId::AgavePaladin),
            (5u16, ClientId::Firedancer),
            (6u16, ClientId::AgaveBam),
            (7u16, ClientId::Sig),
            (12u16, ClientId::Bam),
        ];

        for (client, client_id) in known_clients.iter().cloned() {
            assert_eq!(ClientId::from(client), client_id.clone());
            assert_eq!(u16::try_from(client_id), Ok(client));
            assert_eq!(
                u16::try_from(ClientId::Unknown(client)),
                Err(format!("Invalid client: {client}"))
            );
        }

        for client in 0u16..=u16::MAX {
            if ClientId::is_known(client) {
                continue;
            }
            assert_eq!(ClientId::from(client), ClientId::Unknown(client));
            assert_eq!(u16::try_from(ClientId::Unknown(client)), Ok(client));
        }
    }

    #[test]
    fn test_fmt() {
        assert_eq!(format!("{}", ClientId::SolanaLabs), "SolanaLabs");
        assert_eq!(format!("{}", ClientId::JitoLabs), "JitoLabs");
        assert_eq!(format!("{}", ClientId::Frankendancer), "Frankendancer");
        assert_eq!(format!("{}", ClientId::Agave), "Agave");
        assert_eq!(format!("{}", ClientId::AgavePaladin), "AgavePaladin");
        assert_eq!(format!("{}", ClientId::Firedancer), "Firedancer");
        assert_eq!(format!("{}", ClientId::AgaveBam), "AgaveBam");
        assert_eq!(format!("{}", ClientId::Sig), "Sig");
        assert_eq!(format!("{}", ClientId::Bam), "Bam");
        assert_eq!(format!("{}", ClientId::Unknown(0)), "Unknown(0)");
        assert_eq!(format!("{}", ClientId::Unknown(u16::MAX)), "Unknown(65535)");
    }
}
