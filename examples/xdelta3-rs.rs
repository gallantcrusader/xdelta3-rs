use async_std::fs::File;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "i")]
    input: String,
    #[structopt(short = "s")]
    source: String,
    #[structopt(short = "o")]
    output: String,
}

async fn run(opt: Opt) {
    let input = File::open(&opt.input).await.expect("File::open");
    let source = File::open(&opt.source).await.expect("File::open");
    let out = File::create(&opt.output).await.expect("File::create");

    xdelta3::decode_async(input, source, out).await;
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    println!("hello, world!, opt={:?}", opt);

    async_std::task::block_on(run(opt));
}