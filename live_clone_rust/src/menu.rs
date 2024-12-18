#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApp, NSMenu, NSMenuItem};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSString};
use objc::{msg_send, sel, sel_impl};

#[cfg(target_os = "macos")]
pub fn create_menu_bar() -> Result<(), &'static str> {
    unsafe {
        let _pool = NSAutoreleasePool::new(nil);
        
        // Get the shared application
        let app = NSApp();
        
        // Create the menubar
        let menubar = NSMenu::new(nil);
        if menubar.is_null() {
            return Err("Failed to create menu bar");
        }

        // Create all menus
        create_app_menu(&menubar);
        create_file_menu(&menubar);
        create_edit_menu(&menubar);
        create_create_menu(&menubar);
        create_view_menu(&menubar);
        create_options_menu(&menubar);
        create_help_menu(&menubar);

        // Set the menu bar
        let _: () = msg_send![app, setMainMenu: menubar];
        
        // Finish launching the application
        let _: () = msg_send![app, finishLaunching];
        
        Ok(())
    }
}

#[cfg(target_os = "macos")]
unsafe fn create_menu_item(title: &str, key: &str, action: objc::runtime::Sel) -> id {
    let title_str = NSString::alloc(nil).init_str(title);
    let key = NSString::alloc(nil).init_str(key);
    let item = NSMenuItem::alloc(nil).initWithTitle_action_keyEquivalent_(title_str, action, key);
    
    // Set modifier flags for specific items
    match title {
        // Command only
        "Edit MIDI Map" |
        "Edit Key Map" |
        "Narrow Grid" |
        "Widen Grid" |
        "Triplet Grid" |
        "Snap to Grid" |
        "Fixed Grid" => {
            let _: () = msg_send![item, setKeyEquivalentModifierMask: 1048576]; // Command
        },
        // Command + Shift
        "Save Live Set As..." | 
        "Export Audio/Video..." |
        "Export MIDI Clip..." |
        "Nothing to Redo" |
        "Ungroup" |
        "Cut Time" |
        "Paste Time" |
        "Duplicate Time" |
        "Delete Time" |
        "Quantize Settings..." |
        "Insert MIDI Track" |
        "Capture and Insert Scene" |
        "Insert Empty MIDI Clip(s)" |
        "Capture MIDI" |
        "Second Window" |
        "Follow" => {
            let _: () = msg_send![item, setKeyEquivalentModifierMask: 1048840]; // Command + Shift
        },
        // Command + Option
        "Insert Return Track" |
        "Create Fade" |
        "Full Screen" |
        "Detail View" |
        "Expand Clip View" |
        "Browser" |
        "Groove Pool" |
        "Plug-In Windows" |
        "Video Window" |
        "Overview" |
        "In/Out" |
        "Sends" |
        "Returns" |
        "Mixer" => {
            let _: () = msg_send![item, setKeyEquivalentModifierMask: 524576]; // Command + Option
        },
        // Command + Shift + Option
        "Insert Take Lane" |
        "Toggle Clip/Device View" |
        "Audio Engine On" => {
            let _: () = msg_send![item, setKeyEquivalentModifierMask: 1573416]; // Command + Shift + Option
        },
        // Option only
        "MIDI Note Editor" |
        "Envelopes Editor" |
        "Note Expression Editor" => {
            let _: () = msg_send![item, setKeyEquivalentModifierMask: 524288]; // Option
        },
        // No modifiers
        "Computer MIDI Keyboard" |
        "Draw Mode" |
        "Highlight Scales" => {
            let _: () = msg_send![item, setKeyEquivalentModifierMask: 0]; // No modifiers
        },
        _ => {}
    }
    
    item
}

#[cfg(target_os = "macos")]
unsafe fn create_app_menu(menubar: &id) {
    let app_menu_item = NSMenuItem::new(nil);
    let app_menu = NSMenu::new(nil);
    
    // Disable AutoFill, Smart Dictation, and Emoji & Symbols
    let _: () = msg_send![app_menu, setAutoenablesItems: false];
    
    // Add About item
    let about_item = create_menu_item("About Live Clone Rust", "", sel!(orderFrontStandardAboutPanel:));
    app_menu.addItem_(about_item);
    app_menu.addItem_(NSMenuItem::separatorItem(nil));
    
    // Add Preferences
    let prefs_item = create_menu_item("Preferences...", ",", sel!(openPreferences:));
    app_menu.addItem_(prefs_item);
    app_menu.addItem_(NSMenuItem::separatorItem(nil));
    
    // Add Services menu
    let services_item = NSMenuItem::new(nil);
    let _: () = msg_send![services_item, setTitle: NSString::alloc(nil).init_str("Services")];
    let services_menu = NSMenu::new(nil);
    services_item.setSubmenu_(services_menu);
    app_menu.addItem_(services_item);
    app_menu.addItem_(NSMenuItem::separatorItem(nil));
    
    // Add Hide/Show items
    let hide_item = create_menu_item("Hide Live Clone Rust", "h", sel!(hide:));
    let hide_others_item = create_menu_item("Hide Others", "h", sel!(hideOtherApplications:));
    let show_all_item = create_menu_item("Show All", "", sel!(unhideAllApplications:));
    app_menu.addItem_(hide_item);
    app_menu.addItem_(hide_others_item);
    app_menu.addItem_(show_all_item);
    app_menu.addItem_(NSMenuItem::separatorItem(nil));
    
    // Add Quit item
    let quit_item = create_menu_item("Quit Live Clone Rust", "q", sel!(terminate:));
    app_menu.addItem_(quit_item);
    
    app_menu_item.setSubmenu_(app_menu);
    menubar.addItem_(app_menu_item);
}

#[cfg(target_os = "macos")]
unsafe fn create_file_menu(menubar: &id) {
    let file_menu_item = NSMenuItem::new(nil);
    let file_menu = NSMenu::new(nil);
    let _: () = msg_send![file_menu_item, setTitle: NSString::alloc(nil).init_str("File")];
    let _: () = msg_send![file_menu, setTitle: NSString::alloc(nil).init_str("File")];
    
    // Add items to File menu
    let new_set_item = create_menu_item("New Live Set", "N", sel!(newDocument:));
    let open_set_item = create_menu_item("Open Live Set...", "O", sel!(openDocument:));
    
    // Create Recent Sets submenu
    let recent_menu_item = NSMenuItem::new(nil);
    let recent_menu = NSMenu::new(nil);
    let _: () = msg_send![recent_menu_item, setTitle: NSString::alloc(nil).init_str("Open Recent Set")];
    recent_menu_item.setSubmenu_(recent_menu);
    
    let close_item = create_menu_item("Close Window", "W", sel!(performClose:));
    
    let install_pack_item = create_menu_item("Install Pack...", "", sel!(installPack:));
    
    let manage_files_item = create_menu_item("Manage Files", "", sel!(manageFiles:));
    
    let save_item = create_menu_item("Save Live Set", "S", sel!(saveDocument:));
    let save_as_item = create_menu_item("Save Live Set As...", "S", sel!(saveDocumentAs:));
    let save_copy_item = create_menu_item("Save a Copy...", "", sel!(saveCopy:));
    let collect_save_item = create_menu_item("Collect All and Save", "", sel!(collectAllAndSave:));
    let save_template_item = create_menu_item("Save Live Set As Template...", "", sel!(saveAsTemplate:));
    let save_default_item = create_menu_item("Save Live Set As Default Set...", "", sel!(saveAsDefault:));
    
    let export_audio_item = create_menu_item("Export Audio/Video...", "R", sel!(exportAudioVideo:));
    let export_midi_item = create_menu_item("Export MIDI Clip...", "E", sel!(exportMIDIClip:));
    
    // Add all items to menu
    file_menu.addItem_(new_set_item);
    file_menu.addItem_(open_set_item);
    file_menu.addItem_(recent_menu_item);
    file_menu.addItem_(close_item);
    file_menu.addItem_(NSMenuItem::separatorItem(nil));
    file_menu.addItem_(install_pack_item);
    file_menu.addItem_(NSMenuItem::separatorItem(nil));
    file_menu.addItem_(manage_files_item);
    file_menu.addItem_(NSMenuItem::separatorItem(nil));
    file_menu.addItem_(save_item);
    file_menu.addItem_(save_as_item);
    file_menu.addItem_(save_copy_item);
    file_menu.addItem_(collect_save_item);
    file_menu.addItem_(save_template_item);
    file_menu.addItem_(save_default_item);
    file_menu.addItem_(NSMenuItem::separatorItem(nil));
    file_menu.addItem_(export_audio_item);
    file_menu.addItem_(export_midi_item);
    
    file_menu_item.setSubmenu_(file_menu);
    menubar.addItem_(file_menu_item);
}

#[cfg(target_os = "macos")]
unsafe fn create_edit_menu(menubar: &id) {
    let edit_menu_item = NSMenuItem::new(nil);
    let edit_menu = NSMenu::new(nil);
    let _: () = msg_send![edit_menu_item, setTitle: NSString::alloc(nil).init_str("Edit")];
    let _: () = msg_send![edit_menu, setTitle: NSString::alloc(nil).init_str("Edit")];
    // Prevent system items from being added
    let _: () = msg_send![edit_menu, setAutoenablesItems: false];

    // Basic editing
    let undo_item = create_menu_item("Nothing to Undo", "Z", sel!(undo:));
    let redo_item = create_menu_item("Nothing to Redo", "Z", sel!(redo:));
    let cut_item = create_menu_item("Cut", "X", sel!(cut:));
    let copy_item = create_menu_item("Copy", "C", sel!(copy:));
    let paste_item = create_menu_item("Paste", "V", sel!(paste:));
    let duplicate_item = create_menu_item("Duplicate", "D", sel!(duplicate:));
    let delete_item = create_menu_item("Delete", "\u{0008}", sel!(delete:)); // Backspace key

    // Clip activation
    let activate_clip_item = create_menu_item("Activate/Deactivate Clip(s)", "0", sel!(activateClip:));

    // Selection
    let select_all_item = create_menu_item("Select All", "A", sel!(selectAll:));
    let select_loop_item = create_menu_item("Select Loop", "L", sel!(selectLoop:));
    let activate_loop_item = create_menu_item("Activate Loop", "L", sel!(activateLoop:));

    // Grouping
    let group_item = create_menu_item("Group", "G", sel!(group:));
    let ungroup_item = create_menu_item("Ungroup", "G", sel!(ungroup:));

    // Time operations
    let cut_time_item = create_menu_item("Cut Time", "X", sel!(cutTime:));
    let paste_time_item = create_menu_item("Paste Time", "V", sel!(pasteTime:));
    let duplicate_time_item = create_menu_item("Duplicate Time", "D", sel!(duplicateTime:));
    let delete_time_item = create_menu_item("Delete Time", "\u{0008}", sel!(deleteTime:));

    // Track operations
    let rename_item = create_menu_item("Rename", "R", sel!(rename:));
    let edit_info_item = create_menu_item("Edit Info Text", "", sel!(editInfo:));
    let solo_tracks_item = create_menu_item("Solo Tracks", "S", sel!(soloTracks:));
    let arm_tracks_item = create_menu_item("Arm Tracks", "C", sel!(armTracks:));
    let freeze_track_item = create_menu_item("Freeze Track", "", sel!(freezeTrack:));
    let flatten_track_item = create_menu_item("Flatten Track", "", sel!(flattenTrack:));

    // Split and consolidate
    let split_item = create_menu_item("Split", "E", sel!(split:));
    let consolidate_item = create_menu_item("Consolidate", "J", sel!(consolidate:));

    // Quantization
    let quantize_item = create_menu_item("Quantize", "U", sel!(quantize:));
    let quantize_settings_item = create_menu_item("Quantize Settings...", "U", sel!(quantizeSettings:));
    let extract_groove_item = create_menu_item("Extract Groove(s)", "", sel!(extractGroove:));
    
    // Create Record Quantization submenu
    let record_quantization_item = NSMenuItem::new(nil);
    let record_quantization_menu = NSMenu::new(nil);
    let _: () = msg_send![record_quantization_item, setTitle: NSString::alloc(nil).init_str("Record Quantization")];
    record_quantization_item.setSubmenu_(record_quantization_menu);

    let simplify_envelope_item = create_menu_item("Simplify Envelope", "", sel!(simplifyEnvelope:));

    // Add all items to menu with separators
    edit_menu.addItem_(undo_item);
    edit_menu.addItem_(redo_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(cut_item);
    edit_menu.addItem_(copy_item);
    edit_menu.addItem_(paste_item);
    edit_menu.addItem_(duplicate_item);
    edit_menu.addItem_(delete_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(activate_clip_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(select_all_item);
    edit_menu.addItem_(select_loop_item);
    edit_menu.addItem_(activate_loop_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(group_item);
    edit_menu.addItem_(ungroup_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(cut_time_item);
    edit_menu.addItem_(paste_time_item);
    edit_menu.addItem_(duplicate_time_item);
    edit_menu.addItem_(delete_time_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(rename_item);
    edit_menu.addItem_(edit_info_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(solo_tracks_item);
    edit_menu.addItem_(arm_tracks_item);
    edit_menu.addItem_(freeze_track_item);
    edit_menu.addItem_(flatten_track_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(split_item);
    edit_menu.addItem_(consolidate_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(quantize_item);
    edit_menu.addItem_(quantize_settings_item);
    edit_menu.addItem_(extract_groove_item);
    edit_menu.addItem_(record_quantization_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));
    edit_menu.addItem_(simplify_envelope_item);
    edit_menu.addItem_(NSMenuItem::separatorItem(nil));

    edit_menu_item.setSubmenu_(edit_menu);
    menubar.addItem_(edit_menu_item);
}

#[cfg(target_os = "macos")]
unsafe fn create_create_menu(menubar: &id) {
    let create_item = NSMenuItem::new(nil);
    let create_menu = NSMenu::new(nil);
    let _: () = msg_send![create_item, setTitle: NSString::alloc(nil).init_str("Create")];
    let _: () = msg_send![create_menu, setTitle: NSString::alloc(nil).init_str("Create")];

    // Track creation
    let insert_audio_track = create_menu_item("Insert Audio Track", "T", sel!(insertAudioTrack:));
    let insert_midi_track = create_menu_item("Insert MIDI Track", "T", sel!(insertMIDITrack:));
    let insert_return_track = create_menu_item("Insert Return Track", "T", sel!(insertReturnTrack:));
    let insert_take_lane = create_menu_item("Insert Take Lane", "T", sel!(insertTakeLane:));

    // Scene and clip operations
    let insert_silence = create_menu_item("Insert Silence...", "I", sel!(insertSilence:));
    let capture_scene = create_menu_item("Capture and Insert Scene", "I", sel!(captureAndInsertScene:));
    let consolidate_scene = create_menu_item("Consolidate Time to New Scene", "", sel!(consolidateTimeToNewScene:));
    let insert_midi_clip = create_menu_item("Insert Empty MIDI Clip(s)", "M", sel!(insertEmptyMIDIClip:));
    let capture_midi = create_menu_item("Capture MIDI", "C", sel!(captureMIDI:));

    // Fade and conversion
    let create_fade = create_menu_item("Create Fade", "F", sel!(createFade:));
    let slice_to_midi = create_menu_item("Slice to New MIDI Track", "", sel!(sliceToNewMIDITrack:));
    let convert_harmony = create_menu_item("Convert Harmony to New MIDI Track", "", sel!(convertHarmonyToMIDI:));
    let convert_melody = create_menu_item("Convert Melody to New MIDI Track", "", sel!(convertMelodyToMIDI:));
    let convert_drums = create_menu_item("Convert Drums to New MIDI Track", "", sel!(convertDrumsToMIDI:));

    // Time and locator
    let add_locator = create_menu_item("Add Locator", "", sel!(addLocator:));
    let insert_time_sig = create_menu_item("Insert Time Signature Change", "", sel!(insertTimeSignature:));

    // Import
    let import_file = create_menu_item("Import Audio/MIDI File...", "", sel!(importFile:));

    // Add all items to menu with separators
    create_menu.addItem_(insert_audio_track);
    create_menu.addItem_(insert_midi_track);
    create_menu.addItem_(insert_return_track);
    create_menu.addItem_(insert_take_lane);
    create_menu.addItem_(NSMenuItem::separatorItem(nil));
    create_menu.addItem_(insert_silence);
    create_menu.addItem_(capture_scene);
    create_menu.addItem_(consolidate_scene);
    create_menu.addItem_(NSMenuItem::separatorItem(nil));
    create_menu.addItem_(insert_midi_clip);
    create_menu.addItem_(capture_midi);
    create_menu.addItem_(NSMenuItem::separatorItem(nil));
    create_menu.addItem_(create_fade);
    create_menu.addItem_(NSMenuItem::separatorItem(nil));
    create_menu.addItem_(slice_to_midi);
    create_menu.addItem_(convert_harmony);
    create_menu.addItem_(convert_melody);
    create_menu.addItem_(convert_drums);
    create_menu.addItem_(NSMenuItem::separatorItem(nil));
    create_menu.addItem_(add_locator);
    create_menu.addItem_(insert_time_sig);
    create_menu.addItem_(NSMenuItem::separatorItem(nil));
    create_menu.addItem_(import_file);

    create_item.setSubmenu_(create_menu);
    menubar.addItem_(create_item);
}

#[cfg(target_os = "macos")]
unsafe fn create_view_menu(menubar: &id) {
    let view_menu_item = NSMenuItem::new(nil);
    let view_menu = NSMenu::new(nil);
    let _: () = msg_send![view_menu_item, setTitle: NSString::alloc(nil).init_str("View")];
    let _: () = msg_send![view_menu, setTitle: NSString::alloc(nil).init_str("View")];

    // Window options
    let full_screen = create_menu_item("Full Screen", "F", sel!(toggleFullScreen:));
    let second_window = create_menu_item("Second Window", "W", sel!(openSecondWindow:));
    
    // Info and Help
    let info_item = create_menu_item("Info", "?", sel!(showInfo:));
    let help_view = create_menu_item("Help View", "", sel!(showHelpView:));
    
    // File and View Management
    let file_manager = create_menu_item("File Manager", "", sel!(toggleFileManager:));
    let _: () = msg_send![file_manager, setState: 1]; // Set checkmark
    
    let toggle_view = create_menu_item("Toggle Arrangement/Session View", "\u{2192}", sel!(toggleArrangementView:));
    
    let detail_view = create_menu_item("Detail View", "L", sel!(toggleDetailView:));
    let _: () = msg_send![detail_view, setState: 1]; // Set checkmark
    
    let toggle_clip = create_menu_item("Toggle Clip/Device View", "\u{2192}", sel!(toggleClipView:));
    let expand_clip = create_menu_item("Expand Clip View", "E", sel!(expandClipView:));
    
    let arrange_panels = create_menu_item("Arrange Clip View Panels Automatically", "", sel!(arrangePanels:));
    let _: () = msg_send![arrange_panels, setState: 1]; // Set checkmark
    
    // Editors
    let midi_editor = create_menu_item("MIDI Note Editor", "1", sel!(showMIDIEditor:));
    let _: () = msg_send![midi_editor, setState: 1]; // Set checkmark
    
    let env_editor = create_menu_item("Envelopes Editor", "2", sel!(showEnvelopesEditor:));
    let note_exp_editor = create_menu_item("Note Expression Editor", "3", sel!(showNoteExpressionEditor:));
    let _: () = msg_send![note_exp_editor, setEnabled: false]; // Disable item
    
    // Browser and Tools
    let browser = create_menu_item("Browser", "B", sel!(toggleBrowser:));
    let groove_pool = create_menu_item("Groove Pool", "G", sel!(showGroovePool:));
    let search_browser = create_menu_item("Search in Browser", "F", sel!(searchBrowser:));
    
    // Windows
    let plugin_windows = create_menu_item("Plug-In Windows", "P", sel!(showPluginWindows:));
    let _: () = msg_send![plugin_windows, setState: 1]; // Set checkmark
    
    let video_window = create_menu_item("Video Window", "V", sel!(showVideoWindow:));
    let _: () = msg_send![video_window, setState: 1]; // Set checkmark
    
    // Mixer Views
    let overview = create_menu_item("Overview", "O", sel!(showOverview:));
    let _: () = msg_send![overview, setState: 1]; // Set checkmark
    
    let in_out = create_menu_item("In/Out", "I", sel!(showInOut:));
    
    let sends = create_menu_item("Sends", "S", sel!(showSends:));
    let _: () = msg_send![sends, setState: 1]; // Set checkmark
    
    let returns = create_menu_item("Returns", "R", sel!(showReturns:));
    let mixer = create_menu_item("Mixer", "M", sel!(showMixer:));
    let _: () = msg_send![mixer, setState: 1]; // Set checkmark
    
    let track_delay = create_menu_item("Track Delay", "", sel!(showTrackDelay:));
    let crossfader = create_menu_item("Crossfader", "", sel!(showCrossfader:));
    let _: () = msg_send![crossfader, setEnabled: false]; // Disable item
    
    let perf_impact = create_menu_item("Performance Impact", "", sel!(showPerformanceImpact:));
    let _: () = msg_send![perf_impact, setEnabled: false]; // Disable item
    
    // Automation and Zoom
    let auto_mode = create_menu_item("Automation Mode", "A", sel!(toggleAutomationMode:));
    
    let zoom_in = create_menu_item("Zoom In", "+", sel!(zoomIn:));
    let zoom_out = create_menu_item("Zoom Out", "-", sel!(zoomOut:));
    let zoom_time = create_menu_item("Zoom to Time Selection", "Z", sel!(zoomToTimeSelection:));
    let _: () = msg_send![zoom_time, setEnabled: false]; // Disable item
    
    let zoom_back = create_menu_item("Zoom Back from Time Selection", "X", sel!(zoomBack:));

    // Add all items to menu with separators
    view_menu.addItem_(full_screen);
    view_menu.addItem_(second_window);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(info_item);
    view_menu.addItem_(help_view);
    view_menu.addItem_(file_manager);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(toggle_view);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(detail_view);
    view_menu.addItem_(toggle_clip);
    view_menu.addItem_(expand_clip);
    view_menu.addItem_(arrange_panels);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(midi_editor);
    view_menu.addItem_(env_editor);
    view_menu.addItem_(note_exp_editor);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(browser);
    view_menu.addItem_(groove_pool);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(search_browser);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(plugin_windows);
    view_menu.addItem_(video_window);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(overview);
    view_menu.addItem_(in_out);
    view_menu.addItem_(sends);
    view_menu.addItem_(returns);
    view_menu.addItem_(mixer);
    view_menu.addItem_(track_delay);
    view_menu.addItem_(crossfader);
    view_menu.addItem_(perf_impact);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(auto_mode);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(zoom_in);
    view_menu.addItem_(zoom_out);
    view_menu.addItem_(NSMenuItem::separatorItem(nil));
    view_menu.addItem_(zoom_time);
    view_menu.addItem_(zoom_back);

    view_menu_item.setSubmenu_(view_menu);
    menubar.addItem_(view_menu_item);
}

#[cfg(target_os = "macos")]
unsafe fn create_options_menu(menubar: &id) {
    let options_menu_item = NSMenuItem::new(nil);
    let options_menu = NSMenu::new(nil);
    let _: () = msg_send![options_menu_item, setTitle: NSString::alloc(nil).init_str("Options")];
    let _: () = msg_send![options_menu, setTitle: NSString::alloc(nil).init_str("Options")];

    // MIDI and Key Mapping
    let edit_midi_map = create_menu_item("Edit MIDI Map", "M", sel!(editMIDIMap:));
    let edit_key_map = create_menu_item("Edit Key Map", "K", sel!(editKeyMap:));
    let computer_keyboard = create_menu_item("Computer MIDI Keyboard", "M", sel!(toggleComputerKeyboard:));

    // Sync and Latency
    let external_sync = create_menu_item("External Sync", "", sel!(toggleExternalSync:));
    let _: () = msg_send![external_sync, setEnabled: false];

    let delay_compensation = create_menu_item("Delay Compensation", "", sel!(toggleDelayCompensation:));
    let _: () = msg_send![delay_compensation, setState: 1];

    let reduced_latency = create_menu_item("Reduced Latency When Monitoring", "", sel!(toggleReducedLatency:));

    // Time Ruler Format submenu
    let time_ruler_item = NSMenuItem::new(nil);
    let time_ruler_menu = NSMenu::new(nil);
    let _: () = msg_send![time_ruler_item, setTitle: NSString::alloc(nil).init_str("Time Ruler Format")];
    time_ruler_item.setSubmenu_(time_ruler_menu);

    // Grid Options
    let narrow_grid = create_menu_item("Narrow Grid", "1", sel!(narrowGrid:));
    let widen_grid = create_menu_item("Widen Grid", "2", sel!(widenGrid:));
    let triplet_grid = create_menu_item("Triplet Grid", "3", sel!(tripletGrid:));
    let _: () = msg_send![triplet_grid, setEnabled: false];

    let snap_to_grid = create_menu_item("Snap to Grid", "4", sel!(toggleSnapToGrid:));
    let _: () = msg_send![snap_to_grid, setState: 1];

    let snap_automation = create_menu_item("Snap Automation to Grid", "", sel!(toggleSnapAutomation:));
    let _: () = msg_send![snap_automation, setState: 1];

    let fixed_grid = create_menu_item("Fixed Grid", "5", sel!(toggleFixedGrid:));

    // Drawing and Display
    let draw_mode = create_menu_item("Draw Mode", "B", sel!(toggleDrawMode:));
    let highlight_scales = create_menu_item("Highlight Scales", "K", sel!(toggleHighlightScales:));
    let _: () = msg_send![highlight_scales, setState: 1];

    let follow = create_menu_item("Follow", "F", sel!(toggleFollow:));
    let _: () = msg_send![follow, setState: 1];

    // MIDI Options
    let chase_notes = create_menu_item("Chase MIDI Notes", "", sel!(toggleChaseNotes:));
    let _: () = msg_send![chase_notes, setState: 1];

    let envelope_reset = create_menu_item("MIDI Envelope Auto-Reset", "", sel!(toggleEnvelopeReset:));
    let _: () = msg_send![envelope_reset, setState: 1];

    // Solo and Cue
    let solo_switches = create_menu_item("Solo Switches", "", sel!(toggleSoloSwitches:));
    let _: () = msg_send![solo_switches, setEnabled: false];
    let _: () = msg_send![solo_switches, setState: 1];

    let cue_switches = create_menu_item("Cue Switches", "", sel!(toggleCueSwitches:));
    let _: () = msg_send![cue_switches, setEnabled: false];

    let solo_in_place = create_menu_item("Solo in Place", "", sel!(toggleSoloInPlace:));
    let _: () = msg_send![solo_in_place, setState: 1];

    // Envelopes and Overdub
    let lock_envelopes = create_menu_item("Lock Envelopes", "", sel!(toggleLockEnvelopes:));
    let midi_overdub = create_menu_item("MIDI Arrangement Overdub", "", sel!(toggleMIDIOverdub:));

    // Audio Engine
    let audio_engine = create_menu_item("Audio Engine On", "E", sel!(toggleAudioEngine:));
    let _: () = msg_send![audio_engine, setState: 1];

    // Add all items to menu with separators
    options_menu.addItem_(edit_midi_map);
    options_menu.addItem_(edit_key_map);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(computer_keyboard);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(external_sync);
    options_menu.addItem_(delay_compensation);
    options_menu.addItem_(reduced_latency);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(time_ruler_item);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(narrow_grid);
    options_menu.addItem_(widen_grid);
    options_menu.addItem_(triplet_grid);
    options_menu.addItem_(snap_to_grid);
    options_menu.addItem_(snap_automation);
    options_menu.addItem_(fixed_grid);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(draw_mode);
    options_menu.addItem_(highlight_scales);
    options_menu.addItem_(follow);
    options_menu.addItem_(chase_notes);
    options_menu.addItem_(envelope_reset);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(solo_switches);
    options_menu.addItem_(cue_switches);
    options_menu.addItem_(solo_in_place);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(lock_envelopes);
    options_menu.addItem_(midi_overdub);
    options_menu.addItem_(NSMenuItem::separatorItem(nil));
    options_menu.addItem_(audio_engine);

    options_menu_item.setSubmenu_(options_menu);
    menubar.addItem_(options_menu_item);
}

#[cfg(target_os = "macos")]
unsafe fn create_help_menu(menubar: &id) {
    let help_menu_item = NSMenuItem::new(nil);
    let help_menu = NSMenu::new(nil);
    let _: () = msg_send![help_menu_item, setTitle: NSString::alloc(nil).init_str("Help")];
    let _: () = msg_send![help_menu, setTitle: NSString::alloc(nil).init_str("Help")];
    
    // Add Help menu items
    let lessons_item = create_menu_item("Built-In Lessons", "", sel!(showLessons:));
    let manual_item = create_menu_item("Read the Live Manual...", "", sel!(showManual:));
    let learn_item = create_menu_item("Learn Live at ableton.com...", "", sel!(learnLive:));
    let forum_item = create_menu_item("Join the User Forum...", "", sel!(joinForum:));
    let support_item = create_menu_item("Get Support", "", sel!(getSupport:));
    let account_item = create_menu_item("User Account and Licenses...", "", sel!(manageAccount:));
    let updates_item = create_menu_item("Check for Updates...", "", sel!(checkUpdates:));
    
    // Add all items to menu
    help_menu.addItem_(lessons_item);
    help_menu.addItem_(manual_item);
    help_menu.addItem_(learn_item);
    help_menu.addItem_(NSMenuItem::separatorItem(nil));
    help_menu.addItem_(forum_item);
    help_menu.addItem_(support_item);
    help_menu.addItem_(account_item);
    help_menu.addItem_(NSMenuItem::separatorItem(nil));
    help_menu.addItem_(updates_item);
    
    help_menu_item.setSubmenu_(help_menu);
    menubar.addItem_(help_menu_item);
}

// Individual menu creation functions will go here
// I'll continue with those in the next messages to keep this organized
