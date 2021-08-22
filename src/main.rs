use std::{
    fs::File,
    path::PathBuf,
    net::UdpSocket,
};
use structopt::StructOpt;
use hex;
use std::io::{BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "StructOpt usage.")]
struct Cli {
    #[structopt(parse(from_os_str))]
    file_path: PathBuf,

    #[structopt(short = "a")]
    send_addr: String,

    #[structopt(short = "v", default_value = "0.1")]
    sleep: f64,

    #[structopt(short = "t", default_value = "udp")]
    skt_type: String,


    #[structopt(short = "b", default_value = "0.0.0.0:9314")]
    bind_addr: String,

    #[structopt(default_value = "none")]
    data_type: String,

    #[structopt(short = "c", long = "cycle")]
    cycle: bool,

    #[structopt(short = "fm", default_value = "vr")]
    file_mode: String,
}

fn create_skt(bind_addr: &String, skt_type: String, send_addr: &String) -> UdpSocket {
    let socket = match UdpSocket::bind(&bind_addr) {
        Ok(skt) => {
            skt
        }
        Err(e) => {
            panic!("udp serve 启动失败! {}", e);
        }
    };
    if skt_type == String::from("udp") {
        socket.connect(&send_addr).unwrap();
    }
    socket
}

fn read_file(file_path: &PathBuf) -> BufReader<File> {
    let file_result = File::open(&file_path).unwrap();
    let reader = BufReader::new(file_result);
    reader
}


fn send_line(skt: &UdpSocket, line: &String, send_addr: &String, skt_type: &String, file_mode: &String) {
    let mut buffer: Vec<u8> = vec![];
    if *file_mode == String::from("vr") {
        let res_line: Vec<&str> = line.split(" ").collect();
        let b_len = &res_line[5].len();
        println!("落盘日期: {}-{} ---- 字节长度： {} ---- 码子： {}", &res_line[0], &res_line[1], &b_len, &res_line[5]);
        buffer =  hex::decode(&res_line[5]).unwrap();
    } else {
        buffer =  hex::decode(line).unwrap();
    }

    if *skt_type == String::from("udp") {
        &skt.send(&buffer);
    } else if *skt_type == String::from("mut") {
        &skt.send_to(&buffer, &send_addr);
    } else {
        &skt.send(&buffer);
    }
}

fn main() {
    let opt: Cli = Cli::from_args();
    // println!("{:?}", opt);
    let socket = create_skt(&opt.bind_addr, (*opt.skt_type).parse().unwrap(), &opt.send_addr);
    loop {
        let readers = read_file(&opt.file_path);
        for line in readers.lines() {
            let s_line = line.unwrap();
            send_line(&socket, &s_line, &opt.send_addr, &opt.skt_type, &opt.file_mode);
            // sleep(Duration::from_secs_f64(opt.sleep));
        }
        if !&opt.cycle {
            break;
        }
    }
    println!("+=============回放结束=============+");
}
