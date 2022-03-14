use std::{
        io::{Read, Write},
        net::{TcpListener, TcpStream},
    };

    use crate::tabla::{self, Culoare, TipPiesa};

    fn print_tabla(tabla: &tabla::Tabla) {
        println!("  A B C D E F G H");
        for i in 0..8 {
            print!("{} ", 8 - i);
            for j in 0..8 {
                if let Some(piesa) = &tabla[i][j].piesa {
                    let tip = match piesa.tip {
                        TipPiesa::Pion => "p",
                        TipPiesa::Tura => "r",
                        TipPiesa::Cal => "n",
                        TipPiesa::Nebun => "b",
                        TipPiesa::Regina => "q",
                        TipPiesa::Rege => "k",
                    };
                    if piesa.culoare == Culoare::Alb {
                        print!("{} ", tip.to_uppercase());
                    } else {
                        print!("{} ", tip);
                    }
                } else {
                    print!(". ");
                }
            }
            println!("{}", 8 - i);
        }
        println!("  A B C D E F G H");
    }

    pub(crate) fn game_no_gui() {
        let mut tabla = tabla::generare::tabla_clasica();
        let mut turn = Culoare::Alb;

        //let istoric = &mut Vec::new();

        print_tabla(&tabla);
        loop {
            //your_turn(&mut tabla, &mut turn, match_state, istoric);
        }
    }

    pub(crate) fn game_online() {
        let mut tabla = tabla::generare::tabla_clasica();
        let mut istoric = Vec::new();

        println!("host/join: ");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "host" => {
                let mut turn = Culoare::Alb;
                let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
                println!("listening...");
                match listener.accept() {
                    Ok((mut stream, addr)) => {
                        println!("new client: {:?}", addr);
                        print_tabla(&tabla);
                        loop {
                            your_turn(&mut stream, &mut tabla, &mut turn, &mut istoric);
                            wait_turn(&mut stream, &mut tabla, &mut turn, &mut istoric);
                        }
                    }
                    Err(e) => println!("couldn't get client: {}", e),
                }
            }
            "join" => {
                let mut turn = Culoare::Alb;
                buf = String::new();
                println!("ip to connect: ");
                std::io::stdin().read_line(&mut buf).unwrap();
                let mut stream = TcpStream::connect(buf.trim()).unwrap();
                println!("connected!");
                loop {
                    wait_turn(&mut stream, &mut tabla, &mut turn, &mut istoric);
                    your_turn(&mut stream, &mut tabla, &mut turn, &mut istoric);
                }
            }
            _ => {
                println!("nu e valid");
            }
        }
    }
    fn wait_turn(
        stream: &mut TcpStream,
        tabla: &mut tabla::Tabla,
        turn: &mut Culoare,
        istoric: &mut Vec<String>,
    ) {
        let mut buf = [0; 16];
        let len = stream.read(&mut buf).unwrap();
        let mov = String::from_utf8(buf[0..len].to_vec()).unwrap();
        if let Some((src_poz, dest_poz)) = tabla::istoric::decode_move(tabla, mov.trim(), *turn) {
            tabla::game::muta(tabla, turn, istoric, src_poz, dest_poz);
            print_tabla(&tabla);
        }
    }
    fn your_turn(
        stream: &mut TcpStream,
        tabla: &mut tabla::Tabla,
        turn: &mut Culoare,
        istoric: &mut Vec<String>,
    ) {
        loop {
            let mut buf = String::new();
            std::io::stdin().read_line(&mut buf).unwrap();
            if let Some((src_poz, dest_poz)) = tabla::istoric::decode_move(tabla, buf.trim(), *turn)
            {
                tabla::game::muta(tabla, turn, istoric, src_poz, dest_poz);
                print_tabla(&tabla);
                println!("{:?}", buf.trim());
                stream.write(buf.trim().as_bytes()).unwrap();
                break;
            } else {
                println!("{:?}", tabla);
                println!("nu e valid");
            }
        }
    }