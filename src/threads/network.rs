extern crate noise;
use self::noise::{NoiseModule, Perlin};

use std::sync::mpsc::{Sender, Receiver};
use ::core::messages::client::{ToMeshing, ToNetwork};
use ::block::BlockId;
use ::CHUNK_SIZE;

pub fn start(rx: Receiver<ToNetwork>, meshing_tx: Sender<ToMeshing>) {
    let perlin = Perlin::new();
    for message in rx {
        match message {
            ToNetwork::NewChunk(pos) => {
                let mut chunk = [[[BlockId::from(0); CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
                for i in 0..CHUNK_SIZE {
                    for j in 0..CHUNK_SIZE {
                        let height = (150.0*perlin.get([
                            0.005*(0.0021 + (CHUNK_SIZE as i64 * pos.0 + i as i64) as f64/3.0),
                            0.5,
                            0.005*(0.0021 + (CHUNK_SIZE as i64 * pos.2 + j as i64) as f64/3.0)])) as i64;
                        for k in 0..CHUNK_SIZE {
                            if (pos.1*CHUNK_SIZE as i64 + k as i64) < height {
                                chunk[i][k][j] = BlockId::from(1);
                            }
                            else if (pos.1*CHUNK_SIZE as i64 + k as i64) == height {
                                chunk[i][k][j] = BlockId::from(2);
                            }
                        }
                    }
                }
                println!("Network: processed chunk @ {:?}", pos);
                meshing_tx.send(ToMeshing::NewChunk(pos, Box::new(chunk))).unwrap();
            }
        }
    }
}