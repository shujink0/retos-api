use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SendGridBody<'a> {
    pub subject: String,
    pub from: SendGridFrom<'a>,
    pub content: Vec<SendGridContent<'a>>,
    pub personalizations: Vec<SendGridPersonalization>,
}

#[derive(Serialize, Debug)]
pub struct SendGridPersonalization {
    pub to: Vec<SendGridTo>,
}

#[derive(Serialize, Debug)]
pub struct SendGridTo {
    pub email: String,
}

#[derive(Serialize, Debug)]
pub struct SendGridContent<'a> {
    #[serde(rename = "type")]
    pub _type: &'a str,
    pub value: String,
}

#[derive(Serialize, Debug)]
pub struct SendGridFrom<'a> {
    pub email: &'a str,
}
