use net2::TcpStreamExt;

use pallas::network::{
    miniprotocols::{
        handshake::{n2n::VersionTable, Initiator},
        run_agent, Point, MAINNET_MAGIC,
    },
    multiplexer::Multiplexer,
};

use pallas::network::miniprotocols::blockfetch::{BatchClient, Observer};

use std::net::TcpStream;

#[derive(Debug)]
struct BlockPrinter;

impl Observer for BlockPrinter {
    fn on_block_received(&mut self, body: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        println!("{}", hex::encode(&body));
        println!("----------");
        Ok(())
    }
}

fn main() {
    env_logger::init();

    let bearer = TcpStream::connect("relays-new.cardano-mainnet.iohk.io:3001").unwrap();
    bearer.set_nodelay(true).unwrap();
    bearer.set_keepalive_ms(Some(30_000u32)).unwrap();

    let mut muxer = Multiplexer::setup(bearer, &[0, 3]).unwrap();

    let mut hs_channel = muxer.use_channel(0);
    let versions = VersionTable::v4_and_above(MAINNET_MAGIC);
    let _last = run_agent(Initiator::initial(versions), &mut hs_channel).unwrap();

    let range = (
        Point::Specific(
            97,
            hex::decode("cf7fa60bbd210273d79fa48d11ab1d141242af32b231cc40ce3411230a8d3c61")
                .unwrap(),
        ),
        Point::Specific(
            99,
            hex::decode("a52cca923a67326ea9c409e958a17a77990be72f3607625ec5b3d456202e223e")
                .unwrap(),
        ),
    );

    let mut bf_channel = muxer.use_channel(3);
    let bf = BatchClient::initial(range, BlockPrinter {});
    let bf_last = run_agent(bf, &mut bf_channel);
    println!("{:?}", bf_last);
}
