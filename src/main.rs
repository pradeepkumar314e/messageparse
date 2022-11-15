mod parser;

use crate::parser::{Msg, Field, Segment};
use actix_web::{HttpServer, App, web, Responder};




use std::io;

use nom::{
    
    branch::alt,
    bytes::complete::{tag, take_while},
    error::{ErrorKind, ParseError},
    multi::separated_list0,
    IResult,
};

use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;



impl Msg {

    pub fn slen(&self) -> usize { self.segments.len() }

    pub fn msg_type(&self) -> Option<String> {
    
        if let Some(segment) = self.segments.first() {
           if let Some(field) = segment.fields.get(8) {
                // If 3 components, take last (msg structure) //ADT^A01^ADT_A01
                // else take first and second component       //ADT^A01    //firstcomp^secondcomp
                if field.components.len() == 3 && !field.components.last().unwrap().is_empty()
                {
                    return Some(field.components.last().unwrap().to_string());


                } else {

                    //ADT^A01
                    let mut msg_type = "".to_owned();
                    
                    if let Some(component) = field.components.first() {
                        // TODO: Check if it is valid type
                        msg_type.push_str(&component);
                        msg_type.push('_');
                    
                        if let Some(component) = field.components.get(1) {
                            msg_type.push_str(&component);
                            return Some(msg_type);
                        }
                    }
                }
            }
        }
       None
     }

    pub fn version(&self) -> Option<String> {


        if let Some(segment) = self.segments.first() {
            
            
            if let Some(field) = segment.fields.get(11) {
            
            
                if let Some(component) = field.components.first() {
                        
                        let mut version = "V".to_owned();
                        version.push_str(component.as_str().replace(".", "_").as_str());
                        return Some(version);
            
            
                    
                }
            }
        }
        None
    }

}




fn is_not_cs_fs_or_line_end(i: char) -> bool { i != '^' && i != '|' && i != '\n' && i != '\r' }

fn parse_component(i: Span) -> IResult<Span, Span> { 
    
     take_while(is_not_cs_fs_or_line_end)(i) 
}


fn parse_field(i: Span) -> IResult<Span, Field> {
  


    separated_list0(    tag("^")   ,   parse_component)(i).map(|(i, components)| {
        (i, Field {

            components: components
                .iter()
                .map(|&s| {
                    s.fragment().to_string()
                    
                })
                .collect(),
        })
    })
}


fn parse_segment(i: Span) -> IResult<Span, Segment> {
    separated_list0(tag("|"), parse_field)(i).map(|(i, fields)| {
        (i, Segment {
            fields: fields
                .into_iter()
                .map(|field| {
                       field
                })
                .collect(),
        })
    })
}


pub fn parse_msg(i: Span) -> IResult<Span, Msg> {
    
    let separator = match i.chars().nth(3) {
        Some(c) => {
            if c != '|' {
                return Err(nom::Err::Error(ParseError::from_error_kind(
                    i,
                    ErrorKind::Char,
                )));
            }
            c
        },
        None => {
                return Err(nom::Err::Error(ParseError::from_error_kind(
                i,
                ErrorKind::Eof,
            )));
        },
    };
    separated_list0(alt((tag("\n"), tag("\r"))), parse_segment)(i).map(|(i, segments)| {
        (i, Msg {
            segments: segments
                .into_iter()
                .map(|segment| {
                    
                        segment
                })
                .collect(),
            separator,
        })
    })
}


async fn status()-> impl Responder{
    let input = "MSH|^~\\&|AccMgr|1|||20050110045504||ADT^A08|599102|P|2.2|||\nEVN|A01|20050110045502|||||\nPID|1||10006579^^^1^MRN^1||DUCK^DONALD^D||19241010|M||1|111 DUCK ST^^FOWL^CA^999990000^^M|1|8885551212|8885551212|1|2||40007716^^^AccMgr^VN^1|123121234|||||||||||NO\nNK1|1|DUCK^HUEY|SO|3583 DUCK RD^^FOWL^CA^999990000|8885552222||Y||||||||||||||\nPV1|1|I|PREOP^101^1^1^^^S|3|||37^DISNEY^WALT^^^^^^AccMgr^^^^CI|||01||||1|||37^DISNEY^WALT^^^^^^AccMgr^^^^CI|2|40007716^^^AccMgr^VN|4|||||||||||||||||||1||G|||20050110045253||||||\nGT1|1|8291|DUCK^DONALD^D||111^DUCK ST^^FOWL^CA^999990000|8885551212||19241010|M||1|123121234||||#Cartoon Ducks Inc|111^DUCK ST^^FOWL^CA^999990000|8885551212||PT|\nDG1|1|I9|71596^OSTEOARTHROS NOS-L/LEG ^I9|OSTEOARTHROS NOS-L/LEG ||A|\nIN1|1|MEDICARE|3|MEDICARE|||||||Cartoon Ducks Inc|19891001|||4|DUCK^DONALD^D|1|19241010|111^DUCK ST^^FOWL^CA^999990000|||||||||||||||||123121234A||||||PT|M|111 DUCK ST^^FOWL^CA^999990000|||||8291\nIN2|1||123121234|Cartoon Ducks Inc|||123121234A|||||||||||||||||||||||||||||||||||||||||||||||||||||||||8885551212\nIN1|2|NON-PRIMARY|9|MEDICAL MUTUAL CALIF.|PO BOX 94776^^HOLLYWOOD^CA^441414776||8003621279|PUBSUMB|||Cartoon Ducks Inc||||7|DUCK^DONALD^D|1|19241010|111 DUCK ST^^FOWL^CA^999990000|||||||||||||||||056269770||||||PT|M|111^DUCK ST^^FOWL^CA^999990000|||||8291\nIN2|2||123121234|Cartoon Ducks Inc||||||||||||||||||||||||||||||||||||||||||||||||||||||||||||8885551212\nIN1|3|SELF PAY|1|SELF PAY|||||||||||5||1";
    
    let (_ , msg) = parse_msg(Span::new(input)).unwrap();
    let version = msg.version().unwrap();
    let msgtype = msg.msg_type().unwrap(); 
    
    println!("version : {}",version);
    println!("msgtype : {}",msgtype);
    let jsoon = serde_json::to_string(&msg).unwrap();
    jsoon

}

#[actix_rt::main]
async fn main() -> Result<(), io::Error>{

//   status();
    println!("http://127.0.0.1:8080");
    HttpServer::new(|| {
       App::new()
          .route("/", web::get().to(status))
   })
   .bind("127.0.0.1:8080")?
   .run()
   .await

}


