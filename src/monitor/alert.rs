use crate::{database::DB, email};

pub fn alert(threshold: i32, condition: bool, subject: &str, text: &str) {
    let counted: bool = DB::new().counter(condition, threshold, "pier:alert", subject);
    if counted {
        // println!("{:#?} {:#?}", subject, text);
        email::send(subject, text).expect("[email] send");
    }
}
