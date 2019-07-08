use cgmath::*;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
mod macros;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum Platform {
    Pikachu,
    Domo,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pose {
    orientation: Quaternion<f32>,
    position: Vector3<f32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Robot {
    platform: Platform,
    pose: Pose,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum Agent {
    Leader,
    Id(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swarm {
    max_agents: u8,
    robots: HashMap<Agent, Robot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    mayor: u8,
    minor: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
    ue4_engine: Version,
    nvidia_physx: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Global {
    shield: Swarm,
    engine: Engine,
    build: DateTime<Utc>,
}

fn main() {
    let global = Global {
        shield: Swarm {
            max_agents: 5,
            robots: hashmap![
                Agent::Leader => Robot {
                    platform: Platform::Domo,
                    pose: Pose {
                        orientation: Quaternion::zero(),
                        position: Vector3::zero(),
                    }
                },
                Agent::Id(0) => Robot {
                    platform: Platform::Pikachu,
                    pose: Pose {
                        orientation: Quaternion::zero(),
                        position: Vector3::zero(),
                    }
                },
                Agent::Id(1) => Robot {
                    platform: Platform::Pikachu,
                    pose: Pose {
                        orientation: Quaternion::zero(),
                        position: Vector3::zero(),
                    }
                },
            ],
        },
        engine: Engine {
            ue4_engine: Version {
                mayor: 4,
                minor: 21,
            },
            nvidia_physx: Version { mayor: 4, minor: 0 },
        },
        build: Utc::now(),
    };
    let yaml = serde_yaml::to_string(&global).unwrap();
    fs::write("config.yaml", &yaml).unwrap();
    let com: Global = serde_yaml::from_str(&yaml).unwrap();

    let _leader = com.shield.robots[&Agent::Leader];
    let _agents = com.shield.robots.iter()
        .filter(|(&id, _)| id != Agent::Leader);
    let pikachus = com.shield.robots.iter()
        .filter(|(_, &robot)| robot.platform == Platform::Pikachu);

    println!("{:#?}", pikachus.collect::<Vec<(&Agent, &Robot)>>());
}
