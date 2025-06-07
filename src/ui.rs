use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use ratatui::prelude::*;


use crate::app::App;

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {

	let layout =Layout::default()
	    .direction(Direction::Horizontal)
	    .constraints(vec![Constraint::Percentage(50),
			       Constraint::Percentage(50),]
	    ).split(area);
   
			    
	
        let block_ui = Block::bordered()
            .title("UI")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

	let block_info = Block::bordered()
            .title("INFO")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        /*
        let text = format!(
            "This is a tui template.\n\
                Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                Press left and right to increment and decrement the counter respectively.\n\
                Counter: {}",
            self.counter
        );*/


        let mut board = vec![vec![" ".to_string();self.tycoon_arena_dim[1] as usize]; self.tycoon_arena_dim[0] as usize];
        let mut board_str = vec![vec![vec!["_".to_string(); self.board_dim[1] as usize] ; self.board_dim[0] as usize] ; self.arena_num ]; 
	let mut final_str = "".to_string();
        let mut n =0;
        let mut dir:Vec<i32>=vec![0,0];
	let mut pos : Vec<i32>=vec![0,0];
        let mut argmax_rank:usize = 0 as usize;
	let col:usize=self.board_dim[0] as usize;
	let row:usize=self.board_dim[1] as usize;

	let menu_items:Vec<String> = vec![
	    "Load Custom Weapons and Armor".to_string(),
	    "Start Simulation".to_string(),
	    "Load Saved Gladiators".to_string(),
	    "Tycoon Mode".to_string(),
	];

	let tycoon_menu_items:Vec<String>=vec![
	    "Buy Gladiator".to_string(),
	    "Sell Gladiator".to_string(),
	    "Increase Ticket Price".to_string(),
	    "Decrease Ticket Price".to_string(),
	    "Schedule Fight".to_string(),
	];
	let mut menu_select_idx :usize = 0;
	if self.sim_mode{
	
	    for a in 0..self.arena_num{  
		for n in self.arenas[a].clone(){

		pos[0]=self.npc_pos[n][0];
		pos[1]=self.npc_pos[n][1];

		dir[0]=self.npc_dir[n][0]+pos[0];
		dir[1]=self.npc_dir[n][1]+pos[1];

		if(dir[0]>=self.board_dim[0]){
		    dir[0]-=self.board_dim[0];
		}else if(dir[0]<0){
		    dir[0]+=self.board_dim[0];
		}
		if(dir[1]>=self.board_dim[1]){
		    dir[1]-=self.board_dim[1];
		}else if(dir[1]<0){
		    dir[1]+=self.board_dim[1];
		}
		if n == self.npc_kill_argmax{
		    board_str[a][pos[0] as usize][pos[1] as usize]="%".to_string();
		}else if n==self.arenas[a][0]{
		    board_str[a][pos[0] as usize][pos[1] as usize]="@".to_string();
		}else if n==self.arenas[a][1]{
		    board_str[a][pos[0] as usize][pos[1] as usize]="&".to_string();
		}else{
		    board_str[a][pos[0] as usize][pos[1] as usize]="E".to_string();
		}
		    

		if self.npc_dir_idx[n] == 0{
		    board_str[a][dir[0] as usize][dir[1] as usize]="|".to_string();
		}else if self.npc_dir_idx[n] == 1{
		    board_str[a][dir[0] as usize][dir[1] as usize]="\\".to_string();
		}else if self.npc_dir_idx[n] == 2{
		    board_str[a][dir[0] as usize][dir[1] as usize]="-".to_string();
		}else if self.npc_dir_idx[n] == 3{
		    board_str[a][dir[0] as usize][dir[1] as usize]="/".to_string();
		}else if self.npc_dir_idx[n] == 4{
		    board_str[a][dir[0] as usize][dir[1] as usize]="|".to_string();
		}else if self.npc_dir_idx[n] == 5{
		    board_str[a][dir[0] as usize][dir[1] as usize]="\\".to_string();
		}else if self.npc_dir_idx[n] == 6{
		    board_str[a][dir[0] as usize][dir[1] as usize]="-".to_string();
		}else if self.npc_dir_idx[n] == 7{
		    board_str[a][dir[0] as usize][dir[1] as usize]="/".to_string();
		}
		//board[(dir[0]) as usize][dir[1] as usize]="*";

		}


	    }
	    for x in 0..7*(row+2) {
		final_str+="#";
	    }
	    final_str+="\n";
	    for b in 0..2{
		for i in 0..col {
		    for a in 0..7{
			final_str+="#";
			for j in 0..row {
			    final_str+=&board_str[7*b+a][i][j];
			}
			final_str+="#";
		    }
		    final_str+="\n";
		}
		for x in 0..7*(row+2) {
		    final_str+="#";
		}
		final_str+="\n";//+"#"*self.board_dim[1]*self.arena_num+"\n";
	    }
	}else if self.npc_list_mode && (self.buy_mode || self.sell_mode){

	    
	    final_str="Name : Price    *owned".to_string();
	    //================================
	    //LIST ALL NPCS BY KILL COUNT
	    //let sorted_list = Vec::new();
	    
	    for n in 0..25{
		menu_select_idx=self.counter as usize;
		
		if n as usize == menu_select_idx{
		    final_str=format!("{}\n> {}: ${}",final_str,self.npc_name[self.npc_kill_sort[n]],self.npc_price_list[self.npc_kill_sort[n]]);
		}else{
		    final_str=format!("{}\n{}: ${}",final_str,self.npc_name[self.npc_kill_sort[n]],self.npc_price_list[self.npc_kill_sort[n]]);
		}

		if self.npc_owned[self.npc_kill_sort[n]] {
		    final_str=format!("{} *",final_str);
		}
	    }
	}else if self.schedule_mode{
	    final_str="".to_string();
	    for n in self.npc_owned_list.clone(){
		menu_select_idx=self.counter as usize;
		
		if n as usize == menu_select_idx{
		    final_str=format!("{}\n> {}: ${}",final_str,self.npc_name[n],self.npc_price_list[n]);
		}else{
		    final_str=format!("{}\n{}: ${}",final_str,self.npc_name[n],self.npc_price_list[n]);
		}

		if self.npc_owned[n] {
		    final_str=format!("{} *",final_str);
		}
	    }
	}else if self.tycoon_mode{
	    final_str="".to_string();
	    
	    menu_select_idx=self.counter as usize;
	    if menu_select_idx>=tycoon_menu_items.len(){
		menu_select_idx=0;
	    }
	    n=0;
	    for item in tycoon_menu_items {
		if n as usize == menu_select_idx{
		    final_str+=" > ";
		}
		final_str+=&(item+"\n");

		n+=1;
	    }

	}else if self.main_menu_mode{
	    final_str="".to_string();
	    
	    menu_select_idx=self.counter as usize;
	    if menu_select_idx>=menu_items.len(){
		menu_select_idx=0;
	    }
	    n=0;
	    for item in menu_items {
		if n as usize == menu_select_idx{
		    final_str+=" > ";
		}
		final_str+=&(item+"\n");

		n+=1;
	    }
	}


	//=================================================================================
	//=================================================================================
	let mut text=format!("{}",final_str);
	let paragraph = Paragraph::new(text)
		.block(block_ui)
		.fg(Color::Green)
	        .bg(Color::Black)
	        .centered();
	//paragraph.render(area, buf);
//	paragraph.render(area,layout[0]);
	paragraph.render(layout[0],buf);
	let mut info_str = "".to_string();
	if self.sim_mode{
	    info_str = format!("=== Top Gladiator Stats ===
    Name : {}
    Kill Count : {}
    Hit Die : {}d{}
    Weapon : {}
    === Weapon Usage Stats ===",self.npc_name[self.npc_kill_argmax],
			    self.npc_kill_count[self.npc_kill_argmax],
			    self.weapon_die[self.npc_weapon_idx[self.npc_kill_argmax]][0],
			    self.weapon_die[self.npc_weapon_idx[self.npc_kill_argmax]][1],
			    self.weapon_str[self.npc_weapon_idx[self.npc_kill_argmax]],)
		.to_string();
	    for n in 0..self.weapon_num{
		info_str+=&format!("\n{} : {}%",self.weapon_str[n],self.weapon_usage[n]*100.0);
	    }

	    info_str+="\n===Armor Usage Stats";
	    for n in 0..self.armor_num{
		info_str+=&format!("\n {}:{}%",self.armor_str[n],self.armor_usage[n]*100.0);
	    }
	}else if self.tycoon_mode{ 
	    info_str = "".to_string();//format!("Current Spectators: {}",self.population);
	      
	    for n in self.tycoon_arena.clone(){

	    pos[0]=self.npc_pos[n][0];
	    pos[1]=self.npc_pos[n][1];

	    dir[0]=self.npc_dir[n][0]+pos[0];
	    dir[1]=self.npc_dir[n][1]+pos[1];

	    if(dir[0]>=self.tycoon_arena_dim[0]){
		dir[0]-=self.tycoon_arena_dim[0];
	    }else if(dir[0]<0){
		dir[0]+=self.tycoon_arena_dim[0];
	    }
	    if(dir[1]>=self.tycoon_arena_dim[1]){
		dir[1]-=self.tycoon_arena_dim[1];
	    }else if(dir[1]<0){
		dir[1]+=self.tycoon_arena_dim[1];
	    }
	    if n == self.npc_kill_argmax{
		board[pos[0] as usize][pos[1] as usize]="%".to_string();
	    }else if n == self.tycoon_arena[0]{
		board[pos[0] as usize][pos[1] as usize]="@".to_string();
	    }else if n== self.tycoon_arena[1]{
		board[pos[0] as usize][pos[1] as usize]="&".to_string();
	    }else{
		board[pos[0] as usize][pos[1] as usize]="E".to_string();
	    }


	    if self.npc_dir_idx[n] == 0{
		board[dir[0] as usize][dir[1] as usize]="|".to_string();
	    }else if self.npc_dir_idx[n] == 1{
		board[dir[0] as usize][dir[1] as usize]="\\".to_string();
	    }else if self.npc_dir_idx[n] == 2{
		board[dir[0] as usize][dir[1] as usize]="-".to_string();
	    }else if self.npc_dir_idx[n] == 3{
		board[dir[0] as usize][dir[1] as usize]="/".to_string();
	    }else if self.npc_dir_idx[n] == 4{
		board[dir[0] as usize][dir[1] as usize]="|".to_string();
	    }else if self.npc_dir_idx[n] == 5{
		board[dir[0] as usize][dir[1] as usize]="\\".to_string();
	    }else if self.npc_dir_idx[n] == 6{
		board[dir[0] as usize][dir[1] as usize]="-".to_string();
	    }else if self.npc_dir_idx[n] == 7{
		board[dir[0] as usize][dir[1] as usize]="/".to_string();
	    }
		//board[(dir[0]) as usize][dir[1] as usize]="*";
	    }

	    for x in 0..self.tycoon_arena_dim[0] as usize{
		info_str+="#";
	    }
	    info_str+="\n";
	    for i in 0..self.tycoon_arena_dim[0] as usize{
		info_str+="#";
		for j in 0..self.tycoon_arena_dim[1] as usize{
		    info_str+=&board[i][j];
		}
		info_str+="#\n";
	    }
	    for x in 0..self.tycoon_arena_dim[0] as usize{
		info_str+="#";
	    }
	    info_str+="\n";
	    info_str=format!("{}\nTicket Price: {}, Gold: {}\nSpectators: {}\n==Current Fighters==\n@ <==> {}\n& <==> {}",
			     info_str,
			     self.ticket_price,
			     self.gold,
			     self.population,
			     self.npc_name[self.tycoon_arena[0]],
			     self.npc_name[self.tycoon_arena[1]],
	    );
	    
	}
	

	
		
	text=format!("{}",info_str);
	let npc_stat_chart = Paragraph::new(text)
		.block(block_info)
		.fg(Color::Green)
	        .bg(Color::Black)
	        .centered();
	//npc_stat_chart.render(area, buf);
	npc_stat_chart.render(layout[1], buf);
	//npc_stat_chart.render(area, layout[1]);

    }
}
