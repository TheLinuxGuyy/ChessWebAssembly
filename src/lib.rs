use wqsm_bindgen::prelude::*;
use tungstenite::{connect, Message};
use url::Url;
use serde_json;

fn listening(){
    // Loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        
        println!("{:?}", parsed["result"]);
    }
}


#[wasm_bindgen]
pub fn connect(lobby_number) {
    let (mut socket, _response) = connect(Url::parse("ws://localhost:8765/{}",lobby_number).unwrap()).expect("Can't connect");
    listening(&socket);
}

fn checking_legality(p:String,f:String,t:String,toccupied:bool){ //toccupied should be true if it is an opposit color to the color that is going into the square 
    alphabet = ["a","b","c","d","e","f","g","h"]
    let number_from: i32 = f[1].parse().unwrap();
    let number_to: i32 = t[1].parse().unwrap();
    let letter_from: String = f[0].parse().unwrap();
    let letter_to: String = t[0].parse.unwrap();
    let letter_to_index = alphabet.iter().position(|&r| r == letter_to).unwrap();
    let letter_from_index = alphabet.iter().position(|&r| r == letter_from).unwrap();
    let result=match p{
        "pawn" => {
            let factory_positions = vec!["a7","b7","c7","d7","e7","f7","g7","h7","a2","b2","c2","d2","e2","f2","g2","h2"];
            if(f in factory_positions){
                if(t==f.replace(number_to.to_string(),number_to+1.to_string()) || f.replace(number_to.to_string(),number_to+2.to_string()){
                    "legal" //going forward twice or once
                }else{
                    "illegal" // if you dont go forward once or twice at the start of the round, it is illegal
                }
            }else{
                if(number_from>number_to){ //going backwards
                    "illegal"
                }

                if(letter_to != letter_from){
                    if(toccupied && number_from==number_to+1 && letter_from_index==letter_to_index+1){
                        "legal" // if the pawn changes lane, it has to have an enemy in the other lane
                    }
                    else{
                        "illegal"
                    }
                }
                if(t==f[1].replace(number_to.to_string(),number_to+1.to_string())
            }
        }
        "knight" =>{
            let factory_positions = vec!["g1","b1","b8","g8"]
        }
        "rook" => checking_legality("rook"),
        "bishop" => checking_legality("bishop"),
        "king" => checking_legality("king"),
        "queen" => checking_legality("queen"),
        _ => "not a vaild peice",
    }
}

#[wasm_bindgen]
pub fn send(lobby_number:i32,frompos:String,topos: String,peice:String,toccupied:bool){
    result=checking_legality(peice,frompos,topos,toccupied);
    socket.write_message(Message::Text(result.into())).unwrap();

}