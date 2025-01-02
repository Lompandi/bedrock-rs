macro_rules! export {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

export!(abilities_index);
export!(actor_block_sync_message_id);
export!(actor_damage_cause);
export!(actor_data_ids);
export!(actor_event);
export!(actor_flags);
export!(actor_link_type);
export!(actor_type);
export!(agent_action_type);
export!(animation_mode);
export!(attribute_modifier_operation);
export!(attribute_operands);
export!(book_edit_action);
export!(boss_event_update_type);
export!(build_platform);
export!(camera_shake_action);
export!(camera_shake_type);
export!(chat_restriction_level);
export!(client_play_mode);
export!(code_builder_storage);
export!(command_block_mode);
export!(command_origin_type);
export!(command_output_type);
export!(command_parameter_option);
export!(command_permission_level);
export!(complex_inventory_transaction_type);
export!(connection_fail_reason);
export!(container_enum_name);
export!(container_id);
export!(container_type);
export!(crafting_data_entry_type);
export!(crafting_type);
export!(data_item_type);
export!(difficulty);
export!(easing_type);
export!(editor_world_type);
export!(education_edition_offer);
export!(enchant_type);
export!(game_type);
export!(generator_type);
export!(hud_element);
export!(hud_visibility);
export!(identity_definition_type);
export!(input_mode);
export!(inventory_layout);
export!(inventory_left_tab_index);
export!(inventory_right_tab_index);
export!(inventory_source_flags);
export!(inventory_source_type);
export!(item_descriptor_internal_type);
export!(item_release_inventory_transaction_type);
export!(item_stack_net_result);
export!(item_stack_request_action_type);
export!(item_use_inventory_transaction_type);
export!(item_use_method);
export!(item_use_on_actor_inventory_transaction_type);
export!(lab_table_reaction_type);
export!(lesson_action);
export!(level_event);
export!(minecraft_eventing);
export!(minecraft_packet_ids);
export!(mirror);
export!(modal_form_cancel_reason);
export!(molang_version);
export!(multiplayer_settings_packet_type);
export!(new_interaction_model);
export!(objective_sort_order);
export!(pack_type);
export!(packet_compression_algorithm);
export!(packet_violation_severity);
export!(packet_violation_type);
export!(particle_type);
export!(photo_type);
export!(play_status);
export!(player_action_type);
export!(player_list_packet_type);
export!(player_permission_level);
export!(player_position_mode);
export!(player_respawn_state);
export!(level_sound_event_type);
export!(resource_pack_response);
export!(rotation);
export!(score_packet_type);
export!(scoreboard_identity_packet_type);
export!(server_auth_movement_mode);
export!(show_store_offer_redirect_type);
export!(simulation_type);
export!(game_publish_setting);
export!(soft_enum_update_type);
export!(spawn_biome_type);
export!(spawn_position_type);
export!(structure_block_type);
export!(structure_redstone_save_mode);
export!(structure_template_request_operation);
export!(structure_template_response_type);
export!(text_packet_type);
export!(text_processing_event_origin);
export!(ui_profile);
export!(persona);
export!(prediction_type);