//#[cfg(feature="gui")]
use gtk4::{
    glib::{self, clone},
    prelude::*,
};
use crate::mapping::{Op,calc_table};



fn add_actions(
    application: &gtk4::Application,
    window: &gtk4::ApplicationWindow,
) {

 let help = gtk4::gio::SimpleAction::new("help", None);
    help.connect_activate(clone!(@weak window => move |_, _| {
    
        let textout : &str = 
        " This program allows analysing integers less than 2^128. 
        Functions that accept multiple arguments like gcd, require spaces
        to separate the integers. 
        
        This program has a graphical component and a terminal component.
        To interact with the terminal, run it in terminal with -h flag. ";
                              
        let textview = gtk4::Label::new(Some(textout));
        
                let p = gtk4::Dialog::builder()
                        .title("Help")
                        .build();
                        
        p.set_child(Some(&textview));
       
        p.set_transient_for(Some(&window));
        p.show();
    }));

    let about = gtk4::gio::SimpleAction::new("about", None);
    about.connect_activate(clone!(@weak window => move |_, _| {
        let p = gtk4::AboutDialog::new();
        p.set_authors(&["J.A Sory"]);
        p.set_license_type(gtk4::License::Gpl30);
        p.set_logo_icon_name(None);
        p.set_program_name(Some("GTKPrime"));
        p.set_copyright(Some("© 2024 J.A Sory"));
        p.set_version(Some("1.2.0"));
        p.set_comments(Some("Graphical component of terprime"));
        p.set_transient_for(Some(&window));
        p.show();
    }));


    application.add_action(&about);
    application.add_action(&help);

}

fn build_header_menu(header: &gtk4::HeaderBar){
     let menu = gtk4::gio::Menu::new();
        //menu.append(Some("Set"))
        menu.append(Some("Help"), Some("app.help"));
        menu.append(Some("About"), Some("app.about"));
        let p = gtk4::MenuButton::new();
        p.set_menu_model(Some(&menu));
        header.pack_end(&p);
 }



//#[cfg(feature="gui")]
pub(crate) fn build_ui(app: &gtk4::Application){
   let window = gtk4::ApplicationWindow::new(app);
    window.set_title(Some("GTKPrime"));
    window.set_default_size(480,320);
    
    
    let prime_entry = gtk4::Entry::new();
    
    
        let header_title = gtk4::Label::new(Some("GTKPrime"));
        
        let top = gtk4::HeaderBar::builder()
                  .show_title_buttons(true)
                  .title_widget(&header_title)
                  .build();
    
   build_header_menu(&top);    
    
    window.set_titlebar(Some(&top));
    
    let check_button = gtk4::Button::with_label("Check");
    let fctr_button = gtk4::Button::with_label("Factor");
    let prev_button = gtk4::Button::with_label("Prev");
    let next_button = gtk4::Button::with_label("Next");
    let nth_button = gtk4::Button::with_label("nth");
    let pi_button = gtk4::Button::with_label("pi");
    let list_button = gtk4::Button::with_label("list");
    let solution = gtk4::Label::new(None); 
    let row = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();
        
    let but_array = gtk4::Grid::new();
    but_array.attach(&prev_button,0,0,1,1);
    but_array.attach(&next_button,1,0,1,1);
    but_array.attach(&check_button,2,0,1,1);
    but_array.attach(&fctr_button,3,0,1,1);
    
    prev_button.connect_clicked(clone!(@weak prime_entry,@weak solution => move |_|{
        let txt = prime_entry.text();
        //row.remove(&but_array);
                        //but_array.attach(&list_button,4,0,1,1);
         match txt.parse::<u128>(){
            Err(_) => solution.set_text("Not a valid integer"),
            Ok(x) => {
                prime_entry.set_text(&calc_table(Op::Prev,0,x,0,None).display(true));
                solution.set_text(" ");
               // but_array.attach(&list_button,4,0,1,1);
            }
         }
        } 
    ));
    
   next_button.connect_clicked(clone!(@weak prime_entry,@weak solution => move |_|{
        let txt = prime_entry.text();
         match txt.parse::<u128>(){
            Err(_) => solution.set_text("Not a valid integer"),
            Ok(x) => {
                prime_entry.set_text(&calc_table(Op::Next,0,x,0,None).display(true));
                solution.set_text(" ");
            }
         }
        } 
    )); 
    
    check_button.connect_clicked(clone!(@weak solution , @weak prime_entry  => move |_|{
         let txt = prime_entry.text();
         match txt.parse::<u128>(){
            Err(_) => solution.set_text("Not a valid integer"),
            Ok(x) => {
                solution.set_text(&calc_table(Op::Check,0,x,0,None).display(true))
            }
         }
        } 
    )
    );
    
    fctr_button.connect_clicked(clone!(@weak solution , @weak prime_entry  => move |_|{
         let txt = prime_entry.text();
         match txt.parse::<u128>(){
            Err(_) => solution.set_text("Not a valid integer"),
            Ok(x) => {
                solution.set_text(&calc_table(Op::Factor,0,x,0,None).display(true))
            }
         }
        } 
    )
    );
    
    row.append(&prime_entry);
    row.append(&solution);
    row.append(&but_array);
    
    window.set_child(Some(&row));
        add_actions(app, &window);

    window.present();
}
