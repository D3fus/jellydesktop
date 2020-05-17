use uuid::Uuid;
use crate::models::query;
use std::cmp::Ordering;
use tui::layout::{Constraint, Rect};

pub fn format_pw(pw: String) -> String{
  (0..pw.len()).map(|_| "*").collect()
}

pub fn generate_device_id() -> String {
  Uuid::new_v4().to_string()
}

pub fn compere_items(a: &query::BaseItem, b: &query::BaseItem) -> Ordering {
  if a.IndexNumber.is_some() {
    if a.IndexNumber.unwrap() < b.IndexNumber.unwrap(){
      return Ordering::Less;
    }
    if a.IndexNumber.unwrap() > b.IndexNumber.unwrap() {
      return Ordering::Greater;
    }
  }
 return Ordering::Equal;
}

pub fn server_uri_to_name(uri: &String) -> String {
    let mut name = uri.split("://").collect::<Vec<&str>>()[1];
    if name.contains(":") {
        name = name.split(":").collect::<Vec<&str>>()[0];
    }
    name.to_string()
}

pub fn calc_mid(area: Rect, dir: char, size: u16) -> Vec<Constraint> {
    let wide = match dir {
        'y' => area.height as f64,
        'x' => area.width as f64,
        _ => 0.0
    };
    let mid = (size as f64 / wide) * 100.0;
    let side = (wide - size as f64) / 2.0 / wide * 100.0;
    vec![
        Constraint::Percentage(side as u16),
        Constraint::Percentage(mid as u16),
        Constraint::Percentage(side as u16)
    ]
}
