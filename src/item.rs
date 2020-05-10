use libc::c_int;
use crate::drw::Drw;
use crate::config::Schemes::*;

pub enum MatchCode {Exact, Prefix, Substring, None}

#[derive(Debug)]
pub struct Item { // dmenu entry
    pub text: String,
    pub out: bool,
    pub width: c_int,
}

impl Item {
    pub fn new(text: String, out: bool, drw: &mut Drw) -> Self {
	Self{out, width: drw.textw(Some(&text)), text}
    }
    pub fn draw(&self, x: c_int, y: c_int, w: c_int, drw: &mut Drw) -> c_int {
	drw.text(x, y, w as u32, drw.pseudo_globals.bh as u32, drw.pseudo_globals.lrpad as u32/2, Some(&self.text), false)
    }
    pub fn matches(&self, text: &String) -> MatchCode {
	match self.text.match_indices(text).nth(0) {
	    None => MatchCode::None,
	    Some((0,_)) => if text.len() == self.text.len() {MatchCode::Exact} else {MatchCode::Substring},
	    Some(_) => MatchCode::Substring,
	}
    }
}

pub struct Items {
    data: Vec<Item>,
    data_matches: Vec<Vec<*const Item>>, // seperated into screens // TODO: can this be done with lifetimes?
    pub curr: usize,
}

impl Items {
    pub fn new(data: Vec<Item>) -> Self {
	Self{data, data_matches: Vec::new(), curr: 0}
    }
    pub fn match_len(&self) -> usize {
	self.data_matches.len()
    }
    pub fn draw(&mut self, drw: &mut Drw) { // gets an apropriate vec of matches
	unsafe {
	    
	    let rangle = ">".to_string();
	    let rangle_width =  drw.textw(Some(&rangle));
	    let langle = "<".to_string();
	    let langle_width =  drw.textw(Some(&langle));

	    let mut x = drw.pseudo_globals.promptw + drw.pseudo_globals.inputw
		+ drw.pseudo_globals.lrpad / 2; // TODO: this is a FEW pixels off

	    let (partition_i, partition) = {
		let mut partition_i = self.curr;
		let mut partition = 0;
		for p in &self.data_matches {
		    if partition_i >= p.len() {
			partition_i -= p.len();
			partition += 1;
		    } else {
			break;
		    }
		}
		(partition_i, partition)
	    };

	    
	    if partition > 0 {
		drw.setscheme(drw.pseudo_globals.schemeset[SchemeNorm as usize]);
		x = drw.text(x, 0, langle_width as u32, drw.pseudo_globals.bh as u32, drw.pseudo_globals.lrpad as u32/2, Some(&langle), false);
	    } else {
		x += langle_width;
	    }
	    
	    for index in 0..self.data_matches[partition].len() {
		if index == partition_i {
		    drw.setscheme(drw.pseudo_globals.schemeset[SchemeSel as usize]);
		} else if (*self.data_matches[partition][index]).out {
		    drw.setscheme(drw.pseudo_globals.schemeset[SchemeOut as usize]);
		} else {   
		    drw.setscheme(drw.pseudo_globals.schemeset[SchemeNorm as usize]);
		}
		x = (*self.data_matches[partition][index]).draw(x, 0, (*self.data_matches[partition][index]).width.min(drw.pseudo_globals.mw - x - rangle_width), drw); // in case item overruns
	    }
	    
	    if partition < self.data_matches.len()-1 {
		drw.setscheme(drw.pseudo_globals.schemeset[SchemeNorm as usize]);
		x = drw.text(drw.pseudo_globals.mw - rangle_width, 0, rangle_width as u32, drw.pseudo_globals.bh as u32, drw.pseudo_globals.lrpad as u32/2, Some(&rangle), false);
	    }
	    
	}
    }
    pub fn gen_matches(&mut self, text: &String, drw: &mut Drw) { // TODO: merge into draw?
	unsafe{
	    self.curr = 0;
	    self.data_matches.clear();
	    let mut exact:     Vec<*const Item> = Vec::new();
	    let mut prefix:    Vec<*const Item> = Vec::new();
	    let mut substring: Vec<*const Item> = Vec::new();
	    for item in &self.data {
		match item.matches(text) {
		    MatchCode::Exact => exact.push(item),
		    MatchCode::Prefix => prefix.push(item),
		    MatchCode::Substring => substring.push(item),
		    MatchCode::None => {}
		}
	    }
	    self.data_matches.reserve(prefix.len()+substring.len());
	    for item in prefix { // extend is broken for pointers
		exact.push(item);
	    }
	    for item in substring {
		exact.push(item);
	    }
	    let mut partition = Vec::new();
	    let rangle_width =  drw.textw(Some(&">".to_string()));
	    let mut x = drw.pseudo_globals.promptw + drw.pseudo_globals.inputw
		+ drw.pseudo_globals.lrpad / 2 + drw.textw(Some(&"<".to_string()));
	    for i in 0..exact.len() { // partition PICKUP: this part is borked
		x += (*exact[i]).width; // I'm thinking because we add this twice at some point?
		if x > {
		    if i == exact.len()-1 {
			drw.pseudo_globals.mw
		    } else {
			drw.pseudo_globals.mw - rangle_width
		    }
		}{
		    // not enough room
		    self.data_matches.push(partition);
		    partition = Vec::new();
		    x = drw.pseudo_globals.promptw + drw.pseudo_globals.inputw
			+ drw.pseudo_globals.lrpad / 2 + (*exact[i]).width;
		} else {
		    if i == 42 {
			println!("t: {}, i: {}", (*exact[i]).text, i);
			println!("comp: {}, x: {}", if i == exact.len()-1 {
			    drw.pseudo_globals.mw
			} else {
			    drw.pseudo_globals.mw - rangle_width
			}, x);
		    }
		    partition.push(exact[i]);
		}
	    }
	    if partition.len() > 0 {
		self.data_matches.push(partition);
	    }
	}
    }
}
