macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(actor_link);
export!(actor_runtime_id);
export!(actor_unique_id);
export!(adventure_settings);
export!(base_description);
export!(base_game_version);
export!(block_pos);
export!(camera_instruction);
export!(camera_preset);
export!(camera_presets);
export!(chunk_pos);
export!(command_origin_data);
export!(container_mix_data_entry);
export!(crafting_data_entry);
export!(data_item);
export!(dimension_definition_group);
export!(edu_shared_uri_resource);
export!(education_level_settings);
export!(entity_net_id);
export!(experiments);
export!(game_rules_changed_packet_data);
export!(inventory_action);
export!(inventory_source);
export!(inventory_transaction);
export!(item_data);
export!(item_enchants);
export!(item_instance_user_data);
export!(item_stack_net_id_variant);
export!(item_stack_request_slot_info);
export!(item_stack_response_container_info);
export!(item_stack_response_info);
export!(item_stack_response_slot_info);
export!(level_settings);
export!(map_decoration);
export!(map_item_tracked_actor_unique_id);
export!(material_reducer_data_entry);
export!(molang_variable_map);
export!(move_actor_absolute_data);
export!(move_actor_delta_data);
export!(network_block_position);
export!(network_item_instance_descriptor);
export!(network_item_stack_descriptor);
export!(network_permissions);
export!(packed_item_use_legacy_inventory_transaction);
export!(player_block_action_data);
export!(player_block_actions);
export!(position_tracking_id);
export!(potion_mix_data_entry);
export!(property_sync_data);
export!(recipe_ingredient);
export!(scoreboard_id);
export!(serialized_abilities_data);
export!(serialized_skin);
export!(shaped_chemistry_recipe);
export!(shaped_recipe);
export!(shapeless_chemistry_recipe);
export!(shapeless_recipe);
export!(shulker_box_recipe);
export!(smithing_transform_recipe);
export!(smithing_trim_recipe);
export!(spawn_settings);
export!(structure_editor_data);
export!(structure_settings);
export!(sub_chunk_pos);
export!(synced_player_movement_settings);
export!(web_socket_packet_data);
export!(sub_chunk_pos_offset);
