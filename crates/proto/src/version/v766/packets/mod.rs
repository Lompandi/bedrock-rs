macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(add_actor);
export!(add_player);
export!(award_achievement);
export!(boss_event);
export!(camera_aim_assist);
export!(camera_aim_assist_presets);
export!(camera_instruction);
export!(camera_presets);
export!(change_dimension);
export!(clientbound_close_form);
export!(clientbound_debug_renderer);
export!(clientbound_map_item_data);
export!(code_builder_source);
export!(container_close);
export!(container_registry_cleanup);
export!(correct_player_move_prediction);
export!(current_structure_feature);
export!(disconnect);
export!(editor_network);
export!(emote);
export!(inventory_content);
export!(inventory_slot);
export!(jigsaw_structure_data);
export!(legacy_telemetry_event);
export!(mob_armor_equipment);
export!(movement_effect);
export!(player_armor_damage);
export!(player_auth_input);
export!(player_list);
export!(resource_pack_stack);
export!(resource_packs_info);
export!(server_bound_diagnostics);
export!(server_bound_loading_screen);
export!(set_movement_authority);
export!(set_title);
export!(stop_sound);
export!(text);
export!(transfer_player);
export!(update_attributes);
export!(update_player_game_type);
export!(update_soft_enum);
