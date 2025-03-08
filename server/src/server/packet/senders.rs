use std::net::TcpStream;

use lib::serialize;
use lib::utils::send_packet;

pub mod status {
  use super::*;
  
  pub fn status_response(stream: &mut TcpStream) {
    send_packet(stream, 0x00, 
      serialize::string(
        "{\"version\": {\"name\": \"Oxide 1.21.4\",\"protocol\": 769},\"players\": {\"max\": 9,\"online\": 6,\"sample\": []},\"description\": {\"text\": \"Hello oxide!\"},\"enforcesSecureChat\": true}"
      )
    );
  }
}

pub mod login {
  use super::*;

  pub fn login_success(stream: &mut TcpStream) {
    let packet = lib::packets::clientbound::login::LoginSuccess {
      uuid: 290780920670370370148908686767547353505,
      username: "The__TxT".to_string(),
    };

    send_packet(stream, 0x02, packet.try_into().unwrap());
  }
}

pub mod play {
  use super::*;
  use lib::nbt::NbtTag;

  pub fn login(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();

    output.push(0);
    output.push(0);
    output.push(0);
    output.push(0xdf);
    
    output.push(0);
    output.push(1);
    output.push(255);

    output.push(3);
    output.append(&mut serialize::string("minecraft:overworld"));
    output.append(&mut serialize::string("minecraft:the_nether"));
    output.append(&mut serialize::string("minecraft:the_end"));

    let mut registry_codec = serialize::nbt(NbtTag::TagCompound(Some("".to_string()), vec![
      NbtTag::TagCompound(Some("minecraft:chat_type".to_string()), vec![
        NbtTag::String(Some("type".to_string()), "minecraft:chat_type".to_string()),
        NbtTag::List(Some("value".to_string()), vec![
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::TagCompound(Some("chat".to_string()), vec![
                NbtTag::List(Some("parameters".to_string()), vec![
                  NbtTag::String(None, "sender".to_string()),
                  NbtTag::String(None, "content".to_string()),
                ]),
                NbtTag::String(Some("translation_key".to_string()), "chat.type.text".to_string())
              ]),
              NbtTag::TagCompound(Some("narration".to_string()), vec![
                NbtTag::List(Some("parameters".to_string()), vec![
                  NbtTag::String(None, "sender".to_string()),
                  NbtTag::String(None, "content".to_string())
                ]),
                NbtTag::String(Some("translation_key".to_string()), "chat.type.text.narrate".to_string())
              ])
            ]),
            NbtTag::Int(Some("id".to_string()), 0),
            NbtTag::String(Some("name".to_string()), "minecraft:chat".to_string())
          ])
        ])
      ]),
      NbtTag::TagCompound(Some("minecraft:dimension_type".to_string()), vec![
        NbtTag::String(Some("type".to_string()), "minecraft:dimension_type".to_string()),
        NbtTag::List(Some("value".to_string()), vec![
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::Float(Some("ambient_light".to_string()), 0.0),
              NbtTag::Byte(Some("bed_works".to_string()), 1),
              NbtTag::Double(Some("coordinate_scale".to_string()), 1.0),
              NbtTag::String(Some("effects".to_string()), "minecraft:overworld".to_string()),
              NbtTag::Byte(Some("has_ceiling".to_string()), 0),
              NbtTag::Byte(Some("has_raids".to_string()), 1),
              NbtTag::Byte(Some("has_skylight".to_string()), 1),
              NbtTag::Int(Some("height".to_string()), 384),
              NbtTag::String(Some("infiniburn".to_string()), "#minecraft:infiniburn_overworld".to_string()),
              NbtTag::Int(Some("logical_height".to_string()), 384),
              NbtTag::Int(Some("min_y".to_string()), -64),
              NbtTag::Int(Some("monster_spawn_block_light_limit".to_string()), 0),
              NbtTag::TagCompound(Some("monster_spawn_light_level".to_string()), vec![
                NbtTag::String(Some("type".to_string()), "minecraft:uniform".to_string()),
                NbtTag::TagCompound(Some("value".to_string()), vec![
                  NbtTag::Int(Some("max_inclusive".to_string()), 7),
                  NbtTag::Int(Some("min_inclusive".to_string()), 0)
                ])
              ]),
              NbtTag::Byte(Some("natural".to_string()), 1),
              NbtTag::Byte(Some("piglin_safe".to_string()), 0),
              NbtTag::Byte(Some("respawn_anchor_works".to_string()), 0),
              NbtTag::Byte(Some("ultrawarm".to_string()), 0)
            ]),
            NbtTag::Int(Some("id".to_string()), 0),
            NbtTag::String(Some("name".to_string()), "minecraft:overworld".to_string()),
          ]),
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::Float(Some("ambient_light".to_string()), 0.1),
              NbtTag::Byte(Some("bed_works".to_string()), 0),
              NbtTag::Double(Some("coordinate_scale".to_string()), 8.0),
              NbtTag::String(Some("effects".to_string()), "minecraft:the_nether".to_string()),
              NbtTag::Long(Some("fixed_time".to_string()), 18000),
              NbtTag::Byte(Some("has_ceiling".to_string()), 1),
              NbtTag::Byte(Some("has_raids".to_string()), 0),
              NbtTag::Byte(Some("has_skylight".to_string()), 0),
              NbtTag::Int(Some("height".to_string()), 256),
              NbtTag::String(Some("infiniburn".to_string()), "#minecraft:infiniburn_nether".to_string()),
              NbtTag::Int(Some("logical_height".to_string()), 128),
              NbtTag::Int(Some("min_y".to_string()), 0),
              NbtTag::Int(Some("monster_spawn_block_light_limit".to_string()), 15),
              NbtTag::Int(Some("monster_spawn_light_level".to_string()), 11),
              NbtTag::Byte(Some("natural".to_string()), 0),
              NbtTag::Byte(Some("piglin_safe".to_string()), 1),
              NbtTag::Byte(Some("respawn_anchor_works".to_string()), 1),
              NbtTag::Byte(Some("ultrawarm".to_string()), 1)
            ]),
            NbtTag::Int(Some("id".to_string()), 1),
            NbtTag::String(Some("name".to_string()), "minecraft:the_nether".to_string()),
          ]),
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::Float(Some("ambient_light".to_string()), 0.0),
              NbtTag::Byte(Some("bed_works".to_string()), 0),
              NbtTag::Double(Some("coordinate_scale".to_string()), 1.0),
              NbtTag::String(Some("effects".to_string()), "minecraft:the_end".to_string()),
              NbtTag::Long(Some("fixed_time".to_string()), 6000),
              NbtTag::Byte(Some("has_ceiling".to_string()), 0),
              NbtTag::Byte(Some("has_raids".to_string()), 1),
              NbtTag::Byte(Some("has_skylight".to_string()), 0),
              NbtTag::Int(Some("height".to_string()), 256),
              NbtTag::String(Some("infiniburn".to_string()), "#minecraft:infiniburn_end".to_string()),
              NbtTag::Int(Some("logical_height".to_string()), 256),
              NbtTag::Int(Some("min_y".to_string()), 0),
              NbtTag::Int(Some("monster_spawn_block_light_limit".to_string()), 0),
              NbtTag::TagCompound(Some("monster_spawn_light_level".to_string()), vec![
                NbtTag::String(Some("type".to_string()), "minecraft:uniform".to_string()),
                NbtTag::TagCompound(Some("value".to_string()), vec![
                  NbtTag::Int(Some("max_inclusive".to_string()), 7),
                  NbtTag::Int(Some("min_inclusive".to_string()), 0)
                ])
              ]),
              NbtTag::Byte(Some("natural".to_string()), 0),
              NbtTag::Byte(Some("piglin_safe".to_string()), 0),
              NbtTag::Byte(Some("respawn_anchor_works".to_string()), 0),
              NbtTag::Byte(Some("ultrawarm".to_string()), 0)
            ]),
            NbtTag::Int(Some("id".to_string()), 2),
            NbtTag::String(Some("name".to_string()), "minecraft:the_end".to_string()),
          ]),
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::Float(Some("ambient_light".to_string()), 0.0),
              NbtTag::Byte(Some("bed_works".to_string()), 1),
              NbtTag::Double(Some("coordinate_scale".to_string()), 1.0),
              NbtTag::String(Some("effects".to_string()), "minecraft:overworld".to_string()),
              NbtTag::Byte(Some("has_ceiling".to_string()), 1),
              NbtTag::Byte(Some("has_raids".to_string()), 1),
              NbtTag::Byte(Some("has_skylight".to_string()), 1),
              NbtTag::Int(Some("height".to_string()), 384),
              NbtTag::String(Some("infiniburn".to_string()), "#minecraft:infiniburn_overworld".to_string()),
              NbtTag::Int(Some("logical_height".to_string()), 384),
              NbtTag::Int(Some("min_y".to_string()), -64),
              NbtTag::Int(Some("monster_spawn_block_light_limit".to_string()), 0),
              NbtTag::TagCompound(Some("monster_spawn_light_level".to_string()), vec![
                NbtTag::String(Some("type".to_string()), "minecraft:uniform".to_string()),
                NbtTag::TagCompound(Some("value".to_string()), vec![
                  NbtTag::Int(Some("max_inclusive".to_string()), 7),
                  NbtTag::Int(Some("min_inclusive".to_string()), 0)
                ])
              ]),
              NbtTag::Byte(Some("natural".to_string()), 1),
              NbtTag::Byte(Some("piglin_safe".to_string()), 0),
              NbtTag::Byte(Some("respawn_anchor_works".to_string()), 0),
              NbtTag::Byte(Some("ultrawarm".to_string()), 0)
            ]),
            NbtTag::Int(Some("id".to_string()), 3),
            NbtTag::String(Some("name".to_string()), "minecraft:overworld_caves".to_string()),
          ]),
        ])
      ]),
      NbtTag::TagCompound(Some("minecraft:worldgen/biome".to_string()), vec![
        NbtTag::String(Some("type".to_string()), "minecraft:worldgen/biome".to_string()),
        NbtTag::List(Some("value".to_string()), vec![
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::Float(Some("downfall".to_string()), 0.5),
              NbtTag::TagCompound(Some("effects".to_string()), vec![
                NbtTag::Int(Some("fog_color".to_string()), 12638463),
                NbtTag::TagCompound(Some("mood_sound".to_string()), vec![
                  NbtTag::Int(Some("block_search_extent".to_string()), 8),
                  NbtTag::Double(Some("offset".to_string()), 2.0),
                  NbtTag::String(Some("sound".to_string()), "minecraft:ambient.cave".to_string()),
                  NbtTag::Int(Some("tick_delay".to_string()), 6000)
                ]),
                NbtTag::Int(Some("sky_color".to_string()), 8103167),
                NbtTag::Int(Some("water_color".to_string()), 4159204),
                NbtTag::Int(Some("water_fog_color".to_string()), 329011)
              ]),
              NbtTag::String(Some("precipitation".to_string()), "none".to_string()),
              NbtTag::Float(Some("temperature".to_string()), 0.5)
            ]),
            NbtTag::Int(Some("id".to_string()), 0),
            NbtTag::String(Some("name".to_string()), "minecraft:the_void".to_string())
          ]),
          NbtTag::TagCompound(None, vec![
            NbtTag::TagCompound(Some("element".to_string()), vec![
              NbtTag::Float(Some("downfall".to_string()), 0.5),
              NbtTag::TagCompound(Some("effects".to_string()), vec![
                NbtTag::Int(Some("fog_color".to_string()), 12638463),
                NbtTag::TagCompound(Some("mood_sound".to_string()), vec![
                  NbtTag::Int(Some("block_search_extent".to_string()), 8),
                  NbtTag::Double(Some("offset".to_string()), 2.0),
                  NbtTag::String(Some("sound".to_string()), "minecraft:ambient.cave".to_string()),
                  NbtTag::Int(Some("tick_delay".to_string()), 6000)
                ]),
                NbtTag::Int(Some("sky_color".to_string()), 8103167),
                NbtTag::Int(Some("water_color".to_string()), 4159204),
                NbtTag::Int(Some("water_fog_color".to_string()), 329011)
              ]),
              NbtTag::String(Some("precipitation".to_string()), "none".to_string()),
              NbtTag::Float(Some("temperature".to_string()), 0.5)
            ]),
            NbtTag::Int(Some("id".to_string()), 1),
            NbtTag::String(Some("name".to_string()), "minecraft:plains".to_string())
          ])
        ])
      ])
    ]));
    
    output.append(&mut registry_codec);

    output.append(&mut serialize::string("minecraft:overworld"));
    output.append(&mut serialize::string("minecraft:overworld"));

    output.push(0xca);
    output.push(0x8f);
    output.push(0xde);
    output.push(0x66);
    output.push(0x4f);
    output.push(0xd2);
    output.push(0x2a);
    output.push(0xba);

    output.push(9);
    output.push(11);
    output.push(10);
    
    output.push(0);
    output.push(1);
    output.push(0);
    output.push(0);
    output.push(0);

    send_packet(stream, 0x25, output);
  }

  #[allow(unused)]
  pub enum Difficulty {
    Peaceful, Easy, Normal, Hard
  }

  pub fn change_difficulty(stream: &mut TcpStream, new_difficulty: Difficulty) {
    send_packet(stream, 0x0b, vec![new_difficulty as u8, 1]);
  }

  pub fn player_abilities(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
    output.push(0b00001111);
    output.append(&mut serialize::float(0.05));
    output.append(&mut serialize::float(0.1));
    
    send_packet(stream, 0x31, output);
  }

  //slot_id ranges from 0 to 8
  pub fn set_held_item(stream: &mut TcpStream, slot_id: u8) {
    send_packet(stream, 0x4a, vec![slot_id]);
  }

  pub fn update_recipes(stream: &mut TcpStream) {
    send_packet(stream, 0x6a, vec![0]);
  }
  pub fn update_tags(stream: &mut TcpStream) {
    send_packet(stream, 0x6b, vec![0]);
  }

  pub fn entity_event(stream: &mut TcpStream, entity_id: i32, entity_status: u8) {
    let mut output: Vec<u8> = serialize::int(entity_id);
    output.push(entity_status);
    send_packet(stream, 0x1a, output);
  }

  pub fn update_recipe_book(stream: &mut TcpStream) {
    send_packet(stream, 0x3a, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
  }

  pub fn sync_player_position(stream: &mut TcpStream, x: f64, y: f64, z: f64, yaw: f32, pitch: f32, teleport_id: i32, dismount_vehicle: bool) {
    let mut output: Vec<u8> = Vec::new();

    output.append(&mut serialize::double(x));
    output.append(&mut serialize::double(y));
    output.append(&mut serialize::double(z));
    output.append(&mut serialize::float(yaw));
    output.append(&mut serialize::float(pitch));
    output.push(0x00);
    output.append(&mut serialize::varint(teleport_id));
    output.push(dismount_vehicle as u8);

    send_packet(stream, 0x39, output);
  }

  pub fn set_center_chunk(stream: &mut TcpStream, chunk_x: i32, chunk_y: i32) {
    let mut output: Vec<u8> = serialize::varint(chunk_x);
    output.append(&mut serialize::varint(chunk_y));
    send_packet(stream, 0x4b, output);
  }

  pub fn commands(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();

    output.append(&mut serialize::varint(1));
    output.push(0x00);
    output.append(&mut serialize::varint(0));    
    output.append(&mut serialize::varint(0));

    send_packet(stream, 0x0f, output);
  }

  pub fn chunk_data_and_update_light(stream: &mut TcpStream, chunk_x: i32, chunk_y: i32) {
    //let mut output: Vec<u8> = Vec::new();

    let mut output: Vec<u8> = Vec::new();
    output.append(&mut serialize::int(chunk_x));
    output.append(&mut serialize::int(chunk_y));
    //output.append(&mut std::fs::read("./src/server/java/packet/packet_21_data.bin").unwrap());

    output.append(&mut serialize::nbt(NbtTag::TagCompound(Some("".to_string()), vec![
      NbtTag::LongArray(Some("MOTION_BLOCKING".to_string()), vec![0, 256])
    ])));
    output.append(&mut serialize::varint(0));
    output.push(0x00);
    output.append(&mut serialize::varint(0));
    output.push(0x00);
    output.push(0x00);
    output.push(0x00);
    output.push(0x00);
    output.push(0x00);
    output.push(0x00);
        
    send_packet(stream, 0x21, output);
  }

  #[derive(Clone)]
  #[allow(unused)]
  pub enum PlayerInfoAction {
    AddPlayer, UpdateGamemode, UpdateLatency, UpdateDisplayName, RemovePlayer 
  }

  pub fn player_info(stream: &mut TcpStream, action: PlayerInfoAction) {
    let mut output: Vec<u8> = Vec::new();

    output.append(&mut serialize::varint(action.clone() as i32));
    output.append(&mut serialize::varint(1));
    output.append(&mut serialize::uuid(&(0xdac25e44d1024f3b819978ed62d209a1 as u128)));

    match action {
      PlayerInfoAction::AddPlayer => {
        output.append(&mut serialize::string("The__TxT"));
        output.append(&mut serialize::varint(0));
        output.append(&mut serialize::varint(0));
        output.append(&mut serialize::varint(0));
        output.append(&mut serialize::bool(&false));
        output.append(&mut serialize::bool(&false));
      },
      PlayerInfoAction::UpdateGamemode => todo!(),
      PlayerInfoAction::UpdateLatency => {
        output.append(&mut serialize::varint(0));
      },
      PlayerInfoAction::UpdateDisplayName => todo!(),
      PlayerInfoAction::RemovePlayer => todo!(),
    }

    send_packet(stream, 0x37, output)
  }

  pub fn set_default_spawn_position(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();

    output.append(&mut (0 as u64).to_be_bytes().to_vec());
    output.append(&mut serialize::float(0.0));

    send_packet(stream, 0x4d, output);
  }


  pub fn plugin_message(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::string("minecraft:brand.Oxide"));
  
    send_packet(stream, 0x16, output)
  }

  pub fn set_render_distance(stream: &mut TcpStream, render_distance: u8) {
    send_packet(stream, 0x4c, vec![render_distance]);
  }

  pub fn set_simulation_distance(stream: &mut TcpStream, render_distance: u8) {
    send_packet(stream, 0x5a, vec![render_distance]);
  }

  pub fn initialize_world_border(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::double(0.0));
    output.append(&mut serialize::double(0.0));
    output.append(&mut serialize::double(100000.0));
    output.append(&mut serialize::double(100000.0));
    output.push(0);
    output.append(&mut serialize::varint(100000));
    output.append(&mut serialize::varint(10000));
    output.append(&mut serialize::varint(0));
  
    send_packet(stream, 0x1f, output)
  }

  pub fn update_time(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::long(0));
    output.append(&mut serialize::long(0));
  
    send_packet(stream, 0x5c, output)
  }

  pub fn server_data(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::bool(&false));
    output.append(&mut serialize::bool(&false));
    output.append(&mut serialize::bool(&false));
    output.append(&mut serialize::bool(&false));
  
    send_packet(stream, 0x42, output)
  }

  pub fn set_container_content(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.push(0);
    output.append(&mut serialize::varint(1));
    output.append(&mut serialize::varint(46));
    
    for _ in 0..46 {
      output.append(&mut serialize::bool(&false));
    }
    
    output.append(&mut serialize::bool(&false));
  
    send_packet(stream, 0x11, output)
  }

  pub fn spawn_entity(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::varint(2));
    output.append(&mut serialize::uuid(&290780920670370370148908686767547353505));
    output.append(&mut serialize::varint(116));
    output.append(&mut serialize::double(0.0));
    output.append(&mut serialize::double(0.0));
    output.append(&mut serialize::double(0.0));
    output.push(0);
    output.push(0);
    output.push(0);
    output.append(&mut serialize::varint(0));
    output.append(&mut serialize::short(0));
    output.append(&mut serialize::short(0));
    output.append(&mut serialize::short(0));
  
    send_packet(stream, 0x00, output)
  }

  pub fn update_advancements(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::bool(&true));
    output.append(&mut serialize::varint(0));
    output.append(&mut serialize::varint(0));
    output.append(&mut serialize::varint(0));
  
    send_packet(stream, 0x67, output)
  }

  pub fn set_health(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::float(20.0));
    output.append(&mut serialize::varint(20));
    output.append(&mut serialize::float(5.0));
  
    send_packet(stream, 0x55, output)
  }

  pub fn set_experience(stream: &mut TcpStream) {
    let mut output: Vec<u8> = Vec::new();
  
    output.append(&mut serialize::float(0.0));
    output.append(&mut serialize::varint(0));
    output.append(&mut serialize::varint(0));
  
    send_packet(stream, 0x54, output)
  }

}
