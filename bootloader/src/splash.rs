pub const SPLASH: &'static str =
    "▄              ▄                                                ▄               
               ██▄                                            ▄██               
               █ ▀█▄                                        ▄█▀▐█               
               █    ▀▄                                    ▄▀   ▐█               
               █      ▀█                                █▀     ▐█               
               █  ██▄   ▀█▄                          ▄█▀   ▄█▌ ▐█               
               █  █▌▀█▄   ▀██                      ██▀   ▄█▀▐▌ ▐█               
               █  █▌   ▀▄                              ▄▀   ▐▌ ▐█               
               █ ▐██   ▄▀              ██              ▀▄   ██▌▐█               
               █     █▀                ▐▌                ▀█    ▐█               
               █  ▄█▀                  ▐▌                  ▀█▄ ▐█               
               █▄█▀                    ▐▌                    ▀█▄█               
               █▀                      ▐▌                      ▀█               
                                       ▐▌                                       
                       ▀█▄▄▄▄▄▄▄▄▄▄    ▐▌    ▄▄▄▄▄▄▄▄▄▄█▀                       
                                       ▐▌                                       
                                       ▐▌                                       
          ▄▄                           ▐▌                           ▄▄          
           ▀█▄▄▄▄▄▄▄▄▄▄▄▄▄▄██      ▄▄▄▄▄▄▄▄▄▄      ██▄▄▄▄▄▄▄▄▄▄▄▄▄▄█▀           
                                    ▀██████▀                                    
                ▀▀▀▀▀▀▀▀▀▀▀▀▀▀██      ▀██▀      ██▀▀▀▀▀▀▀▀▀▀▀▀▀▀                
                     ▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▀▀▀▀▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄                     
                                                                                
                             Press any key to start...";

#[allow(dead_code)]
pub fn splash() {
    for c in SPLASH.chars() {
        match c {
            '▀' => print!("{}", 0xdf as char),
            '▐' => print!("{}", 0xde as char),
            '▌' => print!("{}", 0xdd as char),
            '▄' => print!("{}", 0xdc as char),
            '█' => print!("{}", 0xdb as char),
            '\n' => {}
            _ => print!("{}", c),
        }
    }
}
