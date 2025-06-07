use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use rand::Rng;

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self,BufRead};


/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Counter.
    pub counter: i32,
    
    pub seed : i32,
    //Board Characteristics
    
    pub board_dim : Vec<i32>,
    pub board_pop : Vec<Vec<f32>>,
    //
    ///NPC POSITIONS




    ///
    ///

    pub npc_pos : Vec<Vec<i32>>,
    pub npc_dir : Vec<Vec<i32>>,
    pub npc_hp : Vec<i32>,
    pub npc_bleed : Vec<Vec<i32>>,
    pub npc_target_idx : Vec<usize>,
    
    pub npc_dir_idx : Vec<usize>,
    /// Event handler.
    pub events: EventHandler,

    pub npc_Q_table : Vec<Vec<Vec<f32>>>,
    pub total_rewards : Vec<f32>,
    pub npc_num : usize,
    pub npc_NN : Vec<Vec<Vec<Vec<f32>>>>,
    pub npc_armor_idx : Vec<usize>,
    pub armor_str : Vec<String>,
    pub armor_AC : Vec<i32>,
    pub armor_num : usize,
    pub armor_usage : Vec<f32>,
    pub npc_speed :Vec<Vec<i32>>,
    pub npc_max_acc:Vec<i32>,
    pub npc_max_hp : Vec<i32>,
    pub npc_kill_count : Vec<i32>,
    pub npc_ticks_alive: Vec<i32>,
    pub arena_num : usize,
    pub arenas : Vec<Vec<usize>>,
    pub npc_fight_bool : Vec<bool>,

    pub npc_kill_argmax : usize,
    pub npc_name : Vec<String>,
    
    pub npc_weapon_idx : Vec<usize>,
    //Weapon Database Variables
    pub weapon_num : usize,
    pub weapon_str : Vec<String>,
    pub weapon_die : Vec<Vec<i32>>,
    //App specific variables
    pub sim_mode : bool,
    pub main_menu_mode :bool,
    pub user_input_mode :bool,
    pub weapon_usage : Vec<f32>,

    pub old_output : Vec<Vec<f32>>,
    //TYCOON MODE VARIABLES:
    pub tycoon_mode : bool,
    pub pop_cap : i32,
    pub population : i32,
    pub npc_owned : Vec<bool>,
    pub gold : i32,
    pub npc_list_mode : bool,
    pub buy_mode : bool,
    pub sell_mode : bool,
    pub npc_kill_sort : Vec<usize>,
    pub tycoon_arena_dim : Vec<i32>,
    pub tycoon_arena : Vec<usize>,
    pub npc_owned_list : Vec<usize>,
    pub npc_price_list : Vec<i32>,
    pub npc_maim_count : Vec<i32>,
    pub ticket_price : i32,
    pub schedule_mode : bool,
}

impl Default for App {
	fn default() -> Self {

	let npc_num =200 as usize;
	let arena_num = 14 as usize;
	let arenas:Vec<Vec<usize>> = vec![vec![0,1],vec![2,3],vec![4,5],vec![6,7],vec![7,8],
					  vec![9,10],vec![11,12],vec![13,14],vec![15,16],vec![17,18],
					  vec![19,20],vec![21,22],vec![23,24],vec![25,26]];
	let mut npc_fight_bool = vec![false ; npc_num];
	for k in 0..2*arena_num {
	    npc_fight_bool[k]=true; 
	}
	
	let mut npc_NN = vec![
	    vec![//this is the vector containing layers
		vec![
		 vec![0.0;40] ; 50
		],
		vec![
		 vec![0.0;50] ; 50   
		],
		vec![
		 vec![0.0;50] ; 50
		],
		vec![
		 vec![0.0;50];11
		],
		]
		;npc_num ];
	
	
	let mut npc_Q_table = vec![vec![vec![0.0 as f32;11];11];npc_num];
	for n in 0..npc_num{
	    for x in 0..11{
		npc_Q_table[n][x][x]=1.0;
	    }

	}
        Self {
            running: true,
            counter: 0,
            seed : 1010,
            board_dim : vec![12,12],
	    npc_Q_table : npc_Q_table,
	    total_rewards : vec![0.0;npc_num],
	    board_pop : vec![vec![0.0;12];12],
            npc_pos : vec![vec![2,2];npc_num],
            npc_dir : vec![vec![1,0];npc_num],
            npc_hp : vec![20;npc_num],
	    npc_bleed: vec![vec![0;6];npc_num],
	    armor_num : 1,
	    npc_armor_idx : vec![0;npc_num],
	    armor_str : vec!["Old Rags".to_string();npc_num],
	    armor_usage : vec![0.0;npc_num],
	    armor_AC : vec![5;npc_num],
	    npc_speed : vec![vec![0;8];npc_num],
	    npc_max_acc : vec![2;npc_num],
	    npc_max_hp : vec![10;npc_num],
            npc_target_idx :(0..npc_num).collect(),
            npc_dir_idx : vec![0;npc_num],
	    npc_num : npc_num,
            arena_num : arena_num,
            arenas : arenas,
	    npc_fight_bool : npc_fight_bool, 
	    npc_NN : npc_NN,
	    npc_kill_argmax : 0,
	    npc_name : vec!["Joe Schmo".to_string();npc_num],
	    npc_kill_count : vec![0;npc_num],
	    npc_ticks_alive : vec![0;npc_num],
	    npc_weapon_idx : vec![0;npc_num],
	    weapon_num : 1,
	    weapon_str : vec!["A Wooden Stick".to_string()],
	    weapon_die : vec![vec![1,2]],
            events: EventHandler::new(),
            sim_mode : false,
	    main_menu_mode : true,
	    user_input_mode : false,
	    weapon_usage : vec![0.0;npc_num],
	    old_output : vec![vec![0.0;40];npc_num],
	    //TYCOON MODE VARIABLES
	    //Go simple for now. Just text readout
	    tycoon_mode : false,
	    pop_cap : 1000,
	    population : 0,
	    npc_owned : vec![false;npc_num],
	    gold : 1000,
	    npc_list_mode : false,
	    buy_mode : true,
	    sell_mode : false,
	    npc_kill_sort : vec![0;npc_num],
	    tycoon_arena_dim : vec![20,20],
	    tycoon_arena : vec![0,0],
	    npc_owned_list : Vec::new(),
	    npc_price_list : vec![0;npc_num],
	    npc_maim_count : vec![0;npc_num],
	    ticket_price : 1,
	    schedule_mode : false,
        }
    }
}

impl App {
    //==============================================================
    pub fn update_reward_table(&mut self, n:usize,state_init_idx:usize,state_next_idx:usize,action_idx:usize){
	let alpha : f32 = 0.5;
	let gamma : f32 = 0.5;
	self.npc_Q_table[n][state_init_idx][action_idx] = (1.0-alpha)*self.npc_Q_table[n][state_init_idx][action_idx] +
	    alpha*(self.total_rewards[n] + gamma*self.npc_Q_table[n][state_init_idx].iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .unwrap());
	
    }
    //==============================================================
    pub fn tycoon_tick(&mut self){
	
	let mut rng=rand::thread_rng();
	//==================================================
	let mut total_fun : i32= 0;
	let mut r : i32 = 0;
	self.npc_kill_sort=vec![0;self.npc_num];
	self.npc_owned_list = Vec::new();
	for n in 0..self.npc_num{
	    self.npc_kill_sort[n]=n;
	    self.npc_price_list[n]=100+self.npc_kill_count[n]*1000+self.npc_maim_count[n]*200;
	    if self.npc_owned[n]{
		self.npc_owned_list.push(n);
		total_fun+=self.npc_price_list[n]/20
	    }
	}
	if self.population<self.pop_cap{
	    if total_fun>self.ticket_price*1000{
		r=rng.gen_range(-1..2);
		if r>0 {
		    self.gold+=r*self.ticket_price;
		}
		self.population+=r;
	    }else{
		r=rng.gen_range(-2..1);
		if r>0 {
		    self.gold+=r*self.ticket_price;
		}
		self.population+=r;
	    }
	    if self.population>self.pop_cap{
		self.population=self.pop_cap;
	    }
	    if self.population<0{
		self.population=0;
	    }
	}
	if !self.npc_owned[self.tycoon_arena[0]]{
	    self.npc_hp[self.tycoon_arena[0]]=-1;
	}
	if !self.npc_owned[self.tycoon_arena[1]]{
	    self.npc_hp[self.tycoon_arena[1]]=-1;
	}



	//==================================================
	
	let mut argmax = 0 as usize;
	//self.counter=self.counter.saturating_add(1)
	let N:usize = self.npc_pos.len();
	let A:usize = self.arena_num;
	let mut target_pos=vec![0,0];
	let mut target_idx:usize=0;
	let mut v:Vec<i32>=vec![0,0];
	let mut dist:i32=0;
	let mut bounce_flag:bool=false;
	let mut attack_roll:i32=0;
	let mut dir:Vec<i32> = vec![0,0];
	let mut x:i32 = 0;

	let mut nn_data:Vec<f32>=vec![0.0;40];
	let mut nn_out_argmax = 0 as usize;
	let mut move_flag :bool = false;
	let turn_list:Vec<Vec<i32>> = vec![vec![1,0],
			vec![1, 1],
			vec![0, 1],
			vec![-1, 1],
			vec![-1, 0],
			vec![-1, -1],
			vec![0, -1],
			vec![1, -1]];
	let mut arg_min_dist = 0 as usize;
	let mut min_dist = 10000 as i32;
	let mut rand_usize : usize = 0 as usize;
	let mut new_fight_bool : bool = true;

	if self.npc_owned_list.len()>=2{
	self.tycoon_check_new_fight(); 
	self.find_best_npc();
	
	 
	self.npc_target_idx[self.tycoon_arena[0]]=self.tycoon_arena[1];
	self.npc_target_idx[self.tycoon_arena[1]]=self.tycoon_arena[0];


	self.board_pop=vec![vec![0.0;self.tycoon_arena_dim[1] as usize];self.tycoon_arena_dim[0] as usize];
	for n in self.tycoon_arena.clone(){
	    self.board_pop[self.npc_pos[n][0] as usize][self.npc_pos[n][1] as usize]=1.0;
	}
	for n in self.tycoon_arena.clone(){//loop through all npcs
	nn_data=vec![0.0;40];
	if(self.npc_hp[n]>0){//if the npc is alive

	    bounce_flag=false;
	    //==================================ce
	    arg_min_dist=0;
	    min_dist=10000;
	    for m in self.tycoon_arena.clone(){
		dist=(self.npc_pos[m][0]-self.npc_pos[n][0]).pow(2)+
			    (self.npc_pos[m][1]-self.npc_pos[n][1]).pow(2);
			if dist<min_dist && m != n{
				min_dist=dist;
				arg_min_dist=m;
			}
		if(dist==0 && m != n){
		    bounce_flag=true;
			break;
		}
	    }

	    //self.npc_target_idx[n]=arg_min_dist;

	    target_idx=self.npc_target_idx[n];
	    target_pos[0]=self.npc_pos[target_idx][0];
	    target_pos[1]=self.npc_pos[target_idx][1];
				    //Neural Net For Movement


	    x=self.npc_pos[n][1]*target_pos[0]
	    -self.npc_pos[n][0]*target_pos[1];


	    nn_data[0]=(self.npc_pos[n][0] - target_pos[0]) as f32;
	    nn_data[1]=(self.npc_pos[n][1] - target_pos[1]) as f32;
	    nn_data[18]=nn_data[0].powf(2.0) + nn_data[1].powf(2.0);

	    v[0]=(self.npc_pos[n][0]+1)%self.tycoon_arena_dim[0];
	    v[1]=(self.npc_pos[n][1]+0)%self.tycoon_arena_dim[1];
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[2]=1.0;
	    }else{
		nn_data[2]=0.0;
	    }


	    v[0]=(self.npc_pos[n][0]+1)%self.tycoon_arena_dim[0];
	    v[1]=(self.npc_pos[n][1]+1)%self.tycoon_arena_dim[1];
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[3]=1.0;
	    }else{
		nn_data[3]=0.0;
	    }


	    v[0]=(self.npc_pos[n][0]+1)%self.tycoon_arena_dim[0];
	    v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.tycoon_arena_dim[1]);
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[4]=1.0;
	    }else{
		nn_data[4]=0.0;
	    }


	    v[0]=(self.npc_pos[n][0]+0)%self.tycoon_arena_dim[0];
	    v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.tycoon_arena_dim[1]);
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[5]=1.0;
	    }else{
		nn_data[5]=0.0;
	    }


	    v[0]=App::brute_modulo(self.npc_pos[n][0]-1,self.tycoon_arena_dim[0]);
	    v[1]=(self.npc_pos[n][1]+1)%self.tycoon_arena_dim[1];
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[6]=1.0;
	    }else{
		nn_data[6]=0.0;
	    }

	    v[0]=App::brute_modulo(self.npc_pos[n][0]-1,self.tycoon_arena_dim[0]);
	    v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.tycoon_arena_dim[1]);

	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[7]=1.0;
	    }else{
		nn_data[7]=0.0;
	    }


	    v[0]=(self.npc_pos[n][0]+0)%self.tycoon_arena_dim[0];
	    v[1]=(self.npc_pos[n][1]+1)%self.tycoon_arena_dim[1];
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[8]=1.0;
	    }else{
		nn_data[8]=0.0;
	    }


	    v[0]=(self.npc_pos[n][0]+0)%self.tycoon_arena_dim[0];
	    v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.tycoon_arena_dim[1]);
	    if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
		nn_data[9]=1.0;
	    }else{
		nn_data[9]=0.0;
	    }

	    nn_data[10]=0.0;
	    nn_data[11]=0.0;
	    nn_data[12]=0.0;
	    nn_data[13]=0.0;
	    nn_data[14]=0.0;
	    nn_data[15]=0.0;
	    nn_data[16]=0.0;
	    nn_data[17]=0.0;

	    nn_data[self.npc_dir_idx[n] +10]=1.0;


	    nn_data[19]=0.0;
	    nn_data[20]=0.0;
	    nn_data[21]=0.0;
	    nn_data[22]=0.0;
	    nn_data[23]=0.0;
	    nn_data[24]=0.0;
	    nn_data[25]=0.0;
	    nn_data[26]=0.0;

	    nn_data[self.npc_dir_idx[target_idx] +19]=1.0;

	    nn_data[27] = self.npc_hp[n] as f32;
	    nn_data[28] = self.npc_hp[target_idx] as f32;

	    nn_data[29] = self.old_output[n][0];
	    nn_data[30] = self.old_output[n][1];
	    nn_data[31] = self.old_output[n][2];
	    nn_data[32] = self.old_output[n][3];
	    nn_data[33] = self.old_output[n][4];
	    nn_data[34] = self.old_output[n][5];
	    nn_data[35] = self.old_output[n][6];
	    nn_data[36] = self.old_output[n][7];
	    nn_data[37] = self.old_output[n][8];
	    nn_data[38] = self.old_output[n][9];
	    nn_data[39] = self.old_output[n][10];
	    //START FEED FORWARD

	//====================================================================
	    for layer in self.npc_NN[n].clone(){
		nn_data=self.feed_forward(layer,nn_data);
		nn_data=App::relu(nn_data)
	    }




	    nn_out_argmax=9;
	    move_flag=false;
	    for m in 0..nn_data.len() as usize{
		if nn_data[m]>=nn_data[nn_out_argmax] && nn_data[m]>0.0{
		    nn_out_argmax=m;
		    move_flag=true;
		}
		self.old_output[n][m]=nn_data[m];
	    }

	//====================================================================
	    if move_flag{
		if nn_out_argmax==9{
		    //turn counter clock wise
		    self.npc_dir_idx[n]+=1;
		    self.npc_dir_idx[n]%=7;
		    self.npc_dir[n][0]=turn_list[self.npc_dir_idx[n]][0];
		    self.npc_dir[n][1]=turn_list[self.npc_dir_idx[n]][1];

		}else if nn_out_argmax==8{
		    //turn clockwise
		    if self.npc_dir_idx[n]==0 {
			self.npc_dir_idx[n]=7;
		    }else{
			self.npc_dir_idx[n]-=1;
		    }
		    self.npc_dir[n][0]=turn_list[self.npc_dir_idx[n]][0];
		    self.npc_dir[n][1]=turn_list[self.npc_dir_idx[n]][1]; 
		}

		if nn_out_argmax==0{
		    self.npc_pos[n][0]+=1;
		    self.npc_pos[n][1]+=0;
		}else if nn_out_argmax==1{
		    self.npc_pos[n][0]+=1;
		    self.npc_pos[n][1]+=1;

		}else if nn_out_argmax==2{
		    self.npc_pos[n][0]+=1;
		    self.npc_pos[n][1]+=-1;

		}else if nn_out_argmax==3{
		    self.npc_pos[n][0]+=0;
		    self.npc_pos[n][1]+=1;

		}else if nn_out_argmax==4{
		    self.npc_pos[n][0]+=0;
		    self.npc_pos[n][1]+=-1;

		}else if nn_out_argmax==5{
		    self.npc_pos[n][0]+=-1;
		    self.npc_pos[n][1]+=1;

		}else if nn_out_argmax==6{
		    self.npc_pos[n][0]+=-1;
		    self.npc_pos[n][1]+=0;

		}else if nn_out_argmax==7{
		    self.npc_pos[n][0]+=-1;
		    self.npc_pos[n][1]+=-1;

		}

		if nn_out_argmax==10{
		    self.npc_max_acc[n]=(20-self.armor_AC[self.npc_armor_idx[n]])/2;
		    self.npc_speed[n][self.npc_dir_idx[n]]+= self.npc_max_acc[n];
		}
	    }
	    for k in 0..7{
		self.npc_pos[n][0]+=turn_list[k][0]*self.npc_speed[n][k];
		self.npc_pos[n][1]+=turn_list[k][1]*self.npc_speed[n][k];
		if self.npc_speed[n][k]>0{
		    self.npc_speed[n][k]-=1;
		}
	    }


	    if bounce_flag{
		//self.counter=((self.counter+self.seed)%7919);
		self.npc_pos[n][0]+=rng.gen_range(-1..1);
		self.npc_hp[n]-=1;
		//self.counter=((self.counter+self.seed)%7919);
		self.npc_pos[n][1]+=rng.gen_range(-1..1);
	    }
	    if self.npc_pos[n][0]>=self.tycoon_arena_dim[0]-1{
		self.npc_pos[n][0]=self.tycoon_arena_dim[0]-2;
	    }
	    if self.npc_pos[n][0]<1{
		self.npc_pos[n][0]=2;
	    }
	    if self.npc_pos[n][1]>=self.tycoon_arena_dim[1]-1{
		self.npc_pos[n][1]=self.tycoon_arena_dim[1]-2;
	    }
	    if self.npc_pos[n][1]<1{
		self.npc_pos[n][1]=2;
	    }
	    dist=(self.npc_pos[n][0]-target_pos[0]).pow(2)+
		(self.npc_pos[n][1]-target_pos[1]).pow(2);

	    //println!("CALLING ATTACK FUNCTION");
	    self.attack(n,target_idx);
	    self.npc_ticks_alive[n]+=1;

	    if self.npc_ticks_alive[n]>100{
		self.npc_hp[n]-=1;
	    }
	    
	}
        }//MAIN LOOP
        }
    }
    //===============================================================================
    pub fn save_app(&mut self, save_dir : String){
	for n in 0..self.npc_num {
	    let mut dir = format!("{}/{}.txt",save_dir,self.npc_name[n]);
	    let mut data_string = format!("{}\n{}",self.npc_name[n],self.npc_kill_count[n]);
	    for layer in &self.npc_NN[n] {
		for synapse in layer {
		    for cell in synapse{
			data_string=format!{"{}\n{}",data_string,cell};
		    }
		}
	    }
	    fs::write(dir,data_string);
    	}
    }
    pub fn load_app(&mut self,save_dir:String){
	let mut entries = fs::read_dir(save_dir).unwrap();
	let mut n = 0 as usize;
	for entry in entries {
	    let mut k= 0 as usize;
	    let mut str_vec = Vec::new();

	    if let Ok(lines) = App::read_lines(entry.unwrap().path()) {
		for line in lines.map_while(Result::ok){
		    if k>1{
			str_vec.push(line);
		    }else if k==0{
			self.npc_name[n]=line;
		    }else if k==1{
			self.npc_kill_count[n]=line.parse().unwrap();
		    }
		    k+=1;
		}
	    }

	    
	    
	    k=0;
	    for m in 0..self.npc_NN[n].len() {
		for i in 0..self.npc_NN[n][m].len() {
		    for j in 0..self.npc_NN[n][m][i].len(){
			self.npc_NN[n][m][i][j]=str_vec[k].parse().unwrap();
			k+=1;
		    }
		}
	    }
	    
	    n+=1;
        if n>=self.npc_num{
            break;
        }
    	}
	
    
    }
    //===============================================================================
    pub fn find_best_npc(&mut self){
	let mut kill_argmax = self.npc_kill_argmax;
	for n in 0..self.npc_num {
	    if self.npc_kill_count[n]>self.npc_kill_count[kill_argmax]{
		kill_argmax = n as usize;
	    }
	}
	self.npc_kill_argmax = kill_argmax; 
    }

    pub fn get_pop_stats(&mut self){
	//println!("{}",self.weapon_num);
	self.weapon_usage  =  vec![0.0;self.weapon_num];
	self.armor_usage = vec![0.0;self.armor_num];
	for n in 0..self.npc_num{
	    self.weapon_usage[self.npc_weapon_idx[n]]+=1.0;
	    self.armor_usage[self.npc_armor_idx[n]]+=1.0;
	}
	for n in 0..self.weapon_num{
	    self.weapon_usage[n] /= self.npc_num as f32;
	}
	for n in 0..self.armor_num{
	    self.armor_usage[n] /= self.npc_num as f32;
	}
    }
    //===================================================
    pub fn generate_new_name(&mut self, target_idx : usize){
	let mut name = "".to_string();

	let mut rng = rand::thread_rng();
	//==========================
	let first_chunk = vec![
	    "Spa",
	    "Gla",
	    "Ne",
	    "Cea",
	    "Jo",
	];

	let second_chunk = vec![
	    "tic",
	    "li",
	    "r",
	    "om",
	    "s",
	    "hn",
	];

	let third_chunk = vec![
	    "us",
	    "a",
	    "ar",
	    "",
	];

	name+=first_chunk[rng.gen_range(0..first_chunk.len())];
	name+=second_chunk[rng.gen_range(0..second_chunk.len())];
	name+=third_chunk[rng.gen_range(0..third_chunk.len())];
	
	//==========================
/*
	let vowels = vec!["a",
			  "e",
			  "i",
			  "o",
			  "u"];
	let cons = vec!["b",
			"c",
			"d",
			"j",
			"k",
			"l",
			"m",
			"p",
			"q",
			"r",
			"s",
			"t",
			"v",
			"w",
			"x",
			"y",
			"z",
			"th",
			"sh",
			"ph"];

	let mut rng = rand::thread_rng();
	while true{
	    name+=cons[rng.gen_range(0..cons.len())];
	    name+=vowels[rng.gen_range(0..vowels.len())];
	    if rng.gen_range(1..10)<3 {
		break;
	    }
    }
	*/
	self.npc_name[target_idx]=name;
    }
    //===================================================
    pub fn reward(&mut self,n:usize,c:f32){
	self.total_rewards[n]+=c;
    }
    pub fn attack(&mut self,attacker:usize,defender:usize){//function handles attacking
	let n = attacker;
	let target_idx = defender;
	let target_pos = vec![self.npc_pos[target_idx][0],self.npc_pos[target_idx][1]];
	let mut attack_roll : i32 = 0;
	let mut bleed_roll : i32 = 0;
	let dice = &self.weapon_die[self.npc_weapon_idx[n]];
	let mut rng = rand::thread_rng();
	//===========================
	//println!("ATTACK FUNCTION CALLED");
	if  self.npc_pos[n][0]+self.npc_dir[n][0] == target_pos[0] &&
	    self.npc_pos[n][1]+self.npc_dir[n][1] == target_pos[1] &&
	    (target_pos[0]+self.npc_dir[target_idx][0] != self.npc_pos[n][0] ||
	    target_pos[1]+self.npc_dir[target_idx][1] != self.npc_pos[n][1])
	{
	    //println!("ATTACKING!!!!!");
	    attack_roll=rng.gen_range(1..20);
	    if attack_roll>self.armor_AC[self.npc_armor_idx[target_idx]]{ 
		for d in 0..dice[0] as usize{
		    attack_roll+=rng.gen_range(1..dice[1])
		}
	    }

	    
	    self.npc_maim_count[n]+=attack_roll;
	    self.reward(n,attack_roll as f32);
	    self.npc_hp[target_idx]-=attack_roll;
	    if self.npc_hp[target_idx]<=0{//THIS IS THE CODE FOR IF A KILL IS SCORED

		self.reward(attacker,3.0*attack_roll as f32);
		
		self.npc_NN[target_idx]=self.npc_NN[n].clone();
		self.npc_kill_count[n]+=1;
		self.npc_max_hp[target_idx]=self.npc_max_hp[n];
		self.npc_weapon_idx[target_idx]=self.npc_weapon_idx[n];
		self.npc_kill_count[target_idx]=0;
		self.npc_maim_count[target_idx]=0;
		self.npc_owned[target_idx]=false;
		self.generate_new_name(target_idx);
		
	    }
	}
    }
    //===================================================
    pub fn tycoon_check_new_fight(&mut self){
	let mut new_fight_bool = false;
	let mut rand_usize = 0 as usize;
	let N:usize = self.npc_pos.len();
	//let A:usize = self.arena_num;

	let mut rng=rand::thread_rng();
	let turn_list:Vec<Vec<i32>> = vec![vec![1,0],
                    vec![1, 1],
                    vec![0, 1],
                    vec![-1, 1],
                    vec![-1, 0],
                    vec![-1, -1],
                    vec![0, -1],
                    vec![1, -1]];

	new_fight_bool=false;
	if self.npc_hp[self.tycoon_arena[0]]<=0 {
	    new_fight_bool=true;
	    for k in 0..self.npc_owned_list.len(){
		rand_usize=rng.gen_range(0..self.npc_owned_list.len());
		rand_usize=self.npc_owned_list[k];
		if !self.npc_fight_bool[rand_usize] {
		    self.npc_fight_bool[self.tycoon_arena[0]]=false;
		    self.tycoon_arena[0]=rand_usize;
		    self.npc_fight_bool[self.tycoon_arena[0]]=true;

		    self.npc_pos[rand_usize][0]=rng.gen_range(0..self.tycoon_arena_dim[0]);
		    self.npc_pos[rand_usize][1]=rng.gen_range(0..self.tycoon_arena_dim[1]);

		    self.npc_dir_idx[rand_usize]=rng.gen_range(0..turn_list.len());
		    self.npc_dir[rand_usize][0]=turn_list[self.npc_dir_idx[rand_usize]][0];
		    self.npc_dir[rand_usize][1]=turn_list[self.npc_dir_idx[rand_usize]][1];

		    self.npc_NN[rand_usize]=self.mutate(self.npc_NN[rand_usize].clone());

		    
		    break;
		}
	    }
	}
	if self.npc_hp[self.tycoon_arena[1]]<=0 {
	    new_fight_bool=true;
	    
	    for k in 0..self.npc_owned_list.len(){
		rand_usize=rng.gen_range(0..self.npc_owned_list.len());
		rand_usize=self.npc_owned_list[k];
		if !self.npc_fight_bool[rand_usize] {
		    self.npc_fight_bool[self.tycoon_arena[1]]=false;
		    self.tycoon_arena[1]=rand_usize;
		    self.npc_fight_bool[self.tycoon_arena[1]]=true;


		    self.npc_pos[rand_usize][0]=rng.gen_range(0..self.tycoon_arena_dim[0]);
		    self.npc_pos[rand_usize][1]=rng.gen_range(0..self.tycoon_arena_dim[1]);

		    self.npc_dir_idx[rand_usize]=rng.gen_range(0..turn_list.len());
		    self.npc_dir[rand_usize][0]=turn_list[self.npc_dir_idx[rand_usize]][0];
		    self.npc_dir[rand_usize][1]=turn_list[self.npc_dir_idx[rand_usize]][1];


		    //self.npc_dir_idx[rand_usize]=0;
		    //self.npc_dir[rand_usize][0]=turn_list[0][0];
		    //self.npc_dir[rand_usize][1]=turn_list[0][1];

		    self.npc_NN[rand_usize]=self.mutate(self.npc_NN[rand_usize].clone());
		    

		    break;
		  
		}
	    }
	}
	if new_fight_bool {
	    self.npc_hp[self.tycoon_arena[0]]=20;
	    //self.npc_kill_count[self.arenas[a][0]]=0;
	    self.npc_ticks_alive[self.tycoon_arena[0]]=0;

	    self.npc_hp[self.tycoon_arena[1]]=20;
	    //self.npc_kill_count[self.arenas[a][1]]=0;
	    self.npc_ticks_alive[self.tycoon_arena[1]]=0;





	    self.npc_target_idx[self.tycoon_arena[0]]=self.tycoon_arena[1];
	    self.npc_target_idx[self.tycoon_arena[1]]=self.tycoon_arena[0];

	    self.npc_max_hp[self.tycoon_arena[0]]=10;
	    self.npc_max_hp[self.tycoon_arena[1]]=10;


	    }
	
    }
    pub fn check_new_fight(&mut self){
	let mut new_fight_bool = false;
	let mut rand_usize = 0 as usize;
	let N:usize = self.npc_pos.len();
	let A:usize = self.arena_num;

	let mut rng=rand::thread_rng();
	let turn_list:Vec<Vec<i32>> = vec![vec![1,0],
                    vec![1, 1],
                    vec![0, 1],
                    vec![-1, 1],
                    vec![-1, 0],
                    vec![-1, -1],
                    vec![0, -1],
                    vec![1, -1]];

	for a in 0..A {
	    new_fight_bool=false;
	    if self.npc_hp[self.arenas[a][0]]<=0 {
		new_fight_bool=true;
		while(true){
		    rand_usize=rng.gen_range(0..self.npc_num);
		    if !self.npc_fight_bool[rand_usize] {
			self.npc_fight_bool[self.arenas[a][0]]=false;
			self.arenas[a][0]=rand_usize;
			self.npc_fight_bool[self.arenas[a][0]]=true;

			self.npc_pos[rand_usize][0]=rng.gen_range(0..self.board_dim[0]);
			self.npc_pos[rand_usize][1]=rng.gen_range(0..self.board_dim[1]);

			self.npc_dir_idx[rand_usize]=rng.gen_range(0..turn_list.len());
			self.npc_dir[rand_usize][0]=turn_list[self.npc_dir_idx[rand_usize]][0];
			self.npc_dir[rand_usize][1]=turn_list[self.npc_dir_idx[rand_usize]][1];

			self.npc_NN[rand_usize]=self.mutate(self.npc_NN[rand_usize].clone());

			if rng.gen_range(0..13)>10 {
			    self.npc_weapon_idx[self.arenas[a][0]]=rng.gen_range(0..self.weapon_num);
			    self.npc_armor_idx[self.arenas[a][1]]=rng.gen_range(0..self.armor_num);
			}

			self.npc_Q_table[rand_usize]=vec![vec![0.0;11];11];
			for x in 0..11{
			    self.npc_Q_table[rand_usize][x][x]=1.0;
			}
			break;
		    }
		}
	    }
	    if self.npc_hp[self.arenas[a][1]]<=0 {
		new_fight_bool=true;
		while(true){
		    rand_usize=rng.gen_range(0..self.npc_num);
		    if !self.npc_fight_bool[rand_usize] {
			self.npc_fight_bool[self.arenas[a][1]]=false;
			self.arenas[a][1]=rand_usize;
			self.npc_fight_bool[self.arenas[a][1]]=true;


			self.npc_pos[rand_usize][0]=rng.gen_range(0..self.board_dim[0]);
			self.npc_pos[rand_usize][1]=rng.gen_range(0..self.board_dim[1]);

			self.npc_dir_idx[rand_usize]=rng.gen_range(0..turn_list.len());
			self.npc_dir[rand_usize][0]=turn_list[self.npc_dir_idx[rand_usize]][0];
			self.npc_dir[rand_usize][1]=turn_list[self.npc_dir_idx[rand_usize]][1];

			
			//self.npc_dir_idx[rand_usize]=0;
			//self.npc_dir[rand_usize][0]=turn_list[0][0];
			//self.npc_dir[rand_usize][1]=turn_list[0][1];

			self.npc_NN[rand_usize]=self.mutate(self.npc_NN[rand_usize].clone());
			if rng.gen_range(0..13)>11 {
			    self.npc_weapon_idx[self.arenas[a][1]]=rng.gen_range(0..self.weapon_num);
			    self.npc_armor_idx[self.arenas[a][1]]=rng.gen_range(0..self.armor_num);
			}

			
			self.npc_Q_table[rand_usize]=vec![vec![0.0;11];11];
			for x in 0..11{
			    self.npc_Q_table[rand_usize][x][x]=1.0;
			}

			break;
		    }
		}
	    }
	    if new_fight_bool {
		self.npc_hp[self.arenas[a][0]]=20;
		//self.npc_kill_count[self.arenas[a][0]]=0;
		self.npc_ticks_alive[self.arenas[a][0]]=0;

		self.npc_hp[self.arenas[a][1]]=20;
		//self.npc_kill_count[self.arenas[a][1]]=0;
		self.npc_ticks_alive[self.arenas[a][1]]=0;

		
		
		
            
		self.npc_target_idx[self.arenas[a][0]]=self.arenas[a][1];
		self.npc_target_idx[self.arenas[a][1]]=self.arenas[a][0];

		self.npc_max_hp[self.arenas[a][0]]=10;
		self.npc_max_hp[self.arenas[a][1]]=10;


		}
	}
    }
    //===================================================
    pub fn mode_switch(&mut self){
	if self.main_menu_mode {
	    

	    if self.counter==0 {	
		self.load_weapon();
		self.load_armor();
	    }else if self.counter==1{
		self.sim_mode=true;
		self.main_menu_mode=false;

		self.arenas = vec![vec![0,1],vec![2,3],vec![4,5],vec![6,7],vec![7,8],
					  vec![9,10],vec![11,12],vec![13,14],vec![15,16],vec![17,18],
					  vec![19,20],vec![21,22],vec![23,24],vec![25,26]];
		self.npc_fight_bool = vec![false ; self.npc_num];
		for n in 0..self.npc_num{
		    self.npc_pos[n][0]=1;
		    self.npc_pos[n][1]=1;
		}
		for k in 0..2*self.arena_num {
		    self.npc_fight_bool[k]=true; 
		}

	    }else if self.counter==2{
		self.load_app("Saves/".to_string());
	    }else if self.counter==3{
		self.tycoon_mode=true;
		self.main_menu_mode=false;
		for n in 0..self.npc_num{
		    self.npc_fight_bool[n]=false;
		}
		
	    }
	}else if self.tycoon_mode && !self.npc_list_mode{
	    

	    if self.counter==0{
		//call gladiator list mode with selection idx
		self.npc_list_mode=true;
		self.buy_mode=true;
		self.counter=0;
	    }else if self.counter==1{
		//same thing
		self.npc_list_mode=true;
		self.sell_mode=true;
		self.counter=0;
	    }else if self.counter==2{
		self.ticket_price+=1;
	    }else if self.counter==3{
		self.ticket_price-=1;
		if self.ticket_price<0{
		    self.ticket_price=0;
		}
	    }else if self.counter==4{
		self.npc_list_mode=true;
		self.schedule_mode=true;
	    }
	}else if self.tycoon_mode && self.npc_list_mode{
	    if self.buy_mode && self.npc_price_list[self.npc_kill_sort[self.counter as usize]]<=self.gold
	    {
		self.npc_owned[self.npc_kill_sort[self.counter as usize]]=true;
		
		self.gold-=self.npc_price_list[self.npc_kill_sort[self.counter as usize]];
		for n in 0..self.npc_num{
		    self.npc_kill_sort[n]=n;
		    if self.npc_owned[n]{
			self.npc_owned_list.push(n);
		    }
		}
		self.tycoon_check_new_fight();
	    }
	    if self.sell_mode && self.npc_owned[self.npc_kill_sort[self.counter as usize]]{
		self.npc_owned[self.npc_kill_sort[self.counter as usize]]=false;
		self.gold+=self.npc_price_list[self.npc_kill_sort[self.counter as usize]];
		for n in 0..self.npc_num{
		    self.npc_kill_sort[n]=n;
		    if self.npc_owned[n]{
			self.npc_owned_list.push(n);
		    }
		}
		self.tycoon_check_new_fight();
	    }
	}
    }
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>,{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
    }

    pub fn load_armor(&mut self){
	// Create a path to the desired file
	let mut entries = fs::read_dir("Armor/").unwrap();
	self.armor_num = entries.count();
	//println!("{}",self.weapon_num);
	entries = fs::read_dir("Armor/").unwrap();
	self.armor_str=vec!["".to_string();self.armor_num];
	self.armor_AC=vec![0;self.armor_num];
	let mut n = 0;
	let mut m = 0;
	let mut die_line:Vec<&str>=vec![""];
	for entry in entries {

	    //let mut file=File::open(entry.unwrap().path());
	    n=0;
	    if let Ok(lines) = App::read_lines(entry.unwrap().path()) {
		for line in lines.map_while(Result::ok){
		    if n==0 {
			//load descriptive string
			self.armor_str[m]=line.to_string();
			//println!("{}",self.weapon_str[m]);
		    }else if n==1{
			//load dice
			//die_line=line.split("d").collect();
			//println!("{}",die_line[0]);
			//println!("{}",die_line[1]);
			
			self.armor_AC[m]=line.to_string().parse().unwrap();
		    }
		    n+=1;
		}
	    }
	    m+=1;
	}
	// `file` goes out of scope, and the "hello.txt" file gets closed
    }

    pub fn load_weapon(&mut self){
	// Create a path to the desired file
	let mut entries = fs::read_dir("Weapons/").unwrap();
	self.weapon_num = entries.count();
	//println!("{}",self.weapon_num);
	entries = fs::read_dir("Weapons/").unwrap();
	self.weapon_str=vec!["".to_string();self.weapon_num];
	self.weapon_die=vec![vec![1,1];self.weapon_num];
	let mut n = 0;
	let mut m = 0;
	let mut die_line:Vec<&str>=vec![""];
	for entry in entries {

	    //let mut file=File::open(entry.unwrap().path());
	    n=0;
	    if let Ok(lines) = App::read_lines(entry.unwrap().path()) {
		for line in lines.map_while(Result::ok){
		    if n==0 {
			//load descriptive string
			self.weapon_str[m]=line.to_string();
			//println!("{}",self.weapon_str[m]);
		    }else if n==1{
			//load dice
			die_line=line.split("d").collect();
			//println!("{}",die_line[0]);
			//println!("{}",die_line[1]);
			self.weapon_die[m]=vec![die_line[0].parse().unwrap()
						,die_line[1].parse().unwrap()];
		    }
		    n+=1;
		}
	    }
	    m+=1;
	}
	// `file` goes out of scope, and the "hello.txt" file gets closed
    }
    //===================================================
    pub fn brute_modulo(x:i32,c:i32) -> i32{//THIS IS STUPID!!!! OPTIMIZE LATER
        let mut y = x;
        while y>=c{
            y-=c;
        }
        while y<0{
            y+=c;
        }
        y
    }
    ///Functions for Nueral Network
    pub fn relu(x:Vec<f32>) -> Vec<f32>{
        let mut y:Vec<f32> = vec![0.0;x.len()];
        let N=x.len() as usize;
        for n in 0..N{
            if(x[n]>0.0){
            y[n]=x[n];
            }
        }
        y
    }
    pub fn tanh(x:Vec<f32>) -> Vec<f32>{
        let mut y:Vec<f32> = vec![0.0;x.len()];
        let N=x.len() as usize;
        for n in 0..N{
            y[n]=(x[n].exp() - (-x[n]).exp())/(x[n].exp()+(-x[n]).exp())
        }
        y
    }
    pub fn feed_forward(&mut self,Layer:Vec<Vec<f32>>,x:Vec<f32>) -> Vec<f32>{
        let mut y:Vec<f32> = vec![0.0 as f32;Layer.len()];
        let mut n = 0 as usize;
        for row in Layer{
            y[n]=self.dot_product(row,x.clone());
            n+=1;
        }
        y
    }

    pub fn mutate(&mut self,Network:Vec<Vec<Vec<f32>>>)->Vec<Vec<Vec<f32>>>{
        let mut rng = rand::thread_rng();
        let mut out_net=Network.clone();
        let K=Network.len() as usize;
        let mut N = 0 as usize;
        let mut M = 0 as usize;
        for k in 0..K {
            N=Network[k].len() as usize;
            for n in 0..N{
            M=Network[k][n].len() as usize;
            for m in 0..M{
                //entry+=random float
                out_net[k][n][m]+=rng.gen_range(-0.1..0.1);
	
            }
            }
        }
        out_net
    }
    
    pub fn transpose(&mut self,A:Vec<Vec<f32>>) -> Vec<Vec<f32>>{
        let mut A_T =vec![vec![0 as f32; A.len()];A[0].len()];
        let N = A.len() as usize;
        let M = A[0].len() as usize;

        for n in 0..N{
            for m in 0..M{
                A_T[m][n]=A[n][m];
            }
        }
        A_T
    }

    pub fn matmul(&mut self,A:Vec<Vec<f32>>,B:Vec<Vec<f32>>) -> Vec<Vec<f32>>{
        let B_T=self.transpose(B);
	
        let mut C =vec![vec![0 as f32; B_T.len()];A.len()];
        if(A[0].len() == B_T[0].len()){
            let N = A.len() as usize;
            let M = B_T.len() as usize;
               
            for n in 0..N{
                for m in 0..M{
                   C[n][m]=self.dot_product(A[n].clone(),B_T[m].clone());
                }
            }
        }
        C
    }

    pub fn finite_linear_map(&mut self,A:Vec<Vec<f32>>,x:Vec<f32>) -> Vec<f32>{
	let mut b = vec![0 as f32;A.len()];
	for n in 0..b.len(){
	    b[n]=self.dot_product(A[n].clone(),x.clone());
	}

	b
    }

    pub fn dot_product(&mut self,u:Vec<f32>,v:Vec<f32>) -> f32{
        //U and V should be vectors of the same size
        let mut sum : f32 = 0.0;
        if(u.len()==v.len()){
           let mut iter = u.iter().zip(v.iter());
           for i in iter{
                sum+=i.0*i.1;
            }

        }
        sum
    }

    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Increment => self.increment_counter(),
                AppEvent::Decrement => self.decrement_counter(),
                AppEvent::Quit => self.quit(),
		AppEvent::LoadWeapon => self.load_weapon(),
		AppEvent::SelectOption => self.mode_switch(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Down => self.events.send(AppEvent::Increment),
            KeyCode::Up => self.events.send(AppEvent::Decrement),
            // Other handlers you could add here.
	    KeyCode::Enter => self.events.send(AppEvent::SelectOption),
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn pass_to_Q(&mut self,n:usize,data:Vec<f32>) -> Vec<f32>{
	let Q_T=self.transpose(self.npc_Q_table[n].clone());
	self.finite_linear_map(Q_T.clone(),data)
    }
    pub fn pass_to_NN(&mut self, n : usize) -> Vec<f32>{
	let mut nn_data=vec![0.0;40];
	let mut target_pos=vec![0,0];
	let mut target_idx:usize=0;

	let mut v:Vec<i32>=vec![0,0];


	//self.npc_target_idx[n]=arg_min_dist;

	target_idx=self.npc_target_idx[n];
	target_pos[0]=self.npc_pos[target_idx][0];
	target_pos[1]=self.npc_pos[target_idx][1];
				//Neural Net For Movement



	    //nn_data is a 11 entry long vector of f32 values that serves as
	    //the input and output variable for the neural network
	nn_data[0]=(self.npc_pos[n][0] - target_pos[0]) as f32;
	nn_data[1]=(self.npc_pos[n][1] - target_pos[1]) as f32;
	nn_data[18]=nn_data[0].powf(2.0) + nn_data[1].powf(2.0);

	v[0]=(self.npc_pos[n][0]+1)%self.board_dim[0];
	v[1]=(self.npc_pos[n][1]+0)%self.board_dim[1];
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[2]=1.0;
	}else{
	    nn_data[2]=0.0;
	}


	v[0]=(self.npc_pos[n][0]+1)%self.board_dim[0];
	v[1]=(self.npc_pos[n][1]+1)%self.board_dim[1];
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[3]=1.0;
	}else{
	    nn_data[3]=0.0;
	}


	v[0]=(self.npc_pos[n][0]+1)%self.board_dim[0];
	v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.board_dim[1]);
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[4]=1.0;
	}else{
	    nn_data[4]=0.0;
	}


	v[0]=(self.npc_pos[n][0]+0)%self.board_dim[0];
	v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.board_dim[1]);
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[5]=1.0;
	}else{
	    nn_data[5]=0.0;
	}


	v[0]=App::brute_modulo(self.npc_pos[n][0]-1,self.board_dim[0]);
	v[1]=(self.npc_pos[n][1]+1)%self.board_dim[1];
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[6]=1.0;
	}else{
	    nn_data[6]=0.0;
	}

	v[0]=App::brute_modulo(self.npc_pos[n][0]-1,self.board_dim[0]);
	v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.board_dim[1]);

	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[7]=1.0;
	}else{
	    nn_data[7]=0.0;
	}


	v[0]=(self.npc_pos[n][0]+0)%self.board_dim[0];
	v[1]=(self.npc_pos[n][1]+1)%self.board_dim[1];
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[8]=1.0;
	}else{
	    nn_data[8]=0.0;
	}


	v[0]=(self.npc_pos[n][0]+0)%self.board_dim[0];
	v[1]=App::brute_modulo(self.npc_pos[n][1]-1,self.board_dim[1]);
	if self.board_pop[v[0] as usize][v[1] as usize] !=0.0{
	    nn_data[9]=1.0;
	}else{
	    nn_data[9]=0.0;
	}

	nn_data[10]=0.0;
	nn_data[11]=0.0;
	nn_data[12]=0.0;
	nn_data[13]=0.0;
	nn_data[14]=0.0;
	nn_data[15]=0.0;
	nn_data[16]=0.0;
	nn_data[17]=0.0;

	nn_data[self.npc_dir_idx[n] +10]=1.0;


	nn_data[19]=0.0;
	nn_data[20]=0.0;
	nn_data[21]=0.0;
	nn_data[22]=0.0;
	nn_data[23]=0.0;
	nn_data[24]=0.0;
	nn_data[25]=0.0;
	nn_data[26]=0.0;

	nn_data[self.npc_dir_idx[target_idx] +19]=1.0;

	nn_data[27] = self.npc_hp[n] as f32;
	nn_data[28] = self.npc_hp[target_idx] as f32;

	nn_data[29] = self.old_output[n][0];
	nn_data[30] = self.old_output[n][1];
	nn_data[31] = self.old_output[n][2];
	nn_data[32] = self.old_output[n][3];
	nn_data[33] = self.old_output[n][4];
	nn_data[34] = self.old_output[n][5];
	nn_data[35] = self.old_output[n][6];
	nn_data[36] = self.old_output[n][7];
	nn_data[37] = self.old_output[n][8];
	nn_data[38] = self.old_output[n][9];
	nn_data[39] = self.old_output[n][10];
	    //START FEED FORWARD

	    //FORWARD NN_DATA INTO MACHINE LEARNING ALGORITHM
	//====================================================================
	for layer in self.npc_NN[n].clone(){
	    nn_data=self.feed_forward(layer,nn_data);
	    nn_data=App::relu(nn_data)
	}



	nn_data

		       
    }

		   
    pub fn tick(&mut self) {
    let mut rng=rand::thread_rng();
	//self.counter=self.counter.saturating_add(1)
    let N:usize = self.npc_pos.len();
    let A:usize = self.arena_num;
    let mut target_pos=vec![0,0];
    let mut target_idx:usize=0;
    let mut v:Vec<i32>=vec![0,0];
    let mut dist : f32 = 0.0;
    let mut bounce_flag:bool=false;
    let mut attack_roll:i32=0;
    let mut dir:Vec<i32> = vec![0,0];
    let mut x:i32 = 0;

	let mut nn_data:Vec<f32>=vec![0.0;40];
	let mut old_data:Vec<f32>=Vec::new();
	let mut old_argmax = 0 as usize;
	let mut action_idx=0 as usize;
let mut nn_out_argmax = 0 as usize;
    let mut move_flag :bool = false;
    let turn_list:Vec<Vec<i32>> = vec![vec![1,0],
                    vec![1, 1],
                    vec![0, 1],
                    vec![-1, 1],
                    vec![-1, 0],
                    vec![-1, -1],
                    vec![0, -1],
                    vec![1, -1]];
    let mut arg_min_dist = 0 as usize;
	let mut min_dist = 10000.0 as f32;
	let mut new_min_dist = 10000.0 as f32;
    let mut rand_usize : usize = 0 as usize;
	let mut new_fight_bool : bool = true;
	
	//=================================================================
	//Reset Rewards
	self.total_rewards=vec![0.0;self.npc_num];
	
    //================================================================
	if self.main_menu_mode{
	    if self.counter<0{
		self.counter=0;
	    }   //self.counter=self.counter.saturating_add(1)
	    if self.counter>3{
		self.counter=3;
	    }
	
	   
	}else if self.tycoon_mode && !self.npc_list_mode{
	    if self.counter<0{
		self.counter=0;
	    }   //self.counter=self.counter.saturating_add(1)
	    if self.counter>4{
		self.counter=4;
	    }
	}else if self.tycoon_mode && self.npc_list_mode{
	    if self.counter<0{
		self.counter=0;
	    }
	    if self.counter>24{
		self.counter=24;
	    }
	}


    //================================================================
	if self.sim_mode {
	    self.get_pop_stats();	
	    self.check_new_fight(); 
	    self.find_best_npc();
	    for a in 0..A {
		self.npc_target_idx[self.arenas[a][0]]=self.arenas[a][1];
		self.npc_target_idx[self.arenas[a][1]]=self.arenas[a][0];


		self.board_pop=vec![vec![0.0;self.board_dim[1] as usize];self.board_dim[0] as usize];
	
		for n in self.arenas[a].clone(){//loop through all npcs
		    for n in self.arenas[a].clone(){
			self.board_pop[self.npc_pos[n][0] as usize][self.npc_pos[n][1] as usize]=1.0;
		    }
		    bounce_flag=false;
		    //==================================Check Bounce
		    arg_min_dist=0;
		    min_dist=10000.0;
		    for m in self.arenas[a].clone(){
			dist=((self.npc_pos[m][0]-self.npc_pos[n][0]).pow(2) +
				    (self.npc_pos[m][1]-self.npc_pos[n][1]).pow(2)) as f32;
				if dist<min_dist && m != n{
					min_dist=dist;
					arg_min_dist=m;
				}
			if(dist==0.0 && m != n){
			    bounce_flag=true;
				break;
			}
		    }

		    if self.npc_hp[n]>0{
			nn_data = self.pass_to_NN(n);
			old_data=nn_data.clone();
			nn_data = self.pass_to_Q(n,nn_data);
		    
			nn_out_argmax=9;
			move_flag=false;
			for m in 0..nn_data.len() as usize{
			    if nn_data[m]>=nn_data[nn_out_argmax] && nn_data[m]>0.0{
				nn_out_argmax=m;
				move_flag=true;
			    }
			    self.old_output[n][m]=nn_data[m];//old output for feeding back in
			}


			//====================================================================
			//PERFORM ACTION IN THE ENVIROMENT
			if move_flag{
			    if nn_out_argmax==9{
				//turn counter clock wise
				self.npc_dir_idx[n]+=1;
				self.npc_dir_idx[n]%=7;
				self.npc_dir[n][0]=turn_list[self.npc_dir_idx[n]][0];
				self.npc_dir[n][1]=turn_list[self.npc_dir_idx[n]][1];

			    }else if nn_out_argmax==8{
				//turn clockwise
				if self.npc_dir_idx[n]==0 {
				    self.npc_dir_idx[n]=7;
				}else{
				    self.npc_dir_idx[n]-=1;
				}
				self.npc_dir[n][0]=turn_list[self.npc_dir_idx[n]][0];
				self.npc_dir[n][1]=turn_list[self.npc_dir_idx[n]][1]; 
			    }

			    if nn_out_argmax==0{
				self.npc_pos[n][0]+=1;
				self.npc_pos[n][1]+=0;
			    }else if nn_out_argmax==1{
				self.npc_pos[n][0]+=1;
				self.npc_pos[n][1]+=1;

			    }else if nn_out_argmax==2{
				self.npc_pos[n][0]+=1;
				self.npc_pos[n][1]+=-1;

			    }else if nn_out_argmax==3{
				self.npc_pos[n][0]+=0;
				self.npc_pos[n][1]+=1;

			    }else if nn_out_argmax==4{
				self.npc_pos[n][0]+=0;
				self.npc_pos[n][1]+=-1;

			    }else if nn_out_argmax==5{
				self.npc_pos[n][0]+=-1;
				self.npc_pos[n][1]+=1;

			    }else if nn_out_argmax==6{
				self.npc_pos[n][0]+=-1;
				self.npc_pos[n][1]+=0;

			    }else if nn_out_argmax==7{
				self.npc_pos[n][0]+=-1;
				self.npc_pos[n][1]+=-1;

			    }

			    if nn_out_argmax==10{
				self.npc_max_acc[n]=(20-self.armor_AC[self.npc_armor_idx[n]])/2;
				self.npc_speed[n][self.npc_dir_idx[n]]+= self.npc_max_acc[n];
			    }
			}
			for k in 0..7{
			    self.npc_pos[n][0]+=turn_list[k][0]*self.npc_speed[n][k];
			    self.npc_pos[n][1]+=turn_list[k][1]*self.npc_speed[n][k];
			    if self.npc_speed[n][k]>0{
				self.npc_speed[n][k]-=1;
			    }
			}


			if bounce_flag{
			    //self.counter=((self.counter+self.seed)%7919);
			    self.npc_pos[n][0]+=rng.gen_range(-1..1);
			    self.npc_hp[n]-=1;
			    //self.counter=((self.counter+self.seed)%7919);
			    self.npc_pos[n][1]+=rng.gen_range(-1..1);
			}
			if self.npc_pos[n][0]>=self.board_dim[0]-1{
			    self.npc_pos[n][0]=self.board_dim[0]-2;
			}
			if self.npc_pos[n][0]<1{
			    self.npc_pos[n][0]=2;
			}
			if self.npc_pos[n][1]>=self.board_dim[1]-1{
			    self.npc_pos[n][1]=self.board_dim[1]-2;
			}
			if self.npc_pos[n][1]<1{
			    self.npc_pos[n][1]=2;
			}
			dist=((self.npc_pos[n][0]-target_pos[0]).pow(2)+
			    (self.npc_pos[n][1]-target_pos[1]).pow(2))as f32;

			//println!("CALLING ATTACK FUNCTION");
			self.attack(n,target_idx);
			    self.npc_ticks_alive[n]+=1;
			//================================================
			//Give Rewards based on distance
			new_min_dist=10000.0;
			for m in self.arenas[a].clone(){
			    dist=(self.npc_pos[m][0]-self.npc_pos[n][0]).pow(2) as f32+
					(self.npc_pos[m][1]-self.npc_pos[n][1]).pow(2) as f32;
				    if dist<new_min_dist && m != n{
					    new_min_dist=dist;
				    } 
			}
			if new_min_dist<min_dist{
			    self.reward(n,1.0);
			}	
			    //==================================================================================
			    //FEED NEW STATE INTO NEURAL NETWORK AND UPDATE Q-LEARNING ALGO
			//==================================================================================
			//reset board positions
			 for n in self.arenas[a].clone(){
			     self.board_pop[self.npc_pos[n][0] as usize][self.npc_pos[n][1] as usize]=1.0;
			 }
			//collect action and state indicies for Q-Table update
			action_idx=nn_out_argmax.clone();
			old_argmax=0;
			for m in 0..old_data.len() as usize{
			    if old_data[m]>=old_data[old_argmax]{
				
				old_argmax=m;	
			    }
			}
			
			nn_data=self.pass_to_NN(n);
			nn_data=self.pass_to_Q(n,nn_data);
			nn_out_argmax=9;

			for m in 0..nn_data.len() as usize{
			    if nn_data[m]>=nn_data[nn_out_argmax]{
				nn_out_argmax=m;	
			    }
			}
			self.update_reward_table(n,//npc index
						 old_argmax,//old argmax from initial Neural Net Pass
						 nn_out_argmax,//new argmax from new Neural Net Pass
						 action_idx);//Action index, i.e. the output from the Q-Table

			if self.npc_ticks_alive[n]>100{
			    self.npc_hp[n]-=1;
			}
		}
	    }//MAIN LOOP
	    }//arena Loop
        }else if self.tycoon_mode{
	    self.tycoon_tick();
	}//if sim mode vs tycoon mode
    }//tick function

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
	//self.sim_mode = false;
	if self.sim_mode{
	    self.save_app("Saves/".to_string());
	}
	//self.running = false;
	if self.main_menu_mode{
	    self.running = false;
	}else if self.npc_list_mode{
	    self.npc_list_mode=false;
	    self.tycoon_mode=true;
	    self.buy_mode=false;
	    self.sell_mode=false;
	    self.schedule_mode=false;
	}else if self.sim_mode{
	    self.sim_mode=false;
	    self.main_menu_mode=true;
	}else if self.tycoon_mode{
	    self.tycoon_mode=false;
	    self.main_menu_mode=true;
	}
	
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
