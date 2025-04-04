

/*
Algorithm for candidates
In:  list of not necessarily unique letters
Out: List of words that contain only the letters in that list up to the whole list



A necessary condition is that each word contains only the letters provided, so generate the letters that aren't in the set, and filter if any of the words contain them
Since it is necessary that the words contain at least one letter take the first letter and search for it, collecting all words that contain it

*/

use std::io::BufRead;


use gtk4::{
    glib::{self, clone},
    prelude::*,
};
use libadwaita::prelude::*;



struct Dictionary{
  words: Vec<String>,
}

impl Dictionary{

fn load(x: &str, inf: usize, sup: usize) -> Option<Self>{
    match std::fs::File::open(x){
       Ok(f) => {
       let br = std::io::BufReader::new(f);
       let mut words = std::collections::HashSet::new();
       for i in br.lines(){
         match i{
           Ok(x) => {
             if filter_len(&x,inf,sup) && !x.contains("'"){
                words.insert((&x).to_lowercase());
             }
           },
        Err(_) => println!("Invalid line"),
       } 
     }
     
     let mut interim = words.drain().collect::<Vec<String>>();
     interim.sort();
     
    return Some(Self{words: interim});
       },
       Err(_) => None,
    }
   
}
// Takes the difference and removes all letters than 
// Captures the first words to a string that 
fn to_string(&self) -> String{
    if self.words.is_empty(){
       return "No words found".to_string();
    }
   let mut veccy = vec![];
   let mut str_len = 0usize;
   for i in self.words.iter(){
      str_len+=i.len()+1;
      // Restrict the string length to prevent app freezing from
      if str_len > 600{
        break;
      }
      veccy.push(i.clone())
   }
   veccy.join(" ")
}

fn filter_by_complement(&self, comp:&Vec<char> ) -> Self{
    let mut words = vec![];
   for i in self.words.iter(){
      if filter_letter(i,&comp){
         words.push(i.clone());
      }
   }
   Self{words}
}

fn filter_by_distribution(&self, dist: Vec<(char,u32)>) -> Self{
    let mut words = vec![];
    
    for i in self.words.iter(){
        if filter_distribution_max(i,&dist){
           words.push(i.clone())
        }
    }
    Self{words}
}

fn filter_by_pattern(&self, pattern: &str) -> Self{
   let pat = pattern_parser(pattern);
   let mut words = vec![];
   for i in self.words.iter(){
       if filter_pattern(i,&pat) && i.len() == pattern.len(){
          words.push(i.to_string())
       }
    }
    
    Self{words}

}

fn filter_by_substr(&self, substr: &str) -> Self{
   let lset = unique_input(substr);
    let comp = complement(lset.clone());
    let dist = input_distribution(substr,lset);
    let interim = self.filter_by_complement(&comp);
 
    interim.filter_by_distribution(dist)
}

fn filter_by_substr_inclusive(&self, substr: &str) -> Self{
    let charvec = unique_input(substr);
    let mut words = vec![];
    for w in self.words.iter(){
       for i in charvec.iter(){
         if w.contains(*i){
            words.push(w.clone());
            break;
         }
       }
    }
    Self{words}
}

// XOR-Shift seeded using time since the UNIX Epoch. 
// Since this function is called with multiple seconds between it there should be sufficient entropy
// 
fn access_random(&self) -> String{
    if self.words.is_empty(){
       return "No word found".to_string();
    }
    let mut x = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    x ^= x.wrapping_shr(12);
    x ^= x.wrapping_shl(25);
    x ^= x.wrapping_shr(27);
    x = x.wrapping_mul(0x2545F4914F6CDD1D);
    
    let len = self.words.len();
    
    self.words[(x as usize)%len].clone()
}

}

// Take input and determine the length of words to filter
fn parse_len(input: &str) -> Option<(usize,usize)>{
    if input.contains('-'){
       let interim = input.split("-").collect::<Vec<&str>>();
       if interim.len() != 2{
          return None;
       }
       match (interim[0].parse::<usize>(),interim[1].parse::<usize>()){
            (Ok(lhs),Ok(rhs)) => return Some((std::cmp::min(lhs,rhs),std::cmp::max(lhs,rhs))),
            _=> return None,
       }
    }
    
    match input.parse::<usize>(){
      Ok(res) => Some((res,res)),
      Err(_) => None,
    }
}

// Collect unique values of string
fn unique_input(x: &str) -> Vec<char>{
   let mut p = x.chars().collect::<std::collections::HashSet<char>>();
   p.drain().collect::<Vec<char>>()
}

fn input_distribution(x : &str, lset: Vec<char>)-> Vec<(char,u32)>{
   let mut res = vec![];
   for i in lset.iter(){
    let mut count = 0u32;
      for j in x.chars(){
        if *i==j{
          count+=1
        }
      }
      res.push((*i,count))
   }
   res
}


fn filter_distribution_max(x: &str, dist: &Vec<(char,u32)>) -> bool{
    for i in dist.iter(){
      let mut count = 0u32;
       for j in x.chars(){
         if i.0==j{
           count+=1;
         }
       }
       if count > i.1{
         return false
       }
    }
    return true
}


fn complement(letter_set: Vec<char>) -> Vec<char>{
   const LATIN_ALPHABET : [char;26] = [
   'a','b','c','d','e','f','g','h',
   'i','j','k','l','m','n','o','p',
   'q','r','s','t','u','v','w','x',
   'y','z',
   ];
   let mut res = vec![];
   for i in LATIN_ALPHABET{
      if !letter_set.contains(&i){
        res.push(i);
      }
   }
   res
}

fn filter_len(x: &str, inf: usize, sup: usize) -> bool{
    if x.len() >= inf && x.len() <= sup{
      return true;
    }
    false
}

// Defunct but kept incase of future use

// If x contains any letter return false
fn filter_letter(x: &String, lset: &Vec<char>) -> bool{
   for i in lset{
      if x.contains(*i){
         return false
      }
   }
   return true
}

/*
   In: A string ---x--y
   Out: 
*/

fn pattern_parser(x: &str) -> Vec<(char,usize)>{
   let mut res = vec![];
   for (idx,el) in x.chars().enumerate(){
      if el != '-'{
         res.push((el,idx));
      }
   }
   res
}

fn filter_pattern(x: &str, fltr: &Vec<(char,usize)>)-> bool{
   for i in fltr{
      for (idx,el) in x.chars().enumerate(){
         if idx == i.1{
            if el != i.0{
              return false;
            }
         }
      }    
   }
   return true
}


fn add_actions(
    application: &libadwaita::Application,
    window: &libadwaita::ApplicationWindow,
) {

 let help = gtk4::gio::SimpleAction::new("help", None);
    help.connect_activate(clone!(
    #[weak]
    window, move |_, _| {
    
        let textout : &str = 
        " Application filters from the system dictionary according to atleast one of 3 criteria.
        1. Word length, either as a single length {3} or a range {3-5}
        2. List of all the letters that the word can be comprised of.
        3. Matching a pattern of the form -{x}--. where - can be any lower-case Latin character.
        
    The strict flag for option 2 changes from filtering all the words that contain any letter, to ones 
    that only contain the provided letters.\n";
                              
        let textview = gtk4::Label::new(Some(textout));
        
                let p = gtk4::Dialog::builder()
                        .title("Help")
                        .build();
        p.set_child(Some(&textview));
       
        p.set_transient_for(Some(&window));
        p.show();
    }));

    let about = gtk4::gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(
    #[weak]
    window, move |_, _| {
        let p = gtk4::AboutDialog::new();
        p.set_authors(&["J.A Sory"]);
        p.set_license_type(gtk4::License::Gpl30);
        p.set_logo_icon_name(None);
        p.set_program_name(Some("Word Assistant"));
        p.set_copyright(Some("© 2024 J.A Sory"));
        p.set_version(Some("1.1.0"));
        p.set_comments(Some("Word format searcher"));
        p.set_transient_for(Some(&window));
        p.show();
    }));


    application.add_action(&about);
    application.add_action(&help);

}

fn build_header_menu(header: &gtk4::HeaderBar){
     let menu = gtk4::gio::Menu::new();
        menu.append(Some("Help"), Some("app.help"));
        menu.append(Some("About"), Some("app.about"));
        let p = gtk4::MenuButton::new();
        p.set_menu_model(Some(&menu));
        header.pack_end(&p);
 }

fn build_ui(application: &libadwaita::Application) {
   let window = libadwaita::ApplicationWindow::new(application);
    window.set_default_size(384,256);
    
    
    let length_entry = gtk4::Entry::new();
    let word_entry = gtk4::Entry::new();
    let pattern_entry = gtk4::Entry::new();
    let output = gtk4::Entry::new();
    let submit_button = gtk4::Button::with_label("Query");
    let generate_button = gtk4::Button::with_label("Generate");
    let header_title = gtk4::Label::new(Some("Word Assistant"));
    let length_label = gtk4::Label::new(Some("Word Length"));
    let letter_label = gtk4::Label::new(Some("\t Letter List"));
    let pattern_label = gtk4::Label::new(Some("Pattern"));
    let letter_button =gtk4::CheckButton::with_label("(Strict)");
    let short = gtk4::Grid::new();
    short.attach(&letter_button,0,0,1,1);
    short.attach(&letter_label,1,0,1,1);
        length_entry.set_placeholder_text(Some("3-5"));
        word_entry.set_placeholder_text(Some("ndsa"));
        pattern_entry.set_placeholder_text(Some("--n-"));
        
        let top = gtk4::HeaderBar::builder()
                  .show_title_buttons(true)
                  .title_widget(&header_title)
                  .build();

    
   build_header_menu(&top);    
  
    
   let row = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
  
        let list = gtk4::ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(gtk4::SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
            
    
        

    list.append(&length_label);
    list.append(&length_entry);
    list.append(&short);
    list.append(&word_entry);
    list.append(&pattern_label);
    list.append(&pattern_entry);
    list.append(&submit_button);
    list.append(&generate_button);
    list.append(&output);

        row.append(&top);
        row.append(&list);
        
    submit_button.connect_clicked(clone!(
         #[weak]
     length_entry, 
          #[weak]
     word_entry,
          #[weak]
     pattern_entry, 
          #[weak]
     output, 
          #[weak]
     letter_button, move |_|{

        let mut length = (0,22);
        
        match parse_len(&length_entry.text().to_string()){
              Some(x) => length = x,
              None => (),
        }
        
        let word = word_entry.text().to_string();
        let pattern = pattern_entry.text().to_string();
        
        match Dictionary::load("/usr/share/dict/words",length.0,length.1){
              Some(mut k) => {
                
                  if word.len() != 0{
                   if letter_button.is_active(){
                    k = k.filter_by_substr(&word);
                    }
                    else{
                    k = k.filter_by_substr_inclusive(&word);
                    }
                  }
                 
                if pattern.len() != 0{
                    k = k.filter_by_pattern(&pattern);
                }

               let p = k.to_string();
                output.set_text(&p);
              }
              None => output.set_text("Failed to load dictionary"),
        }
    }));
    
    generate_button.connect_clicked(clone!(
    #[weak]
     length_entry,
     #[weak]
     word_entry,
     #[weak]
     pattern_entry, 
     #[weak]
     output, 
     #[weak]
     letter_button, move |_|{

        let mut length = (0,22);
        
        match parse_len(&length_entry.text().to_string()){
              Some(x) => length = x,
              None => (),
        }
        
        let word = word_entry.text().to_string();
        let pattern = pattern_entry.text().to_string();
        
        match Dictionary::load("/usr/share/dict/words",length.0,length.1){
              Some(mut k) => {
                
                  if word.len() != 0{
                   if letter_button.is_active(){
                    k = k.filter_by_substr(&word);
                    }
                    else{
                    k = k.filter_by_substr_inclusive(&word);
                    }
                  }
                 
                if pattern.len() != 0{
                    k = k.filter_by_pattern(&pattern);
                }

               let p = k.access_random();
                output.set_text(&p);
              }
              None => output.set_text("Failed to load dictionary"),
        }
    }));

    window.set_content(Some(&row));

        add_actions(application, &window);
        window.set_title(Some("Word Assistant"));

    window.present();

}

fn main() -> glib::ExitCode{

    let application = libadwaita::Application::builder()
        .application_id("com.github.jasory.word-assistant")
        .build();

    application.connect_activate(build_ui);
    application.run()
}
